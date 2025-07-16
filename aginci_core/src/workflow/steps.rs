mod build;
mod checkout;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "uses")]
pub enum Step {
    #[serde(rename = "aginrocks/checkout")]
    Checkout(checkout::CheckoutStep),
    #[serde(rename = "aginrocks/build")]
    Build(build::BuildStep),
}

#[macro_export]
macro_rules! define_step {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Serialize, Deserialize, JsonSchema)]
        pub struct $name {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub continue_on_error: Option<bool>,

            $($field)*
        }
    };
}
