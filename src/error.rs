use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use validator::ValidationErrors;
use axum::extract::rejection::FormRejection;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Form rejection error: {0}")]
    FormRejection(#[from] FormRejection),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::FormRejection(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), self.to_string()).into_response()
    }
}
