use color_eyre::eyre::{self};
use hmac::{Hmac, Mac};
use mongodb::{Database, bson::doc};
use sha2::Sha256;
use tracing::info;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::Project,
    utils::normalize_git_url,
};

pub fn verify_signature(secret: &str, signature: &str, body: &[u8]) -> AxumResult<()> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    let expected = mac.finalize().into_bytes();
    let expected_hex = hex::encode(expected);
    let expected_signature = format!("sha256={}", expected_hex);

    info!(
        "Expected signature: {}, got signature: {}, secret: {}",
        expected_signature, signature, secret
    );

    if signature == expected_signature {
        Ok(())
    } else {
        Err(AxumError::unauthorized(eyre::eyre!(
            "Invalid webhook signature"
        )))
    }
}

pub async fn get_repo_secret(database: &Database, repo_url: &str) -> AxumResult<String> {
    let repo_url = normalize_git_url(repo_url)?;

    let project = database
        .collection::<Project>("projects")
        .find_one(doc! { "repository.url": repo_url })
        .await?
        .ok_or(AxumError::not_found(eyre::eyre!(
            "No Agin CI project found for this repository"
        )))?;

    let secret = project
        .repository
        .webhook_secret
        .ok_or(AxumError::bad_request(eyre::eyre!(
            "Webhook secret is not configured in Agin CI for this repository"
        )))?;

    Ok(secret)
}
