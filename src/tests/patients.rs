use crate::{
    repositories::patients::MockPatientsRepositoryTrait,
    repositories::remedies::MockRemediesRepositoryTrait, services::patients::PatientsService,
};

fn factory() -> PatientsService<MockPatientsRepositoryTrait, MockRemediesRepositoryTrait> {
    PatientsService {
        patient_repository: MockPatientsRepositoryTrait::new(),
        remedy_repository: MockRemediesRepositoryTrait::new(),
    }
}

mod create {
    use sea_orm::{entity::prelude::Date, sqlx::types::Uuid};

    use crate::{
        entities::sea_orm_active_enums::PatientBloodType,
        error::DomainError,
        repositories::patients::{CreatePatientPayload, CreatePatientRemediesPayload},
    };

    #[tokio::test]
    async fn success_without_remedy() {
        let mut service = super::factory();

        service
            .patient_repository
            .expect_create()
            .returning(|_| Ok(Uuid::default()));

        let result = service
            .create(CreatePatientPayload {
                name: "Test Patient".to_string(),
                birthdate: "2007-02-25".parse::<Date>().unwrap(),
                blood_type: PatientBloodType::ANegative,
                id_user: Uuid::default(),
                is_male: true,
                remedies: vec![],
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn success_with_remedy() {
        let mut service = super::factory();

        service
            .remedy_repository
            .expect_exists_by_ids()
            .returning(|_| Ok(true));
        service
            .patient_repository
            .expect_create()
            .returning(|_| Ok(Uuid::default()));

        let result = service
            .create(CreatePatientPayload {
                name: "Test Patient".to_string(),
                birthdate: "2007-02-25".parse::<Date>().unwrap(),
                blood_type: PatientBloodType::ANegative,
                id_user: Uuid::default(),
                is_male: true,
                remedies: vec![CreatePatientRemediesPayload {
                    remedy: Uuid::default(),
                    quantity: 1,
                }],
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn error_remedy_not_found() {
        let mut service = super::factory();

        service
            .remedy_repository
            .expect_exists_by_ids()
            .returning(|_| Ok(false));

        let result = service
            .create(CreatePatientPayload {
                name: "Test Patient".to_string(),
                birthdate: "2007-02-25".parse::<Date>().unwrap(),
                blood_type: PatientBloodType::ANegative,
                id_user: Uuid::default(),
                is_male: true,
                remedies: vec![CreatePatientRemediesPayload {
                    remedy: Uuid::default(),
                    quantity: 1,
                }],
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), DomainError::RemedyNotFound));
    }
}
