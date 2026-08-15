use sea_orm::sqlx::types::Uuid;

use crate::error::DomainError;
use crate::repositories::patients::{
    CreatePatientPayload, CreatePatientRemediesPayload, PatientsRepository, PatientsRepositoryTrait,
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

    pub async fn update_remedies(
        &self,
        user_id: Uuid,
        patient_id: Uuid,
        remedies: Vec<CreatePatientRemediesPayload>,
    ) -> Result<(), DomainError> {
        let exists = self
            .patient_repository
            .get_user_id_by_patient_id(patient_id)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(patient) = exists else {
            return Err(DomainError::PatientNotFound);
        };

        if patient.id_user != user_id {
            return Err(DomainError::UserNotAuthorized);
        }

        self.patient_repository
            .update_remedies(patient_id, remedies)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        Ok(())
    }

    pub async fn delete(&self, user_id: Uuid, patient_id: Uuid) -> Result<(), DomainError> {
        let exists = self
            .patient_repository
            .get_user_id_by_patient_id(patient_id)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let Some(patient) = exists else {
            return Err(DomainError::PatientNotFound);
        };

        if patient.id_user != user_id {
            return Err(DomainError::UserNotAuthorized);
        }

        self.patient_repository
            .delete(patient_id)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        Ok(())
    }
}
