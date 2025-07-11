use sea_orm::DatabaseConnection;

pub mod article;
pub mod file;
pub mod log;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}
