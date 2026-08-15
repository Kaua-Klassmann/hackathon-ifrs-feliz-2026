use crate::{
    repositories::remedies::MockRemediesRepositoryTrait, services::remedies::RemediesService,
};

fn factory() -> RemediesService<MockRemediesRepositoryTrait> {
    RemediesService {
        remedy_repository: MockRemediesRepositoryTrait::new(),
    }
}

mod login {

    #[tokio::test]
    async fn success() {
        let mut service = super::factory();

        service
            .remedy_repository
            .expect_list()
            .returning(|| Ok(vec![]));

        let result = service.list().await;

        assert!(result.is_ok());
    }
}
