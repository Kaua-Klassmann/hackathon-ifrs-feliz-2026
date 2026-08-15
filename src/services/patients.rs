use sea_orm::sqlx::types::Uuid;

use crate::error::DomainError;
use crate::repositories::patients::{
    CreatePatientPayload, PatientsRepository, PatientsRepositoryTrait,
};
use crate::repositories::remedies::{RemediesRepository, RemediesRepositoryTrait};
pub struct PatientsService<
    PR: PatientsRepositoryTrait = PatientsRepository,
    RR: RemediesRepositoryTrait = RemediesRepository,
> {
    pub patient_repository: PR,
    pub remedy_repository: RR,
}

impl PatientsService<PatientsRepository, RemediesRepository> {
    pub fn new() -> Self {
        Self {
            patient_repository: PatientsRepository::new(),
            remedy_repository: RemediesRepository::new(),
        }
    }
}

impl<PR: PatientsRepositoryTrait, RR: RemediesRepositoryTrait> PatientsService<PR, RR> {
    pub async fn create(&self, payload: CreatePatientPayload) -> Result<Uuid, DomainError> {
        let remedies = payload
            .remedies
            .iter()
            .map(|r| r.remedy)
            .collect::<Vec<Uuid>>();

        if !remedies.is_empty() {
            let exists = self
                .remedy_repository
                .exists_by_ids(remedies)
                .await
                .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

            if !exists {
                return Err(DomainError::RemedyNotFound);
            }
        }

        let patient = self
            .patient_repository
            .create(payload)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        Ok(patient)
    }
}
