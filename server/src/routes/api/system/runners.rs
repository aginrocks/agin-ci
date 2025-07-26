use axum::{Extension, Json, extract::State};
use axum_valid::Valid;
use color_eyre::eyre::{Context, ContextCompat};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;
use uuid::Uuid;
use validator::Validate;

use crate::{
    axum_error::AxumResult,
    database::{HostOS, PartialRunner, Runner},
    middlewares::{
        require_auth::UnauthorizedError, require_org_permissions::ForbiddenError,
        require_server_permissions::ServerAdmin,
    },
    routes::RouteProtectionLevel,
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/system/runners";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_runners, register_runner),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get runners
///
/// This endpoint returns all runners that are registered in the system.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<Runner>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "System"
)]
async fn get_runners(
    State(state): State<AppState>,
    _: ServerAdmin,
) -> AxumResult<Json<Vec<Runner>>> {
    let cursor = state
        .database
        .collection::<Runner>("runners")
        .find(doc! {})
        .await
        .wrap_err("Failed to fetch runners")?;

    let runners: Vec<Runner> = cursor
        .try_collect()
        .await
        .wrap_err("Failed to collect runners")?;

    Ok(Json(runners))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRunnerBody {
    #[validate(length(min = 1, max = 32))]
    pub display_name: String,

    pub host_os_type: HostOS,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterRunnerResponse {
    pub success: bool,
    pub id: String,
    pub uuid: String,
    pub token: String,
}

/// Register runner
///
/// Registers a new runner in the system. Returns the token that the runner can use to authenticate itself.
#[utoipa::path(
    method(post),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = RegisterRunnerResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    request_body = RegisterRunnerBody,
    tag = "System"
)]
async fn register_runner(
    State(state): State<AppState>,
    _: ServerAdmin,
    Valid(Json(body)): Valid<Json<RegisterRunnerBody>>,
) -> AxumResult<Json<RegisterRunnerResponse>> {
    let uuid = Uuid::new_v4();

    let runner = PartialRunner {
        display_name: body.display_name,
        uuid,
        host_os_type: Some(body.host_os_type),
        host_os: None,
        host_os_version: None,
        host_arch: None,
        last_ping: None,
        runner_version: None,
    };

    let inserted = state
        .database
        .collection::<PartialRunner>("runners")
        .insert_one(runner)
        .await
        .wrap_err("Failed to create runner")?;

    let id = inserted
        .inserted_id
        .as_object_id()
        .wrap_err("Failed to fetch runner ID")?;

    // TODO: Sign a Pulsar token and create a Pulsar namespace for the runner

    Ok(Json(RegisterRunnerResponse {
        success: true,
        id: id.to_string(),
        uuid: uuid.to_string(),
        token: "".to_string(),
    }))
}
