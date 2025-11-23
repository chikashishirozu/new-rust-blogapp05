pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20220101_000002_create_post_table;
mod m20251023_061052_add_user_fields;
mod m20251023_083404_update_user_model;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20220101_000002_create_post_table::Migration),
            Box::new(m20251023_061052_add_user_fields::Migration),
            Box::new(m20251023_083404_update_user_model::Migration),
        ]
    }
}
