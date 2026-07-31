pub use sea_orm_migration::prelude::*;

mod m20260627_191418_product;
mod m20260627_192403_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260627_192403_user::Migration),
            Box::new(m20260627_191418_product::Migration),
        ]
    }
}
