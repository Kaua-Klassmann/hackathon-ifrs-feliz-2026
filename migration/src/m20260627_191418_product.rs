use sea_orm_migration::prelude::*;

use crate::m20260627_192403_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::IdUser).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("idx-product-user-id_user")
                            .from(Product::Table, Product::IdUser)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    IdUser,
}
