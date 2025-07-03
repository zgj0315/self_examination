use std::net::SocketAddr;

use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use server::{AppState, article};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("./config/log4rs.yml", Default::default())?;
    // let db_url = "sqlite::memory:";
    // let db_url = "sqlite://data/zhaogj_db.sqlite";
    // let pg_conn = Database::connect(db_url).await?;

    let mut opt =
        ConnectOptions::new("postgres://zhaogj_user:zhaogj_password@127.0.0.1:5432/zhaogj_db");
    opt.max_connections(5);
    opt.sqlx_logging_level(log::LevelFilter::Trace);
    let pg_conn = Database::connect(opt).await?;

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
