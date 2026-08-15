use crate::utils::validate_iso_date;
use crate::{
    error::DomainError,
    jwt::JwtClaims,
    services::analysis::{AnalysisService, CreatePayload},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    entity::prelude::Date,
    sqlx::types::{Decimal, Uuid},
};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateAnalysisPayload {
    pub velocity: Decimal,
    pub time: Decimal,
    #[validate(range(min = 0, message = "Passos não podem ser negativos"))]
    pub steps: i32,
    #[validate(range(min = 0, message = "Cadência não pode ser negativa"))]
    pub cadence: i32,
    pub bounce: Decimal,
    pub time_to_up: Decimal,
    #[validate(custom(function = "validate_iso_date", message = "Data de análise inválida",))]
    pub date: String,
}

pub struct AnalysisController;

impl AnalysisController {
    pub async fn create(
        State(analysis_service): State<Arc<AnalysisService>>,
        token: JwtClaims,
        Path(id_patient): Path<Uuid>,
        Json(payload): Json<CreateAnalysisPayload>,
    ) -> impl IntoResponse {
        if let Err(err) = payload.validate() {
            return DomainError::UnprocessableEntity(err).into_response();
        }

        let result = analysis_service
            .create(CreatePayload {
                id_user: token.user_id,
                id_patient,
                velocity: payload.velocity,
                time: payload.time,
                steps: payload.steps,
                cadence: payload.cadence,
                bounce: payload.bounce,
                time_to_up: payload.time_to_up,
                date: payload.date.parse::<Date>().unwrap(),
            })
            .await;

        match result {
            Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
            Err(err) => err.into_response(),
        }
    }

    pub async fn list_by_patient_id(
        State(analysis_service): State<Arc<AnalysisService>>,
        token: JwtClaims,
        Path(id_patient): Path<Uuid>,
    ) -> impl IntoResponse {
        let result = analysis_service
            .list_by_patient_id(token.user_id, id_patient)
            .await;

        match result {
            Ok(response) => (StatusCode::OK, Json(response)).into_response(),
            Err(err) => err.into_response(),
        }
    }
}
