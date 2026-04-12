use axum::{ body::Body, http::{Response, StatusCode}, response::IntoResponse};
use thiserror::Error;


#[derive(Debug,Error)]
pub enum AppError{
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error)

}

impl IntoResponse for AppError{

    fn into_response(self) -> Response<Body>{

        let (code,message) = match self{
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR,"Database Error when creating link"),
            AppError::NotFound => (StatusCode::NOT_FOUND,"Invalid data"),
            AppError::CacheError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            AppError::RedisError(_) => (StatusCode::INTERNAL_SERVER_ERROR,"Redis Error"),
            AppError::SerdeJsonError(_) => (StatusCode::INTERNAL_SERVER_ERROR,"Redis Error"),

        };

        (code,message).into_response()

    }

}
