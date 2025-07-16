use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_step;

define_step!(BuildStep { test: String });
