pub mod steps;

use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::LazyLock};

use crate::workflow::steps::Step;

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OS {
    Linux,
    MacOS,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Job {
    pub name: Option<String>,

    #[serde(rename = "runs-on")]
    pub runs_on: OS,

    /// Base image for the job, can be any valid Docker image. By default, it's pulled from the Docker Hub. Defaults to ubuntu:latest on Linux and chroot on macOS. Warning: Image entrypoint will be ignored.
    #[serde(rename = "base-image")]
    pub base_image: Option<String>,

    pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Workflow {
    /// The name of the workflow. It is shown in the UI.
    pub name: String,

    #[serde(rename = "run-name", skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,

    pub jobs: HashMap<String, Job>,
}

pub static WORKFLOW_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema = schema_for!(Workflow);
    serde_json::to_value(schema).expect("Failed to convert workflow schema to JSON")
});
