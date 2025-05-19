use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;

use crate::server::error::{ServerError, ServerResult, Success};
use crate::server::ServerApp;
use crate::server::swagger::SwaggerExamples;
use crate::switcher::model::{CreateMovie, CreatePayment, CreateUser, Movie, Payment, User};

#[utoipa::path(
    post,
    path = "/health",
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
    Ok(Json(Success::success()))
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
            status = 200,
            body = Movie,
            content_type="application/json",
            description = "Movie has been created",
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
    State(mut state): State<Arc<ServerApp>>,
    Json(form): Json<CreateMovie>,
) -> ServerResult<impl IntoResponse> {
    state.publish("movie", form).await?;
    Ok(Json(Success::success()))
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
            status = 200,
            body = User,
            content_type="application/json",
            description = "User has been created",
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
    State(mut state): State<Arc<ServerApp>>,
    Json(form): Json<CreateUser>,
) -> ServerResult<impl IntoResponse> {
    state.publish("users", form).await?;
    Ok(Json(Success::success()))
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
            status = 200,
            body = Payment,
            content_type="application/json",
            description = "Payment has been created",
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
    State(mut state): State<Arc<ServerApp>>,
    Json(form): Json<CreatePayment>,
) -> ServerResult<impl IntoResponse> {
    state.publish("payments", form).await?;
    Ok(Json(Success::success()))
}
