use std::sync::Arc;

use axum::routing::{delete, put};
use axum::{Router, routing::post};

use crate::controllers::patients::PatientsController;
use crate::services::patients::PatientsService;

pub fn configure_routes() -> Router {
    let patients_service = Arc::new(PatientsService::new());

    Router::new()
        .route("/", post(PatientsController::create))
        .route(
            "/{patient_id}/remedies",
            put(PatientsController::update_remedies),
        )
        .route("/{patient_id}", delete(PatientsController::delete))
        .with_state(patients_service)
}
