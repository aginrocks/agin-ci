pub mod build;
pub mod checkout;
pub mod restore_cache;
pub mod run;
pub mod save_cache;
pub mod upload_artifact;

#[cfg(feature = "step_executor")]
use {
    crate::{
        runner_messages::report_progress::ProgressReport, workflow::step_executor::StepExecutor,
    },
    tokio::sync::broadcast::Receiver,
};

use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

#[enum_dispatch(Step)]
pub trait StepInfo {
    fn name(&self) -> Option<String> {
        None
    }
    fn step_name(&self) -> String;
    fn continue_on_error(&self) -> bool {
        false
    }
    fn id(&self) -> Option<String> {
        None
    }
}

#[macro_export]
macro_rules! define_step {
    ($tag_value:literal, $struct_name:ident { $($field:tt)* }, $run:ident) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
            #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
            pub enum [<Uses$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }

            #[cfg(feature = "step_executor")]
            use {
                $crate::workflow::{step_executor::StepExecutor, steps::StepInfo},
                tokio::sync::broadcast::{self, Receiver},
            };

            #[cfg(feature = "step_executor")]
            impl StepExecutor for $struct_name {
                fn execute(&self) -> Receiver<ProgressReport> {
                    let (progress_tx, progress_rx) = broadcast::channel::<ProgressReport>(1000);

                    tokio::spawn($run(self.clone(), progress_tx));

                    progress_rx
                }
            }

            impl StepInfo for $struct_name {
                fn name(&self) -> Option<String> {
                    self.name.clone()
                }
                fn step_name(&self) -> String {
                    $tag_value.to_string()
                }
                fn continue_on_error(&self) -> bool {
                    self.continue_on_error.unwrap_or(false)
                }
                fn id(&self) -> Option<String> {
                    self.id.clone()
                }
            }
        }
    };
}
