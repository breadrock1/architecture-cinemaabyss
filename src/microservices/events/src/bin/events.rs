use events::config::ServiceConfig;
use events::{logger, server};
use events::server::ServerApp;
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
    let brokers = config.kafka_brokers().to_owned();
    let app = Arc::new(ServerApp::new(brokers).await?);
    let app = server::init_server(app).layer(trace_layer).layer(cors_layer);
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    }

    Ok(())
}
