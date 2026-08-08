use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{controllers::auth::AuthController, services::auth::AuthService};

pub fn configure_routes() -> Router {
    let auth_service = Arc::new(AuthService::new());

    Router::new()
        .route("/register", post(AuthController::register))
        .route("/login", post(AuthController::login))
        .with_state(auth_service)
}
