pub mod login;
pub mod logout;
pub mod whoami;

use clap::Subcommand;
use miette::Result;

use crate::Cli;

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommands {
    Whoami,
    Login,
    Logout,
}

pub async fn handle_auth(cli: &Cli, cmd: AuthCommands) -> Result<()> {
    match cmd {
        AuthCommands::Whoami => whoami::run(cli).await,
        AuthCommands::Login => login::run().await,
        AuthCommands::Logout => logout::run(cli).await,
    }
}
