use crate::controllers::analysis::AnalysisController;
use crate::services::analysis::AnalysisService;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn configure_routes() -> Router {
    let analysis_service = Arc::new(AnalysisService::new());

    Router::new()
        .route("/{id_patient}", get(AnalysisController::list_by_patient_id))
        .route("/{id_patient}", post(AnalysisController::create))
        .with_state(analysis_service)
}
