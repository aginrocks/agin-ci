mod commands;

use clap::{Parser, Subcommand};

/// Agin CI CLI (https://github.com/aginrocks/agin-ci)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Auth {
        #[command(subcommand)]
        subcommand: commands::auth::AuthCommands,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { subcommand } => {
            commands::auth::handle_auth(subcommand);
        }
    }
}
