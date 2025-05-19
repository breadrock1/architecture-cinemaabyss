use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use std::sync::Arc;

use crate::server::error::{ServerError, ServerResult, Success};
use crate::server::ServerApp;
use crate::server::swagger::SwaggerExamples;
use crate::switcher::model::{CreateMovie, CreatePayment, CreateSubscription, CreateUser, Movie, Payment, Subscription, User};

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
    path = "/api/movies/health",
    tag = "movies",
    responses(
        (
            status = 200,
            body = Success,
            content_type="application/json",
            description = "Movie service is healthy",
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
pub async fn health_movie(
    State(state): State<Arc<ServerApp>>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let movie = provider.health_movie().await?;
    Ok(Json(movie))
}

#[utoipa::path(
    post,
    path = "/api/movies",
    tag = "movies",
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
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreateMovie>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let movie = provider.create_movie(form).await?;
    Ok(Json(movie))
}

#[utoipa::path(
    get,
    path = "/api/movies",
    tag = "movies",
    responses(
        (
            status = 200,
            body = Vec<Movie>,
            content_type="application/json",
            description = "Returns all movies",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting movies",
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
pub async fn get_movies(
    State(state): State<Arc<ServerApp>>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let movies = provider.get_movies().await?;
    Ok(Json(movies))
}

#[utoipa::path(
    get,
    path = "/api/movies/{movie_id}",
    tag = "movies",
    params(
        (
            "movie_id" = i32,
            description = "Movie id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Vec<Movie>,
            content_type="application/json",
            description = "Returns movie by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting movie by id",
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
pub async fn get_movie(
    State(state): State<Arc<ServerApp>>,
    Path(movie_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let movie = provider.get_movie(movie_id).await?;
    Ok(Json(movie))
}

#[utoipa::path(
    delete,
    path = "/api/movies/{movie_id}",
    tag = "movies",
    params(
        (
            "movie_id" = i32,
            description = "Movie id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Success,
            content_type="application/json",
            description = "Deletes movie by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while deleting movie",
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
pub async fn delete_movie(
    State(state): State<Arc<ServerApp>>,
    Path(movie_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    provider.delete_movie(movie_id).await?;
    Ok(Json(Success::success()))
}

#[utoipa::path(
    post,
    path = "/api/users",
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
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreateUser>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let user = provider.create_user(form).await?;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    responses(
        (
            status = 200,
            body = Vec<User>,
            content_type="application/json",
            description = "Returns all users",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting users",
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
pub async fn get_users(
    State(state): State<Arc<ServerApp>>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let users = provider.get_users().await?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    tag = "users",
    params(
        (
            "user_id" = i32,
            description = "User id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Vec<User>,
            content_type="application/json",
            description = "Returns user by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting user by id",
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
pub async fn get_user(
    State(state): State<Arc<ServerApp>>,
    Path(user_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let user = provider.get_user(user_id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    tag = "users",
    params(
        (
            "user_id" = i32,
            description = "User id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Success,
            content_type="application/json",
            description = "Deletes user by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while deleting user",
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
pub async fn delete_user(
    State(state): State<Arc<ServerApp>>,
    Path(user_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    provider.delete_user(user_id).await?;
    Ok(Json(Success::success()))
}

#[utoipa::path(
    post,
    path = "/api/payments",
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
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreatePayment>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let payment = provider.create_payment(form).await?;
    Ok(Json(payment))
}

#[utoipa::path(
    get,
    path = "/api/payments",
    tag = "payments",
    responses(
        (
            status = 200,
            body = Vec<Payment>,
            content_type="application/json",
            description = "Returns all payments",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting payments",
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
pub async fn get_payments(
    State(state): State<Arc<ServerApp>>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let payments = provider.get_payments().await?;
    Ok(Json(payments))
}

#[utoipa::path(
    get,
    path = "/api/payments/{payment_id}",
    tag = "payments",
    params(
        (
            "payment_id" = i32,
            description = "Payment id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Vec<Payment>,
            content_type="application/json",
            description = "Returns payment by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting payment by id",
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
pub async fn get_payment(
    State(state): State<Arc<ServerApp>>,
    Path(payment_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let payment = provider.get_payment(payment_id).await?;
    Ok(Json(payment))
}

#[utoipa::path(
    delete,
    path = "/api/payments/{payment_id}",
    tag = "payments",
    params(
        (
            "payment_id" = i32,
            description = "Payment id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Success,
            content_type="application/json",
            description = "Deletes payment by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while deleting payment",
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
pub async fn delete_payment(
    State(state): State<Arc<ServerApp>>,
    Path(payment_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    provider.delete_payment(payment_id).await?;
    Ok(Json(Success::success()))
}

#[utoipa::path(
    post,
    path = "/api/subscriptions",
    tag = "subscriptions",
    request_body(
        content = CreateSubscription,
    ),
    responses(
        (
            status = 200,
            body = Subscription,
            content_type="application/json",
            description = "Subscription has been created",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while creating subscription",
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
pub async fn create_subscription(
    State(state): State<Arc<ServerApp>>,
    Json(form): Json<CreateSubscription>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let subscription = provider.create_subscription(form).await?;
    Ok(Json(subscription))
}

#[utoipa::path(
    get,
    path = "/api/subscriptions",
    tag = "subscriptions",
    responses(
        (
            status = 200,
            body = Vec<Subscription>,
            content_type="application/json",
            description = "Returns all subscriptions",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting subscriptions",
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
pub async fn get_subscriptions(
    State(state): State<Arc<ServerApp>>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let subscriptions = provider.get_subscriptions().await?;
    Ok(Json(subscriptions))
}

#[utoipa::path(
    get,
    path = "/api/subscriptions/{subscription_id}",
    tag = "subscriptions",
    params(
        (
            "subscription_id" = i32,
            description = "Subscription id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Vec<Subscription>,
            content_type="application/json",
            description = "Returns subscription by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while getting subscription by id",
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
pub async fn get_subscription(
    State(state): State<Arc<ServerApp>>,
    Path(subscription_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    let subscription = provider.get_subscription(subscription_id).await?;
    Ok(Json(subscription))
}

#[utoipa::path(
    delete,
    path = "/api/subscriptions/{subscription_id}",
    tag = "subscriptions",
    params(
        (
            "subscription_id" = i32,
            description = "Subscription id",
            example = 1,
        ),
    ),
    responses(
        (
            status = 200,
            body = Success,
            content_type="application/json",
            description = "Deletes subscription by id",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while deleting subscription",
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
pub async fn delete_subscription(
    State(state): State<Arc<ServerApp>>,
    Path(subscription_id): Path<i32>,
) -> ServerResult<impl IntoResponse> {
    let provider = state.provider.choose_cinema_provider(state.percent);
    provider.delete_subscription(subscription_id).await?;
    Ok(Json(Success::success()))
}
