use axum::{
    Json, Router,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use entity::tbl_file;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde_json::json;

use crate::AppState;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/files", post(upload))
        .with_state(state)
}

async fn upload(app_state: State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    let mut file_ids = Vec::new();
    while let Some(field) = match multipart.next_field().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("multipart.next_field err: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                [("code", "400"), ("msg", "multipart.next_field err")],
                Json(json!({})),
            );
        }
    } {
        if let Some(name) = field.name() {
            log::info!("name: {name}");
        }
        if let Some(file_name) = field.file_name() {
            log::info!("file_name: {file_name}");
        }
        if let Some(content_type) = field.content_type() {
            log::info!("content_type: {content_type}");
        }

        match field.bytes().await {
            Ok(content_bytes) => {
                log::info!("content_bytes len: {}", content_bytes.len());
                let tbl_file_am = tbl_file::ActiveModel {
                    content: Set(content_bytes.to_vec()),
                    ..Default::default()
                };
                match tbl_file::Entity::insert(tbl_file_am)
                    .exec(&app_state.db_conn)
                    .await
                {
                    Ok(insert_result) => {
                        file_ids.push(insert_result.last_insert_id);
                    }
                    Err(e) => {
                        log::error!("tbl_file insert err: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            [("code", "500"), ("msg", "tbl_file insert err")],
                            Json(json!({})),
                        );
                    }
                }
            }
            Err(e) => {
                log::error!("field.bytes err: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    [("code", "400"), ("msg", "multipart.next_field err")],
                    Json(json!({})),
                );
            }
        }
    }

    (
        StatusCode::OK,
        [("code", "200"), ("msg", "ok")],
        Json(json!({
            "file_ids":file_ids
        })),
    )
}
