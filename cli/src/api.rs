use api_client::apis::configuration::Configuration;
use keyring::Entry;
use miette::{IntoDiagnostic, Result};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::config::init_config;

static API_CONFIG: OnceCell<Arc<Configuration>> = OnceCell::const_new();

pub fn create_api_config(base_url: &str, token: &str) -> Result<Configuration> {
    let headers = HeaderMap::from_iter([(
        AUTHORIZATION,
        format!("Bearer {token}").parse().into_diagnostic()?,
    )]);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .into_diagnostic()?;

    let config = Configuration {
        base_path: base_url.to_string(),
        user_agent: Some(format!("aginci-cli/{}", env!("CARGO_PKG_VERSION"))),
        client,
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: Some(token.to_string()),
        api_key: None,
    };

    Ok(config)
}

pub async fn init_api_config() -> Result<&'static Arc<Configuration>> {
    API_CONFIG
        .get_or_try_init(|| async {
            let app_config = init_config().await?;

            let entry = Entry::new("aginci-cli", &app_config.username).into_diagnostic()?;
            let token = entry.get_password().into_diagnostic()?;

            let config = create_api_config(&app_config.base_url, &token)?;

            Ok(Arc::new(config))
        })
        .await
}
