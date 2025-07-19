use api_client::apis::auth_api;
use inquire::{Password, Text};
use miette::{Context, IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use tokio::task;

use crate::{api::create_config, errors::UserInfoFetchFailed, utils::print_success};

pub async fn run() -> Result<()> {
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

    let config = create_config(&base_url, &token)
        .wrap_err("Failed to create HTTP client. Ensure that the server URL is valid.")?;

    let user = auth_api::get_user(&config)
        .await
        .map_err(|_| UserInfoFetchFailed)?;

    print_success(&format!("Logged in as {}", user.email.bold()));

    Ok(())
}
