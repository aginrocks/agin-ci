use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::{
        require_auth::{GodMode, UnauthorizedError},
        require_org_permissions::ForbiddenError,
        require_server_permissions::ServerAdmin,
    },
    routes::RouteProtectionLevel,
};

use super::Route;

const PATH: &str = "/api/god";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_god_mode, change_god_mode),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GodModeStatus {
    pub enabled: bool,
}

/// Get God Mode status
///
/// God Mode is a special mode that allows the user to bypass every permission check.
/// It can only be anabled by system admins.
///
/// This endpoint won't return a 403 Forbidden error even if you don't have the required permissions to enable God Mode.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = GodModeStatus, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "God Mode"
)]
async fn get_god_mode(
    Extension(GodMode(enabled)): Extension<GodMode>,
) -> AxumResult<Json<GodModeStatus>> {
    Ok(Json(GodModeStatus { enabled }))
}

#[derive(Deserialize, ToSchema)]
pub struct GodModeBody {
    pub enable: bool,
}

/// Change God Mode status
///
/// Enable or disable God Mode.
#[utoipa::path(
    method(patch),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = GodModeStatus, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    request_body = GodModeBody,
    tag = "God Mode"
)]
async fn change_god_mode(
    _: ServerAdmin,
    session: Session,
    Json(body): Json<GodModeBody>,
) -> AxumResult<Json<GodModeStatus>> {
    session.insert("god_mode", body.enable).await?;

    Ok(Json(GodModeStatus {
        enabled: body.enable,
    }))
}
