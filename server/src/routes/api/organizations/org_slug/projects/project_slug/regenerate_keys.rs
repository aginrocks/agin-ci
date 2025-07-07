use axum::{
    Json,
    extract::{Path, State},
};
use color_eyre::eyre::{self, Context};
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
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/projects/{project_slug}/regenerate-keys";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(regenerate_project_keys),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Serialize, ToSchema)]
struct RegenerateKeysResponse {
    deploy_public_key: String,
}

/// Regenerate project deploy keys
///
/// These keys are used to pull the repository. You can get the public key from the project details.
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
async fn regenerate_project_keys(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, project_slug)): Path<(String, String)>,
) -> AxumResult<Json<RegenerateKeysResponse>> {
    let project = fetch_project(&state.database, org.id, project_slug).await?;

    if project.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Project not found")));
    }

    let project = project.unwrap();

    let (public_key, private_key) = project
        .repository
        .generate_deploy_keys()
        .wrap_err("Failed to generate keys")?;

    state
        .database
        .collection::<Project>("projects")
        .find_one_and_update(
            doc! { "_id": project.id },
            doc! {
                "$set": {
                    "repository.deploy_public_key": &public_key,
                    "repository.deploy_private_key": private_key.to_string(),
                }
            },
        )
        .await?;

    Ok(Json(RegenerateKeysResponse {
        deploy_public_key: public_key,
    }))
}
