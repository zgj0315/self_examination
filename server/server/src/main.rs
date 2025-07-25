use std::{
    fs::{self, File},
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use axum::{Router, middleware::from_extractor_with_state};
use axum_server::tls_rustls::RustlsConfig;
use migration::{Migrator, MigratorTrait};
use rustls::crypto;
use sea_orm::Database;
use server::{
    auth::{self, RequireAuth},
    config::SERVER_TOML,
};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("./config/log4rs.yml", Default::default())?;

    let db_dir = Path::new("./data");
    if !db_dir.exists() {
        fs::create_dir_all(db_dir)?;
        log::info!("create dir: {}", db_dir.to_string_lossy());
    }

    let db_path = db_dir.join("zhaogj_db.sqlite");
    if !db_path.exists() {
        File::create(&db_path)?;
        log::info!("create file: {}", db_path.to_string_lossy());
    }

    let db_url = format!("sqlite://{}", db_path.to_string_lossy());
    let db_conn = Database::connect(&db_url).await?;
    log::info!("connect to {}", db_url);

    Migrator::up(&db_conn, None).await?;

    let sled_db = sled::open("./data/sled_db")?;
    auth::token_expired_task(sled_db.clone()).await?;
    let app_state = server::AppState { db_conn, sled_db };
    let dist_path = if Path::new("../../ui/dist").exists() {
        // 工程目录
        "../../ui/dist"
    } else {
        // 部署目录
        "./html"
    };
    let app = Router::new()
        .fallback_service(
            ServeDir::new(dist_path).fallback(ServeFile::new(format!("{dist_path}/index.html"))),
        )
        .nest("/api", server::article::routers(app_state.clone()))
        .nest("/api", server::log::routers(app_state.clone()))
        .nest("/api", server::file::routers(app_state.clone()))
        .nest("/api", server::pdf_article::routers(app_state.clone()))
        .nest(
            "/api",
            server::pdf_article_access_log::routers(app_state.clone()),
        )
        .nest("/api", server::home::routers(app_state.clone()))
        .nest("/api", server::auth::routers(app_state.clone()))
        .layer(from_extractor_with_state::<RequireAuth, _>(Arc::new(
            app_state,
        )));

    if let Err(e) = crypto::ring::default_provider().install_default() {
        log::error!("default_provider install err: {:?}", e);
    }
    let config = RustlsConfig::from_pem_file(
        PathBuf::from("./config").join("zhaogj-ca.crt"),
        PathBuf::from("./config").join("zhaogj-ca.key"),
    )
    .await?;
    let addr = SocketAddr::from_str(&SERVER_TOML.server.addr)?;
    log::info!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}
