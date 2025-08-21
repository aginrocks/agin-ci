use base64::{Engine, engine::general_purpose};
use color_eyre::eyre::{Result, bail};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use serde_json::Error;

pub mod runner_messages;
pub mod workflow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunnerRegistrationMetadata {
    #[serde(rename = "url")]
    pub public_url: String,

    #[serde(rename = "v")]
    pub core_version: String,
}

impl RunnerRegistrationMetadata {
    pub fn new(public_url: String) -> Self {
        Self {
            public_url,
            core_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn encode(&self) -> Result<String, Error> {
        let stringified = serde_json::to_string(self)?;
        let encoded = general_purpose::STANDARD.encode(stringified);
        Ok(encoded)
    }

    pub fn decode(encoded: &str) -> Result<Self> {
        let decoded = general_purpose::STANDARD.decode(encoded)?;
        let metadata: Self = serde_json::from_slice(&decoded)?;
        Ok(metadata)
    }
}

pub struct RunnerRegistration {
    pub metadata: RunnerRegistrationMetadata,
    pub token: String,
}

impl RunnerRegistration {
    pub fn new(metadata: RunnerRegistrationMetadata, token: String) -> Self {
        Self { metadata, token }
    }

    pub fn new_random(metadata: RunnerRegistrationMetadata) -> Self {
        let rng = rand::rngs::ThreadRng::default();

        let token: String = rng
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();

        RunnerRegistration::new(metadata, token)
    }

    pub fn encode(&self) -> Result<String, Error> {
        let metadata_encoded = self.metadata.encode()?;
        Ok(format!(
            "aginci.runnerreg.{token}.{metadata_encoded}",
            token = self.token
        ))
    }

    pub fn decode(encoded: &str) -> Result<Self> {
        let parts = encoded.splitn(4, '.').collect::<Vec<_>>();
        if parts.len() != 4 || parts[0] != "aginci" || parts[1] != "runnerreg" {
            bail!("Invalid registration token format");
        }

        let token = parts[2].to_string();
        let metadata_encoded = parts[3];

        let metadata = RunnerRegistrationMetadata::decode(metadata_encoded)?;

        Ok(RunnerRegistration::new(metadata, token))
    }
}
