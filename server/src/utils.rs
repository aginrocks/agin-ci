use color_eyre::eyre::{ContextCompat, Result};
use git_url_parse::GitUrl;

pub fn normalize_git_url(url: &str) -> Result<String> {
    if url.is_empty() {
        return Ok(String::new());
    }

    let parsed_url = GitUrl::parse(url)?;

    let user = parsed_url.user.unwrap_or("git".to_string());
    let host = parsed_url.host.wrap_err("Missing host in the URL")?;
    let fullname = parsed_url.fullname;

    let normalized_url = format!("{user}@{host}:{fullname}.git");

    Ok(normalized_url)
}
