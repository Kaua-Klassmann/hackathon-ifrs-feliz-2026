use axum::Router;

mod product;

pub fn configure_routes() -> Router {
    Router::new().nest("/products", product::configure_routes())
}
