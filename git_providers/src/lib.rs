use std::sync::LazyLock;

pub mod git_provider;
pub mod providers;

static AGINCI_USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
