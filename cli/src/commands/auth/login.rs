use api_client::apis::auth_api;
use color_eyre::eyre::{Context, Result};
use inquire::{Password, Text};
use tokio::task;

use crate::api::create_config;

pub async fn run() -> Result<()> {
    let base_url = task::spawn_blocking(|| Text::new("Server URL").prompt()).await??;

    let token = task::spawn_blocking(|| {
        Password::new("Token")
            .without_confirmation()
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .prompt()
    })
    .await??;

    let config = create_config(&base_url, &token)
        .wrap_err("Failed to create HTTP client. Ensure that the server URL is valid.")?;

    let user = auth_api::get_user(&config).await.wrap_err(
        "Failed to fetch user information. Ensure that the base URL and token is valid.",
    )?;

    println!("Logged in as: {}", user.email);

    Ok(())
}
