use aginci_core::{
    runner_messages::report_progress::ProgressReport,
    workflow::{Job, step_executor::StepExecutor},
};
use color_eyre::eyre::Result;
use tracing::info;

use crate::socket;

pub async fn run_job(job: Job) -> Result<()> {
    let job_name = job.clone().name.unwrap_or("Unknown".to_string());
    info!("Running job {job_name}...");

    let total_steps = job.steps.len();
    info!("Total steps: {total_steps}");

    for (index, step) in job.steps.iter().enumerate() {
        info!("Running step {}/{total_steps}", index + 1);
        step.execute(Box::new(|report: ProgressReport| {
            Box::pin(async move {
                let socket = socket::init_socket().await?;

                socket
                    .emit("report_progress", serde_json::to_value(report)?)
                    .await?;

                Ok(())
            })
        }))
        .await?;
    }

    Ok(())
}
