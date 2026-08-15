use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, DeriveIntoActiveModel, DerivePartialModel,
    EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    entity::prelude::Date,
    sqlx::types::{Decimal, Uuid},
};
use serde::Serialize;

use crate::{connections::database, entities::analysis};

#[derive(DeriveIntoActiveModel)]
#[sea_orm(active_model = "analysis::ActiveModel")]
pub struct CreateAnalysisPayload {
    pub id_patient: Uuid,
    pub velocity: Decimal,
    pub velocity_diff: Option<f32>,
    pub time: Decimal,
    pub time_diff: Option<f32>,
    pub steps: i32,
    pub steps_diff: Option<f32>,
    pub cadence: i32,
    pub cadence_diff: Option<f32>,
    pub bounce: Decimal,
    pub bounce_diff: Option<f32>,
    pub time_to_up: Decimal,
    pub time_to_up_diff: Option<f32>,
    pub date: Date,
}

#[derive(DerivePartialModel, Serialize)]
#[sea_orm(entity = "analysis::Entity")]
pub struct ListAnalysisPayload {
    pub velocity: Decimal,
    pub velocity_diff: Option<f32>,
    pub time: Decimal,
    pub time_diff: Option<f32>,
    pub steps: i32,
    pub steps_diff: Option<f32>,
    pub cadence: i32,
    pub cadence_diff: Option<f32>,
    pub bounce: Decimal,
    pub bounce_diff: Option<f32>,
    pub time_to_up: Decimal,
    pub time_to_up_diff: Option<f32>,
    pub date: Date,
}

#[cfg_attr(test, mockall::automock)]
pub trait AnalysisRepositoryTrait {
    async fn create(&self, payload: CreateAnalysisPayload) -> Result<Uuid, DbErr>;
    async fn list_by_patient_id(&self, patient_id: Uuid)
    -> Result<Vec<ListAnalysisPayload>, DbErr>;
}

pub struct AnalysisRepository {
    db: DatabaseConnection,
}

impl AnalysisRepository {
    pub fn new() -> Self {
        Self {
            db: database::get_database_connection(),
        }
    }
}

impl AnalysisRepositoryTrait for AnalysisRepository {
    async fn create(&self, payload: CreateAnalysisPayload) -> Result<Uuid, DbErr> {
        let result = analysis::Entity::insert(payload.into_active_model())
            .exec(&self.db)
            .await?;

        Ok(result.last_insert_id)
    }

    async fn list_by_patient_id(
        &self,
        patient_id: Uuid,
    ) -> Result<Vec<ListAnalysisPayload>, DbErr> {
        let result = analysis::Entity::find()
            .filter(
                Condition::all()
                    .add(analysis::Column::IdPatient.eq(patient_id))
                    .add(analysis::Column::DeletedAt.is_null()),
            )
            .order_by_asc(analysis::Column::CreatedAt)
            .into_partial_model::<ListAnalysisPayload>()
            .all(&self.db)
            .await?;

        Ok(result)
    }
}
