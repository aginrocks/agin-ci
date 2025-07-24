use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "step_executor")]
use {
    crate::runner_messages::report_progress::ProgressReport, color_eyre::eyre::Result,
    tokio::sync::broadcast::Sender,
};

use crate::define_step;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RunStepWith {
    /// The user ID or name to run the command as. Works only on Linux. Defaults to "1000"
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// The shell to use for the command. Defaults to "bash" on Linux and "zsh" on macOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
}

#[cfg(feature = "step_executor")]
async fn execute(step: RunStep, progress_tx: Sender<ProgressReport>) -> Result<()> {
    use std::process::Stdio;

    use color_eyre::eyre::ContextCompat;
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
    };
    use tracing::info;

    use crate::runner_messages::report_progress::{ProgressReport, ProgressReportExit};

    let shell = step
        .with
        .as_ref()
        .and_then(|w| w.shell.as_ref())
        .cloned()
        .unwrap_or("bash".to_string());

    let current_dir = std::env::current_dir()?
        .to_str()
        .wrap_err("Failed to get pwd")?
        .to_string();

    let pwd = step.working_directory.clone().unwrap_or(current_dir);

    let mut child = Command::new(shell)
        .arg("-c")
        .arg(step.run.clone())
        .env("CI", "true")
        .env("DEBIAN_FRONTEND", "noninteractive")
        .env("TERM", "xterm-256color")
        .env("FORCE_COLOR", "1")
        .env(
            "AGINCI_STEP_ID",
            step.id.clone().unwrap_or("UNKNOWN".to_string()),
        )
        .env("LANG", "en_US.UTF-8")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(pwd)
        .spawn()?;

    // TODO: Capture stderr as well

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        use crate::runner_messages::report_progress::{
            OutputType, ProgressReport, ProgressReportOutput,
        };

        progress_tx.send(ProgressReport::Output(ProgressReportOutput {
            output_type: OutputType::Stdout,
            body: line.clone(),
        }))?;

        println!("stdout: {line}");
    }

    let status = child.wait().await?;
    info!("Child process exited with status: {status}");

    progress_tx.send(ProgressReport::Exit(ProgressReportExit {
        exit_code: status.code().unwrap_or(0),
    }))?;

    Ok(())
}

define_step!(
    "aginci/run",
    RunStep {
        pub run: String,
        pub env: Option<HashMap<String, String>>,
        pub with: Option<RunStepWith>,
    },
    execute
);
