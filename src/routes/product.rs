use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{controllers::product::ProductController, services::product::ProductService};

pub fn configure_routes() -> Router {
    let product_service = Arc::new(ProductService::new());

    Router::new()
        .route("/", get(ProductController::list))
        .route("/{id}", get(ProductController::get_by_id))
        .route("/", post(ProductController::register))
        .with_state(product_service)
}
