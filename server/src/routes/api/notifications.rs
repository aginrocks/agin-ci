use axum::{Extension, Json};
use color_eyre::eyre::Context;
use futures::TryStreamExt;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::Notification,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/notifications";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_notifications),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get notifications
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<Notification>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Notifications"
)]
async fn get_notifications(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<Vec<Notification>>> {
    let cursor = state
        .database
        .collection::<Notification>("notifications")
        .find(doc! {
            "recipients.user": *user_id,
        })
        .sort(doc! { "created_at": -1 })
        .await
        .wrap_err("Failed to fetch notifications")?;

    let notifications: Vec<_> = cursor.try_collect().await?;

    Ok(Json(notifications))
}
