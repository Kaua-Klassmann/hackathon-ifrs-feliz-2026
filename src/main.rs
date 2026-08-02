use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::configs::app::get_app_config;

mod app;
mod configs;
mod connections;
mod controllers;
mod entities;
mod error;
mod middlewares;
mod repositories;
mod routes;
mod services;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(false)
                .with_span_list(false),
        )
        .with(tracing_subscriber::filter::Targets::new().with_target("api", tracing::Level::INFO))
        .init();

    let app = app::create_app().await;

    let port = get_app_config().port;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running on port {}", port);

    axum::serve(listener, app).await.unwrap()
}
