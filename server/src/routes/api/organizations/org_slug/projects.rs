mod project_slug;

use axum::{Json, extract::State};
use axum_valid::Valid;
use color_eyre::eyre::{self, ContextCompat};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{
        PartialProject, Project, ProjectRepository, ProjectRepositorySource, PublicProject,
        fetch_project,
    },
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgDataMember, OrgDataViewer},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
    utils::normalize_git_url,
    validators::slug_validator,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}/projects";

pub fn routes() -> Vec<Route> {
    [
        vec![(
            routes!(get_projects, create_project),
            RouteProtectionLevel::Authenticated,
        )],
        project_slug::routes(),
    ]
    .concat()
}

/// Get projects
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    responses(
        (status = OK, description = "Success", body = Vec<PublicProject>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Projects"
)]
async fn get_projects(
    org: OrgDataViewer,
    State(state): State<AppState>,
) -> AxumResult<Json<Vec<PublicProject>>> {
    let cursor = state
        .database
        .collection::<Project>("projects")
        .find(doc! {
            "organization_id": org.id,
        })
        .await?;

    let projects: Vec<Project> = cursor.try_collect().await?;

    let safe_projects: Vec<PublicProject> = projects.iter().map(|p| p.to_public()).collect();

    Ok(Json(safe_projects))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateProjectBodyRepository {
    pub url: String,
    pub source: ProjectRepositorySource,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateProjectBody {
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[validate(custom(function = "slug_validator"), length(min = 1, max = 32))]
    pub slug: String,

    pub repository: CreateProjectBodyRepository,
}

/// Create project
#[utoipa::path(
    method(post),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    request_body = CreateProjectBody,
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Projects"
)]
async fn create_project(
    org: OrgDataMember,
    State(state): State<AppState>,
    Valid(Json(body)): Valid<Json<CreateProjectBody>>,
) -> AxumResult<Json<CreateSuccess>> {
    let already_exists = fetch_project(&state.database, org.id, body.slug.clone()).await?;

    if already_exists.is_some() {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Project with this slug already exists in the organization"
        )));
    }

    let project = PartialProject {
        organization_id: org.id,
        name: body.name,
        slug: body.slug,
        repository: ProjectRepository {
            url: normalize_git_url(&body.repository.url)?,
            source: body.repository.source,
            webhook_secret: None,
            deploy_key: None,
        },
    };

    let inserted = state
        .database
        .collection("projects")
        .insert_one(project)
        .await?;

    let id = inserted
        .inserted_id
        .as_object_id()
        .wrap_err("Failed to fetch project ID")?
        .to_string();

    Ok(Json(CreateSuccess { success: true, id }))
}
