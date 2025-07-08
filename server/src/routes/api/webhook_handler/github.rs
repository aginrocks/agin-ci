use axum::{Extension, Json, body::Bytes};
use color_eyre::eyre::{self, ContextCompat};
use http::HeaderMap;
use octocrab::models::webhook_events::WebhookEvent;
use tracing::info;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::{
        RouteProtectionLevel,
        api::webhook_handler::{
            WebhookHandlerSuccess,
            common::{get_repo_secret, verify_signature},
        },
    },
    state::AppState,
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
    request_body = String,
    responses(
        (status = OK, description = "Success", body = WebhookHandlerSuccess),
    ),
    tag = "Webhook Handlers"
)]
async fn github_webhook_handler(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> AxumResult<Json<WebhookHandlerSuccess>> {
    let header = headers
        .get("X-GitHub-Event")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            AxumError::bad_request(eyre::eyre!("Missing or invalid X-GitHub-Event header"))
        })?;

    let event = WebhookEvent::try_from_header_and_body(header, &body)?;

    let git_url = event
        .repository
        .wrap_err("Missing repository")?
        .ssh_url
        .wrap_err("Missing git URL in repository")?
        .to_string();

    let secret = get_repo_secret(&state.database, &git_url).await?;
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            AxumError::bad_request(eyre::eyre!("Missing or invalid X-Hub-Signature-256 header"))
        })?;

    verify_signature(secret.as_str(), signature, &body)?;

    info!("Received GitHub event: {:?}", event.kind);
    info!("REPO {git_url}");

    Ok(Json(WebhookHandlerSuccess { success: true }))
}
