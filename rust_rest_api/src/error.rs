use axum::{
    http::StatusCode,
    response::{Response,IntoResponse},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Password hashing failed")]
    PasswordHashError,

    #[error("Token creation failed")]
    TokenError,

    #[error("Unauthorized access")]
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidCredentials | AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            AppError::DatabaseError(_) | AppError::PasswordHashError | AppError::TokenError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}