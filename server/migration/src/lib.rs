pub use sea_orm_migration::prelude::*;

mod m20250703_151630_create_tbl_article;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250703_151630_create_tbl_article::Migration)]
    }
}
