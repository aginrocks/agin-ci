use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to fetch user information")]
#[diagnostic(
    code(user::info_fetch_failed),
    help("Ensure that the base URL and token is valid"),
    url("https://docs.agin.rocks/platform/cli/errors#{}", self.code().unwrap())
)]
pub struct UserInfoFetchFailed;

#[derive(Error, Debug, Diagnostic)]
#[error("You are not logged in")]
#[diagnostic(
    code(auth::not_logged_in),
    help("Use the 'aginci auth login' command to log in"),
    url("https://docs.agin.rocks/platform/cli/errors#{}", self.code().unwrap())
)]
pub struct NotLoggedIn;

#[derive(Error, Debug, Diagnostic)]
#[error("Config saving failed")]
#[diagnostic(code(config::save_fail), help("Unable to save configuration file"))]
pub struct ConfigSavingFailed;

#[derive(Error, Debug, Diagnostic)]
#[error("Cannot specify organization or project when running locally")]
#[diagnostic(
    code(local_run::org_project_specified),
    help("When running locally, the organization and project are determined from your working directory and cannot be specified."),
    url("https://docs.agin.rocks/platform/cli/errors#{}", self.code().unwrap())
)]
pub struct LocalOrgProjectSpecified;
