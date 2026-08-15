use std::sync::Arc;

use crate::{
    entities::sea_orm_active_enums::PatientBloodType, jwt::JwtClaims,
    repositories::patients::CreatePatientRemediesPayload, utils::validate_iso_date,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::entity::prelude::{Date, Uuid};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::DomainError, repositories::patients::CreatePatientPayload,
    services::patients::PatientsService,
};

#[derive(Deserialize, Validate)]
pub struct CreatePayload {
    #[validate(length(min = 3, message = "Nome não pode contar menos de 3 caracteres"))]
    name: String,
    #[validate(custom(
        function = "validate_iso_date",
        message = "Data de nascimento inválida",
    ))]
    birth_date: String,
    blood_type: PatientBloodType,
    is_male: bool,
    remedies: Vec<CreateRemediesPayload>,
}

#[derive(Deserialize, Validate)]
pub struct CreateRemediesPayload {
    remedy: Uuid,
    #[validate(range(min = 0, message = "Quantidade deve ser no minimo 0"))]
    quantity: i32,
}

pub struct PatientsController;

impl PatientsController {
    pub async fn create(
        State(service): State<Arc<PatientsService>>,
        token: JwtClaims,
        Json(payload): Json<CreatePayload>,
    ) -> impl IntoResponse {
        if let Err(errors) = payload.validate() {
            return DomainError::UnprocessableEntity(errors).into_response();
        }

        let result = service
            .create(CreatePatientPayload {
                name: payload.name,
                birthdate: payload.birth_date.parse::<Date>().unwrap(),
                blood_type: payload.blood_type,
                id_user: token.user_id,
                is_male: payload.is_male,
                remedies: payload
                    .remedies
                    .into_iter()
                    .map(|r| CreatePatientRemediesPayload {
                        remedy: r.remedy,
                        quantity: r.quantity,
                    })
                    .collect::<Vec<CreatePatientRemediesPayload>>(),
            })
            .await;

        match result {
            Ok(patient_id) => (StatusCode::CREATED, Json(patient_id)).into_response(),
            Err(err) => err.into_response(),
        }
    }

    pub async fn update_remedies(
        State(service): State<Arc<PatientsService>>,
        token: JwtClaims,
        Path(patient_id): Path<Uuid>,
        Json(payload): Json<Vec<CreateRemediesPayload>>,
    ) -> impl IntoResponse {
        if let Err(errors) = payload.validate() {
            return DomainError::UnprocessableEntity(errors).into_response();
        }

        let result = service
            .update_remedies(
                token.user_id,
                patient_id,
                payload
                    .into_iter()
                    .map(|r| CreatePatientRemediesPayload {
                        remedy: r.remedy,
                        quantity: r.quantity,
                    })
                    .collect::<Vec<CreatePatientRemediesPayload>>(),
            )
            .await;

        match result {
            Ok(_) => (StatusCode::NO_CONTENT, Json(())).into_response(),
            Err(err) => err.into_response(),
        }
    }

    pub async fn delete(
        State(service): State<Arc<PatientsService>>,
        token: JwtClaims,
        Path(patient_id): Path<Uuid>,
    ) -> impl IntoResponse {
        let result = service.delete(token.user_id, patient_id).await;

        match result {
            Ok(_) => (StatusCode::NO_CONTENT, Json(())).into_response(),
            Err(err) => err.into_response(),
        }
    }
}
