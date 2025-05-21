mod error;
mod routes;
mod swagger;
pub mod model;

use axum::Router;
use axum::routing::{get, post};
use futures_util::StreamExt;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::{BorrowedMessage, ToBytes};
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig, Message};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::server::swagger::ApiDoc;

const OPENAPI_PATH: &str = "/api-docs/openapi.json";
const OPENAPI_URL_PATH: &str = "/rapidoc";

const EVENTS_TOPICS: &[&str] = &["movie-events", "user-events", "payment-events"];

#[derive(Clone)]
pub struct ServerApp {
    producer: Arc<Mutex<FutureProducer>>,
}

impl ServerApp {
    pub async fn new(brokers: String) -> anyhow::Result<Self> {
        let publisher = Self::create_publisher(&brokers).await?;
        tokio::spawn(async move { Self::start_consumer(brokers.to_owned()).await });
        Ok(ServerApp {
            producer: Arc::new(Mutex::new(publisher)),
        })
    }

    pub async fn create_publisher(brokers: &String) -> anyhow::Result<FutureProducer> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()?;

        Ok(producer)
    }

    pub async fn start_consumer(brokers: String) -> anyhow::Result<()> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "360000")
            .set("enable.auto.commit", "true")
            .set("group.id", "cinemaabyss")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()?;

        consumer.subscribe(EVENTS_TOPICS)?;

        let mut stream = consumer.stream();
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(msg) => {
                    Self::process_message(&msg).await;
                    consumer.commit_message(&msg, CommitMode::Async)?;
                }
                Err(err) => {
                    tracing::error!(err=?err, "error receiving message from kafka");
                },
            }
        }

        Ok(())
    }

    pub async fn publish<T>(&self, topic: &str, data: &T) -> anyhow::Result<(i32, i64)>
    where
        T: ToBytes + ?Sized,
    {
        let key = format!("{topic}-key");
        let timeout = Timeout::After(Duration::from_secs(5));
        let record = FutureRecord::to(topic)
            .key(&key)
            .payload(data);

        let guard = self.producer.lock().await;
        let result = guard
            .send(record, timeout)
            .await
            .map_err(|(err, _)| anyhow::Error::from(err))?;

        Ok(result)
    }

    async fn process_message(msg: &BorrowedMessage<'_>) {
        tracing::info!(
            key=?msg.key(),
            partition=?msg.partition(),
            offset=?msg.offset(),
            time=?msg.timestamp(),
            payload=?msg.payload(),
            "received new message from kafka",
        );
    }
}

pub fn init_server(app: Arc<ServerApp>) -> Router {
    Router::new()
        .merge(RapiDoc::with_openapi(OPENAPI_PATH, ApiDoc::openapi()).path(OPENAPI_URL_PATH))
        .route("/api/events/health", get(routes::health))
        .route("/api/events/movie", post(routes::create_movie))
        .route("/api/events/user", get(routes::create_user))
        .route("/api/events/payment", get(routes::create_payment))
        .with_state(app)
}
