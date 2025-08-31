mod auth;
mod config;
mod handler;
mod pulsar_client;

use std::sync::Arc;

use aginci_core::pulsar::ToWorkerMessage;
use color_eyre::eyre::{Context, Result};
use futures::TryStreamExt;
use librunner::WorkflowRunner;
use pulsar::Consumer;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{auth::init_auth, handler::handle_message, pulsar_client::init_pulsar};

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

    let (registration, metadata) = init_auth().await?;
    info!("Initialized authentication");

    let mut runner = WorkflowRunner::new()?;
    runner.serve().await.wrap_err("Failed to start server")?;
    let runner = Arc::new(runner);
    info!("Initialized workflow runner");

    let pulsar = init_pulsar(registration).await?;
    info!("Initialized Pulsar client");

    // Start listening for jobs
    // TODO: Handle different tenant names
    let topic = format!("persistent://aginci/{}/jobs", metadata.runner_id);
    dbg!(&topic);
    let consumer_name = format!("runner/{}", metadata.runner_id);
    let mut consumer: Consumer<ToWorkerMessage, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_consumer_name(consumer_name)
        .build()
        .await?;

    info!("Listening for jobs...");

    while let Some(msg) = consumer.try_next().await? {
        handle_message(&mut consumer, &runner, msg)
            .await
            .map_err(|e| {
                error!(error = %e, "An error occurred while handling a message");
            })
            .ok();
    }

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
