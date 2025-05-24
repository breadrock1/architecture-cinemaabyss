mod error;
mod routes;
mod swagger;

use axum::Router;

use axum::routing::{get, post};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::server::swagger::ApiDoc;
use crate::switcher::CinemaProvider;
use crate::switcher::events::Events;
use crate::switcher::monolith::Monolith;

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

    pub fn get_events(&self) -> Arc<Events> {
        self.provider.get_events()
    }

    pub fn get_monolith(&self) -> &Monolith {
        self.provider.get_monolith()
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
        // .route("/api/events/health", get(routes::health_events))
        // .route("/api/events/movie", post(routes::create_movie_by_events))
        // .route("/api/events/user", post(routes::create_user_by_events))
        // .route("/api/events/payment", post(routes::create_payment_by_events))
        .with_state(app)
}
