use axum::{Extension, Json, extract::Path};
use color_eyre::eyre::eyre;
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{self, Document, doc, oid::ObjectId};
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::Notification,
    middlewares::{
        require_auth::{UnauthorizedError, UserId},
        require_org_permissions::ForbiddenError,
    },
    notifications::{Detailed, DetailedNotification, query_detailed_notifications},
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/notifications/{notification_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_notification),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get notification
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Notification<Detailed>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Notifications"
)]
async fn get_notification(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(notification_id): Path<ObjectId>,
) -> AxumResult<Json<Notification<Detailed>>> {
    let pipeline = [
        vec![doc! {
            "$match": {
                "_id": notification_id,
            },
        }],
        query_detailed_notifications(),
    ]
    .concat();

    dbg!(&pipeline);

    let mut cursor = state
        .database
        .collection::<DetailedNotification>("notifications")
        .aggregate(pipeline)
        .await?;

    let document = cursor
        .try_next()
        .await?
        .ok_or_else(|| AxumError::not_found(eyre!("Notification not found")))?;

    let notification: DetailedNotification = bson::from_document(document)?;

    let has_permissions = notification.recipients.iter().any(|r| r.user == *user_id);
    if !has_permissions {
        return Err(AxumError::forbidden(eyre!(
            "You do not have sufficient permissions to perform this action"
        )));
    }

    Ok(Json(notification))
}
