mod user_id;

use axum::{Json, extract::State};
use futures::TryStreamExt;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::User,
    middlewares::{
        require_auth::UnauthorizedError, require_org_permissions::ForbiddenError,
        require_server_permissions::ServerAdmin,
    },
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/system/users";

pub fn routes() -> Vec<Route> {
    [
        vec![(
            routes!(get_system_users),
            RouteProtectionLevel::Authenticated,
        )],
        user_id::routes(),
    ]
    .concat()
}

/// Get all users
///
/// Returns every user in the system.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<User>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "System"
)]
async fn get_system_users(
    State(state): State<AppState>,
    _: ServerAdmin,
) -> AxumResult<Json<Vec<User>>> {
    let users = state
        .database
        .collection::<User>("users")
        .find(doc! {})
        .await?;

    let users = users.try_collect().await?;

    Ok(Json(users))
}
