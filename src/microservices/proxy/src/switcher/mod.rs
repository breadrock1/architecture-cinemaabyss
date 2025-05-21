pub mod distributed;
pub mod model;
pub mod monolith;
pub mod helper;
pub mod events;

use std::ops::Deref;
use std::sync::Arc;

use crate::switcher::distributed::Distributed;
use crate::switcher::events::Events;
use crate::switcher::model::{CreateMovie, CreatePayment, CreateSubscription, CreateUser};
use crate::switcher::model::{Movie, Payment, ServiceHealth, Subscription, User};
use crate::switcher::monolith::Monolith;

const HEALTH_URL: &str = "/health";
const USERS_URL: &str = "/api/users";
const MOVIES_URL: &str = "/api/movies";
const PAYMENTS_URL: &str = "/api/payments";
const SUBSCRIPTIONS_URL: &str = "/api/subscriptions";

#[async_trait::async_trait]
pub trait HealthProvider {
    async fn health(&self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait MovieProvider {
    async fn health_movie(&self) -> anyhow::Result<ServiceHealth>;
    async fn create_movie(&self, movie: CreateMovie) -> anyhow::Result<Movie>;
    async fn get_movies(&self) -> anyhow::Result<Vec<Movie>>;
    async fn get_movie(&self, movie_id: i32) -> anyhow::Result<Movie>;
    async fn delete_movie(&self, movie_id: i32) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait UserProvider {
    async fn create_user(&self, user: CreateUser) -> anyhow::Result<User>;
    async fn get_users(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user(&self, user_id: i32) -> anyhow::Result<User>;
    async fn delete_user(&self, user_id: i32) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait PaymentProvider {
    async fn create_payment(&self, payment: CreatePayment) -> anyhow::Result<Payment>;
    async fn get_payments(&self) -> anyhow::Result<Vec<Payment>>;
    async fn get_payment(&self, payment_id: i32) -> anyhow::Result<Payment>;
    async fn delete_payment(&self, payment_id: i32) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait SubscriptionProvider {
    async fn create_subscription(&self, subscription: CreateSubscription) -> anyhow::Result<Subscription>;
    async fn get_subscriptions(&self) -> anyhow::Result<Vec<Subscription>>;
    async fn get_subscription(&self, subscription_id: i32) -> anyhow::Result<Subscription>;
    async fn delete_subscription(&self, subscription_id: i32) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait APIProvider: MovieProvider + UserProvider + PaymentProvider + SubscriptionProvider {

}

#[derive(Clone)]
pub struct CinemaProvider {
    monolith: Arc<Monolith>,
    distributed: Arc<Distributed>,
    events: Arc<Events>,
}

impl CinemaProvider {
    pub fn new(
        mono: Arc<Monolith>,
        distr: Arc<Distributed>,
        events: Arc<Events>,
    ) -> Self {
        CinemaProvider {
            monolith: mono,
            distributed: distr,
            events: events.to_owned(),
        }
    }

    pub fn choose_cinema_provider(&self, percent: u32) -> &dyn APIProvider {
        match rand::random::<u32>() >= percent {
            true => self.monolith.deref(),
            false => self.distributed.deref(),
        }
    }

    pub fn get_events(&self) -> Arc<Events> {
        self.events.clone()
    }
}
