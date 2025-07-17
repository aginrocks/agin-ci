mod socket;

use aginci_core::{runner_messages::auth::Auth, workflow::Job};
use color_eyre::eyre::{Context, Result, eyre};
use reqwest::StatusCode;
use rust_socketio::asynchronous::ClientBuilder;
use serde_json::json;
use std::{env, time::Duration};
use tracing::{info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::socket::deserialize_payload;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let server_url = env::var("AGINCI_LIBRUNNER_URL").wrap_err("Missing AGINCI_LIBRUNNER_URL")?;

    let token = env::var("AGINCI_LIBRUNNER_TOKEN").wrap_err("Missing AGINCI_LIBRUNNER_TOKEN")?;

    let socket = ClientBuilder::new(server_url)
        .auth(serde_json::to_value(Auth { token })?)
        .connect()
        .await
        .wrap_err("Failed to connect to LibRunner")?;

    info!("Successfully connected to LibRunner");

    socket
        .emit_with_ack(
            "get_job",
            json!(null),
            Duration::from_secs(2),
            handler!(Job, async |job: Job| {
                info!("job {}", job.name.unwrap());
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
