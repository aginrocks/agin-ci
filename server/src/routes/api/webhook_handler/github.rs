use axum::Json;
use color_eyre::eyre::{self};
use http::HeaderMap;
use octocrab::models::webhook_events::WebhookEvent;
use tracing::info;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::{RouteProtectionLevel, api::webhook_handler::WebhookHandlerSuccess},
};

use super::Route;

const PATH: &str = "/api/webhook-handler/github";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(github_webhook_handler),
        RouteProtectionLevel::Public,
    )]
}

/// GitHub Webhook handler
///
/// Handles incoming GitHub webhooks
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = WebhookHandlerSuccess),
    ),
    tag = "Webhook Handlers"
)]
async fn github_webhook_handler(
    headers: HeaderMap,
    body: String,
) -> AxumResult<Json<WebhookHandlerSuccess>> {
    let header = headers
        .get("X-GitHub-Event")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            AxumError::bad_request(eyre::eyre!("Missing or invalid X-GitHub-Event header"))
        })?;

    let event = WebhookEvent::try_from_header_and_body(header, &body)?;
    // .map_err(|_| AxumError::bad_request(eyre::eyre!("Invalid webhook body")))?;

    info!("Received GitHub event: {:?}", event.kind);

    Ok(Json(WebhookHandlerSuccess { success: true }))
}
