use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;

mod product;

pub fn configure_routes() -> Router {
    Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "") }))
        .nest("/products", product::configure_routes())
}
