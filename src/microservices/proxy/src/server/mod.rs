mod error;
mod routes;
mod swagger;

use axum::Router;

use std::sync::Arc;
use axum::routing::{get, post};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::server::swagger::ApiDoc;
use crate::switcher::CinemaProvider;

const OPENAPI_PATH: &str = "/api-docs/openapi.json";
const OPENAPI_URL_PATH: &str = "/rapidoc";

pub struct ServerApp {
    provider: Arc<CinemaProvider>,
    percent: u32,
}

impl ServerApp {
    pub fn new(percent: u32, provider: Arc<CinemaProvider>) -> Self {
        ServerApp { provider, percent }
    }
}

pub fn init_server(app: Arc<ServerApp>) -> Router {
    Router::new()
        .merge(RapiDoc::with_openapi(OPENAPI_PATH, ApiDoc::openapi()).path(OPENAPI_URL_PATH))
        .route("/health", get(routes::health))
        .route("/api/movies", post(routes::create_movie).get(routes::get_movies))
        .route("/api/movies/health", get(routes::health_movie))
        .route("/api/movies/{movie_id}", get(routes::get_movie).delete(routes::delete_movie))
        .route("/api/users", post(routes::create_user).get(routes::get_users))
        .route("/api/users/{user_id}", get(routes::get_user).delete(routes::delete_user))
        .route("/api/payments", post(routes::create_payment).get(routes::get_payments))
        .route("/api/payments/{payment_id}", get(routes::get_payment).delete(routes::delete_payment))
        .route("/api/subscriptions", post(routes::create_subscription).get(routes::get_subscriptions))
        .route("/api/subscriptions/{subscription_id}", get(routes::get_subscription).delete(routes::delete_subscription))
        .with_state(app)
}
