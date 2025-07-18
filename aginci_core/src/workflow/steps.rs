pub mod build;
pub mod checkout;
pub mod restore_cache;
pub mod run;
pub mod save_cache;
pub mod upload_artifact;

#[cfg(feature = "step_executor")]
use {
    crate::workflow::step_executor::{ReportCallback, StepExecutor},
    color_eyre::eyre::Result,
    std::pin::Pin,
};

use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "step_executor")]
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(untagged)]
#[enum_dispatch]
pub enum Step {
    Checkout(checkout::CheckoutStep),
    Build(build::BuildStep),
    Run(run::RunStep),
    RestoreCache(restore_cache::RestoreCacheStep),
    SaveCache(save_cache::SaveCacheStep),
    UploadArtifact(upload_artifact::UploadArtifactStep),
}

#[macro_export]
macro_rules! define_step {
    ($tag_value:literal, $struct_name:ident { $($field:tt)* }) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
            pub struct $struct_name {
                pub uses: [<Uses$struct_name>],

                /// You can reference a step by ID to access its outputs in subsequent steps.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub id: Option<String>,

                #[serde(skip_serializing_if = "Option::is_none")]
                pub name: Option<String>,

                #[serde(rename = "continue-on-error", skip_serializing_if = "Option::is_none")]
                pub continue_on_error: Option<bool>,

                /// Working directory for the step. If not specified, the default working directory is the root of the repository.
                #[serde(rename = "working-directory", skip_serializing_if = "Option::is_none")]
                pub working_directory: Option<String>,

                $($field)*
            }

            #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
            pub enum [<Uses$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }
        }
    };
}
