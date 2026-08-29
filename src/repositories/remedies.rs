use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait, PaginatorTrait,
    QueryFilter, sqlx::types::Uuid,
};
use serde::Serialize;

use crate::{connections::database, entities::remedies};

#[derive(DerivePartialModel, Serialize)]
#[sea_orm(entity = "remedies::Entity")]
pub struct ListRemediesResult {
    pub id: Uuid,
    pub name: String,
}

#[cfg_attr(test, mockall::automock)]
pub trait RemediesRepositoryTrait {
    async fn list(&self) -> Result<Vec<ListRemediesResult>, DbErr>;
    async fn exists_by_ids(&self, ids: Vec<Uuid>) -> Result<bool, DbErr>;
}

pub struct RemediesRepository {
    db: DatabaseConnection,
}

impl RemediesRepository {
    pub fn new() -> Self {
        Self {
            db: database::get_database_connection(),
        }
    }
}

impl RemediesRepositoryTrait for RemediesRepository {
    async fn list(&self) -> Result<Vec<ListRemediesResult>, DbErr> {
        let remedies = remedies::Entity::find()
            .into_partial_model::<ListRemediesResult>()
            .all(&self.db)
            .await?;

        Ok(remedies)
    }

    async fn exists_by_ids(&self, ids: Vec<Uuid>) -> Result<bool, DbErr> {
        let count = remedies::Entity::find()
            .filter(remedies::Column::Id.is_in(ids.clone()))
            .count(&self.db)
            .await?;

        Ok(count == ids.len() as u64)
    }
}
