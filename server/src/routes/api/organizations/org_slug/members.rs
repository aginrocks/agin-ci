pub mod member_id;

use axum::{Extension, Json};
use color_eyre::eyre;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Membership, Organization, OrganizationRole},
    middlewares::{
        require_auth::{UnauthorizedError, UserId},
        require_org_permissions::{ForbiddenError, OrgData, OrgId},
    },
    mongo_id::object_id_as_string_required,
    routes::{
        RouteProtectionLevel,
        api::{CreateSuccess, organizations::org_slug::members::member_id::get_membership_details},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/members";

pub fn routes() -> Vec<Route> {
    [
        vec![
            (
                routes!(get_organization_members),
                RouteProtectionLevel::OrgViewer,
            ),
            (
                routes!(add_organization_member),
                RouteProtectionLevel::OrgAdmin,
            ),
        ],
        member_id::routes(),
    ]
    .concat()
}

#[derive(Serialize, Deserialize, ToSchema)]
struct Member {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    _id: ObjectId,
    role: OrganizationRole,
    email: String,
    name: String,
}

/// Get org members
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    responses(
        (status = OK, description = "Success", body = Vec<Member>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn get_organization_members(
    Extension(state): Extension<AppState>,
    Extension(org_id): Extension<OrgId>,
) -> AxumResult<Json<Vec<Member>>> {
    let pipeline = vec![
        doc! { "$match": { "_id": org_id.0 } },
        doc! { "$unwind": "$members" },
        doc! {
            "$lookup": {
                "from": "users",
                "localField": "members.user_id",
                "foreignField": "_id",
                "as": "user"
            }
        },
        doc! { "$unwind": "$user" },
        doc! {
            "$project": {
                "_id": "$members.user_id",
                "role": "$members.role",
                "email": "$user.email",
                "name": "$user.name"
            }
        },
    ];

    let cursor = state
        .database
        .collection::<Organization>("organizations")
        .aggregate(pipeline)
        .await?;

    let documents: Vec<mongodb::bson::Document> = cursor.try_collect().await?;
    let results: Vec<Member> = documents
        .into_iter()
        .map(mongodb::bson::from_document)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(results))
}

/// Add org member
#[utoipa::path(
    method(put),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    request_body = Membership,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn add_organization_member(
    Extension(state): Extension<AppState>,
    Extension(org_id): Extension<OrgId>,
    Extension(org): Extension<OrgData>,
    Extension(user_id): Extension<UserId>,
    Json(body): Json<Membership>,
) -> AxumResult<Json<CreateSuccess>> {
    let membership = get_membership_details(&org.0, body.user_id);

    if membership.is_ok() {
        return Err(AxumError::bad_request(eyre::eyre!(
            "User is already a member of the organization."
        )));
    }

    let your_membership = get_membership_details(&org.0, user_id.0)?;

    if body.role > your_membership.role {
        return Err(AxumError::forbidden(eyre::eyre!(
            "You cannot assign a role highier than your own."
        )));
    }

    if body.role == OrganizationRole::Owner {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Cannot add an owner to the organization."
        )));
    }

    state
        .database
        .collection::<Organization>("organizations")
        .update_one(
            doc! { "_id": org_id.0 },
            doc! {
                "$push": {
                    "members": {
                        "user_id": body.user_id,
                        "role": body.role,
                    }
                }
            },
        )
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: org_id.0.to_string(),
    }))
}
