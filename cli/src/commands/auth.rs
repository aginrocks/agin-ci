pub mod login;
pub mod logout;
pub mod whoami;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    Whoami,
    Login,
    Logout,
}

pub fn handle_auth(cmd: AuthCommands) {
    // match cmd {
    //     AuthCommands::Whoami => whoami::run(),
    //     AuthCommands::Login => login::run(),
    //     AuthCommands::Logout => logout::run(),
    // }
}
