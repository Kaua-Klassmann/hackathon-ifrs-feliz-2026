use sea_orm::sqlx::types::Uuid;
use sea_orm::sqlx::types::chrono::Utc;

use crate::error::DomainError;
use crate::repositories::metrics::{GetMetricsResponse, MetricsRepository, MetricsRepositoryTrait};
use crate::repositories::patients::{PatientsRepository, PatientsRepositoryTrait};

pub struct MetricsService<
    MR: MetricsRepositoryTrait = MetricsRepository,
    PR: PatientsRepositoryTrait = PatientsRepository,
> {
    pub metrics_repository: MR,
    pub patients_repository: PR,
}

impl MetricsService<MetricsRepository, PatientsRepository> {
    pub fn new() -> Self {
        Self {
            metrics_repository: MetricsRepository::new(),
            patients_repository: PatientsRepository::new(),
        }
    }
}

impl<MR: MetricsRepositoryTrait, PR: PatientsRepositoryTrait> MetricsService<MR, PR> {
    pub async fn get_metrics(&self, patient_id: Uuid) -> Result<GetMetricsResponse, DomainError> {
        let patient_opt = self
            .patients_repository
            .get_user_id_by_patient_id(patient_id)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(patient) = patient_opt else {
            return Err(DomainError::PatientNotFound);
        };

        let age = Utc::now()
            .date_naive()
            .signed_duration_since(patient.birthdate)
            .num_days()
            / 365;

        let metrics_opt = self
            .metrics_repository
            .get_metrics(age as i32, patient.is_male)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(metrics) = metrics_opt else {
            return Err(DomainError::MetricsNotFound);
        };

        Ok(metrics)
    }
}
