use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait,
    FromQueryResult, QueryFilter, QuerySelect, Set, TransactionTrait,
    entity::prelude::Date,
    sea_query::Expr,
    sqlx::types::{Uuid, chrono},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    connections::database,
    entities::{patients, patients_remedies, sea_orm_active_enums::PatientBloodType},
};

#[derive(FromQueryResult)]
struct ListPatientsQuery {
    pub id: Uuid,
    pub name: String,
    pub birthdate: Date,
    pub is_male: bool,
    pub blood_type: PatientBloodType,
    pub remedies: Value,
}

#[derive(Serialize)]
pub struct ListPatientsResponse {
    pub id: Uuid,
    pub name: String,
    pub birthdate: Date,
    pub is_male: bool,
    pub blood_type: PatientBloodType,
    pub remedies: Vec<ListPatientsRemediesResponse>,
}

#[derive(Deserialize, Serialize)]
pub struct ListPatientsRemediesResponse {
    pub id: Uuid,
    pub remedy: String,
    pub quantity: i32,
}

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

#[derive(DerivePartialModel)]
#[sea_orm(entity = "patients::Entity")]
pub struct GetUserIdByPatientIdResponse {
    pub id_user: Uuid,
}

#[cfg_attr(test, mockall::automock)]
pub trait PatientsRepositoryTrait {
    async fn list(&self, user_id: Uuid) -> Result<Vec<ListPatientsResponse>, DbErr>;
    async fn create(&self, payload: CreatePatientPayload) -> Result<Uuid, DbErr>;
    async fn get_user_id_by_patient_id(
        &self,
        patient_id: Uuid,
    ) -> Result<Option<GetUserIdByPatientIdResponse>, DbErr>;
    async fn update_remedies(
        &self,
        patient_id: Uuid,
        remedies: Vec<CreatePatientRemediesPayload>,
    ) -> Result<(), DbErr>;
    async fn delete(&self, patient_id: Uuid) -> Result<(), DbErr>;
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
    async fn list(&self, user_id: Uuid) -> Result<Vec<ListPatientsResponse>, DbErr> {
        let result = patients::Entity::find()
            .select_only()
            .columns([
                patients::Column::Id,
                patients::Column::Name,
                patients::Column::Birthdate,
            ])
            .column_as(patients::Column::IsMale, "is_male")
            .column_as(patients::Column::BloodType, "blood_type")
            .column_as(
                Expr::cust(
                    r#"
                    COALESCE(
                        (
                            SELECT JSON_AGG(JSON_BUILD_OBJECT(
                                'id', patients_remedies.id,
                                'remedy', remedies.name,
                                'quantity', patients_remedies.quantity
                            ))
                            FROM patients_remedies
                            JOIN remedies ON remedies.id = patients_remedies."idRemedy"
                            WHERE patients_remedies."idPatient" = patients.id
                                AND patients_remedies."deletedAt" IS NULL
                        )
                    , '[]'::json)
                    "#,
                ),
                "remedies",
            )
            .filter(
                Condition::all()
                    .add(patients::Column::IdUser.eq(user_id))
                    .add(patients::Column::DeletedAt.is_null()),
            )
            .into_model::<ListPatientsQuery>()
            .all(&self.db)
            .await?;

        let r = result
            .into_iter()
            .map(|patient| ListPatientsResponse {
                id: patient.id,
                name: patient.name,
                birthdate: patient.birthdate,
                is_male: patient.is_male,
                blood_type: patient.blood_type,
                remedies: serde_json::from_value::<Vec<ListPatientsRemediesResponse>>(
                    patient.remedies,
                )
                .unwrap(),
            })
            .collect::<Vec<ListPatientsResponse>>();

        Ok(r)
    }

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

    async fn get_user_id_by_patient_id(
        &self,
        patient_id: Uuid,
    ) -> Result<Option<GetUserIdByPatientIdResponse>, DbErr> {
        let patient = patients::Entity::find_by_id(patient_id)
            .filter(patients::Column::DeletedAt.is_null())
            .into_partial_model::<GetUserIdByPatientIdResponse>()
            .one(&self.db)
            .await?;

        Ok(patient)
    }

    async fn update_remedies(
        &self,
        patient_id: Uuid,
        remedies: Vec<CreatePatientRemediesPayload>,
    ) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;

        if let Err(err) = patients_remedies::Entity::update_many()
            .col_expr(
                patients_remedies::Column::DeletedAt,
                Expr::val(Some(chrono::Utc::now().naive_utc())),
            )
            .filter(patients_remedies::Column::IdPatient.eq(patient_id))
            .exec(&txn)
            .await
        {
            txn.rollback().await?;
            return Err(err);
        }

        patients_remedies::Entity::insert_many(
            remedies
                .into_iter()
                .map(|remedy| patients_remedies::ActiveModel {
                    id_patient: Set(patient_id),
                    id_remedy: Set(remedy.remedy),
                    quantity: Set(remedy.quantity),
                    ..Default::default()
                })
                .collect::<Vec<patients_remedies::ActiveModel>>(),
        )
        .exec(&txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    async fn delete(&self, patient_id: Uuid) -> Result<(), DbErr> {
        let txn = self.db.begin().await?;

        if let Err(err) = patients_remedies::Entity::update_many()
            .col_expr(
                patients::Column::DeletedAt,
                Expr::val(Some(chrono::Utc::now().naive_utc())),
            )
            .filter(patients_remedies::Column::IdPatient.eq(patient_id))
            .exec(&txn)
            .await
        {
            txn.rollback().await?;
            return Err(err);
        }

        if let Err(err) = patients::Entity::update_many()
            .col_expr(
                patients::Column::DeletedAt,
                Expr::val(Some(chrono::Utc::now().naive_utc())),
            )
            .filter(patients::Column::Id.eq(patient_id))
            .exec(&txn)
            .await
        {
            txn.rollback().await?;
            return Err(err);
        }

        txn.commit().await?;

        Ok(())
    }
}
