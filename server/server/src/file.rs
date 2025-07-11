use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use entity::tbl_file;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::AppState;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/files", post(upload).get(query))
        .route("/files/{id}", get(download))
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
        let file_name = field.file_name().unwrap_or_default().to_string();
        log::info!("file_name: {file_name}");

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
                name: Set(file_name),
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

#[derive(Deserialize, Debug, Validate)]
struct QueryInputDto {
    name: Option<String>,
    size: u64,
    page: u64,
}

#[derive(Serialize, Debug)]
struct QueryOutputDto {
    id: i32,
    name: String,
    created_at: i64,
}
async fn query(
    app_state: State<AppState>,
    Query(query_input_dto): Query<QueryInputDto>,
) -> impl IntoResponse {
    let mut select = tbl_file::Entity::find();

    if let Some(name) = query_input_dto.name {
        if !name.is_empty() {
            let like_pattern = format!("%{name}%");
            select = select.filter(tbl_file::Column::Name.like(like_pattern));
        }
    }
    let paginator = select
        .order_by_desc(tbl_file::Column::CreatedAt)
        .paginate(&app_state.db_conn, query_input_dto.size);
    let num_pages = match paginator.num_pages().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("num_pages err: {}", e);
            return (
                StatusCode::OK,
                [("code", "500"), ("msg", "pg connection err")],
                Json(json!( {
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "msg": "pg connection err".to_string(),
                })),
            );
        }
    };
    let num_items = match paginator.num_items().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("num_items err: {}", e);
            return (
                StatusCode::OK,
                [("code", "500"), ("msg", "pg connection err")],
                Json(json!( {
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "msg": "pg connection err".to_string(),
                })),
            );
        }
    };
    let tbl_files = match paginator.fetch_page(query_input_dto.page).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("fetch_page err: {}", e);
            return (
                StatusCode::OK,
                [("code", "200"), ("msg", "ok")],
                Json(json!( {
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "msg": "pg content err".to_string(),
                })),
            );
        }
    };
    let mut files = Vec::new();
    for tbl_file in tbl_files {
        files.push(QueryOutputDto {
            id: tbl_file.id,
            name: tbl_file.name,
            created_at: tbl_file.created_at.and_utc().timestamp_millis(),
        });
    }
    (
        StatusCode::OK,
        [("code", "200"), ("msg", "ok")],
        Json(json!(
            {
            "page":{
              "size":query_input_dto.size,
              "total_elements":num_items,
              "total_pages":num_pages
            },
            "_embedded":{
                "file":files
            }
           }
        )),
    )
}

async fn download(Path(id): Path<i32>, State(app_state): State<AppState>) -> impl IntoResponse {
    match tbl_file::Entity::find_by_id(id)
        .one(&app_state.db_conn)
        .await
    {
        Ok(tbl_file_op) => match tbl_file_op {
            Some(tbl_file) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/octet-stream"),
                );
                headers.insert(
                    header::CONTENT_DISPOSITION,
                    HeaderValue::from_str(&format!("attachment; filename=\"{}\"", tbl_file.name))
                        .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
                );
                return (StatusCode::OK, headers, tbl_file.content).into_response();
            }
            None => {
                log::warn!("not find file_id: {}", id);
                return (
                    StatusCode::BAD_REQUEST,
                    [("code", "400"), ("msg", "not find file id")],
                    Json(json!({})),
                )
                    .into_response();
            }
        },
        Err(e) => {
            log::error!("find file_id: {}, err: {}", id, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("code", "500"), ("msg", "find file err")],
                Json(json!({})),
            )
                .into_response();
        }
    }
}
