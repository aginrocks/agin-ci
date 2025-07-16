mod steps;

use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::LazyLock};

use crate::workflow::steps::Step;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum OS {
    Linux,
    MacOS,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Job {
    pub name: Option<String>,

    #[serde(rename = "runs-on")]
    pub runs_on: OS,

    pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Workflow {
    pub name: String,

    #[serde(rename = "run-name", skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,

    pub jobs: HashMap<String, Job>,
}

pub static WORKFLOW_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema = schema_for!(Workflow);
    serde_json::to_value(schema).expect("Failed to convert workflow schema to JSON")
});
