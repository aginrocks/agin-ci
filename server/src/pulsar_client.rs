use std::sync::Arc;

use color_eyre::eyre::Result;
use pulsar::{Authentication, Pulsar, TokioExecutor};
use pulsar_admin_sdk::apis::configuration::Configuration;

use crate::settings::Settings;

fn create_admin_config(base_url: &str, token: &str) -> Result<Configuration> {
    let config = Configuration {
        base_path: base_url.to_string(),
        user_agent: Some(format!("aginci-cli/{}", env!("CARGO_PKG_VERSION"))),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: Some(token.to_string()),
        api_key: None,
        ..Default::default()
    };

    Ok(config)
}

pub async fn init_pulsar(
    settings: &Settings,
) -> Result<(Arc<Pulsar<TokioExecutor>>, Arc<Configuration>)> {
    let mut builder = Pulsar::builder(&settings.pulsar.connection_string, TokioExecutor);

    let authentication = Authentication {
        name: "token".to_string(),
        data: settings.pulsar.token.clone().into_bytes(),
    };

    builder = builder.with_auth(authentication);
    let pulsar = builder.build().await?;

    let config = create_admin_config(&settings.pulsar.admin_url, &settings.pulsar.token)?;

    Ok((Arc::new(pulsar), Arc::new(config)))
}
