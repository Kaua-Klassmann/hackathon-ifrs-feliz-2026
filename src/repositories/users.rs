use crate::{connections, entities::users};
use sea_orm::{
    DatabaseConnection, DbErr, DeriveIntoActiveModel, DerivePartialModel, EntityTrait,
    IntoActiveModel, PaginatorTrait, sqlx::types::Uuid,
};

#[derive(DerivePartialModel)]
#[sea_orm(entity = "users::Entity")]
pub struct GetToLoginUserResponse {
    pub id: Uuid,
    pub password: String,
}

#[derive(DeriveIntoActiveModel)]
#[sea_orm(active_model = "users::ActiveModel")]
pub struct RegisterUserPayload {
    pub email: String,
    pub password: String,
}

#[cfg_attr(test, mockall::automock)]
pub trait UsersRepositoryTrait {
    async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr>;
    async fn get_to_login(&self, email: &str) -> Result<Option<GetToLoginUserResponse>, DbErr>;
    async fn register(&self, payload: RegisterUserPayload) -> Result<Uuid, DbErr>;
}

pub struct UsersRepository {
    db: DatabaseConnection,
}

impl UsersRepository {
    pub fn new() -> Self {
        Self {
            db: connections::database::get_database_connection(),
        }
    }
}

impl UsersRepositoryTrait for UsersRepository {
    async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr> {
        let count = users::Entity::find_by_email(email).count(&self.db).await?;

        Ok(count > 0)
    }

    async fn get_to_login(&self, email: &str) -> Result<Option<GetToLoginUserResponse>, DbErr> {
        users::Entity::find_by_email(email)
            .into_partial_model::<GetToLoginUserResponse>()
            .one(&self.db)
            .await
    }

    async fn register(&self, payload: RegisterUserPayload) -> Result<Uuid, DbErr> {
        let res = users::Entity::insert(payload.into_active_model())
            .exec(&self.db)
            .await?;

        Ok(res.last_insert_id)
    }
}
