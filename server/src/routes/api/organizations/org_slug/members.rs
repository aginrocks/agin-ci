use axum::{Extension, Json};
use color_eyre::eyre;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{MutableOrganization, Organization, OrganizationRole, User},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgData, OrgId},
    },
    mongo_id::object_id_as_string_required,
    routes::{
        RouteProtectionLevel,
        api::{CreateSuccess, organizations::org_slug},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/members";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_organization_members),
        RouteProtectionLevel::OrgViewer,
    )]
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

/// Get organization members
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<Member>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    )
)]
async fn get_organization_members(
    Extension(org): Extension<OrgData>,
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
                "_id": 1,
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
