mod api;
mod commands;
mod config;
mod errors;
mod report_handler;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use miette::Result;
use std::process;

use crate::{report_handler::ErrorReportHandler, utils::get_render_config};

/// Agin CI CLI (https://github.com/aginrocks/agin-ci)
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(global = true, long, value_enum, default_value_t = OutputType::Text, short = 'o')]
    output: OutputType,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug, Clone)]
pub struct SelectOrgArgs {
    /// Organization slug, defaults to current project's organization
    #[arg(long, short = 'O')]
    pub org: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct SelectProjectArgs {
    /// Project slug, defaults to current project's slug
    #[arg(long, short = 'p')]
    pub project: Option<String>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
enum OutputType {
    Text,
    Json,
    Yaml,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Auth {
        #[command(subcommand)]
        subcommand: commands::auth::AuthCommands,
    },
    Run {
        workflow: String,

        #[command(flatten)]
        org: SelectOrgArgs,

        #[command(flatten)]
        project: SelectProjectArgs,

        /// Run the workflow locally (in Docker)
        #[arg(long)]
        local: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| Box::new(ErrorReportHandler::new())))?;
    inquire::set_global_render_config(get_render_config());

    let cli = Cli::parse();

    let result = match cli.clone().command {
        Commands::Auth { subcommand } => commands::auth::handle_auth(&cli, subcommand).await,
        Commands::Run {
            workflow,
            org,
            project,
            local,
        } => commands::run::handle_run(&cli, workflow, org, project, local).await,
    };

    if let Err(e) = result {
        eprintln!("{e:?}");
        process::exit(1);
    }

    Ok(())
}
