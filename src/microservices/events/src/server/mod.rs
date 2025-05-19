mod error;
mod routes;
mod swagger;
mod model;

use axum::Router;
use axum::routing::{get, post};
use kafka::producer::{AsBytes, Producer, Record, RequiredAcks};
use std::sync::Arc;
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::server::swagger::ApiDoc;

const OPENAPI_PATH: &str = "/api-docs/openapi.json";
const OPENAPI_URL_PATH: &str = "/rapidoc";

pub struct ServerApp {
    producer: Arc<Producer>,
}

impl ServerApp {
    pub async fn new(
        brokers: Vec<String>,
    ) -> anyhow::Result<Self> {
        let publisher = Self::create_publisher(brokers.clone()).await?;
        Ok(ServerApp {
            producer: Arc::new(publisher),
        })
    }

    pub async fn create_publisher(brokers: Vec<String>) -> anyhow::Result<Producer> {
        let producer = Producer::from_hosts(brokers)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()?;

        Ok(producer)
    }

    pub async fn publish<T>(&mut self, topic: &str, data: T) -> anyhow::Result<()>
    where
        T: AsBytes,
    {
        let rec = Record::from_value(topic, data);
        self.producer.send(&rec).await?;
        Ok(())
    }

    // pub async fn run_consumer(
    //     &self,
    //     brokers: Vec<String>,
    //     topic: String,
    //     group: String,
    // ) -> anyhow::Result<()> {
    //     let mut con = Consumer::from_hosts(brokers)
    //         .with_topic(topic)
    //         .with_group(group)
    //         .with_fallback_offset(FetchOffset::Earliest)
    //         .with_offset_storage(Some(GroupOffsetStorage::Kafka))
    //         .create()?;
    //
    //     loop {
    //         let mss = con.poll()?;
    //         if mss.is_empty() {
    //             println!("No messages available right now.");
    //             return Ok(());
    //         }
    //
    //         for ms in mss.into_iter() {
    //             for m in ms.messages() {
    //                 println!(
    //                     "{}:{}@{}: {:?}",
    //                     ms.topic(),
    //                     ms.partition(),
    //                     m.offset,
    //                     m.value
    //                 );
    //
    //
    //
    //             }
    //             let _ = con.consume_messageset(&ms);
    //         }
    //         con.commit_consumed()?;
    //     }
    // }
}

pub fn init_server(app: Arc<ServerApp>) -> Router {
    Router::new()
        .merge(RapiDoc::with_openapi(OPENAPI_PATH, ApiDoc::openapi()).path(OPENAPI_URL_PATH))
        .route("/health", get(routes::health))
        .route("/api/events/movie", post(routes::create_movie))
        .route("/api/events/user", get(routes::create_user))
        .route("/api/events/payment", get(routes::create_payment))
        .with_state(app)
}
