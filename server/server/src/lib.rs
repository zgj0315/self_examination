use sea_orm::DatabaseConnection;

pub mod article;

#[derive(Clone)]
pub struct AppState {
    pub pg_conn: DatabaseConnection,
}
