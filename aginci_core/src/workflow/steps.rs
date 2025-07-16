mod build;
mod checkout;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Step {
    // #[serde(rename = "aginrocks/checkout")]
    Checkout(checkout::CheckoutStep),
    // #[serde(rename = "aginrocks/build")]
    Build(build::BuildStep),
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

                #[serde(skip_serializing_if = "Option::is_none")]
                pub continue_on_error: Option<bool>,

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
