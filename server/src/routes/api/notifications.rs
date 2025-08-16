use axum::{Extension, Json};
use futures::TryStreamExt;
use mongodb::bson::{Document, doc};
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::{UnauthorizedError, UserId},
    notifications::{
        Detailed, DetailedNotification, VecNotification, query_detailed_notifications,
    },
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
        (status = OK, description = "Success", body = VecNotification<Detailed>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Notifications"
)]
async fn get_notifications(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<Vec<DetailedNotification>>> {
    let pipeline = [
        vec![doc! {
            "$match": {
                "recipients.user": *user_id,
            },
        }],
        query_detailed_notifications(),
    ]
    .concat();

    let cursor = state
        .database
        .collection::<DetailedNotification>("notifications")
        .aggregate(pipeline)
        .await?;

    let documents: Vec<Document> = cursor.try_collect().await?;
    let notifications = documents
        .into_iter()
        .map(mongodb::bson::from_document)
        .collect::<Result<Vec<DetailedNotification>, _>>()?;

    Ok(Json(notifications))
}
