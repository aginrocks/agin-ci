use aginci_core::workflow::{Job, step_executor::StepExecutor, steps::Step};
use tracing::info;

pub async fn run_job(job: Job) {
    let job_name = job.clone().name.unwrap_or("Unknown".to_string());
    info!("Running job {job_name}...");

    let total_steps = job.steps.len();
    info!("Total steps: {total_steps}");

    for (index, step) in job.steps.iter().enumerate() {
        info!("Running step {}/{total_steps}", index + 1);
        step.execute().await;
    }
}

pub async fn run_step(step: &Step) {}
