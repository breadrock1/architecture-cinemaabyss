use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::server::error::{ServerError, ServerResult, Success};
use crate::server::model::{CreateMovie, CreatePayment, CreateUser};
use crate::server::model::{Event, ServiceHealth};
use crate::server::swagger::SwaggerExamples;
use crate::server::ServerApp;

#[utoipa::path(
    get,
    path = "/api/events/health",
    tag = "health",
    responses(
        (
            status = 200,
            body = Success,
            content_type="application/json",
            description = "Proxy service is healthy",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while get service health",
            example = json!(ServerError::example(Some("internal error".to_string()))),
        ),
        (
            status = 503,
            body = ServerError,
            description = "Server does not available",
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn health() -> ServerResult<impl IntoResponse> {
    Ok(Json(ServiceHealth::default()))
}

#[utoipa::path(
    post,
    path = "/api/events/movie",
    tag = "events",
    request_body(
        content = CreateMovie,
    ),
    responses(
        (
            status = 201,
            body = Success,
            content_type="application/json",
            description = "Success",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while creating movie",
            example = json!(ServerError::example(Some("internal error".to_string()))),
        ),
        (
            status = 503,
            body = ServerError,
            description = "Server does not available",
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn create_movie(
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreateMovie>,
) -> ServerResult<impl IntoResponse> {
    let event = Event::try_from(form)?;
    let bytes = serde_json::to_string(&event).unwrap();
    state.publish("movie-events", bytes.as_bytes()).await?;
    Ok((StatusCode::CREATED, Json(Success::success())).into_response())
}

#[utoipa::path(
    post,
    path = "/api/events/user",
    tag = "users",
    request_body(
        content = CreateUser,
    ),
    responses(
        (
            status = 201,
            body = Success,
            content_type="application/json",
            description = "Success",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while creating user",
            example = json!(ServerError::example(Some("internal error".to_string()))),
        ),
        (
            status = 503,
            body = ServerError,
            description = "Server does not available",
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn create_user(
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreateUser>,
) -> ServerResult<impl IntoResponse> {
    let event = Event::try_from(form)?;
    let bytes = serde_json::to_string(&event).unwrap();
    state.publish("user-events", bytes.as_bytes()).await.unwrap();
    Ok((StatusCode::CREATED, Json(Success::success())).into_response())
}

#[utoipa::path(
    post,
    path = "/api/events/payment",
    tag = "payments",
    request_body(
        content = CreatePayment,
    ),
    responses(
        (
            status = 201,
            body = Success,
            content_type="application/json",
            description = "Success",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while creating payment",
            example = json!(ServerError::example(Some("internal error".to_string()))),
        ),
        (
            status = 503,
            body = ServerError,
            description = "Server does not available",
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn create_payment(
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreatePayment>,
) -> ServerResult<impl IntoResponse> {
    let event = Event::try_from(form)?;
    let bytes = serde_json::to_string(&event).unwrap();
    state.publish("payment-events", bytes.as_bytes()).await.unwrap();
    Ok((StatusCode::CREATED, Json(Success::success())).into_response())
}
