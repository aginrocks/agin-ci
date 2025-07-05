use axum::response::IntoResponse;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/login";

pub fn routes() -> Vec<Route> {
    vec![(routes!(log_in), RouteProtectionLevel::Redirect)]
}

/// Log in
#[utoipa::path(
    method(get),
    path = PATH,
    tag = "Auth"
)]
async fn log_in() -> impl IntoResponse {
    axum::response::Redirect::to("/app")
}
