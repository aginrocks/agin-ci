use {
    crate::define_step,
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};

#[cfg(feature = "step_executor")]
use {
    crate::runner_messages::report_progress::ProgressReport, color_eyre::eyre::Result,
    tokio::sync::broadcast::Sender,
};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UploadArtifactStepWith {
    /// Path to the artifact to upload.
    pub path: String,

    /// Artifact name visible in the UI.
    pub name: String,
}

#[cfg(feature = "step_executor")]
async fn execute(step: UploadArtifactStep, progress_tx: Sender<ProgressReport>) -> Result<()> {
    Ok(())
}

define_step!(
    "aginci/upload-artifact",
    UploadArtifactStep {
        with: UploadArtifactStepWith,
    },
    execute
);
