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

    let mut progress = runner
        .run_workflow(JobRun {
            id: Uuid::new_v4(),
            job: Job {
                base_image: Some("rust:latest".to_string()),
                name: Some("Example Job".to_string()),
                runs_on: OS::Linux,
                steps: vec![Step::Run(RunStep {
                    run: "ls".to_string(),
                    uses: aginci_core::workflow::steps::run::UsesRunStep::Value,
                    id: Some("example_step".to_string()),
                    name: Some("Run".to_string()),
                    continue_on_error: Some(false),
                    working_directory: None,
                    env: None,
                    with: Some(RunStepWith {
                        shell: Some("nu".to_string()),
                        user: None,
                    }),
                })],
            },
        })
        .await?;

    while let Ok(report) = progress.recv().await {
        match report {
            ProgressReport::Output(output) => {
                info!("Received output: {:?}", output);
                // Handle stdout/stderr
            }
            ProgressReport::Exit(exit) => {
                info!("Received exit: {:?}", exit);
                // Handle exit code
            }
            ProgressReport::Step(step) => {
                info!("Running step: {:?}", step);
            }
        }
    }

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
