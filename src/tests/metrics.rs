use crate::{
    repositories::{metrics::MockMetricsRepositoryTrait, patients::MockPatientsRepositoryTrait},
    services::metrics::MetricsService,
};

fn factory() -> MetricsService<MockMetricsRepositoryTrait, MockPatientsRepositoryTrait> {
    MetricsService {
        metrics_repository: MockMetricsRepositoryTrait::new(),
        patients_repository: MockPatientsRepositoryTrait::new(),
    }
}

mod list_by_patient_id {
    use sea_orm::sqlx::types::{Decimal, Uuid};

    use crate::repositories::{
        metrics::GetMetricsResponse, patients::GetUserIdByPatientIdResponse,
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
                    velocity: Decimal::new(100, 0),
                    bounce: Decimal::new(100, 0),
                    time: Decimal::new(100, 0),
                    time_to_up: Decimal::new(100, 0),
                    cadence: 100,
                    steps: 100,
                }))
            });

        let response = service.get_metrics(Uuid::default()).await;

        assert!(response.is_ok());
    }
}
