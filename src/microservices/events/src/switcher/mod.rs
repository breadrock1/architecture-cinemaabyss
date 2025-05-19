pub mod model;

use std::ops::Deref;
use std::sync::Arc;

use crate::config::ServiceConfig;
use crate::switcher::model::{CreateMovie, CreatePayment, CreateSubscription, CreateUser, MovieHealth};
use crate::switcher::model::{Movie, Payment, Subscription, User};

const HEALTH_URL: &str = "/health";
const USERS_URL: &str = "/api/users";
const MOVIES_URL: &str = "/api/movies";
const PAYMENTS_URL: &str = "/api/payments";
const SUBSCRIPTIONS_URL: &str = "/api/subscriptions";
