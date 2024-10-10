use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

pub type Result<T, Error = AppError> = anyhow::Result<T, Error>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found.")]
    NotFound,

    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unexpected error: {0}")]
    Error(#[from] anyhow::Error),
}

#[derive(Serialize)]
pub struct ErrResponse {
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrResponse {
                    message: self.to_string(),
                }),
            )
                .into_response(),

            Self::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrResponse {
                    message: self.to_string(),
                }),
            )
                .into_response(),

            Self::Error(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrResponse {
                    message: self.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
