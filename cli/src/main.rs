mod api;
mod commands;
mod errors;
mod report_handler;
mod utils;

use clap::{Parser, Subcommand};
use miette::Result;

use crate::report_handler::ErrorReportHandler;

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
async fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| Box::new(ErrorReportHandler::new())))?;

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Auth { subcommand } => commands::auth::handle_auth(subcommand).await,
    };

    if let Err(e) = result {
        eprintln!("{e:?}");
    }

    Ok(())
}
