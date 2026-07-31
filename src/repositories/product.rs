use sea_orm::{
    DatabaseConnection, DbErr, DeriveIntoActiveModel, DerivePartialModel, EntityTrait,
    IntoActiveModel, sqlx::types::Uuid,
};
use serde::Serialize;

use crate::{connections, entities::product};

#[derive(DerivePartialModel, Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[sea_orm(entity = "product::Entity")]
pub struct ListProductsResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(DerivePartialModel, Serialize)]
#[sea_orm(entity = "product::Entity")]
pub struct GetByIdProductResponse {
    pub id_user: Uuid,
    pub name: String,
}

#[derive(DeriveIntoActiveModel)]
#[sea_orm(active_model = "product::ActiveModel")]
pub struct RegisterProductPayload {
    pub id_user: Uuid,
    pub name: String,
}

#[cfg_attr(test, mockall::automock)]
pub trait ProductRepositoryTrait {
    async fn list(&self) -> Result<Vec<ListProductsResponse>, DbErr>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<GetByIdProductResponse>, DbErr>;
    async fn register(&self, payload: RegisterProductPayload) -> Result<Uuid, DbErr>;
}

pub struct ProductRepository {
    db: DatabaseConnection,
}

impl ProductRepository {
    pub fn new() -> Self {
        Self {
            db: connections::database::get_database_connection(),
        }
    }
}

impl ProductRepositoryTrait for ProductRepository {
    async fn list(&self) -> Result<Vec<ListProductsResponse>, DbErr> {
        product::Entity::find()
            .into_partial_model::<ListProductsResponse>()
            .all(&self.db)
            .await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<GetByIdProductResponse>, DbErr> {
        product::Entity::find_by_id(id)
            .into_partial_model::<GetByIdProductResponse>()
            .one(&self.db)
            .await
    }

    async fn register(&self, payload: RegisterProductPayload) -> Result<Uuid, DbErr> {
        let result = product::Entity::insert(payload.into_active_model())
            .exec(&self.db)
            .await?;

        Ok(result.last_insert_id)
    }
}
