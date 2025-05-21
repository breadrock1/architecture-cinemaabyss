use anyhow::Error;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::server::swagger::SwaggerExamples;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("internal service error: {0}")]
    InternalError(String),
    #[error("service unavailable")]
    ServiceUnavailable,
}

impl From<anyhow::Error> for ServerError {
    fn from(value: Error) -> Self {
        ServerError::InternalError(value.to_string())
    }
}

impl ServerError {
    pub fn status_code(&self) -> (String, StatusCode) {
        match self {
            ServerError::InternalError(msg) => (msg.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::ServiceUnavailable => (
                "service unavailable".to_string(),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (msg, status) = self.status_code();
        let mut resp = Json(ErrorResponse { message: msg }).into_response();
        *resp.status_mut() = status;
        resp
    }
}

impl SwaggerExamples for ServerError {
    type Example = Self;

    fn example(value: Option<String>) -> Self::Example {
        match value {
            None => ServerError::ServiceUnavailable,
            Some(msg) => ServerError::InternalError(msg),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Success {
    status: String,
}

impl Success {
    pub fn success() -> Success {
        Success {
            status: "success".to_owned(),
        }
    }
}

impl SwaggerExamples for Success {
    type Example = Self;

    fn example(value: Option<String>) -> Self::Example {
        Success {
            status: value.unwrap_or("success".to_owned()),
        }
    }
}
