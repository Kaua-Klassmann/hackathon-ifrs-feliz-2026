use tokio::net::TcpListener;

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

    let app = app::create_app().await;

    let port = get_app_config().port;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running on port {}", port);

    axum::serve(listener, app).await.unwrap()
}
