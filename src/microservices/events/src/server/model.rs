use chrono::{DateTime, SecondsFormat, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(unused_imports)]
use serde_json::json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealth {
    #[schema(example = true)]
    status: bool,
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self { status: true }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
    Movie,
    User,
    Payment,
}

#[derive(Builder, Clone, Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Event {
    #[schema(example = "795732d4-ddc3-4ce8-8f59-5c35fc43066e")]
    id: String,
    #[serde(rename = "type")]
    #[schema(example = "Movie")]
    event_type: EventType,
    #[schema(example = "2025-01-15T14:30:00Z")]
    timestamp: DateTime<Utc>,
    #[schema(example = "{}")]
    payload: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum EventStatus {
    Success,
    Failed,
}

#[derive(Builder, Clone, Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct EventResponse {
    #[schema(example = "Success")]
    status: EventStatus,
    #[schema(example = "movies-partition")]
    partition: String,
    #[schema(example = 1)]
    offset: i32,
    event: Event,
}

impl EventResponse {
    pub fn builder() -> EventResponseBuilder {
        EventResponseBuilder::default()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Movie {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = 9.0)]
    rating: f64,
    #[schema(example = "Star Wars: Here we go again")]
    title: String,
    #[schema(example = "Star Wars movie")]
    description: String,
    #[schema(example = json!(vec!["Action, Adventure, Fantasy"]))]
    genres: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateMovie {
    #[schema(example = 9.0)]
    rating: f64,
    #[schema(example = "Star Wars: Here we go again")]
    title: String,
    #[schema(example = "Star Wars movie")]
    description: String,
    #[schema(example = json!(vec!["Action, Adventure, Fantasy"]))]
    genres: Option<Vec<String>>,
}

impl TryFrom<CreateMovie> for Event {
    type Error = anyhow::Error;

    fn try_from(value: CreateMovie) -> Result<Self, Self::Error> {
        let value = serde_json::to_value(value)?;
        let event = EventBuilder::default()
            .id(uuid::Uuid::new_v4().to_string())
            .event_type(EventType::Movie)
            .timestamp(Utc::now())
            .payload(value)
            .build()?;

        Ok(event)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = "user")]
    username: String,
    #[schema(example = "user@email.net")]
    email: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateUser {
    #[schema(example = "user")]
    username: String,
    #[schema(example = "user@email.net")]
    email: String,
}

impl TryFrom<CreateUser> for Event {
    type Error = anyhow::Error;

    fn try_from(value: CreateUser) -> Result<Self, Self::Error> {
        let value = serde_json::to_value(value)?;
        let event = EventBuilder::default()
            .id(uuid::Uuid::new_v4().to_string())
            .event_type(EventType::Movie)
            .timestamp(Utc::now())
            .payload(value)
            .build()?;

        Ok(event)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Payment {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = 1)]
    user_id: i32,
    #[schema(example = 19.0)]
    amount: f32,
    #[schema(example = "2025-01-15T14:30:00Z")]
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreatePayment {
    #[schema(example = 1)]
    user_id: i32,
    #[schema(example = 19.0)]
    amount: f32,
    #[schema(example = "2025-01-15T14:30:00Z")]
    #[serde(default = "default_payment_timestamp")]
    timestamp: Option<String>,
}

impl TryFrom<CreatePayment> for Event {
    type Error = anyhow::Error;

    fn try_from(value: CreatePayment) -> Result<Self, Self::Error> {
        let value = serde_json::to_value(value)?;
        let event = EventBuilder::default()
            .id(uuid::Uuid::new_v4().to_string())
            .event_type(EventType::Movie)
            .timestamp(Utc::now())
            .payload(value)
            .build()?;

        Ok(event)
    }
}

fn default_payment_timestamp() -> Option<String> {
    Some(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Subscription {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = 1)]
    user_id: i32,
    #[schema(example = "premium")]
    plan_type: String,
    #[schema(example = "1980-01-15T14:30:00Z")]
    start_date: String,
    #[schema(example = "2025-01-15T14:30:00Z")]
    end_date: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateSubscription {
    #[schema(example = 1)]
    user_id: i32,
    #[schema(example = "premium")]
    plan_type: String,
    #[schema(example = "1980-01-15T14:30:00Z")]
    start_date: String,
    #[schema(example = "2025-01-15T14:30:00Z")]
    end_date: String,
}
