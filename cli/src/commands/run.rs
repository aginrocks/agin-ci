use std::time::Duration;

use aginci_core::{
    runner_messages::report_progress::ProgressReport,
    workflow::{read_workflow_by_name, steps::StepInfo},
};
use indicatif::ProgressBar;
use librunner::{WorkflowRunner, tokens_manager::JobRun};
use miette::{Result, miette};
use owo_colors::OwoColorize;
use tracing::{info, info_span};
use uuid::Uuid;

use crate::{
    Cli, SelectOrgArgs, SelectProjectArgs,
    config::init_config,
    errors::LocalOrgProjectSpecified,
    utils::{colored_exit_code, get_spinner_style},
};

pub async fn handle_run(
    _cli: &Cli,
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
                bar.set_message(format!("{}", "Waiting for runner".bold()));
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
                    bar.suspend(|| match report.clone() {
                        ProgressReport::Output(output) => {
                            println!("      {}", output.body);
                        }
                        ProgressReport::Exit(exit) => {
                            println!(
                                "    {}",
                                colored_exit_code(
                                    &format!("← Exited with {}", exit.exit_code),
                                    exit.exit_code
                                ),
                            );
                        }
                        ProgressReport::Step(step) => {
                            let step_info = job.steps.get(step.index as usize);
                            let step_label = match step_info {
                                Some(info) => info.name().unwrap_or(info.step_name()),
                                None => "Unknown Step".to_string(),
                            };
                            info!("{} {}", "Running step".dimmed(), step_label.bold().blue());
                        }
                    });
                    if let ProgressReport::Step(step) = report {
                        bar.set_message(format!(
                            "{} {} {} {}",
                            "Step".bold(),
                            (step.index + 1).to_string().bold().blue(),
                            "out of".bold(),
                            job.steps.len().to_string().bold().blue(),
                        ));
                    }
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
