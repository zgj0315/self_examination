use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use entity::{tbl_pdf_article, tbl_pdf_article_access_log};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;

use crate::AppState;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/home/pdf_article_stat", get(pdf_article_stat))
        .with_state(state)
}

async fn pdf_article_stat(app_state: State<AppState>) -> impl IntoResponse {
    let pdf_article_count = match tbl_pdf_article::Entity::find()
        .count(&app_state.db_conn)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            log::error!("tbl_pdf_article count err: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };
    let pdf_article_access_log_count = match tbl_pdf_article_access_log::Entity::find()
        .count(&app_state.db_conn)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            log::error!("tbl_pdf_article_access_log count err: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };
    return (
        StatusCode::OK,
        Json(json!( {
                "pdf_article_count": pdf_article_count,
                "pdf_article_access_log_count": pdf_article_access_log_count,
        })),
    );
}
