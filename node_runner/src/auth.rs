use std::env;

use aginci_core::{RunnerRegistration, RunnerRegistrationMetadata};
use api_client::{
    apis::{configuration::Configuration, system_api},
    models::FinishRegistrationBody,
};
use color_eyre::eyre::{Context, Result};
use sha2::{Digest, Sha256};
use tracing::warn;

use crate::config::{AppConfig, init_config};

pub async fn exchange_token(
    token: String,
    metadata: &RunnerRegistrationMetadata,
) -> Result<String> {
    let config = Configuration {
        base_path: metadata.public_url.clone(),
        user_agent: Some(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )),
        ..Default::default()
    };
    let body = FinishRegistrationBody::new(token.clone());

    let result = system_api::finish_runner_registration(&config, body).await?;

    Ok(result.access_token)
}

/// Initializes authentication by exchanging a registration token for an access token.
/// If the configuration is already initialized, it will use the existing token.
///
/// Returns the access token on success.
pub async fn init_auth() -> Result<(String, RunnerRegistrationMetadata)> {
    let registration_token = env::var("REGISTRATION_TOKEN");
    if let Ok(config) = init_config().await {
        let hash_matched = {
            if let Ok(token) = &registration_token {
                let hash = format!("{:x}", Sha256::digest(token));
                hash == config.registration_token_hash
            } else {
                true // If no token is provided, we can safely use the existing token
            }
        };
        if hash_matched {
            return Ok((config.pulsar_token.clone(), config.metadata.clone()));
        } else {
            warn!("Registration token has changed");
        }
    }

    // The user provided a different token or the config is not initialized
    let registration_token = registration_token
        .wrap_err("Missing registration token (REGISTRATION_TOKEN env variable)")?;

    let decoded = RunnerRegistration::decode(&registration_token)
        .wrap_err("Failed to decode registration token")?;

    let pulsar_token = exchange_token(registration_token.clone(), &decoded.metadata).await?;

    let config = AppConfig {
        pulsar_token: pulsar_token.clone(),
        registration_token_hash: format!("{:x}", Sha256::digest(registration_token)),
        metadata: decoded.metadata.clone(),
    };
    config
        .save()
        .await
        .map_err(|_| warn!("Failed to save config. Registration token will be used on every restart until it expires. This is not recommended for production use."))
        .ok();

    Ok((pulsar_token, decoded.metadata))
}
