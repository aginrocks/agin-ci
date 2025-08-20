use std::sync::Arc;

use base64::{Engine, engine::general_purpose};
use color_eyre::eyre::{ContextCompat, Result};
use git_url_parse::GitUrl;
use rand::{Rng, RngCore, distr::Alphanumeric};
use sha2::{Digest, Sha256};

use crate::pulsar_client::PulsarAdmin;

pub fn normalize_git_url(url: &str) -> Result<String> {
    if url.is_empty() {
        return Ok(String::new());
    }

    let parsed_url = GitUrl::parse(url)?;

    let user = parsed_url.user.unwrap_or("git".to_string());
    let host = parsed_url.host.wrap_err("Missing host in the URL")?;
    let fullname = parsed_url.fullname;
    let mut port = parsed_url.port.unwrap_or(22).to_string();
    if port == "22" {
        port = "".to_string();
    } else {
        port = format!(":{port}");
    }

    let normalized_url = format!("ssh://{user}@{host}{port}/{fullname}.git");

    Ok(normalized_url)
}

pub fn generate_webhook_secret() -> String {
    let mut rng = rand::rngs::ThreadRng::default();

    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    general_purpose::STANDARD.encode(bytes)
}

pub fn generate_pat() -> String {
    let rng = rand::rngs::ThreadRng::default();

    let token: String = rng
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();

    format!("aginci_pat_{token}")
}

pub fn hash_pat(pat: &str) -> String {
    format!("{:x}", Sha256::digest(pat))
}

pub async fn sign_worker_token(admin: Arc<PulsarAdmin>, worker_id: &str) -> Result<String> {
    let role = format!("worker_{worker_id}");

    let permissions = vec!["produce".to_string(), "consume".to_string()];

    admin.create_namespace(worker_id, None).await.ok();

    admin
        .grant_permissions_on_namespace(worker_id, &role, Some(permissions))
        .await
        .ok();

    let token = admin.key.sign_token(role)?;

    Ok(token)
}
