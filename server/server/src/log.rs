use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use entity::tbl_log;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::AppState;

pub fn routers(state: AppState) -> Router {
    Router::new().route("/logs", get(query)).with_state(state)
}

#[derive(Deserialize, Debug, Validate)]
struct QueryInputDto {
    content: Option<String>,
    size: u64,
    page: u64,
}

#[derive(Serialize, Debug)]
struct QueryOutputDto {
    id: i32,
    content: String,
    created_at: i64,
}
async fn query(
    app_state: State<AppState>,
    Query(query_input_dto): Query<QueryInputDto>,
) -> impl IntoResponse {
    let mut select = tbl_log::Entity::find();

    if let Some(content) = query_input_dto.content {
        if !content.is_empty() {
            let like_pattern = format!("%{content}%");
            select = select.filter(tbl_log::Column::Content.like(like_pattern));
        }
    }
    let paginator = select
        .order_by_desc(tbl_log::Column::CreatedAt)
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
    let tbl_articles = match paginator.fetch_page(query_input_dto.page).await {
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
    let mut articles = Vec::new();
    for tbl_article in tbl_articles {
        articles.push(QueryOutputDto {
            id: tbl_article.id,
            content: tbl_article.content.chars().take(100).collect(),
            created_at: tbl_article.created_at.and_utc().timestamp_millis(),
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
                "log":articles
            }
           }
        )),
    )
}
