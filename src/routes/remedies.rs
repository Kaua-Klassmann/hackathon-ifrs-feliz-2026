use crate::controllers::remedies::RemediesController;
use crate::services::remedies::RemediesService;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn configure_routes() -> Router {
    let remedies_service = Arc::new(RemediesService::new());

    Router::new()
        .route("/", get(RemediesController::list))
        .with_state(remedies_service)
}
