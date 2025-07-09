use axum::{Extension, Json, body::Bytes, extract::Path};
use color_eyre::eyre::{self, ContextCompat};
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::{
        RouteProtectionLevel,
        api::webhooks::{
            WebhookHandlerSuccess,
            common::{get_project_secret, verify_repostiory, verify_signature},
        },
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/webhooks/{project_id}/gitea";

pub fn routes() -> Vec<Route> {
    vec![(routes!(gitea_webhook_handler), RouteProtectionLevel::Public)]
}

/// Forgejo or Gitea Webhook handler
///
/// Handles incoming Forgejo or Gitea webhooks
#[utoipa::path(
    method(post),
    path = PATH,
    params(
        ("project_id" = String, Path, description = "Project ID")
    ),
    request_body = String,
    responses(
        (status = OK, description = "Success", body = WebhookHandlerSuccess),
    ),
    tag = "Webhook Handlers"
)]
async fn gitea_webhook_handler(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path(project_id): Path<String>,
    body: Bytes,
) -> AxumResult<Json<WebhookHandlerSuccess>> {
    let (project, secret) = get_project_secret(&state.database, &project_id).await?;
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            AxumError::bad_request(eyre::eyre!("Missing or invalid X-Hub-Signature-256 header"))
        })?;

    verify_signature(secret.as_str(), signature, &body)?;

    let event = serde_json::from_slice::<WebhookPayload>(&body)?;

    let git_url = event
        .repository
        .wrap_err("Missing repository")?
        .ssh_url
        .wrap_err("Missing git URL in repository")?
        .to_string();

    verify_repostiory(&git_url, &project.repository.url)?;

    info!("Received Forgejo event");
    // info!("REPO {git_url}");

    Ok(Json(WebhookHandlerSuccess { success: true }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub r#ref: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub compare_url: Option<String>,
    pub commits: Option<Vec<Commit>>,
    pub total_commits: Option<u32>,
    pub head_commit: Option<Commit>,
    pub repository: Option<Repository>,
    pub pusher: Option<User>,
    pub sender: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: Option<String>,
    pub message: Option<String>,
    pub url: Option<String>,
    pub author: Option<CommitUser>,
    pub committer: Option<CommitUser>,
    pub verification: Option<Verification>,
    pub timestamp: Option<String>,
    pub added: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
    pub modified: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    // Add fields if needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: Option<u64>,
    pub owner: Option<User>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub description: Option<String>,
    pub empty: Option<bool>,
    pub private: Option<bool>,
    pub fork: Option<bool>,
    pub template: Option<bool>,
    pub parent: Option<Box<Repository>>,
    pub mirror: Option<bool>,
    pub size: Option<u64>,
    pub language: Option<String>,
    pub languages_url: Option<String>,
    pub html_url: Option<String>,
    pub url: Option<String>,
    pub link: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub original_url: Option<String>,
    pub website: Option<String>,
    pub stars_count: Option<u64>,
    pub forks_count: Option<u64>,
    pub watchers_count: Option<u64>,
    pub open_issues_count: Option<u64>,
    pub open_pr_counter: Option<u64>,
    pub release_counter: Option<u64>,
    pub default_branch: Option<String>,
    pub archived: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub archived_at: Option<String>,
    pub permissions: Option<Permissions>,
    pub has_issues: Option<bool>,
    pub internal_tracker: Option<InternalTracker>,
    pub has_wiki: Option<bool>,
    pub wiki_branch: Option<String>,
    pub globally_editable_wiki: Option<bool>,
    pub has_pull_requests: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_releases: Option<bool>,
    pub has_packages: Option<bool>,
    pub has_actions: Option<bool>,
    pub ignore_whitespace_conflicts: Option<bool>,
    pub allow_merge_commits: Option<bool>,
    pub allow_rebase: Option<bool>,
    pub allow_rebase_explicit: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_fast_forward_only_merge: Option<bool>,
    pub allow_rebase_update: Option<bool>,
    pub default_delete_branch_after_merge: Option<bool>,
    pub default_merge_style: Option<String>,
    pub default_allow_maintainer_edit: Option<bool>,
    pub default_update_style: Option<String>,
    pub avatar_url: Option<String>,
    pub internal: Option<bool>,
    pub mirror_interval: Option<String>,
    pub object_format_name: Option<String>,
    pub mirror_updated: Option<String>,
    pub repo_transfer: Option<String>,
    pub topics: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub login: Option<String>,
    pub login_name: Option<String>,
    pub source_id: Option<u64>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub html_url: Option<String>,
    pub language: Option<String>,
    pub is_admin: Option<bool>,
    pub last_login: Option<String>,
    pub created: Option<String>,
    pub restricted: Option<bool>,
    pub active: Option<bool>,
    pub prohibit_login: Option<bool>,
    pub location: Option<String>,
    pub pronouns: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub followers_count: Option<u64>,
    pub following_count: Option<u64>,
    pub starred_repos_count: Option<u64>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: Option<bool>,
    pub push: Option<bool>,
    pub pull: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTracker {
    pub enable_time_tracker: Option<bool>,
    pub allow_only_contributors_to_track_time: Option<bool>,
    pub enable_issue_dependencies: Option<bool>,
}
