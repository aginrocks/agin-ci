use axum::{Extension, extract::Request, middleware::Next, response::Response};
use axum_oidc::OidcClaims;
use color_eyre::eyre::{self, ContextCompat};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    GroupClaims,
    axum_error::{AxumError, AxumResult},
    database::User,
    state::AppState,
};

/// User ID type for request extensions
#[derive(Clone, Debug)]
pub struct UserData(pub User);

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    claims: Option<OidcClaims<GroupClaims>>,
    Extension(state): Extension<AppState>,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
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
        .wrap_err("User not found (wtf?")?;

    request.extensions_mut().insert(UserData(user));

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
