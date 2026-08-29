use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260815_024003_adiciona_tabelas"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("remedies")
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new("name").text().not_null())
                    .col(
                        ColumnDef::new("createdAt")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new("updatedAt")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum("patient_blood_type")
                    .values([
                        "a_positive",
                        "a_negative",
                        "b_positive",
                        "b_negative",
                        "ab_positive",
                        "ab_negative",
                        "o_positive",
                        "o_negative",
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("patients")
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new("name").text().not_null())
                    .col(ColumnDef::new("birthdate").date().not_null())
                    .col(ColumnDef::new("isMale").boolean().not_null())
                    .col(
                        ColumnDef::new("bloodType")
                            .custom("patient_blood_type")
                            .not_null(),
                    )
                    .col(ColumnDef::new("idUser").uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-patients-idUser")
                            .from("patients", "idUser")
                            .to("users", "id"),
                    )
                    .col(
                        ColumnDef::new("createdAt")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new("updatedAt")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new("deletedAt").timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("patients_remedies")
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new("idPatient").uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-patients_remedies-idPatient")
                            .from("patients_remedies", "idPatient")
                            .to("patients", "id"),
                    )
                    .col(ColumnDef::new("idRemedy").uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-patients_remedies-idRemedy")
                            .from("patients_remedies", "idRemedy")
                            .to("remedies", "id"),
                    )
                    .col(ColumnDef::new("quantity").integer().not_null())
                    .col(
                        ColumnDef::new("createdAt")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new("updatedAt")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new("deletedAt").timestamp())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name("patient_blood_type").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("patients_remedies").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("patients").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("remedies").to_owned())
            .await?;

        Ok(())
    }
}
