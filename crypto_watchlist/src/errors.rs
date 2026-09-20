use axum::{http::StatusCode,response::{IntoResponse,Response},Json};


#[derive(Debug)]
pub enum AppError{
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    UpstreamError(String),
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        let (status,message)=match &self{
            AppError::NotFound(msg)=> (StatusCode::NOT_FOUND,msg.clone()),
            AppError::Unauthorized(msg)=>(StatusCode::UNAUTHORIZED,msg.clone()),
            AppError::BadRequest(msg)=>(StatusCode::BAD_REQUEST,msg.clone()),
            AppError::Internal(msg)=>(StatusCode::INTERNAL_SERVER_ERROR,msg.clone()),
            AppError::UpstreamError(msg)=>(StatusCode::BAD_GATEWAY,msg.clone()),
        };

        tracing::warn!(staus =%status ,error =%message , "request failed");
        Json(serde_json::json!({"error": message})).into_response()
    }
}