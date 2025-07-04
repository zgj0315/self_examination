use std::{
    fs::{self, File},
    net::SocketAddr,
    path::Path,
};

use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use server::{AppState, article};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("./config/log4rs.yml", Default::default())?;

    let db_dir = Path::new("./data");
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir)?;
        log::info!("create dir: {}", db_dir.to_string_lossy());
    }

    let db_path = db_dir.join("zhaogj_db.sqlite");
    if !db_path.exists() {
        File::create(&db_path)?;
        log::info!("create file: {}", db_path.to_string_lossy());
    }

    let db_url = format!("sqlite://{}", db_path.to_string_lossy());
    let pg_conn = Database::connect(&db_url).await?;
    log::info!("connect to {}", db_url);

    Migrator::up(&pg_conn, None).await?;

    let app_state = AppState { pg_conn };
    let app = Router::new()
        .fallback_service(ServeDir::new("../../ui/dist"))
        .nest("/api/article", article::routers(app_state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    log::info!("listening on {}", listener.local_addr()?);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
