/*
 * server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScopeTypeOneOf1 : Allows access to all organizations and projects
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeTypeOneOf1 {
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ScopeTypeOneOf1 {
    /// Allows access to all organizations and projects
    pub fn new(r#type: Type) -> ScopeTypeOneOf1 {
        ScopeTypeOneOf1 {
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "global")]
    Global,
}

impl Default for Type {
    fn default() -> Type {
        Self::Global
    }
}

