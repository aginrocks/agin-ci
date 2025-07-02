mod members;
mod projects;
mod secrets;

use axum::{Extension, Json, response::IntoResponse};
use axum_valid::Valid;
use color_eyre::eyre;
use futures::TryStreamExt;
use http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId};
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{MutableOrganization, Organization, Project, Secret},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{ForbiddenError, OrgData, OrgId},
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_slug}";

pub fn routes() -> Vec<Route> {
    [
        vec![
            (routes!(get_organization), RouteProtectionLevel::OrgViewer),
            (routes!(edit_organization), RouteProtectionLevel::OrgAdmin),
            (routes!(delete_organization), RouteProtectionLevel::OrgOwner),
        ],
        members::routes(),
        secrets::routes(),
        projects::routes(),
    ]
    .concat()
}

/// Get org
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    responses(
        (status = OK, description = "Success", body = Organization),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn get_organization(Extension(org): Extension<OrgData>) -> AxumResult<Json<Organization>> {
    Ok(Json(org.0))
}

/// Edit org
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    request_body = MutableOrganization,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn edit_organization(
    Extension(org_id): Extension<OrgId>,
    Extension(org): Extension<OrgData>,
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<MutableOrganization>>,
) -> AxumResult<Json<CreateSuccess>> {
    if org.0.slug != body.slug {
        let already_exists = state
            .database
            .collection::<Organization>("organizations")
            .find_one(doc! { "slug": &body.slug })
            .await?;

        if already_exists.is_some() {
            return Err(AxumError::forbidden(eyre::eyre!(
                "Organization with this slug already exists"
            )));
        }
    }

    let _updated = state
        .database
        .collection::<Organization>("organizations")
        .update_one(
            doc! { "_id": org_id.0 },
            doc! {
                "$set": {
                    "name": body.name,
                    "slug": body.slug,
                    "description": body.description
                }
            },
        )
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: org_id.0.to_string(),
    }))
}

/// Delete org
///
/// Dangerous!
#[utoipa::path(
    method(delete),
    path = PATH,
    params(
        ("org_slug" = String, Path, description = "Organization slug")
    ),
    responses(
        (status = NO_CONTENT, description = "Success"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Organization"
)]
async fn delete_organization(
    Extension(org_id): Extension<OrgId>,
    Extension(state): Extension<AppState>,
) -> AxumResult<impl IntoResponse> {
    let projects = state.database.collection::<Project>("projects");
    let secrets = state.database.collection::<Secret>("secrets");
    let organizations = state.database.collection::<Organization>("organizations");

    // Step 1: Find all project IDs belonging to the organization
    let projects_cursor = projects.find(doc! { "organization_id": org_id.0 }).await?;

    let project_ids: Vec<ObjectId> = projects_cursor
        .try_collect::<Vec<Project>>()
        .await?
        .into_iter()
        .filter_map(|p| p.id)
        .collect();

    // Step 2: Delete the projects
    projects
        .delete_many(doc! { "organization_id": org_id.0 })
        .await?;

    // Step 3: Delete associated secrets (by organization_id or project_id in project_ids)
    secrets
        .delete_many(doc! {
            "$or": [
                { "organization_id": org_id.0 },
                { "project_id": { "$in": project_ids } }
            ]
        })
        .await?;

    // Step 4: Delete the organization itself
    organizations.delete_one(doc! { "_id": org_id.0 }).await?;

    Ok((StatusCode::NO_CONTENT, ()))
}
