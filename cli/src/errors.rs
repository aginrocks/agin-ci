use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to fetch user information")]
#[diagnostic(
    code(userinfo::fetch_failed),
    help("Ensure that the base URL and token is valid"),
    url("https://docs.agin.rocks/platform/cli/errors#{}", self.code().unwrap())
)]
pub struct UserInfoFetchFailed;
