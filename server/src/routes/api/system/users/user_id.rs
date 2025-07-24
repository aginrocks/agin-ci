use axum::{
    Json,
    extract::{Path, State},
};
use color_eyre::eyre::eyre;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{ServerRole, User},
    middlewares::{
        require_auth::UnauthorizedError, require_org_permissions::ForbiddenError,
        require_server_permissions::ServerAdmin,
    },
    routes::{RouteProtectionLevel, api::CreateSuccess},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/system/users/{user_id}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(edit_system_user),
        RouteProtectionLevel::Authenticated,
    )]
}

#[derive(Serialize, Deserialize, ToSchema)]
struct EditServerRoleBody {
    role: ServerRole,
}

/// Edit user's role
#[utoipa::path(
    method(patch),
    path = PATH,
    params(
        ("user_id" = String, Path, description = "User ID"),
    ),
    request_body = EditServerRoleBody,
    responses(
        (status = OK, description = "Success", body = Vec<User>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = ForbiddenError, content_type = "application/json")
    ),
    tag = "System"
)]
async fn edit_system_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _: ServerAdmin,
    Json(body): Json<EditServerRoleBody>,
) -> AxumResult<Json<CreateSuccess>> {
    let parsed_id = ObjectId::parse_str(&user_id)?;

    let updated = state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": &parsed_id },
            doc! {
                "$set": {
                    "role": &body.role,
                }
            },
        )
        .await?;

    if updated.matched_count == 0 {
        return Err(AxumError::not_found(eyre!("User not found")));
    }

    Ok(Json(CreateSuccess {
        success: true,
        id: user_id,
    }))
}
