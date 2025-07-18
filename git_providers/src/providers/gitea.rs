use gitea_sdk::{Auth, Client};
use std::sync::Arc;

use crate::git_provider::{GitProvider, GitProviderCreateOptions};

pub struct GiteaProvider {
    client: Arc<Client>,
}

impl GitProvider for GiteaProvider {
    fn new(options: GitProviderCreateOptions) -> Self {
        let base_url = options
            .base_url
            .unwrap_or_else(|| "https://codeberg.org".to_string());

        let client = Client::new(base_url, Auth::Token(options.token));

        GiteaProvider {
            client: Arc::new(client),
        }
    }
}
