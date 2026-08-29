use crate::error::DomainError;
use crate::repositories::remedies::{
    ListRemediesResult, RemediesRepository, RemediesRepositoryTrait,
};

pub struct RemediesService<RR: RemediesRepositoryTrait = RemediesRepository> {
    pub remedy_repository: RR,
}

impl RemediesService<RemediesRepository> {
    pub fn new() -> Self {
        Self {
            remedy_repository: RemediesRepository::new(),
        }
    }
}

impl<RR: RemediesRepositoryTrait> RemediesService<RR> {
    pub async fn list(&self) -> Result<Vec<ListRemediesResult>, DomainError> {
        let remedies = self
            .remedy_repository
            .list()
            .await
            .map_err(|e| DomainError::InternalServerError(e.to_string()))?;

        Ok(remedies)
    }
}
