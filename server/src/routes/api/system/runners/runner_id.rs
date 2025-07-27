use axum::{
    Json,
    extract::{Path, State},
};
use axum_valid::Valid;
use color_eyre::eyre::{Context, eyre};
use mongodb::bson::{doc, oid::ObjectId};
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::PartialRunner,
    middlewares::{
        require_auth::UnauthorizedError, require_org_permissions::ForbiddenError,
        require_server_permissions::ServerAdmin,
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/system/runners/{runner_id}";

pub fn routes() -> Vec<Route> {
    vec![(routes!(edit_runner), RouteProtectionLevel::Authenticated)]
}

/// Edit runner
///
/// Edit runner's details.
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("runner_id" = String, Path, description = "Runner ID"),
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    request_body = super::RegisterRunnerBody,
    tag = "System"
)]
async fn edit_runner(
    State(state): State<AppState>,
    _: ServerAdmin,
    Path(runner_id): Path<String>,
    Valid(Json(body)): Valid<Json<super::RegisterRunnerBody>>,
) -> AxumResult<Json<CreateSuccess>> {
    let runner_id = ObjectId::parse_str(&runner_id)?;

    let updated = state
        .database
        .collection::<PartialRunner>("runners")
        .find_one_and_update(
            doc! { "_id": runner_id },
            doc! {
                "$set": {
                    "display_name": body.display_name,
                    "host_os_type": body.host_os_type,
                }
            },
        )
        .await
        .wrap_err("Failed to update runner")?;

    if updated.is_none() {
        return Err(AxumError::not_found(eyre!("Runner not found")));
    }

    Ok(Json(CreateSuccess {
        success: true,
        id: runner_id.to_string(),
    }))
}
