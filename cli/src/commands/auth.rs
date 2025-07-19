pub mod login;
pub mod logout;
pub mod whoami;

use clap::Subcommand;
use miette::Result;

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    Whoami,
    Login,
    Logout,
}

pub async fn handle_auth(cmd: AuthCommands) -> Result<()> {
    match cmd {
        // AuthCommands::Whoami => whoami::run(),
        AuthCommands::Login => login::run().await,
        // AuthCommands::Logout => logout::run(),
        _ => todo!(),
    }
}
