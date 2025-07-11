use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
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
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 4))
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
            let content_bytes = if content_type.eq("application/json") {
                match field.text().await {
                    Ok(text) => text.into_bytes(),
                    Err(e_text) => {
                        log::error!("field get text err: {}", e_text);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            [("code", "500"), ("msg", "tbl_file insert err")],
                            Json(json!({})),
                        );
                    }
                }
            } else {
                match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e_bytes) => {
                        log::error!("get field bytes err: {}", e_bytes);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            [("code", "500"), ("msg", "tbl_file insert err")],
                            Json(json!({})),
                        );
                    }
                }
            };
            let tbl_file_am = tbl_file::ActiveModel {
                content: Set(content_bytes),
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
    }

    (
        StatusCode::OK,
        [("code", "200"), ("msg", "ok")],
        Json(json!({
            "file_ids":file_ids
        })),
    )
}
