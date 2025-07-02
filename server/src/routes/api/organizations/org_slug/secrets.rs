use axum::{Extension, Json};
use futures::TryStreamExt;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{PublicSecret, Secret},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgId},
    },
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/secrets";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_organization_secrets),
        RouteProtectionLevel::OrgViewer,
    )]
}

/// Get organization secrets
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<PublicSecret>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    )
)]
async fn get_organization_secrets(
    Extension(org_id): Extension<OrgId>,
    Extension(state): Extension<AppState>,
) -> AxumResult<Json<Vec<PublicSecret>>> {
    let collection = state.database.collection::<Secret>("secrets");

    let mut cursor = collection
        .find(doc! { "organization_id": org_id.0 })
        .await
        .map_err(AxumError::from)?;

    let mut secrets = Vec::new();
    while let Some(secret) = cursor.try_next().await.map_err(AxumError::from)? {
        secrets.push(secret.to_public());
    }

    Ok(Json(secrets))
}
