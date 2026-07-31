use crate::{repositories::product::MockProductRepositoryTrait, services::product::ProductService};

fn factory() -> ProductService<MockProductRepositoryTrait> {
    ProductService {
        product_repository: MockProductRepositoryTrait::new(),
    }
}

mod list {
    use sea_orm::{DbErr, prelude::Uuid};

    use crate::{error::DomainError, repositories::product::ListProductsResponse};

    #[tokio::test]
    async fn success() {
        let mut service = super::factory();

        service.product_repository.expect_list().returning(|| {
            Ok(vec![ListProductsResponse {
                id: Uuid::parse_str("00000000-0000-4000-0000-000000000001").unwrap(),
                name: "Test Product".into(),
            }])
        });

        let result = service.list().await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![ListProductsResponse {
                id: Uuid::parse_str("00000000-0000-4000-0000-000000000001").unwrap(),
                name: "Test Product".into(),
            }]
        );
    }

    #[tokio::test]
    async fn error() {
        let mut service = super::factory();

        service
            .product_repository
            .expect_list()
            .returning(|| Err(DbErr::Custom("".to_string())));

        let result = service.list().await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::InternalServerError(_)
        ));
    }
}
