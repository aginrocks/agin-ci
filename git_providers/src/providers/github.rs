use octocrab::Octocrab;
use std::sync::Arc;

use crate::git_provider::{GitProvider, GitProviderCreateOptions};

pub struct GitHubProvider {
    client: Arc<Octocrab>,
}

impl GitProvider for GitHubProvider {
    fn new(options: GitProviderCreateOptions) -> Self {
        let base_url = options
            .base_url
            .unwrap_or_else(|| "https://api.github.com".to_string());

        let client = Octocrab::builder()
            .personal_token(options.token)
            .base_uri(base_url)
            .expect("Failed to create Octocrab client")
            .build()
            .expect("Failed to create Octocrab client");

        GitHubProvider {
            client: Arc::new(client),
        }
    }
}
