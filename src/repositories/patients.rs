use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, Set, TransactionTrait,
    sqlx::types::{Uuid, chrono},
};

use crate::{
    connections::database,
    entities::{patients, patients_remedies, sea_orm_active_enums::PatientBloodType},
};

pub struct CreatePatientPayload {
    pub name: String,
    pub birthdate: chrono::NaiveDate,
    pub is_male: bool,
    pub blood_type: PatientBloodType,
    pub id_user: Uuid,
    pub remedies: Vec<CreatePatientRemediesPayload>,
}

pub struct CreatePatientRemediesPayload {
    pub remedy: Uuid,
    pub quantity: i32,
}

#[cfg_attr(test, mockall::automock)]
pub trait PatientsRepositoryTrait {
    async fn create(&self, payload: CreatePatientPayload) -> Result<Uuid, DbErr>;
}

pub struct PatientsRepository {
    db: DatabaseConnection,
}

impl PatientsRepository {
    pub fn new() -> Self {
        Self {
            db: database::get_database_connection(),
        }
    }
}

impl PatientsRepositoryTrait for PatientsRepository {
    async fn create(&self, payload: CreatePatientPayload) -> Result<Uuid, DbErr> {
        let txn = self.db.begin().await?;

        let result = match patients::Entity::insert(patients::ActiveModel {
            name: Set(payload.name),
            birthdate: Set(payload.birthdate),
            is_male: Set(payload.is_male),
            blood_type: Set(payload.blood_type),
            id_user: Set(payload.id_user),
            ..Default::default()
        })
        .exec(&txn)
        .await
        {
            Ok(result) => result,
            Err(err) => {
                txn.rollback().await?;
                return Err(err);
            }
        };

        let remedies = payload
            .remedies
            .into_iter()
            .map(|remedy| patients_remedies::ActiveModel {
                id_patient: Set(result.last_insert_id),
                id_remedy: Set(remedy.remedy),
                quantity: Set(remedy.quantity),
                ..Default::default()
            })
            .collect::<Vec<patients_remedies::ActiveModel>>();

        if !remedies.is_empty() {
            if let Err(err) = patients_remedies::Entity::insert_many(remedies)
                .exec(&txn)
                .await
            {
                txn.rollback().await?;
                return Err(err);
            }
        }

        txn.commit().await?;

        Ok(result.last_insert_id)
    }
}
