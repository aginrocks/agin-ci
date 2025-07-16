use color_eyre::eyre::{Context, Result};
use librunner::WorkflowRunner;
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

    let mut runner = WorkflowRunner::new()?;

    info!("Initialized workflow runner");

    runner.serve().await.wrap_err("Failed to start server")?;
    // runner.run_workflow(JobRun {
    //     id: Uuid::new_v4(),
    //     job: Job {
    //         base_image: Some("rust:latest".to_string()),
    //         name: Some("Example Job".to_string()),
    //         runs_on: OS::Linux,
    //         steps: vec![],
    //     },
    // })

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
