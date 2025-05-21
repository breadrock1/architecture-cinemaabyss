use serde_json::Value;

use crate::switcher::helper;
use crate::switcher::model::{CreateMovie, CreatePayment, CreateUser};
use crate::switcher::model::ServiceHealth;

const EVENTS_URL: &str = "/api/events";

#[derive(Clone)]
pub struct Events {
    url: String,
}

impl Events {
    pub fn new(url: &str) -> Events {
        Events { url: url.to_owned() }
    }
}

impl Events {
    pub async fn health_events(&self) -> anyhow::Result<ServiceHealth> {
        let url = format!("{}{EVENTS_URL}/health", self.url);
        let status = helper::send_get_request::<ServiceHealth>(&url).await?;
        Ok(status)
    }

    pub async fn create_movie(&self, movie: CreateMovie) -> anyhow::Result<Value> {
        let url = format!("{}{EVENTS_URL}/movie", self.url);
        let movie = helper::send_post_request::<CreateMovie, Value>(&url, movie).await?;
        Ok(movie)
    }

    pub async fn create_user(&self, user: CreateUser) -> anyhow::Result<Value> {
        let url = format!("{}{EVENTS_URL}/user", self.url);
        let user = helper::send_post_request::<CreateUser, Value>(&url, user).await?;
        Ok(user)
    }

    pub async fn create_payment(&self, payment: CreatePayment) -> anyhow::Result<Value> {
        let url = format!("{}{EVENTS_URL}/payment", self.url);
        let payment = helper::send_post_request::<CreatePayment, Value>(&url, payment).await?;
        Ok(payment)
    }
}

