use std::env;

use color_eyre::eyre::{Context, Result, eyre};
use reqwest::StatusCode;
use rust_socketio::asynchronous::ClientBuilder;
use serde_json::json;
use tracing::{info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

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

    let client = reqwest::Client::new();

    client
        .get(&server_url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .wrap_err("Unable to reach LibRunner")?
        .error_for_status()
        .map_err(|e| match e.status() {
            Some(StatusCode::UNAUTHORIZED) => eyre!("Invalid LibRunner token"),
            None => eyre!("Unable to reach LibRunner"),
            Some(status) => eyre!("HTTP error: {}", status),
        })?;

    info!("Successfully authenticated to LibRunner");

    let socket = ClientBuilder::new(server_url)
        .opening_header("Authorization", format!("Bearer {token}"))
        .connect()
        .await
        .wrap_err("Failed to connect to LibRunner")?;

    info!("Successfully connected to LibRunner");

    socket.emit("get_job", json!(null)).await?;

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
