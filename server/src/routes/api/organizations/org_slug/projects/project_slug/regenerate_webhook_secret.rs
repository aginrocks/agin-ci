use axum::{
    Json,
    extract::{Path, State},
};
use color_eyre::eyre::{self};
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Project, PublicProject, fetch_project},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgDataMember},
    },
    routes::RouteProtectionLevel,
    state::AppState,
    utils::generate_webhook_secret,
};

use super::Route;

const PATH: &str =
    "/api/organizations/{org_slug}/projects/{project_slug}/regenerate-webhook-secret";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(regenerate_webhook_secret),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Serialize, ToSchema)]
struct RegenerateSecretResponse {
    webhook_secret: String,
}

/// Regenerate webhook secret
///
/// This secret is used to verify the authenticity of webhooks sent by the repository service. You won't be able to view it again after this call.
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug"),
        ("project_slug" = String, Path, description = "Project slug"),
    ),
    responses(
        (status = OK, description = "Success", body = PublicProject, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Project"
)]
async fn regenerate_webhook_secret(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, project_slug)): Path<(String, String)>,
) -> AxumResult<Json<RegenerateSecretResponse>> {
    let project = fetch_project(&state.database, org.id, project_slug).await?;

    if project.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Project not found")));
    }

    let project = project.unwrap();

    let webhook_secret = generate_webhook_secret();

    state
        .database
        .collection::<Project>("projects")
        .find_one_and_update(
            doc! { "_id": project.id },
            doc! {
                "$set": {
                    "repository.webhook_secret": &webhook_secret
                }
            },
        )
        .await?;

    Ok(Json(RegenerateSecretResponse { webhook_secret }))
}
