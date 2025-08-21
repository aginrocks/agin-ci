mod auth;
mod config;

use aginci_core::{
    runner_messages::report_progress::ProgressReport,
    workflow::{
        Job, OS,
        steps::{
            Step,
            run::{RunStep, RunStepWith},
        },
    },
};
use color_eyre::eyre::{Context, Result};
use librunner::{WorkflowRunner, tokens_manager::JobRun};
use tracing::{info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use crate::{auth::init_auth, config::init_config};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let (token, metadata) = init_auth().await?;
    info!("Initialized authentication");

    let mut runner = WorkflowRunner::new()?;
    runner.serve().await.wrap_err("Failed to start server")?;
    info!("Initialized workflow runner");

    // Simulating queue connection for now
    tokio::time::sleep(std::time::Duration::MAX).await;

    Ok(())
}

fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}
