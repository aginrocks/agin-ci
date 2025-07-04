mod secret_id;

use axum::{Json, extract::State};
use color_eyre::eyre::{self, ContextCompat};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{PartialSecret, PublicSecret, Secret, SecretScope},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgDataMember, OrgDataViewer},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/secrets";

pub fn routes() -> Vec<Route> {
    [
        vec![(
            routes!(get_organization_secrets, create_organization_secret),
            RouteProtectionLevel::Authenticated,
        )],
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
    org: OrgDataViewer,
    State(state): State<AppState>,
) -> AxumResult<Json<Vec<PublicSecret>>> {
    let collection = state.database.collection::<Secret>("secrets");

    let mut cursor = collection
        .find(doc! { "organization_id": org.id, "scope": SecretScope::Organization })
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
    org: OrgDataMember,
    State(state): State<AppState>,
    Json(body): Json<CreateOrgSecretBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let existing_secret = state
        .database
        .collection::<Secret>("secrets")
        .find_one(doc! {
            "organization_id": org.id,
            "name": &body.name,
            "scope": SecretScope::Organization,
        })
        .await?;

    if existing_secret.is_some() {
        return Err(AxumError::conflict(eyre::eyre!(
            "Secret with this name already exists in the organization."
        )));
    }

    let new_secret = PartialSecret {
        organization_id: org.id,
        name: body.name,
        secret: body.secret,
        scope: SecretScope::Organization,
        project_id: None,
    };

    let inserted = state
        .database
        .collection("secrets")
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
