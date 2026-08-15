use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    sqlx::types::Uuid,
};

use crate::{connections::database, entities::remedies};

#[cfg_attr(test, mockall::automock)]
pub trait RemediesRepositoryTrait {
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
    async fn exists_by_ids(&self, ids: Vec<Uuid>) -> Result<bool, DbErr> {
        let count = remedies::Entity::find()
            .filter(remedies::Column::Id.is_in(ids.clone()))
            .count(&self.db)
            .await?;

        Ok(count == ids.len() as u64)
    }
}
