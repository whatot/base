use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub struct AppError(pub StatusCode, pub anyhow::Error);

#[derive(Serialize, Debug)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(ErrorResponse {
                status: Status::Error,
                message: self.1.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, error.into())
    }
}
