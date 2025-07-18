pub use sea_orm_migration::prelude::*;

mod m20250703_151630_create_tbl_article;
mod m20250703_153326_create_tbl_log;
mod m20250711_022548_create_tbl_file;
mod m20250715_014127_create_tbl_auth_user;
mod m20250716_151156_create_tbl_pdf_article;
mod m20250717_002329_create_tbl_pdf_article_access_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250703_151630_create_tbl_article::Migration),
            Box::new(m20250703_153326_create_tbl_log::Migration),
            Box::new(m20250711_022548_create_tbl_file::Migration),
            Box::new(m20250715_014127_create_tbl_auth_user::Migration),
            Box::new(m20250716_151156_create_tbl_pdf_article::Migration),
            Box::new(m20250717_002329_create_tbl_pdf_article_access_log::Migration),
        ]
    }
}
