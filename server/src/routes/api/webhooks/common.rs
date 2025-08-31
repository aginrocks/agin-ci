use aginci_core::workflow;
use color_eyre::eyre::{self, Context, ContextCompat, Result};
use git_providers::{
    git_provider::{GitProvider, GitProviderCreateOptions},
    providers::{gitea::GiteaProvider, github::GitHubProvider},
    webhook_actions::WebhookEvent,
};
use git_url_parse::GitUrl;
use hmac::{Hmac, Mac};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use sha2::Sha256;
use tracing::error;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Project, ProjectRepositorySource, Workflow},
    utils::normalize_git_url,
};

pub fn verify_signature(secret: &str, signature: &str, body: &[u8]) -> AxumResult<()> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    let expected = mac.finalize().into_bytes();
    let expected_hex = hex::encode(expected);
    let expected_signature = format!("sha256={expected_hex}");

    if signature == expected_signature {
        Ok(())
    } else {
        Err(AxumError::unauthorized(eyre::eyre!(
            "Invalid webhook signature"
        )))
    }
}

pub async fn get_project_secret(
    database: &Database,
    project_id: &str,
) -> AxumResult<(Project, String)> {
    let project_id = ObjectId::parse_str(project_id)?;

    let project = database
        .collection::<Project>("projects")
        .find_one(doc! { "_id": project_id })
        .await?
        .ok_or(AxumError::not_found(eyre::eyre!("Project not found")))?;

    let secret = project
        .clone()
        .repository
        .webhook_secret
        .ok_or(AxumError::bad_request(eyre::eyre!(
            "Webhook secret is not configured in Agin CI for this repository"
        )))?;

    Ok((project, secret))
}

/// Verifies that the claimed repository URL matches the project's repository URL.
/// This is used to ensure that webhook events aren't spoofed
pub fn verify_repostiory(webhook_claimed_url: &str, repository_url: &str) -> AxumResult<()> {
    let normalized_claimed_url =
        normalize_git_url(webhook_claimed_url).wrap_err("Failed to normalize git URL")?;

    if normalized_claimed_url != repository_url {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Webhook repository does not match the project repository. You likely pasted URL from another project or need to update repository URL in the Agin CI Dashboard."
        )));
    }

    Ok(())
}

pub struct WorkflowReader {
    provider: Box<dyn GitProvider>,
    owner: String,
    repo: String,
}

impl WorkflowReader {
    pub fn new(provider: Box<dyn GitProvider>, owner: String, repo: String) -> Self {
        Self {
            provider,
            owner,
            repo,
        }
    }

    pub async fn read_workflows(&self, r#ref: String) -> Result<Vec<workflow::Workflow>> {
        let files = self
            .provider
            .get_folder_contents(
                self.owner.clone(),
                self.repo.clone(),
                ".aginci/workflows".to_string(),
                r#ref,
            )
            .await?;

        let workflow_files = files
            .into_iter()
            .filter(|f| f.name.ends_with(".yaml"))
            .collect::<Vec<_>>();

        let mut workflows = Vec::new();
        for file in workflow_files {
            let workflow = self.read_workflow(file.download_url).await;
            match workflow {
                Ok(workflow) => workflows.push(workflow),
                Err(err) => {
                    error!(error = %err, "Failed to read workflow file");
                }
            }
        }

        Ok(workflows)
    }

    pub async fn read_workflow(&self, download_url: Option<String>) -> Result<workflow::Workflow> {
        let download_url = download_url.wrap_err("No download URL")?;
        let content = self.provider.raw_file(download_url).await?;

        let workflow: workflow::Workflow =
            serde_yaml::from_str(&content).wrap_err("Invalid workflow file")?;
        Ok(workflow)
    }
}

/// Processes a webhook event by reading workflows from the repository, filtering them based on the event, and executing them.
pub async fn process_webhook_event(
    database: Database,
    project: &Project,
    event: WebhookEvent,
) -> Result<()> {
    // Extract access token from project repository
    let token = project
        .repository
        .access_token
        .as_ref()
        .ok_or_else(|| eyre::eyre!("No access token configured for this repository"))?
        .clone();

    let options = GitProviderCreateOptions {
        token,
        base_url: None,
    };

    let provider = match project.repository.source {
        ProjectRepositorySource::GitHub => GitHubProvider::new(options)?,
        ProjectRepositorySource::Gitea => GiteaProvider::new(options)?,
        ProjectRepositorySource::GenericGit => {
            return Err(eyre::eyre!("Generic Git is not supported for webhooks"));
        }
    };

    let parsed_url = GitUrl::parse(&project.repository.url)?;
    let owner = parsed_url
        .owner
        .ok_or_else(|| eyre::eyre!("Could not extract owner from repository URL"))?;
    let repo = parsed_url.name;

    let reader = WorkflowReader::new(provider, owner, repo);

    let matching_workflows = match event {
        WebhookEvent::Push(event) => {
            let workflows = reader.read_workflows(event.branch.clone()).await?;

            workflows
                .into_iter()
                .filter(|w| {
                    // Only run workflows that have a push trigger
                    if w.on.push.is_none() {
                        return false;
                    }

                    let push = w.on.push.as_ref().unwrap();

                    // If the workflow has a branch specified, only run it if the branch matches
                    if let Some(branches) = &push.branches
                        && !branches.contains(&event.branch)
                    {
                        return false;
                    }

                    // TODO: Implement tags
                    true
                })
                .collect::<Vec<_>>()
        }
    };

    Ok(())
}
