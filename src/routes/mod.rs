use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

mod auth;
mod patients;
mod remedies;

pub fn configure_routes() -> Router {
    Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "") }))
        .nest("/auth", auth::configure_routes())
        .nest("/patients", patients::configure_routes())
        .nest("/remedies", remedies::configure_routes())
}
