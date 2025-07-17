use std::collections::HashMap;

#[cfg(feature = "step_executor")]
use {color_eyre::eyre::Result, std::pin::Pin};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;
#[cfg(feature = "step_executor")]
use crate::workflow::step_executor::StepExecutor;

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
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move { Ok(()) })
    }
}
