pub use sea_orm_migration::prelude::*;

mod m20260807_221720_users;
mod m20260815_024003_adiciona_tabelas;
mod m20260815_043905_adiciona_seed_de_remedio;
mod m20260815_083225_tabela_analise;
mod m20260815_104144_seed_analysis;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260807_221720_users::Migration),
            Box::new(m20260815_024003_adiciona_tabelas::Migration),
            Box::new(m20260815_043905_adiciona_seed_de_remedio::Migration),
            Box::new(m20260815_083225_tabela_analise::Migration),
            Box::new(m20260815_104144_seed_analysis::Migration),
        ]
    }
}
