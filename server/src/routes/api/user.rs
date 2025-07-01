use axum::{Extension, Json, response::IntoResponse};
use axum_oidc::OidcClaims;
use serde_json::json;
use utoipa_axum::routes;

use crate::{GroupClaims, middlewares::require_auth::UserData, routes::RouteProtectionLevel};

use super::Route;

const PATH: &str = "/api/user";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_user), RouteProtectionLevel::Authenticated)]
}

/// Get user details
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    )
)]
async fn get_user(Extension(user): Extension<UserData>) -> impl IntoResponse {
    Json(json!({
        "id": user.0
        // "id": claims.subject().to_string(),
    }))
}
