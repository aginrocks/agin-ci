use aginci_core::{
    runner_messages::report_progress::{OrderedReport, ProgressReport, ProgressReportStep},
    workflow::{Job, step_executor::StepExecutor, steps::StepInfo},
};
use color_eyre::eyre::{Result, bail};
use tracing::info;

use crate::socket;

pub async fn run_job(job: Job) -> Result<()> {
    let socket = socket::init_socket().await?;

    let job_name = job.clone().name.unwrap_or("Unknown".to_string());
    info!("Running job {job_name}...");

    let total_steps = job.steps.len();
    info!("Total steps: {total_steps}");

    let mut seq = 0;

    for (index, step) in job.steps.iter().enumerate() {
        info!("Running step {}/{total_steps}", index + 1);
        let step_report = OrderedReport {
            ord: seq,
            body: ProgressReport::Step(ProgressReportStep {
                index: index as u32,
            }),
        };

        socket
            .emit("report_progress", serde_json::to_value(step_report)?)
            .await?;

        seq += 1;

        let mut progress = step.execute();

        while let Ok(report) = progress.recv().await {
            let ordered = OrderedReport {
                ord: seq,
                body: report.clone(),
            };
            seq += 1;

            socket
                .emit("report_progress", serde_json::to_value(ordered)?)
                .await?;

            if let ProgressReport::Exit(exit) = report
                && exit.exit_code != 0
                && !step.continue_on_error()
            {
                info!("Job failed with exit code {}", exit.exit_code);
                bail!("Job failed, aborting workflow");
            }
        }
    }

    Ok(())
}
