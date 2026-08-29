use crate::{
    repositories::{
        analysis::MockAnalysisRepositoryTrait, metrics::MockMetricsRepositoryTrait,
        patients::MockPatientsRepositoryTrait,
    },
    services::analysis::AnalysisService,
};

fn factory() -> AnalysisService<
    MockAnalysisRepositoryTrait,
    MockPatientsRepositoryTrait,
    MockMetricsRepositoryTrait,
> {
    AnalysisService {
        analysis_repository: MockAnalysisRepositoryTrait::new(),
        patients_repository: MockPatientsRepositoryTrait::new(),
        metrics_repository: MockMetricsRepositoryTrait::new(),
    }
}

mod list_by_patient_id {
    use sea_orm::sqlx::types::Uuid;

    use crate::repositories::patients::GetUserIdByPatientIdResponse;

    #[tokio::test]
    async fn success() {
        let mut service = super::factory();

        service
            .patients_repository
            .expect_get_user_id_by_patient_id()
            .returning(|_| {
                Ok(Some(GetUserIdByPatientIdResponse {
                    id_user: Uuid::default(),
                    birthdate: "1960-01-01".parse().unwrap(),
                    is_male: true,
                }))
            });
        service
            .analysis_repository
            .expect_list_by_patient_id()
            .returning(|_| Ok(vec![]));

        let response = service
            .list_by_patient_id(Uuid::default(), Uuid::default())
            .await;

        assert!(response.is_ok());
    }
}

mod create {
    use sea_orm::sqlx::types::{Decimal, Uuid};

    use crate::{
        repositories::{metrics::GetMetricsResponse, patients::GetUserIdByPatientIdResponse},
        services::analysis::CreatePayload,
    };

    #[tokio::test]
    async fn success() {
        let mut service = super::factory();

        service
            .patients_repository
            .expect_get_user_id_by_patient_id()
            .returning(|_| {
                Ok(Some(GetUserIdByPatientIdResponse {
                    id_user: Uuid::default(),
                    birthdate: "1960-01-01".parse().unwrap(),
                    is_male: true,
                }))
            });
        service
            .metrics_repository
            .expect_get_metrics()
            .returning(|_, _| {
                Ok(Some(GetMetricsResponse {
                    velocity: Decimal::new(1, 0),
                    time: Decimal::new(1, 0),
                    steps: 1,
                    cadence: 1,
                    bounce: Decimal::new(1, 0),
                    time_to_up: Decimal::new(1, 0),
                }))
            });
        service
            .analysis_repository
            .expect_create()
            .returning(|_| Ok(Uuid::default()));

        let response = service
            .create(CreatePayload {
                id_user: Uuid::default(),
                id_patient: Uuid::default(),
                velocity: Decimal::new(1, 0),
                time: Decimal::new(1, 0),
                steps: 1,
                cadence: 1,
                bounce: Decimal::new(1, 0),
                time_to_up: Decimal::new(1, 0),
                date: "2023-01-01".parse().unwrap(),
            })
            .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn patient_not_found() {
        let mut service = super::factory();

        service
            .patients_repository
            .expect_get_user_id_by_patient_id()
            .returning(|_| Ok(None));

        let response = service
            .create(CreatePayload {
                id_user: Uuid::default(),
                id_patient: Uuid::default(),
                velocity: Decimal::new(1, 0),
                time: Decimal::new(1, 0),
                steps: 1,
                cadence: 1,
                bounce: Decimal::new(1, 0),
                time_to_up: Decimal::new(1, 0),
                date: "2023-01-01".parse().unwrap(),
            })
            .await;

        assert!(response.is_err());
        assert!(matches!(
            response.err().unwrap(),
            crate::error::DomainError::PatientNotFound
        ));
    }

    #[tokio::test]
    async fn user_not_authorized() {
        let mut service = super::factory();

        service
            .patients_repository
            .expect_get_user_id_by_patient_id()
            .returning(|_| {
                Ok(Some(GetUserIdByPatientIdResponse {
                    id_user: Uuid::default(),
                    birthdate: "1960-01-01".parse().unwrap(),
                    is_male: true,
                }))
            });

        let response = service
            .create(CreatePayload {
                id_user: "00000000-0000-0000-0000-000000000001".parse().unwrap(),
                id_patient: Uuid::default(),
                velocity: Decimal::new(0, 0),
                time: Decimal::new(0, 0),
                steps: 0,
                cadence: 0,
                bounce: Decimal::new(0, 0),
                time_to_up: Decimal::new(0, 0),
                date: "2023-01-01".parse().unwrap(),
            })
            .await;

        assert!(response.is_err());
        assert!(matches!(
            response.err().unwrap(),
            crate::error::DomainError::UserNotAuthorized
        ));
    }

    #[tokio::test]
    async fn metrics_not_found() {
        let mut service = super::factory();

        service
            .patients_repository
            .expect_get_user_id_by_patient_id()
            .returning(|_| {
                Ok(Some(GetUserIdByPatientIdResponse {
                    id_user: Uuid::default(),
                    birthdate: "1960-01-01".parse().unwrap(),
                    is_male: true,
                }))
            });
        service
            .metrics_repository
            .expect_get_metrics()
            .returning(|_, _| Ok(None));

        let response = service
            .create(CreatePayload {
                id_user: Uuid::default(),
                id_patient: Uuid::default(),
                velocity: Decimal::new(0, 0),
                time: Decimal::new(0, 0),
                steps: 0,
                cadence: 0,
                bounce: Decimal::new(0, 0),
                time_to_up: Decimal::new(0, 0),
                date: "2023-01-01".parse().unwrap(),
            })
            .await;

        assert!(response.is_err());
        assert!(matches!(
            response.err().unwrap(),
            crate::error::DomainError::MetricsNotFound
        ));
    }
}
