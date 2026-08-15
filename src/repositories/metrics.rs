use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait,
    QueryFilter, sqlx::types::Decimal,
};

use crate::{connections::database, entities::metrics};

#[derive(DerivePartialModel)]
#[sea_orm(entity = "metrics::Entity")]
pub struct GetMetricsResponse {
    pub velocity: Decimal,
    pub time: Decimal,
    pub steps: i32,
    pub cadence: i32,
    pub bounce: Decimal,
    pub time_to_up: Decimal,
}

#[cfg_attr(test, mockall::automock)]
pub trait MetricsRepositoryTrait {
    async fn get_metrics(
        &self,
        age: i32,
        is_male: bool,
    ) -> Result<Option<GetMetricsResponse>, DbErr>;
}

pub struct MetricsRepository {
    db: DatabaseConnection,
}

impl MetricsRepository {
    pub fn new() -> Self {
        Self {
            db: database::get_database_connection(),
        }
    }
}

impl MetricsRepositoryTrait for MetricsRepository {
    async fn get_metrics(
        &self,
        age: i32,
        is_male: bool,
    ) -> Result<Option<GetMetricsResponse>, DbErr> {
        let result = metrics::Entity::find()
            .filter(
                Condition::all()
                    .add(metrics::Column::MinAge.lte(age))
                    .add(metrics::Column::MaxAge.gte(age))
                    .add(metrics::Column::IsMale.eq(is_male)),
            )
            .into_partial_model::<GetMetricsResponse>()
            .one(&self.db)
            .await?;

        Ok(result)
    }
}
