mod api;
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

use crate::utils::make_link;

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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Auth { subcommand } => commands::auth::handle_auth(subcommand).await,
    };

    if let Err(e) = result {
        eprintln!("{} {e}\n", "error:".bold().red());
        eprintln!(
            "For more information visit {} {}",
            make_link("Agin CI Documentation", "https://docs.ci.agin.rocks"),
            "(control-click the link)".dimmed()
        );
        std::process::exit(1);
    }
}
