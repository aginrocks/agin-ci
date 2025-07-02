use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/health";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_health), RouteProtectionLevel::Public)]
}

/// Get health of the service (returns "ok")
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    ),
    tag = "Other"
)]
async fn get_health() -> &'static str {
    "ok"
}
