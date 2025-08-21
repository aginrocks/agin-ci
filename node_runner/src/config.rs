use std::{env, path::PathBuf, sync::Arc};

use aginci_core::RunnerRegistrationMetadata;
use color_eyre::eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::OnceCell};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub pulsar_token: String,

    /// Hash is stored in order to detect the user passing a different token.
    /// In that case the token exchange should happen again.
    pub registration_token_hash: String,

    pub metadata: RunnerRegistrationMetadata,
}

impl AppConfig {
    pub async fn save(&self) -> Result<()> {
        let data_dir = get_data_dir();

        fs::create_dir_all(&data_dir)
            .await
            .wrap_err("Failed to create data directory")?;

        let config_path = data_dir.join("config.toml");

        let config_data = toml::to_string(&self).wrap_err("Failed to serialize config")?;
        fs::write(config_path, config_data)
            .await
            .wrap_err("Failed to write config file")?;

        Ok(())
    }
}

static APP_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::const_new();

pub fn get_data_dir() -> PathBuf {
    let data_dir = env::var("DATA_DIR").unwrap_or("/etc/aginci/data".to_string());
    PathBuf::from(data_dir)
}

pub async fn init_config() -> Result<&'static Arc<AppConfig>> {
    APP_CONFIG
        .get_or_try_init(|| async {
            let data_dir = get_data_dir();
            let config_path = data_dir.join("config.toml");

            let config = fs::read_to_string(config_path)
                .await
                .wrap_err("Failed to read config")?;

            let config: AppConfig = toml::from_str(&config).wrap_err("Failed to parse config")?;

            Ok(Arc::new(config))
        })
        .await
}
