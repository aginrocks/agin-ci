use aginci_core::{
    runner_messages::report_progress::OrderedReport,
    workflow::{Job, step_executor::StepExecutor},
};
use color_eyre::eyre::Result;
use tracing::info;

use crate::socket;

pub async fn run_job(job: Job) -> Result<()> {
    let socket = socket::init_socket().await?;

    let job_name = job.clone().name.unwrap_or("Unknown".to_string());
    info!("Running job {job_name}...");

    let total_steps = job.steps.len();
    info!("Total steps: {total_steps}");

    for (index, step) in job.steps.iter().enumerate() {
        info!("Running step {}/{total_steps}", index + 1);
        // socket
        //     .emit(
        //         "report_progress",
        //         serde_json::to_value(ProgressReport::Step(ProgressReportStep {
        //             index: index as u32,
        //         }))?,
        //     )
        //     .await?;

        let mut progress = step.execute();
        let mut seq = 0;

        while let Ok(report) = progress.recv().await {
            let ordered = OrderedReport {
                ord: seq,
                body: report,
            };
            seq += 1;

            socket
                .emit("report_progress", serde_json::to_value(ordered)?)
                .await?;
        }
    }

    Ok(())
}
