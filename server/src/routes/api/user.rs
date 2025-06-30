use axum::{Json, response::IntoResponse};
use axum_oidc::OidcClaims;
use serde_json::json;
use tower_sessions::Session;
use utoipa_axum::routes;

use crate::{GroupClaims, routes::RouteProtectionLevel};

use super::Route;

const PATH: &str = "/api/user";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_user), RouteProtectionLevel::Public)]
}

/// Get user details
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    )
)]
async fn get_user(claims: Option<OidcClaims<GroupClaims>>) -> impl IntoResponse {
    let iss = match claims {
        Some(claims) => claims.issuer().to_string(),
        None => "".to_string(),
    };

    Json(json!({
        "id": iss
        // "id": claims.subject().to_string(),
    }))
}
