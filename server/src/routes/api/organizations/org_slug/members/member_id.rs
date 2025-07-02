use axum::{Extension, Json, extract::Path, response::IntoResponse};
use color_eyre::eyre;
use http::StatusCode;
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
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/members/{member_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(edit_organization_member, delete_organization_member),
        RouteProtectionLevel::OrgAdmin,
    )]
}

pub fn get_membership_details(org: &Organization, member_id: ObjectId) -> AxumResult<Membership> {
    let member = org.members.iter().find(|m| m.user_id == member_id);
    if member.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Member not found")));
    }
    Ok(member.unwrap().clone())
}

/// Delete organization member
#[utoipa::path(
    method(delete),
    path = PATH,
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn delete_organization_member(
    Extension(state): Extension<AppState>,
    Extension(org): Extension<OrgData>,
    Extension(org_id): Extension<OrgId>,
    Extension(user_id): Extension<UserId>,
    Path((_org_slug, member_id)): Path<(String, String)>,
) -> AxumResult<impl IntoResponse> {
    let member_id = ObjectId::parse_str(&member_id)?;

    if user_id.0 == member_id {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Cannot remove yourself from the organization. Use the leave organization endpoint instead."
        )));
    }

    let membership = get_membership_details(&org.0, member_id)?;
    if membership.role == OrganizationRole::Owner {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Cannot remove organization owner."
        )));
    }

    state
        .database
        .collection::<Organization>("organizations")
        .update_one(
            doc! { "_id": org_id.0, "members.user_id": member_id },
            doc! { "$pull": { "members": { "user_id": member_id } } },
        )
        .await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

#[derive(Serialize, Deserialize, ToSchema)]
struct EditRoleBody {
    role: OrganizationRole,
}

/// Edit organization member's role
#[utoipa::path(
    method(patch),
    path = PATH,
    request_body = EditRoleBody,
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn edit_organization_member(
    Extension(state): Extension<AppState>,
    Extension(org): Extension<OrgData>,
    Extension(org_id): Extension<OrgId>,
    Extension(user_id): Extension<UserId>,
    Path((_org_slug, member_id)): Path<(String, String)>,
    body: Json<EditRoleBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let member_id = ObjectId::parse_str(&member_id)?;

    let membership = get_membership_details(&org.0, member_id)?;
    if membership.role == OrganizationRole::Owner {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Cannot edit organization owner."
        )));
    }

    let your_membership = get_membership_details(&org.0, user_id.0)?;

    if body.role > your_membership.role {
        return Err(AxumError::forbidden(eyre::eyre!(
            "You cannot assign a role highier than your own."
        )));
    }

    if body.role == OrganizationRole::Owner && your_membership.role == OrganizationRole::Owner {
        // Demote the current owner to admin

        state
            .database
            .collection::<Organization>("organizations")
            .update_one(
                doc! { "_id": org_id.0, "members.user_id": user_id.0 },
                doc! {
                    "$set": {
                        "members.$.role": "admin",
                    }
                },
            )
            .await?;
    }

    state
        .database
        .collection::<Organization>("organizations")
        .update_one(
            doc! { "_id": org_id.0, "members.user_id": member_id },
            doc! {
                "$set": {
                    "members.$.role": mongodb::bson::to_bson(&body.role)?,
                }
            },
        )
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: member_id.to_string(),
    }))
}
