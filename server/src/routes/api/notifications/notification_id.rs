use axum::{Extension, Json, extract::Path};
use chrono::Utc;
use color_eyre::eyre::{Context, eyre};
use futures::TryStreamExt;
use mongodb::bson::{self, Bson, doc, oid::ObjectId};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::Notification,
    middlewares::{
        require_auth::{UnauthorizedError, UserId},
        require_org_permissions::ForbiddenError,
    },
    notifications::{
        Detailed, DetailedNotification, NotificationStatus, Simple, query_detailed_notifications,
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/notifications/{notification_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_notification, edit_notification_status),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get notification
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("notification_id" = String, Path, description = "Notification ID")
    ),
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

#[derive(Deserialize, ToSchema)]
pub struct EditNotificationBody {
    status: NotificationStatus,
}

/// Edit notification status
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("notification_id" = String, Path, description = "Notification ID")
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "Notifications"
)]
async fn edit_notification_status(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(notification_id): Path<ObjectId>,
    Json(body): Json<EditNotificationBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let notification = state
        .database
        .collection::<Notification<Simple>>("notifications")
        .find_one(doc! {
            "_id": notification_id,
        })
        .await
        .wrap_err("Failed to fetch notification")?
        .ok_or_else(|| AxumError::not_found(eyre!("Notification not found")))?;

    let has_permissions = notification.recipients.iter().any(|r| r.user == *user_id);
    if !has_permissions {
        return Err(AxumError::forbidden(eyre!(
            "You do not have sufficient permissions to perform this action"
        )));
    }

    let mut update = doc! {
        "recipients.$.status": body.status.clone(),
    };

    if body.status == NotificationStatus::Read {
        update.insert("recipients.$.read_at", Utc::now().to_string());
    } else if body.status == NotificationStatus::Unread {
        update.insert("recipients.$.read_at", Bson::Null);
    }

    state
        .database
        .collection::<Notification<Simple>>("notifications")
        .update_one(
            doc! { "_id": notification_id, "recipients.user": *user_id },
            doc! { "$set": update },
        )
        .await
        .wrap_err("Failed to update notification status")?;

    Ok(Json(CreateSuccess {
        id: notification_id.to_string(),
        success: true,
    }))
}
