use keyring::Entry;
use miette::{IntoDiagnostic, Result};
use tokio::fs;

use crate::{
    config::{get_config_directory, init_config},
    utils::print_success,
};

pub async fn run() -> Result<()> {
    let config = init_config().await?;

    let config_dir = get_config_directory();
    let config_path = config_dir.join("config.toml");
    if config_path.exists() {
        fs::remove_file(config_path).await.into_diagnostic()?;
    }

    let entry = Entry::new("aginci-cli", &config.username).into_diagnostic()?;
    entry.delete_credential().into_diagnostic()?;

    print_success("Logged out successfully");

    Ok(())
}
