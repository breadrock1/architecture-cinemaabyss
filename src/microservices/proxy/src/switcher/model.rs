use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(unused_imports)]
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealth {
    #[schema(example = true)]
    status: bool,
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

fn default_payment_timestamp() -> Option<String> {
    Some(chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true))
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
