use axum::{Extension, Json};
use utoipa_axum::routes;

use crate::{
    database::User,
    middlewares::require_auth::{UnauthorizedError, UserData},
    routes::RouteProtectionLevel,
};

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
        (status = OK, description = "Success", body = User),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    )
)]
async fn get_user(Extension(user): Extension<UserData>) -> Json<User> {
    Json(user.0)
}
