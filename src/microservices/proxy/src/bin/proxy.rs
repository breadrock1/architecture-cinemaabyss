use proxy::config::ServiceConfig;
use proxy::{logger, server};
use proxy::server::ServerApp;
use proxy::switcher::distributed::Distributed;
use proxy::switcher::events::Events;
use proxy::switcher::monolith::Monolith;
use proxy::switcher::CinemaProvider;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors, trace};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    logger::init_logger(&config)?;

    let address = format!("{}:{}", config.address(), config.port());
    let listener = TcpListener::bind(address).await?;
    let cors_layer = cors::CorsLayer::permissive();
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    tracing::debug!(config=?config, "launching events service with config");
    let events = Arc::new(Events::new(config.events_service_url()));
    let mono = Arc::new(Monolith::new(config.monolith_url()));
    let distr = Arc::new(Distributed::new(&config));
    let percent = config.movies_migration_percent().clone();
    let provider = Arc::new(CinemaProvider::new(mono, distr, events));
    let server_app = Arc::new(ServerApp::new(percent, provider));
    let app = server::init_server(server_app).layer(trace_layer).layer(cors_layer);
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    }

    Ok(())
}
