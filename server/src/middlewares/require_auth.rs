use std::ops::Deref;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_oidc::OidcClaims;
use color_eyre::eyre::Result;
use color_eyre::eyre::{self, ContextCompat};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::{
    GroupClaims,
    axum_error::{AxumError, AxumResult},
    database::{AccessToken, PartialUser, ServerRole, User},
    state::AppState,
    utils::hash_pat,
};

/// User data type for request extensions
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct UserData(pub User);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub ObjectId);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GodMode(pub bool);

impl Deref for UserId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for UserData {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for GodMode {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

async fn get_user_by_subject(db: &Database, sub: String) -> Result<Option<User>> {
    let user = db
        .collection::<User>("users")
        .find_one(doc! { "subject": &sub })
        .await?;

    Ok(user)
}

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    claims: Option<OidcClaims<GroupClaims>>,
    State(state): State<AppState>,
    session: Session,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    // TODO: Implement token scopes
    let headers = request.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        let token = auth_header
            .to_str()
            .map_err(|_| AxumError::bad_request(eyre::eyre!("Invalid Authorization header")))?
            .strip_prefix("Bearer ")
            .ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Invalid Authorization scheme")))?;

        let hashed_token = hash_pat(token);

        let token = state
            .database
            .collection::<AccessToken>("tokens")
            .find_one(doc! { "hashed_token": hashed_token })
            .await?
            .ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

        let user = state
            .database
            .collection::<User>("users")
            .find_one(doc! { "_id": token.user_id })
            .await?
            .wrap_err("User not found (wtf?)")?;

        request.extensions_mut().insert(UserData(user.clone()));
        request.extensions_mut().insert(UserId(user.id));

        return Ok(next.run(request).await);
    }

    let claims = claims.ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    let sub = claims.subject().to_string();
    let name = claims
        .name()
        .wrap_err("Name is required")?
        .get(None)
        .wrap_err("Name is required")?
        .to_string();
    let email = claims.email().wrap_err("Email is required")?.to_string();

    let user = get_user_by_subject(&state.database, sub.clone()).await?;
    let user = match user {
        Some(user) => user,
        None => {
            state
                .database
                .collection::<PartialUser>("users")
                .insert_one(PartialUser {
                    email,
                    name,
                    role: ServerRole::ReadOnly,
                    subject: sub.clone(),
                })
                .await?;

            get_user_by_subject(&state.database, sub.clone())
                .await?
                .wrap_err("Failed to create user")?
        }
    };

    let god_mode_enabled =
        session.get::<bool>("god_mode").await?.unwrap_or(false) && user.role == ServerRole::Admin;

    request.extensions_mut().insert(UserData(user.clone()));
    request.extensions_mut().insert(UserId(user.id));
    request.extensions_mut().insert(GodMode(god_mode_enabled));

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
