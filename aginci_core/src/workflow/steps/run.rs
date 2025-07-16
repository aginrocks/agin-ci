use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;

#[derive(Serialize, Deserialize, JsonSchema)]
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
