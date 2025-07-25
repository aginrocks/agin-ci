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
pub struct Membership {
    #[serde(rename = "role")]
    pub role: models::OrganizationRole,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl Membership {
    pub fn new(role: models::OrganizationRole, user_id: String) -> Membership {
        Membership {
            role,
            user_id,
        }
    }
}

