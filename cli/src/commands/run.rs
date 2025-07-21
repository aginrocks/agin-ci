use std::time::Duration;

use aginci_core::{
    runner_messages::report_progress::ProgressReport, workflow::read_workflow_by_name,
};
use indicatif::ProgressBar;
use librunner::{WorkflowRunner, tokens_manager::JobRun};
use miette::{Result, miette};
use owo_colors::OwoColorize;
use tracing::{info, info_span};
use uuid::Uuid;

use crate::{
    Cli, SelectOrgArgs, SelectProjectArgs, config::init_config, errors::LocalOrgProjectSpecified,
    utils::get_spinner_style,
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

            info!(
                "{} {}",
                "Running workflow".dimmed(),
                workflow.name.bold().blue()
            );

            let mut runner =
                WorkflowRunner::new().map_err(|_| miette!("Failed to start workflow runner"))?;

            runner
                .serve()
                .await
                .map_err(|_| miette!("Failed to start server"))?;

            for (job_id, job) in workflow.jobs.iter() {
                let span = info_span!("aginci_cli::nested");
                let _enter = span.enter();

                let job_name = job.name.clone().unwrap_or(job_id.clone());

                info!("{} {}", "Running job".dimmed(), job_name.bold().blue());

                let span = info_span!("aginci_cli::nested::nested");
                let _enter = span.enter();

                let bar = ProgressBar::new_spinner();
                bar.set_style(get_spinner_style());
                bar.set_message(format!(
                    "{} {}",
                    "Starting job".dimmed(),
                    job_name.bold().blue()
                ));
                bar.enable_steady_tick(Duration::from_millis(100));

                let job_run = JobRun {
                    id: Uuid::new_v4(),
                    job: job.clone(),
                };

                let mut progress = runner
                    .run_workflow(job_run)
                    .await
                    .map_err(|_| miette!("Failed to run workflow"))?;

                while let Ok(report) = progress.recv().await {
                    bar.suspend(|| match report {
                        ProgressReport::Output(output) => {
                            info!("{}", output.body);
                        }
                        ProgressReport::Exit(exit) => {
                            info!(
                                "{} {}",
                                "Exited with code".bold(),
                                exit.exit_code.bold().green()
                            );
                        }
                        ProgressReport::Step(step) => {
                            info!(
                                "{} {}",
                                "Running step".dimmed(),
                                step.index.to_string().bold().blue()
                            );
                            bar.set_message(format!(
                                "{} {}",
                                "Running step".dimmed(),
                                step.index.to_string().bold().blue()
                            ));
                        }
                    });
                }

                bar.finish_and_clear();
            }
        }
        false => {
            let _config = init_config().await?;
        }
    }

    Ok(())
}
