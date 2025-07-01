use axum::{Extension, Json};
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::Organization,
    middlewares::require_auth::UnauthorizedError,
    routes::RouteProtectionLevel,
};

use super::Route;

const PATH: &str = "/api/organizations/{org_id}";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_organization), RouteProtectionLevel::OrgViewer)]
}

/// Get organization
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Organization),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    )
)]
async fn get_organization(
    Extension(org): Extension<Organization>,
) -> AxumResult<Json<Organization>> {
    Ok(Json(org))
}
