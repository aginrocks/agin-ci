use axum::{
    Extension,
    extract::{Path, Request},
    middleware::Next,
    response::Response,
};
use color_eyre::eyre::{self, ContextCompat};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Organization, OrganizationRole},
    middlewares::require_auth::UserData,
    state::AppState,
};

/// Organization data type for request extensions
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrgData(pub Organization);

/// Middleware that ensures the user has sufficient permissions for the organization
pub async fn require_org_permissions(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserData>,
    Path(org_id): Path<String>,
    mut request: Request,
    next: Next,
    required_role: OrganizationRole,
) -> AxumResult<Response> {
    let user_id = user.0.id.wrap_err("User not found")?;

    let org_id = ObjectId::parse_str(&org_id)
        .map_err(|_| AxumError::bad_request(eyre::eyre!("Invalid organization ID")))?;

    let org = state
        .database
        .collection::<Organization>("organizations")
        .find_one(doc! {
            "_id": org_id,
        })
        .await?;

    if org.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Organization not found")));
    }

    let org = org.unwrap();

    let membership = org.members.iter().find(|m| m.user_id == user_id);

    if membership.is_none() {
        return Err(AxumError::forbidden(eyre::eyre!(
            "You do not have sufficient permissions to perform this action"
        )));
    }

    let membership = membership.unwrap();
    let has_access = membership.role >= required_role;

    if !has_access {
        return Err(AxumError::forbidden(eyre::eyre!(
            "You do not have sufficient permissions to perform this action"
        )));
    }

    request.extensions_mut().insert(OrgData(org));

    Ok(next.run(request).await)
}

pub async fn require_org_permissions_viewer(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserData>,
    Path(org_id): Path<String>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    require_org_permissions(
        Extension(state),
        Extension(user),
        Path(org_id),
        request,
        next,
        OrganizationRole::Viewer,
    )
    .await
}

pub async fn require_org_permissions_member(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserData>,
    Path(org_id): Path<String>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    require_org_permissions(
        Extension(state),
        Extension(user),
        Path(org_id),
        request,
        next,
        OrganizationRole::Member,
    )
    .await
}

pub async fn require_org_permissions_admin(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserData>,
    Path(org_id): Path<String>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    require_org_permissions(
        Extension(state),
        Extension(user),
        Path(org_id),
        request,
        next,
        OrganizationRole::Admin,
    )
    .await
}

pub async fn require_org_permissions_owner(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserData>,
    Path(org_id): Path<String>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    require_org_permissions(
        Extension(state),
        Extension(user),
        Path(org_id),
        request,
        next,
        OrganizationRole::Owner,
    )
    .await
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "You do not have sufficient permissions to perform this action"}))]
pub struct ForbiddenError {
    error: String,
}
