use axum::Router;

use crate::{connections, middlewares, routes::configure_routes};

pub async fn create_app() -> Router {
    connections::init_connections().await;

    configure_routes()
        .layer(middlewares::cors::get_cors())
        .layer(middlewares::body_limit::get_body_limit())
        .layer(middlewares::compression::get_compression())
        .layer(middlewares::error::get_catch())
}
