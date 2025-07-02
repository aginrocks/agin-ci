mod secret_id;

use axum::{Extension, Json};
use color_eyre::eyre::{self, ContextCompat};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{PublicSecret, Secret, SecretScope},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgId},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/secrets";

pub fn routes() -> Vec<Route> {
    [
        vec![
            (
                routes!(get_organization_secrets),
                RouteProtectionLevel::OrgViewer,
            ),
            (
                routes!(create_organization_secret),
                RouteProtectionLevel::OrgMember,
            ),
        ],
        secret_id::routes(),
    ]
    .concat()
}

/// Get org secrets
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    responses(
        (status = OK, description = "Success", body = Vec<PublicSecret>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Secrets"
)]
async fn get_organization_secrets(
    Extension(org_id): Extension<OrgId>,
    Extension(state): Extension<AppState>,
) -> AxumResult<Json<Vec<PublicSecret>>> {
    let collection = state.database.collection::<Secret>("secrets");

    let mut cursor = collection
        .find(doc! { "organization_id": org_id.0, "scope": SecretScope::Organization })
        .await?;

    let mut secrets = Vec::new();
    while let Some(secret) = cursor.try_next().await? {
        secrets.push(secret.to_public());
    }

    Ok(Json(secrets))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateOrgSecretBody {
    pub name: String,
    pub secret: String,
}

/// Create org secret
#[utoipa::path(
    method(post),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    request_body = CreateOrgSecretBody,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Secrets"
)]
async fn create_organization_secret(
    Extension(org_id): Extension<OrgId>,
    Extension(state): Extension<AppState>,
    Json(body): Json<CreateOrgSecretBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let existing_secret = state
        .database
        .collection::<Secret>("secrets")
        .find_one(doc! {
            "organization_id": org_id.0,
            "name": &body.name,
            "scope": SecretScope::Organization,
        })
        .await?;

    if existing_secret.is_some() {
        return Err(AxumError::conflict(eyre::eyre!(
            "Secret with this name already exists in the organization."
        )));
    }

    let new_secret = Secret {
        id: None,
        organization_id: org_id.0,
        name: body.name,
        secret: body.secret,
        scope: SecretScope::Organization,
        project_id: None,
    };

    let inserted = state
        .database
        .collection::<Secret>("secrets")
        .insert_one(new_secret)
        .await
        .map_err(AxumError::from)?;

    let id = inserted
        .inserted_id
        .as_object_id()
        .wrap_err("Failed to fetch secret ID")?
        .to_string();

    Ok(Json(CreateSuccess { success: true, id }))
}
