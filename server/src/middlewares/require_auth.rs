use std::ops::Deref;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_oidc::OidcClaims;
use color_eyre::eyre::{self, Context, ContextCompat, eyre};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    GroupClaims,
    axum_error::{AxumError, AxumResult},
    database::{AccessToken, User},
    state::AppState,
    utils::hash_pat,
};

/// User data type for request extensions
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct UserData(pub User);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub ObjectId);

impl Deref for UserId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    claims: Option<OidcClaims<GroupClaims>>,
    State(state): State<AppState>,
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

    let user = state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "sub": &sub },
            doc! {
                "$set": {
                    "subject": sub,
                    "name": name,
                    "email": email,
                }
            },
        )
        .upsert(true)
        .return_document(ReturnDocument::After)
        .await?
        .wrap_err("User not found (wtf?)")?;

    request.extensions_mut().insert(UserData(user.clone()));
    request.extensions_mut().insert(UserId(user.id));

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
