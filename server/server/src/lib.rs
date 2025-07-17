use sea_orm::DatabaseConnection;

pub mod article;
pub mod auth;
pub mod file;
pub mod log;
pub mod pdf_article;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub sled_db: sled::Db,
}
