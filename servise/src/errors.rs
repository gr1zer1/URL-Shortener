use axum::{ body::Body, http::{Response, StatusCode}, response::IntoResponse};
use thiserror::Error;


#[derive(Debug,Error)]
pub enum AppError{
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error)

}

impl IntoResponse for AppError{

    fn into_response(self) -> Response<Body>{
        
        let (code,massage) = match self{
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR,"Database Error when creating link")
        };

        (code,massage).into_response()

    }

}