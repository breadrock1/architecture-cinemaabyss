use crate::switcher::{helper, MOVIES_URL, PAYMENTS_URL, USERS_URL};
use crate::switcher::model::{CreateMovie, CreatePayment, CreateUser, Movie, MovieHealth, Payment, User};

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
    pub async fn health_events(&self) -> anyhow::Result<MovieHealth> {
        let url = format!("{}/health", self.url);
        let status = helper::send_get_request::<MovieHealth>(&url).await?;
        Ok(status)
    }

    pub async fn create_movie(&self, movie: CreateMovie) -> anyhow::Result<Movie> {
        let url = format!("{}{}", self.url, MOVIES_URL);
        let movie = helper::send_post_request::<CreateMovie, Movie>(&url, movie).await?;
        Ok(movie)
    }

    pub async fn create_user(&self, user: CreateUser) -> anyhow::Result<User> {
        let url = format!("{}{}", self.url, USERS_URL);
        let user = helper::send_post_request::<CreateUser, User>(&url, user).await?;
        Ok(user)
    }

    pub async fn create_payment(&self, payment: CreatePayment) -> anyhow::Result<Payment> {
        let url = format!("{}{}", self.url, PAYMENTS_URL);
        let payment = helper::send_post_request::<CreatePayment, Payment>(&url, payment).await?;
        Ok(payment)
    }
}

