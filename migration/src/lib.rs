pub use sea_orm_migration::prelude::*;

mod m20260807_221720_users;
mod m20260815_024003_adiciona_tabelas;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260807_221720_users::Migration),
            Box::new(m20260815_024003_adiciona_tabelas::Migration),
        ]
    }
}
