use color_eyre::eyre::{self, Context};
use hmac::{Hmac, Mac};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use sha2::Sha256;

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

    if signature == expected_signature {
        Ok(())
    } else {
        Err(AxumError::unauthorized(eyre::eyre!(
            "Invalid webhook signature"
        )))
    }
}

pub async fn get_project_secret(
    database: &Database,
    project_id: &str,
) -> AxumResult<(Project, String)> {
    let project_id = ObjectId::parse_str(project_id)?;

    let project = database
        .collection::<Project>("projects")
        .find_one(doc! { "_id": project_id })
        .await?
        .ok_or(AxumError::not_found(eyre::eyre!("Project not found")))?;

    let secret = project
        .clone()
        .repository
        .webhook_secret
        .ok_or(AxumError::bad_request(eyre::eyre!(
            "Webhook secret is not configured in Agin CI for this repository"
        )))?;

    Ok((project, secret))
}

/// Verifies that the claimed repository URL matches the project's repository URL.
/// This is used to ensure that webhook events aren't spoofed
pub fn verify_repostiory(webhook_claimed_url: &str, repository_url: &str) -> AxumResult<()> {
    let normalized_claimed_url =
        normalize_git_url(&webhook_claimed_url).wrap_err("Failed to normalize git URL")?;

    if normalized_claimed_url != repository_url {
        return Err(AxumError::bad_request(eyre::eyre!(
            "Webhook repository does not match the project repository. You likely pasted URL from another project or need to update repository URL in the Agin CI Dashboard."
        )));
    }

    Ok(())
}
