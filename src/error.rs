//! Error Handling — Typed errors with HTMX-aware responses
//!
//! Errors automatically render as HTML fragments suitable for HTMX swaps,
//! with proper HTTP status codes and optional HX-Retarget headers.

use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    Database(String),
}

impl AppError {
    /// HTTP status code for this error type
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) | AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Internal(_) | AppError::Anyhow(_) | AppError::Database(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// CSS class for styling the error alert
    fn alert_class(&self) -> &'static str {
        match self {
            AppError::NotFound(_) => "warning",
            AppError::BadRequest(_) | AppError::Validation(_) => "warning",
            AppError::Unauthorized => "danger",
            _ => "danger",
        }
    }

    /// Icon for the error type
    fn icon(&self) -> &'static str {
        match self {
            AppError::NotFound(_) => "search",
            AppError::BadRequest(_) | AppError::Validation(_) => "exclamation-triangle",
            AppError::Unauthorized => "lock",
            _ => "x-circle",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let alert_class = self.alert_class();
        let icon = self.icon();
        let message = self.to_string();

        // Render as HTML fragment for HTMX
        let body = format!(
            r#"<div class="alert alert-{alert_class}" role="alert">
    <div class="alert-title"><i class="bi bi-{icon}"></i> <strong>Error {code}</strong></div>
    <div class="alert-body">{message}</div>
</div>"#,
            alert_class = alert_class,
            icon = icon,
            code = status.as_u16(),
            message = message,
        );

        // Build response with HTMX-friendly headers
        let mut response = (status, Html(body)).into_response();

        // Tell HTMX to show error in a specific target (if toast/notification area exists)
        response.headers_mut().insert(
            header::HeaderName::from_static("hx-retarget"),
            "#error-toast".parse().unwrap(),
        );
        response.headers_mut().insert(
            header::HeaderName::from_static("hx-reswap"),
            "innerHTML".parse().unwrap(),
        );

        response
    }
}

// Convenience constructors
impl AppError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
}

