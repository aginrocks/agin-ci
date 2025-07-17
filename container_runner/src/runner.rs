use aginci_core::workflow::Job;
use tracing::info;

pub async fn run_job(job: Job) {
    let job_name = job.clone().name.unwrap_or("Unknown".to_string());
    info!("Running job {job_name}...");
}
