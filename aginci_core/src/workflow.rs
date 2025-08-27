pub mod step_executor;
pub mod steps;

use color_eyre::eyre::Result;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};
use uuid::Uuid;

use crate::workflow::steps::Step;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum OS {
    Linux,
    MacOS,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Job {
    pub name: Option<String>,

    #[serde(rename = "runs-on")]
    pub runs_on: OS,

    /// Base image for the job, can be any valid Docker image. By default, it's pulled from the Docker Hub. Defaults to ubuntu:latest on Linux and chroot on macOS. Warning: Image entrypoint will be ignored.
    #[serde(rename = "base-image")]
    pub base_image: Option<String>,

    pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

#[cfg(feature = "reader")]
pub async fn read_workflow(path: PathBuf) -> Result<Workflow> {
    use tokio::fs;

    let content = fs::read_to_string(path).await?;
    let workflow: Workflow = serde_yaml::from_str(&content)?;

    Ok(workflow)
}

#[cfg(feature = "reader")]
pub async fn read_workflows(root_path: PathBuf) -> Result<Vec<Workflow>> {
    use tokio::fs;

    let mut dir = fs::read_dir(root_path).await?;
    let mut workflows = Vec::new();

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if let Some(ext) = path.extension()
            && ext == "yaml"
        {
            workflows.push(read_workflow(path).await?);
        }
    }

    Ok(workflows)
}

#[cfg(feature = "reader")]
pub async fn read_current_workflows() -> Result<Vec<Workflow>> {
    use std::env;

    let current_dir = env::current_dir()?.join(".aginci/workflows");
    read_workflows(current_dir).await
}

#[cfg(feature = "reader")]
pub async fn read_workflow_by_name(name: String) -> Result<Workflow> {
    use std::env;

    let workflow_path = env::current_dir()?
        .join(".aginci/workflows")
        .join(format!("{name}.yaml"));

    let workflow = read_workflow(workflow_path).await?;

    Ok(workflow)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobRun {
    pub id: Uuid,
    pub job: Job,
}
