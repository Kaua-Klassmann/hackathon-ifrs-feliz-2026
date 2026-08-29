use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260815_083225_tabela_analise"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("analysis")
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new("idPatient").uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-analysis-idPatient")
                            .from("analysis", "idPatient")
                            .to("patients", "id"),
                    )
                    .col(ColumnDef::new("velocity").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("velocity_diff").float())
                    .col(ColumnDef::new("time").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("time_diff").float())
                    .col(ColumnDef::new("steps").integer().not_null())
                    .col(ColumnDef::new("steps_diff").float())
                    .col(ColumnDef::new("cadence").integer().not_null())
                    .col(ColumnDef::new("cadence_diff").float())
                    .col(ColumnDef::new("bounce").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("bounce_diff").float())
                    .col(ColumnDef::new("timeToUp").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("timeToUp_diff").float())
                    .col(ColumnDef::new("date").date().not_null())
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
            .create_index(
                Index::create()
                    .name("idx-analysis-idPatient")
                    .table("analysis")
                    .col("idPatient")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-analysis-date")
                    .table("analysis")
                    .col("date")
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("metrics")
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new("minAge").integer().not_null())
                    .col(ColumnDef::new("maxAge").integer().not_null())
                    .col(ColumnDef::new("isMale").boolean().not_null())
                    .col(ColumnDef::new("velocity").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("time").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("steps").integer().not_null())
                    .col(ColumnDef::new("cadence").integer().not_null())
                    .col(ColumnDef::new("bounce").decimal_len(4, 2).not_null())
                    .col(ColumnDef::new("timeToUp").decimal_len(4, 2).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("metrics").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("analysis").to_owned())
            .await?;

        Ok(())
    }
}
