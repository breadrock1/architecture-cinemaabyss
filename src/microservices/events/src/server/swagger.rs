use utoipa::OpenApi;

use crate::server::routes::*;
use crate::server::model::*;

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
        health,
        create_movie,
        create_user,
        create_payment,
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
