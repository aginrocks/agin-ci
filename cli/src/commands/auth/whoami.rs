use api_client::apis::auth_api;
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;

use crate::{Cli, OutputType, api::init_api_config, errors::UserInfoFetchFailed};

pub async fn run(cli: &Cli) -> Result<()> {
    let config = init_api_config().await?;

    let user = auth_api::get_user(config)
        .await
        .map_err(|_| UserInfoFetchFailed)?;

    if cli.output == OutputType::Json {
        println!("{}", serde_json::to_string_pretty(&user).into_diagnostic()?);
        return Ok(());
    }

    println!(
        "Logged in as {} {}",
        user.name.bold(),
        format!("({})", user.email).dimmed()
    );

    println!("{}", format!("id: {}", user._id).dimmed());
    println!("{}", format!("subject: {}", user.subject).dimmed());

    Ok(())
}
