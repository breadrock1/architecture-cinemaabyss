use utoipa::OpenApi;

use crate::server::routes::*;
use crate::switcher::model::*;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "Proxy Service."
    ),
    tags(
        (
            name = "movies",
            description = "Movies API"
        ),
        (
            name = "events",
            description = "Events API"
        ),
    ),
    paths(
        health_movie,
        create_movie,
        get_movies,
        get_movie,
        delete_movie,
        create_user,
        get_users,
        get_user,
        delete_user,
        create_payment,
        get_payments,
        get_payment,
        delete_payment,
        create_subscription,
        get_subscriptions,
        get_subscription,
        delete_subscription,
    ),
    components(
        schemas(
            CreateMovie,
            Movie,
            CreatePayment,
            Payment,
            CreateUser,
            User,
            CreateSubscription,
            Subscription,
        ),
    ),
)]
pub(super) struct ApiDoc;

pub trait SwaggerExamples {
    type Example: serde::Serialize;

    fn example(value: Option<String>) -> Self::Example;
}
