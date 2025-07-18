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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicSecret {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<String>>,
    #[serde(rename = "scope")]
    pub scope: models::SecretScope,
}

impl PublicSecret {
    pub fn new(_id: String, name: String, organization_id: String, scope: models::SecretScope) -> PublicSecret {
        PublicSecret {
            _id,
            name,
            organization_id,
            project_id: None,
            scope,
        }
    }
}

