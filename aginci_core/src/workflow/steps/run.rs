use std::collections::HashMap;

#[cfg(feature = "step_executor")]
use {color_eyre::eyre::Result, std::pin::Pin};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::{ReportCallback, StepExecutor};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct RunStepWith {
    /// The user ID or name to run the command as. Works only on Linux. Defaults to "1000"
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// The shell to use for the command. Defaults to "bash" on Linux and "zsh" on macOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
}

define_step!(
    "aginci/run",
    RunStep {
        pub run: String,
        pub env: Option<HashMap<String, String>>,
        pub with: Option<RunStepWith>,
    }
);

#[cfg(feature = "step_executor")]
impl StepExecutor for RunStep {
    fn execute(
        &self,
        report_callback: ReportCallback,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            use std::process::Stdio;

            use color_eyre::eyre::ContextCompat;
            use tokio::{
                io::{AsyncBufReadExt, BufReader},
                process::Command,
            };

            let shell = self
                .with
                .as_ref()
                .and_then(|w| w.shell.as_ref())
                .cloned()
                .unwrap_or("bash".to_string());

            let current_dir = std::env::current_dir()?
                .to_str()
                .wrap_err("Failed to get pwd")?
                .to_string();

            let pwd = self.clone().working_directory.unwrap_or(current_dir);

            let mut child = Command::new(shell)
                .arg("-c")
                .arg(self.run.clone())
                .env("CI", "true")
                .env("DEBIAN_FRONTEND", "noninteractive")
                .env("TERM", "xterm-256color")
                .env("FORCE_COLOR", "1")
                .env(
                    "AGINCI_STEP_ID",
                    self.id.clone().unwrap_or("UNKNOWN".to_string()),
                )
                .env("LANG", "en_US.UTF-8")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .current_dir(pwd)
                .spawn()?;

            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await? {
                use crate::runner_messages::report_progress::{
                    OutputType, ProgressReport, ProgressReportOutput,
                };

                (report_callback)(ProgressReport::Output(ProgressReportOutput {
                    output_type: OutputType::Stdout,
                    body: line.clone(),
                }))
                .await;
                println!("stdout: {line}");
            }

            let status = child.wait().await?;
            println!("Exited with: {status}");

            Ok(())
        })
    }
}
