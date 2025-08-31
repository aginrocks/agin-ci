use axum::{
    Json,
    extract::{Path, State},
};
use color_eyre::eyre::{self};
use mongodb::bson::doc;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Project, fetch_project},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgDataMember},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/projects/{project_slug}/access-token";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(set_access_token),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Deserialize, ToSchema)]
struct SetAccessTokenBody {
    access_token: String,
}

/// Set access token
///
/// Set access token for the repository. For now this is the only avalibale option. Later, Agin CI will be directly integrated with the GitHub API.
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug"),
        ("project_slug" = String, Path, description = "Project slug"),
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Project"
)]
async fn set_access_token(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, project_slug)): Path<(String, String)>,
    Json(body): Json<SetAccessTokenBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let project = fetch_project(&state.database, org.id, project_slug).await?;

    if project.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Project not found")));
    }

    let project = project.unwrap();

    state
        .database
        .collection::<Project>("projects")
        .find_one_and_update(
            doc! { "_id": project.id },
            doc! {
                "$set": {
                    "repository.access_token": body.access_token,
                }
            },
        )
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: project.id.to_string(),
    }))
}
