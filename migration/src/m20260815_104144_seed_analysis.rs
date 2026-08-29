use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260815_104144_seed_analysis"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table("metrics")
                    .columns([
                        "minAge", "maxAge", "isMale", "velocity", "time", "steps", "cadence",
                        "bounce", "timeToUp",
                    ])
                    .values_panic([
                        60.into(),
                        69.into(),
                        true.into(),
                        0.6.into(),
                        10.into(),
                        8.into(),
                        112.into(),
                        11.4.into(),
                        2.3.into(),
                    ])
                    .values_panic([
                        60.into(),
                        69.into(),
                        false.into(),
                        0.6.into(),
                        10.into(),
                        10.into(),
                        115.into(),
                        11.4.into(),
                        2.3.into(),
                    ])
                    .values_panic([
                        70.into(),
                        74.into(),
                        true.into(),
                        0.55.into(),
                        10.91.into(),
                        9.into(),
                        107.into(),
                        12.6.into(),
                        2.5.into(),
                    ])
                    .values_panic([
                        70.into(),
                        74.into(),
                        false.into(),
                        0.55.into(),
                        10.91.into(),
                        10.into(),
                        112.into(),
                        12.6.into(),
                        2.5.into(),
                    ])
                    .values_panic([
                        75.into(),
                        79.into(),
                        true.into(),
                        0.5.into(),
                        12.into(),
                        10.into(),
                        106.into(),
                        12.6.into(),
                        2.5.into(),
                    ])
                    .values_panic([
                        75.into(),
                        79.into(),
                        false.into(),
                        0.5.into(),
                        12.into(),
                        11.into(),
                        111.into(),
                        12.6.into(),
                        2.5.into(),
                    ])
                    .values_panic([
                        80.into(),
                        84.into(),
                        true.into(),
                        0.45.into(),
                        13.33.into(),
                        10.into(),
                        104.into(),
                        14.8.into(),
                        3.0.into(),
                    ])
                    .values_panic([
                        80.into(),
                        84.into(),
                        false.into(),
                        0.45.into(),
                        13.33.into(),
                        12.into(),
                        109.into(),
                        14.8.into(),
                        3.0.into(),
                    ])
                    .values_panic([
                        85.into(),
                        120.into(),
                        true.into(),
                        0.4.into(),
                        15.into(),
                        11.into(),
                        101.into(),
                        14.8.into(),
                        3.0.into(),
                    ])
                    .values_panic([
                        85.into(),
                        120.into(),
                        false.into(),
                        0.4.into(),
                        15.into(),
                        13.into(),
                        106.into(),
                        14.8.into(),
                        3.0.into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(Query::delete().from_table("metrics").to_owned())
            .await?;

        Ok(())
    }
}
