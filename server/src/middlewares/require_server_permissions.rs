use axum::extract::FromRequestParts;
use color_eyre::eyre::{self};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    axum_error::AxumError, database::ServerRole, middlewares::require_auth::UserData,
    state::AppState,
};

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct ServerAdmin;

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct ServerWrite;

macro_rules! impl_server_role_extractor {
    ($struct_name:ident, $required_role:expr) => {
        impl FromRequestParts<AppState> for $struct_name {
            type Rejection = AxumError;

            async fn from_request_parts(
                parts: &mut http::request::Parts,
                _state: &AppState,
            ) -> Result<Self, Self::Rejection> {
                let user_data = parts
                    .extensions
                    .get::<UserData>()
                    .ok_or(AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

                let has_access = user_data.0.role >= $required_role;

                if !has_access {
                    return Err(AxumError::forbidden(eyre::eyre!(
                        "You do not have sufficient permissions to perform this action"
                    )));
                }

                Ok($struct_name)
            }
        }
    };
}

impl_server_role_extractor!(ServerAdmin, ServerRole::Admin);
impl_server_role_extractor!(ServerWrite, ServerRole::User);
