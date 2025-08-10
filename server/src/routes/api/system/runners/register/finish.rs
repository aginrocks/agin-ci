use aginci_core::RunnerRegistration;
use axum::{Json, extract::State};
use axum_valid::Valid;
use color_eyre::eyre::{Context, eyre};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::Runner,
    middlewares::require_auth::UnauthorizedError,
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/system/runners/register/finish";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(finish_runner_registration),
        RouteProtectionLevel::Public,
    )]
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct FinishRegistrationBody {
    /// Token that the user generated using `POST /api/system/runners`
    #[validate(length(min = 1))]
    token: String,
}

#[derive(Serialize, ToSchema)]
pub struct FinishRegistrationResponse {
    /// Access token that can be used to authenticate directly to Apache Pulsar
    access_token: String,
}

/// Finish runner registration
///
/// This endpoint allows to exchange the registration token for a long-lived access token.
/// The token can be used to authenticate directly to Apache Pulsar.
///
/// No normal authentication is required, but the registration token must be valid.
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = FinishRegistrationResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "System"
)]
async fn finish_runner_registration(
    State(state): State<AppState>,
    Valid(Json(body)): Valid<Json<FinishRegistrationBody>>,
) -> AxumResult<Json<FinishRegistrationResponse>> {
    let registration = RunnerRegistration::decode(&body.token)
        .map_err(|_| AxumError::bad_request(eyre!("Invalid registration token format")))?;

    let runner = state
        .database
        .collection::<Runner>("runners")
        .find_one(doc! { "token_hash": format!("{:x}", Sha256::digest(registration.token)) })
        .await
        .wrap_err("Failed to fetch runner information")?
        .ok_or(AxumError::unauthorized(eyre!("Invalid registration token")))?;

    todo!()
}
