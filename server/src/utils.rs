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
    let mut port = parsed_url.port.unwrap_or(22).to_string();
    if port == "22" {
        port = "".to_string();
    } else {
        port = format!(":{}", port);
    }

    let normalized_url = format!("ssh://{user}@{host}{port}/{fullname}.git");

    Ok(normalized_url)
}
