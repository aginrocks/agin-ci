use api_client::apis::auth_api;
use miette::Result;
use owo_colors::OwoColorize;

use crate::{api::init_api_config, errors::UserInfoFetchFailed};

pub async fn run() -> Result<()> {
    let config = init_api_config().await?;

    let user = auth_api::get_user(config)
        .await
        .map_err(|_| UserInfoFetchFailed)?;

    println!(
        "Logged in as {} {}",
        user.name.bold(),
        format!("({})", user.email).dimmed()
    );

    println!("{}", format!("id: {}", user._id).dimmed());
    println!("{}", format!("subject: {}", user.subject).dimmed());

    Ok(())
}
