use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use color_eyre::Report;
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub struct AxumError {
    pub report: Report,
    pub status_code: StatusCode,
}

impl AxumError {
    pub fn new(report: Report) -> Self {
        Self {
            report,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn with_status(report: Report, status_code: StatusCode) -> Self {
        Self {
            report,
            status_code,
        }
    }

    pub fn bad_request(report: Report) -> Self {
        Self::with_status(report, StatusCode::BAD_REQUEST)
    }

    pub fn unauthorized(report: Report) -> Self {
        Self::with_status(report, StatusCode::UNAUTHORIZED)
    }

    pub fn forbidden(report: Report) -> Self {
        Self::with_status(report, StatusCode::FORBIDDEN)
    }

    pub fn not_found(report: Report) -> Self {
        Self::with_status(report, StatusCode::NOT_FOUND)
    }

    pub fn conflict(report: Report) -> Self {
        Self::with_status(report, StatusCode::CONFLICT)
    }

    #[allow(dead_code)]
    pub fn unprocessable_entity(report: Report) -> Self {
        Self::with_status(report, StatusCode::UNPROCESSABLE_ENTITY)
    }
}

impl<E: Into<Report>> From<E> for AxumError {
    fn from(error: E) -> Self {
        Self::new(error.into())
    }
}

impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        #[cfg(debug_assertions)]
        error!(error = ?self.report, "An error occurred in an axum handler");

        let body = json!({
            "error": self.report.to_string()
        });
        Response::builder()
            .status(self.status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap_or_else(|e| format!("{e:?}").into_response())
    }
}

pub type AxumResult<T, E = AxumError> = std::result::Result<T, E>;
