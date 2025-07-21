use api_client::apis::auth_api;
use inquire::{Password, Text};
use keyring::Entry;
use miette::{Context, IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use tokio::task;
use tracing::{event, warn};

use crate::{
    api::create_api_config,
    config::{AppConfig, init_config},
    errors::UserInfoFetchFailed,
    success,
};

pub async fn run() -> Result<()> {
    if init_config().await.is_ok() {
        warn!(
            "You are already logged in. By entering credentials you'll override the existing ones.",
        );
    }

    let base_url = task::spawn_blocking(|| Text::new("Server URL").prompt())
        .await
        .into_diagnostic()?
        .into_diagnostic()?;

    let token = task::spawn_blocking(|| {
        Password::new("Token")
            .without_confirmation()
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .prompt()
    })
    .await
    .into_diagnostic()?
    .into_diagnostic()?;

    let config = create_api_config(&base_url, &token)
        .wrap_err("Failed to create HTTP client. Ensure that the server URL is valid.")?;

    let user = auth_api::get_user(&config)
        .await
        .map_err(|_| UserInfoFetchFailed)?;

    let entry = Entry::new("aginci-cli", &user.name).into_diagnostic()?;
    entry.set_password(&token).into_diagnostic()?;

    let config = AppConfig {
        base_url: base_url.clone(),
        username: user.name.clone(),
    };

    config.save().await?;
    success!(
        "Logged in as {} {}",
        user.name.bold(),
        format!("({})", user.email).dimmed()
    );

    Ok(())
}
