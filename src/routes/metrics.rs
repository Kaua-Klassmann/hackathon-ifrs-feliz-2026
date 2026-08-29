use crate::controllers::metrics::MetricsController;
use crate::services::metrics::MetricsService;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn configure_routes() -> Router {
    let metrics_service = Arc::new(MetricsService::new());

    Router::new()
        .route("/{id_patient}", get(MetricsController::list))
        .with_state(metrics_service)
}
