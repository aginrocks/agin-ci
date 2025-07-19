use async_trait::async_trait;
use color_eyre::eyre::{Result, eyre};
use gitea_client::models::ContentsResponse;
use gitea_sdk::{Auth, Client};
use octocrab::models::repos::{Content, ContentLinks};
use reqwest::header::{AUTHORIZATION, HeaderValue, USER_AGENT};
use std::sync::Arc;
use url::Url;

use crate::{
    AGINCI_USER_AGENT,
    git_provider::{GitProvider, GitProviderCreateOptions},
};

pub struct GiteaProvider {
    client: Arc<Client>,
    token: String,
}

#[async_trait]
impl GitProvider for GiteaProvider {
    fn new(options: GitProviderCreateOptions) -> Result<Self> {
        let base_url = options
            .base_url
            .unwrap_or_else(|| "https://codeberg.org".to_string());

        let client = Client::new(base_url.clone(), Auth::Token(options.token.clone()));

        Ok(GiteaProvider {
            client: Arc::new(client),
            token: options.token,
        })
    }
    async fn get_folder_contents(
        &self,
        owner: String,
        repo: String,
        path: String,
        r#ref: String,
    ) -> Result<Vec<Content>> {
        let path = format!(
            "repos/{}/{}/contents/{}",
            urlencoding::encode(&owner),
            urlencoding::encode(&repo),
            path,
        );

        let results = self
            .client
            .get(path)
            .query(&[("ref", r#ref)])
            .send()
            .await
            .unwrap()
            .json::<Vec<ContentsResponse>>()
            .await
            .unwrap();

        let parsed_results = results
            .iter()
            .map(|result| {
                let links = result._links.as_ref();

                let git = links
                    .and_then(|l| l.git.as_ref())
                    .and_then(|s| Url::parse(s).ok());

                let html = links
                    .and_then(|l| l.html.as_ref())
                    .and_then(|s| Url::parse(s).ok());

                let self_url = links
                    .and_then(|l| l.param_self.as_ref())
                    .and_then(|s| Url::parse(s).ok())
                    .ok_or_else(|| eyre!("Missing or invalid `_self` URL"))?;

                Ok(Content {
                    name: result.name.clone().unwrap_or_default(),
                    path: result.path.clone().unwrap_or_default(),
                    sha: result.sha.clone().unwrap_or_default(),
                    encoding: result.encoding.clone(),
                    content: result.content.clone(),
                    size: result.size.unwrap_or_default(),
                    url: result.url.clone().unwrap_or_default(),
                    html_url: result.html_url.clone(),
                    git_url: result.git_url.clone(),
                    download_url: result.download_url.clone(),
                    r#type: result.r#type.clone().unwrap_or_default(),
                    links: ContentLinks {
                        git,
                        html,
                        _self: self_url,
                    },
                    license: None,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(parsed_results)
    }
    async fn raw_file(&self, raw_url: String) -> Result<String> {
        let client = reqwest::Client::new();
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
