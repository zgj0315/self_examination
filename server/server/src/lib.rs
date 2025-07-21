use sea_orm::DatabaseConnection;

pub mod article;
pub mod auth;
pub mod config;
pub mod file;
pub mod home;
pub mod log;
pub mod pdf_article;
pub mod pdf_article_access_log;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub sled_db: sled::Db,
}
