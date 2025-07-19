use std::{env, path::PathBuf, sync::Arc};

use directories::ProjectDirs;
use miette::{IntoDiagnostic, Result, miette};
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::OnceCell};

use crate::{
    errors::{ConfigSavingFailed, NotLoggedIn},
    utils::print_warning,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub base_url: String,
    pub username: String,
}

impl AppConfig {
    pub async fn save(&self) -> Result<()> {
        let config_dir = get_config_directory();

        fs::create_dir_all(&config_dir)
            .await
            .map_err(|_| ConfigSavingFailed)?;

        let config_path = config_dir.join("config.toml");

        let config_data = toml::to_string(&self).map_err(|_| ConfigSavingFailed)?;
        fs::write(config_path, config_data)
            .await
            .map_err(|_| ConfigSavingFailed)?;

        Ok(())
    }
}

static APP_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::const_new();

pub fn get_config_directory() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("rocks", "agin", "aginci-cli") {
        return proj_dirs.config_dir().to_path_buf();
    };

    print_warning("Unable to get config path. Defaulting to current directory.");
    env::current_dir().unwrap_or(PathBuf::from("."))
}

pub async fn init_config() -> Result<&'static Arc<AppConfig>> {
    APP_CONFIG
        .get_or_try_init(|| async {
            let config_dir = get_config_directory();
            let config_path = config_dir.join("config.toml");

            let config = fs::read_to_string(config_path)
                .await
                .map_err(|_| NotLoggedIn)?;

            let config: AppConfig =
                toml::from_str(&config).map_err(|_| miette!("Failed to parse config file"))?;

            Ok(Arc::new(config))
        })
        .await
}
