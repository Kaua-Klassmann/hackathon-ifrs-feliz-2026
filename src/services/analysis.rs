use sea_orm::{
    entity::prelude::Date,
    sea_query::value::prelude::rust_decimal::prelude::ToPrimitive,
    sqlx::types::{Decimal, Uuid, chrono::Utc},
};
use serde::Serialize;

use crate::{
    error::DomainError,
    repositories::{
        analysis::{
            AnalysisRepository, AnalysisRepositoryTrait, CreateAnalysisPayload, ListAnalysisPayload,
        },
        metrics::{MetricsRepository, MetricsRepositoryTrait},
        patients::{PatientsRepository, PatientsRepositoryTrait},
    },
};

pub struct CreatePayload {
    pub id_user: Uuid,
    pub id_patient: Uuid,
    pub velocity: Decimal,
    pub time: Decimal,
    pub steps: i32,
    pub cadence: i32,
    pub bounce: Decimal,
    pub time_to_up: Decimal,
    pub date: Date,
}

#[derive(Serialize)]
pub struct CreateResponse {
    velocity_diff: Option<f32>,
    time_diff: Option<f32>,
    steps_diff: Option<f32>,
    cadence_diff: Option<f32>,
    bounce_diff: Option<f32>,
    time_to_up_diff: Option<f32>,
}

pub struct AnalysisService<
    AR: AnalysisRepositoryTrait = AnalysisRepository,
    PR: PatientsRepositoryTrait = PatientsRepository,
    MR: MetricsRepositoryTrait = MetricsRepository,
> {
    pub analysis_repository: AR,
    pub patients_repository: PR,
    pub metrics_repository: MR,
}

impl AnalysisService<AnalysisRepository> {
    pub fn new() -> Self {
        Self {
            analysis_repository: AnalysisRepository::new(),
            patients_repository: PatientsRepository::new(),
            metrics_repository: MetricsRepository::new(),
        }
    }
}

impl<AR: AnalysisRepositoryTrait, PR: PatientsRepositoryTrait, MR: MetricsRepositoryTrait>
    AnalysisService<AR, PR, MR>
{
    pub async fn create(&self, payload: CreatePayload) -> Result<CreateResponse, DomainError> {
        let user_opt = self
            .patients_repository
            .get_user_id_by_patient_id(payload.id_patient)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(user) = user_opt else {
            return Err(DomainError::PatientNotFound);
        };

        if user.id_user != payload.id_user {
            return Err(DomainError::UserNotAuthorized);
        }

        let age = Utc::now()
            .date_naive()
            .signed_duration_since(user.birthdate)
            .num_days()
            / 365;

        let metric_opt = self
            .metrics_repository
            .get_metrics(age as i32, user.is_male)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(metric) = metric_opt else {
            return Err(DomainError::MetricsNotFound);
        };

        let mut create_payload = CreateAnalysisPayload {
            id_patient: payload.id_patient,
            velocity: payload.velocity,
            velocity_diff: None,
            time: payload.time,
            time_diff: None,
            steps: payload.steps,
            steps_diff: None,
            cadence: payload.cadence,
            cadence_diff: None,
            bounce: payload.bounce,
            bounce_diff: None,
            time_to_up: payload.time_to_up,
            time_to_up_diff: None,
            date: payload.date,
        };

        create_payload.velocity_diff = Some(
            ((payload.velocity - metric.velocity) / metric.velocity)
                .to_f32()
                .unwrap()
                * 100.0,
        );
        create_payload.time_diff = Some(
            ((payload.time - metric.time) / metric.time)
                .to_f32()
                .unwrap()
                * 100.0,
        );
        create_payload.steps_diff =
            Some(((payload.steps - metric.steps) as f32 / metric.steps as f32) * 100.0);
        create_payload.cadence_diff =
            Some(((payload.cadence - metric.cadence) as f32 / metric.cadence as f32) * 100.0);
        create_payload.bounce_diff = Some(
            ((payload.bounce - metric.bounce) / metric.bounce)
                .to_f32()
                .unwrap()
                * 100.0,
        );
        create_payload.time_to_up_diff = Some(
            ((payload.time_to_up - metric.time_to_up) / metric.time_to_up)
                .to_f32()
                .unwrap()
                * 100.0,
        );

        let response = CreateResponse {
            velocity_diff: create_payload.velocity_diff,
            time_diff: create_payload.time_diff,
            steps_diff: create_payload.steps_diff,
            cadence_diff: create_payload.cadence_diff,
            bounce_diff: create_payload.bounce_diff,
            time_to_up_diff: create_payload.time_to_up_diff,
        };

        self.analysis_repository
            .create(create_payload)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        Ok(response)
    }

    pub async fn list_by_patient_id(
        &self,
        id_user: Uuid,
        id_patient: Uuid,
    ) -> Result<Vec<ListAnalysisPayload>, DomainError> {
        let user_opt = self
            .patients_repository
            .get_user_id_by_patient_id(id_patient)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(user) = user_opt else {
            return Err(DomainError::PatientNotFound);
        };

        if user.id_user != id_user {
            return Err(DomainError::UserNotAuthorized);
        }

        let result = self
            .analysis_repository
            .list_by_patient_id(id_patient)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        Ok(result)
    }
}
