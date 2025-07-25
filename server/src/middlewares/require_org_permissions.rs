use std::ops::Deref;

use axum::extract::{FromRequestParts, RawPathParams};
use color_eyre::eyre::{self};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    axum_error::AxumError,
    database::{Organization, OrganizationRole},
    middlewares::require_auth::{GodMode, UserId},
    state::AppState,
};

// Organization data type for request extensions
// #[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
// pub struct OrgData(pub Organization);

/// Organization data with viewer role requirement
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrgDataViewer(pub Organization);

/// Organization data with member role requirement
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrgDataMember(pub Organization);

/// Organization data with admin role requirement
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrgDataAdmin(pub Organization);

/// Organization data with owner role requirement
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct OrgDataOwner(pub Organization);

// Macro to implement role-specific extractors
macro_rules! impl_org_data_extractor {
    ($struct_name:ident, $required_role:expr) => {
        impl FromRequestParts<AppState> for $struct_name {
            type Rejection = AxumError;

            async fn from_request_parts(
                parts: &mut http::request::Parts,
                state: &AppState,
            ) -> Result<Self, Self::Rejection> {
                let params = RawPathParams::from_request_parts(parts, state)
                    .await
                    .map_err(|_| {
                        AxumError::bad_request(eyre::eyre!("Invalid organization path"))
                    })?;

                let user_id = parts
                    .extensions
                    .get::<UserId>()
                    .ok_or(AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

                // Extract org_slug from the request path
                let org_slug = params
                    .iter()
                    .find_map(|(param, value)| {
                        if param == "org_slug" {
                            Some(value)
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| {
                        AxumError::bad_request(eyre::eyre!("Invalid organization path"))
                    })?;

                let org = state
                    .database
                    .collection::<Organization>("organizations")
                    .find_one(doc! {
                        "slug": org_slug,
                    })
                    .await?;

                if org.is_none() {
                    return Err(AxumError::not_found(eyre::eyre!("Organization not found")));
                }

                let org = org.unwrap();

                let god_mode = parts.extensions.get::<GodMode>().unwrap_or(&GodMode(false));
                if god_mode.0 {
                    // If God Mode is enabled, skip permission checks
                    return Ok(Self(org));
                }

                let membership = org.members.iter().find(|m| &m.user_id == user_id.deref());

                if membership.is_none() {
                    return Err(AxumError::forbidden(eyre::eyre!(
                        "You do not have sufficient permissions to perform this action"
                    )));
                }

                let membership = membership.unwrap();
                let has_access = membership.role >= $required_role;

                if !has_access {
                    return Err(AxumError::forbidden(eyre::eyre!(
                        "You do not have sufficient permissions to perform this action"
                    )));
                }

                Ok(Self(org))
            }
        }
        impl Deref for $struct_name {
            type Target = Organization;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

// Implement extractors for each role
impl_org_data_extractor!(OrgDataViewer, OrganizationRole::Viewer);
impl_org_data_extractor!(OrgDataMember, OrganizationRole::Member);
impl_org_data_extractor!(OrgDataAdmin, OrganizationRole::Admin);
impl_org_data_extractor!(OrgDataOwner, OrganizationRole::Owner);

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "You do not have sufficient permissions to perform this action"}))]
pub struct ForbiddenError {
    pub error: String,
}
