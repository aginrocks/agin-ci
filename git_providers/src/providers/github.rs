use async_trait::async_trait;
use color_eyre::eyre::Context;
use color_eyre::eyre::Result;
use octocrab::Octocrab;
use octocrab::models::repos::Content;
use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderValue, USER_AGENT},
};
use std::sync::Arc;

use crate::AGINCI_USER_AGENT;
use crate::git_provider::{GitProvider, GitProviderCreateOptions};

pub struct GitHubProvider {
    client: Arc<Octocrab>,
    token: String,
}

#[async_trait]
impl GitProvider for GitHubProvider {
    fn new_boxed(options: GitProviderCreateOptions) -> Result<Box<dyn GitProvider>> {
        let base_url = options
            .base_url
            .unwrap_or_else(|| "https://api.github.com".to_string());

        let client = Octocrab::builder()
            .personal_token(options.token.clone())
            .base_uri(base_url)?
            .build()?;

        Ok(Box::new(GitHubProvider {
            client: Arc::new(client),
            token: options.token,
        }))
    }
    async fn get_folder_contents(
        &self,
        owner: String,
        repo: String,
        path: String,
        r#ref: String,
    ) -> Result<Vec<Content>> {
        dbg!(&owner, &repo, &path, &r#ref);
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
    async fn raw_file(&self, raw_url: String) -> Result<String> {
        let client = Client::new();
        let res = client
            .get(raw_url)
            .header(USER_AGENT, HeaderValue::from_static(&AGINCI_USER_AGENT))
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("token {}", self.token))?,
            )
            .send()
            .await?;

        Ok(res.text().await?)
    }
}
