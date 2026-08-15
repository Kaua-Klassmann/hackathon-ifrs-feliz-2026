use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

mod analysis;
mod auth;
mod metrics;
mod patients;
mod remedies;

pub fn configure_routes() -> Router {
    Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "") }))
        .nest("/auth", auth::configure_routes())
        .nest("/patients", patients::configure_routes())
        .nest("/remedies", remedies::configure_routes())
        .nest("/analysis", analysis::configure_routes())
        .nest("/metrics", metrics::configure_routes())
}
