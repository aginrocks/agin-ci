mod access_token;
mod regenerate_webhook_secret;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use axum_valid::Valid;
use color_eyre::eyre;
use http::StatusCode;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{PartialProject, Project, ProjectRepository, PublicProject, Secret, fetch_project},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgDataMember, OrgDataViewer},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
    utils::normalize_git_url,
};

use super::{CreateProjectBody, Route};

const PATH: &str = "/api/organizations/{org_slug}/projects/{project_slug}";

pub fn routes() -> Vec<Route> {
    [
        vec![(
            routes!(get_project, edit_project, delete_project),
            RouteProtectionLevel::Authenticated,
        )],
        regenerate_webhook_secret::routes(),
        access_token::routes(),
    ]
    .concat()
}

/// Get project
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
async fn get_project(
    org: OrgDataViewer,
    State(state): State<AppState>,
    Path((_org_slug, project_slug)): Path<(String, String)>,
) -> AxumResult<Json<PublicProject>> {
    let project = fetch_project(&state.database, org.id, project_slug).await?;

    if project.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Project not found")));
    }

    let project = project.unwrap();

    let safe_project = project.to_public();

    Ok(Json(safe_project))
}

/// Edit project
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug"),
        ("project_slug" = String, Path, description = "Project slug"),
    ),
    request_body = CreateProjectBody,
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Project"
)]
async fn edit_project(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, project_slug)): Path<(String, String)>,
    Valid(Json(body)): Valid<Json<CreateProjectBody>>,
) -> AxumResult<Json<CreateSuccess>> {
    if body.slug != project_slug {
        let already_exists = fetch_project(&state.database, org.id, body.slug.clone()).await?;

        if already_exists.is_some() {
            return Err(AxumError::forbidden(eyre::eyre!(
                "Project with this slug already exists in the organization"
            )));
        }
    }

    let project = PartialProject {
        organization_id: org.id,
        name: body.name,
        slug: body.slug,
        repository: ProjectRepository {
            url: normalize_git_url(&body.repository.url)?,
            source: body.repository.source,
            webhook_secret: None,
            access_token: None,
        },
    };

    let updated = state
        .database
        .collection::<Project>("projects")
        .find_one_and_update(
            doc! {
                "organization_id": org.id,
                "slug": project_slug,
            },
            doc! {
                "$set": {
                    "name": &project.name,
                    "slug": &project.slug,
                    "repository.url": &project.repository.url,
                    "repository.source": &project.repository.source,
                }
            },
        )
        .await?;

    if updated.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Project not found")));
    }

    let id = updated.unwrap().id.to_string();

    Ok(Json(CreateSuccess { success: true, id }))
}

/// Delete project
#[utoipa::path(
    method(delete),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug"),
        ("project_slug" = String, Path, description = "Project slug"),
    ),
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Project"
)]
async fn delete_project(
    org: OrgDataMember,
    State(state): State<AppState>,
    Path((_org_slug, project_slug)): Path<(String, String)>,
) -> AxumResult<impl IntoResponse> {
    let result = state
        .database
        .collection::<Project>("projects")
        .find_one_and_delete(doc! {
            "organization_id": org.id,
            "slug": project_slug,
        })
        .await?;

    if result.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Project not found")));
    }

    let result = result.unwrap();

    state
        .database
        .collection::<Secret>("secrets")
        .delete_many(doc! { "project_id": result.id })
        .await?;

    Ok((StatusCode::NO_CONTENT, ()))
}
