use std::ops::Deref;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::Next,
};
use tracing::info;

use crate::{AppState, tokens_manager::JobRun};

#[derive(Clone)]
pub struct UserData(pub JobRun);

impl Deref for UserData {
    type Target = JobRun;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    info!("got token {token}");

    let token_info = {
        let token_read = state.tokens.read().await;
        let token = token_read.tokens.get(token);

        match token {
            Some(job_run) => job_run.clone(),
            None => return Err(StatusCode::UNAUTHORIZED),
        }
    };

    request.extensions_mut().insert(UserData(token_info));

    Ok(next.run(request).await)
}
