use std::time::Duration;

use aginci_core::workflow::read_workflow_by_name;
use indicatif::ProgressBar;
use miette::{Result, miette};
use owo_colors::OwoColorize;

use crate::{
    Cli, SelectOrgArgs, SelectProjectArgs,
    config::init_config,
    errors::LocalOrgProjectSpecified,
    utils::{get_spinner_style, print_with_arrow},
};

pub async fn handle_run(
    cli: &Cli,
    workflow: String,
    org: SelectOrgArgs,
    project: SelectProjectArgs,
    local: bool,
) -> Result<()> {
    match local {
        true => {
            if org.org.is_some() || project.project.is_some() {
                return Err(LocalOrgProjectSpecified.into());
            }

            // TODO: Add better workflow error handling
            let workflow = read_workflow_by_name(workflow)
                .await
                .map_err(|_| miette!("Failed to read workflow"))?;

            print_with_arrow(&format!(
                "{} {}",
                "Running workflow".dimmed(),
                workflow.name.bold().blue()
            ));

            for (job_id, job) in workflow.jobs.iter() {
                let job_name = job.name.clone().unwrap_or(job_id.clone());

                let bar = ProgressBar::new_spinner();
                bar.set_style(get_spinner_style());
                bar.set_message(format!(
                    "{} {}",
                    "Starting job".dimmed(),
                    job_name.bold().blue()
                ));
                bar.enable_steady_tick(Duration::from_millis(100));

                // println!("{}", workflow.name);

                tokio::time::sleep(Duration::from_secs(10)).await;

                bar.finish();
            }
        }
        false => {
            let _config = init_config().await?;
        }
    }

    Ok(())
}
