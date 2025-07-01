mod members;

use axum::{Extension, Json};
use color_eyre::eyre;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{MutableOrganization, Organization},
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
        ],
        members::routes(),
    ]
    .concat()
}

/// Get organization
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Organization),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    )
)]
async fn get_organization(Extension(org): Extension<OrgData>) -> AxumResult<Json<Organization>> {
    Ok(Json(org.0))
}

/// Edit organization
#[utoipa::path(
    method(patch),
    path = PATH,
    request_body = MutableOrganization,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    )
)]
async fn edit_organization(
    Extension(org_id): Extension<OrgId>,
    Extension(org): Extension<OrgData>,
    Extension(state): Extension<AppState>,
    Json(body): Json<MutableOrganization>,
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
