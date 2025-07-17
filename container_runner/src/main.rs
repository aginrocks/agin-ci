mod runner;
mod socket;

use aginci_core::workflow::Job;
use color_eyre::eyre::{Context, Result};
use serde_json::json;
use std::{env, time::Duration};
use tracing::{info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{runner::run_job, socket::deserialize_payload};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );
    let socket = socket::init_socket().await?;

    info!("Successfully connected to LibRunner");

    socket
        .emit_with_ack(
            "get_job",
            json!(null),
            Duration::from_secs(2),
            handler!(Job, async |job: Job| {
                run_job(job).await.ok();
            }),
        )
        .await?;

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
