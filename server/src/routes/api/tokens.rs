use axum::{Extension, Json};
use axum_valid::Valid;
use color_eyre::eyre::Context;
use serde::Serialize;

use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::{AccessTokenCreateBody, PartialAccessToken, User},
    middlewares::require_auth::{UnauthorizedError, UserData, UserId},
    routes::RouteProtectionLevel,
    state::AppState,
    utils::{generate_pat, hash_pat},
};

use super::Route;

const PATH: &str = "/api/tokens";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_tokens, create_token),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get tokens
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = User),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Tokens"
)]
async fn get_tokens(Extension(_user): Extension<UserData>) -> Json<User> {
    todo!()
}

#[derive(Serialize, ToSchema)]
struct AccessTokenCreateResponse {
    /// Store the token securely, as it will not be shown again
    token: String,
}

/// Create token
#[utoipa::path(
    method(post),
    path = PATH,
    request_body = AccessTokenCreateBody,
    responses(
        (status = OK, description = "Success", body = AccessTokenCreateResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Tokens"
)]
async fn create_token(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Valid(Json(body)): Valid<Json<AccessTokenCreateBody>>,
) -> AxumResult<Json<AccessTokenCreateResponse>> {
    let token = generate_pat();

    let hashed_token = hash_pat(&token);

    let token_object = PartialAccessToken {
        user_id: *user_id,
        hashed_token,
        display_name: body.display_name,
        scopes: body.scopes,
    };

    state
        .database
        .collection("tokens")
        .insert_one(token_object)
        .await
        .wrap_err("Failed to create access token")?;

    Ok(Json(AccessTokenCreateResponse { token }))
}
