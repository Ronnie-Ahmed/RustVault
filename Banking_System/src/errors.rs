use core::fmt;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    UnAuthorize(String),
    TooManyRequests(String),
    BadRequest(String),
    UpstreamError(String),
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Internal(msg) => write!(f, "{}", msg),
            AppError::UpstreamError(msg) => write!(f, "{}", msg),
            AppError::TooManyRequests(msg) => write!(f, "{}", msg),
            AppError::UnAuthorize(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg.clone()),
            AppError::UpstreamError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::UnAuthorize(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
        };
        tracing::warn!(status= %status,error=%message,"request failed");
        (status, Json(serde_json::json!({"error":message}))).into_response()
    }
}
