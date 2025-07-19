use async_trait::async_trait;
use color_eyre::eyre::Context;
use color_eyre::eyre::Result;
use http_body::Body;
use http_body_util::BodyExt;
use octocrab::Octocrab;
use octocrab::models::repos::Content;
use std::sync::Arc;

use crate::git_provider::{GitProvider, GitProviderCreateOptions};

pub struct GitHubProvider {
    client: Arc<Octocrab>,
}

#[async_trait]
impl GitProvider for GitHubProvider {
    fn new(options: GitProviderCreateOptions) -> Result<Self> {
        let base_url = options
            .base_url
            .unwrap_or_else(|| "https://api.github.com".to_string());

        let client = Octocrab::builder()
            .personal_token(options.token)
            .base_uri(base_url)?
            .build()?;

        Ok(GitHubProvider {
            client: Arc::new(client),
        })
    }
    async fn get_folder_contents(
        &self,
        owner: String,
        repo: String,
        path: String,
        r#ref: String,
    ) -> Result<Vec<Content>> {
        let result = self
            .client
            .repos(owner, repo)
            .get_content()
            .path(path)
            .r#ref(r#ref)
            .send()
            .await
            .wrap_err("Failed to fetch repository contents")?;

        Ok(result.items)
    }
    async fn raw_file(
        &self,
        owner: String,
        repo: String,
        path: String,
        r#ref: String,
    ) -> Result<String> {
        let response = self.client.repos(owner, repo).raw_file(r#ref, path).await?;

        let (_, body) = response.into_parts();
        let body = body.collect().await?.to_bytes();
        let file = str::from_utf8(&body)?;

        Ok(file.to_string())
    }
}
