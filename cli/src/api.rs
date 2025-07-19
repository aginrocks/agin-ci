use api_client::apis::configuration::Configuration;
use color_eyre::eyre::Result;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use std::sync::Arc;
use tokio::sync::OnceCell;

static CONFIG: OnceCell<Arc<Configuration>> = OnceCell::const_new();

pub fn create_config(base_url: &str, token: &str) -> Result<Configuration> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse()?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

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

pub async fn init_config() -> Result<&'static Arc<Configuration>> {
    CONFIG
        .get_or_try_init(|| async {
            let config = create_config(
                "http://localhost:8080",
                "aginci_pat_tzfTGiSMgcFFzEWV7b0H4BNXIHaBHK1unn87qRjsqyYcaCwM",
            )?;

            Ok(Arc::new(config))
        })
        .await
}
