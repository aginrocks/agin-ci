mod build;
mod checkout;
mod run;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Step {
    Checkout(checkout::CheckoutStep),
    Build(build::BuildStep),
    Run(run::RunStep),
}

#[macro_export]
macro_rules! define_step {
    ($tag_value:literal, $struct_name:ident { $($field:tt)* }) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            pub struct $struct_name {
                pub uses: [<Uses$struct_name>],

                #[serde(skip_serializing_if = "Option::is_none")]
                pub name: Option<String>,

                #[serde(rename = "continue-on-error", skip_serializing_if = "Option::is_none")]
                pub continue_on_error: Option<bool>,

                /// Working directory for the step. If not specified, the default working directory is the root of the repository.
                #[serde(rename = "working-directory", skip_serializing_if = "Option::is_none")]
                pub working_directory: Option<String>,

                $($field)*
            }

            #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            pub enum [<Uses$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }
        }
    };
}
