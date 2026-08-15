use std::sync::Arc;

use crate::{jwt::JwtClaims, services::metrics::MetricsService};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::sqlx::types::Uuid;

pub struct MetricsController;

impl MetricsController {
    pub async fn list(
        State(service): State<Arc<MetricsService>>,
        _: JwtClaims,
        Path(patient_id): Path<Uuid>,
    ) -> impl IntoResponse {
        let result = service.get_metrics(patient_id).await;

        match result {
            Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
            Err(err) => err.into_response(),
        }
    }
}
