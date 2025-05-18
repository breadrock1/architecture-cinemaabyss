use crate::config::ServiceConfig;
use crate::switcher::model::{CreateMovie, CreatePayment, CreateSubscription, CreateUser, Movie, MovieHealth, Payment, Subscription, User};
use crate::switcher::{helper, APIProvider, HealthProvider, MovieProvider, PaymentProvider, SubscriptionProvider, UserProvider};
use crate::switcher::{MOVIES_URL, HEALTH_URL, PAYMENTS_URL, SUBSCRIPTIONS_URL, USERS_URL};

#[derive(Clone)]
pub struct Distributed {
    movie_url: String,
    user_url: String,
    payment_url: String,
    subscription_url: String,
}

impl Distributed {
    pub fn new(config: &ServiceConfig) -> Self {
        Distributed {
            movie_url: config.movies_service_url().to_owned(),
            user_url: config.movies_service_url().to_owned(),
            payment_url: config.movies_service_url().to_owned(),
            subscription_url: config.movies_service_url().to_owned(),
        }
    }
}

impl APIProvider for Distributed {}

#[async_trait::async_trait]
impl HealthProvider for Distributed {
    async fn health(&self) -> anyhow::Result<()> {
        let url = format!("{}{}", self.movie_url, HEALTH_URL);
        helper::send_get_request::<()>(&url).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl MovieProvider for Distributed {
    async fn health_movie(&self) -> anyhow::Result<MovieHealth> {
        let url = format!("{}/health", self.movie_url);
        let status = helper::send_get_request::<MovieHealth>(&url).await?;
        Ok(status)
    }

    async fn create_movie(&self, movie: CreateMovie) -> anyhow::Result<Movie> {
        let url = format!("{}{}", self.movie_url, MOVIES_URL);
        let movie = helper::send_post_request::<CreateMovie, Movie>(&url, movie).await?;
        Ok(movie)
    }

    async fn get_movies(&self) -> anyhow::Result<Vec<Movie>> {
        let url = format!("{}{}", self.movie_url, MOVIES_URL);
        let movies = helper::send_get_request::<Vec<Movie>>(&url).await?;
        Ok(movies)
    }

    async fn get_movie(&self, movie_id: i32) -> anyhow::Result<Movie> {
        let url = format!("{}{}/{}", &self.movie_url, MOVIES_URL, movie_id);
        let movie = helper::send_get_request::<Movie>(&url).await?;
        Ok(movie)
    }

    async fn delete_movie(&self, movie_id: i32) -> anyhow::Result<()> {
        let url = format!("{}{}/{}", &self.movie_url, MOVIES_URL, movie_id);
        helper::send_delete_request(&url).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserProvider for Distributed {
    async fn create_user(&self, user: CreateUser) -> anyhow::Result<User> {
        let url = format!("{}{}", self.user_url, USERS_URL);
        let user = helper::send_post_request::<CreateUser, User>(&url, user).await?;
        Ok(user)
    }

    async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        let url = format!("{}{}", self.user_url, USERS_URL);
        let users = helper::send_get_request::<Vec<User>>(&url).await?;
        Ok(users)
    }

    async fn get_user(&self, user_id: i32) -> anyhow::Result<User> {
        let url = format!("{}{}/{}", &self.user_url, USERS_URL, user_id);
        let user = helper::send_get_request::<User>(&url).await?;
        Ok(user)
    }

    async fn delete_user(&self, user_id: i32) -> anyhow::Result<()> {
        let url = format!("{}{}/{}", &self.user_url, USERS_URL, user_id);
        helper::send_delete_request(&url).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl PaymentProvider for Distributed {
    async fn create_payment(&self, payment: CreatePayment) -> anyhow::Result<Payment> {
        let url = format!("{}{}", self.payment_url, PAYMENTS_URL);
        let payment = helper::send_post_request::<CreatePayment, Payment>(&url, payment).await?;
        Ok(payment)
    }

    async fn get_payments(&self) -> anyhow::Result<Vec<Payment>> {
        let url = format!("{}{}", self.payment_url, PAYMENTS_URL);
        let payments = helper::send_get_request::<Vec<Payment>>(&url).await?;
        Ok(payments)
    }

    async fn get_payment(&self, payment_id: i32) -> anyhow::Result<Payment> {
        let url = format!("{}{}/{}", &self.payment_url, PAYMENTS_URL, payment_id);
        let payment = helper::send_get_request::<Payment>(&url).await?;
        Ok(payment)
    }

    async fn delete_payment(&self, payment_id: i32) -> anyhow::Result<()> {
        let url = format!("{}{}/{}", &self.payment_url, PAYMENTS_URL, payment_id);
        helper::send_delete_request(&url).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl SubscriptionProvider for Distributed {
    async fn create_subscription(&self, subscription: CreateSubscription) -> anyhow::Result<Subscription> {
        let url = format!("{}{}", self.subscription_url, SUBSCRIPTIONS_URL);
        let subscription = helper::send_post_request::<CreateSubscription, Subscription>(&url, subscription).await?;
        Ok(subscription)
    }

    async fn get_subscriptions(&self) -> anyhow::Result<Vec<Subscription>> {
        let url = format!("{}{}", self.subscription_url, SUBSCRIPTIONS_URL);
        let subscriptions = helper::send_get_request::<Vec<Subscription>>(&url).await?;
        Ok(subscriptions)
    }

    async fn get_subscription(&self, subscription_id: i32) -> anyhow::Result<Subscription> {
        let url = format!("{}{}/{}", &self.subscription_url, SUBSCRIPTIONS_URL, subscription_id);
        let subscription = helper::send_get_request::<Subscription>(&url).await?;
        Ok(subscription)
    }

    async fn delete_subscription(&self, subscription_id: i32) -> anyhow::Result<()> {
        let url = format!("{}{}/{}", &self.subscription_url, SUBSCRIPTIONS_URL, subscription_id);
        helper::send_delete_request(&url).await?;
        Ok(())
    }
}
