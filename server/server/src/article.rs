use crate::AppState;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use entity::tbl_article;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/search", get(search))
        .with_state(state)
}

#[derive(Deserialize, Debug, Validate)]
struct SearchInputDto {
    title: Option<String>,
    content: Option<String>,
    size: u64,
    page: u64,
}

#[derive(Serialize, Debug)]
struct SearchOutputDto {
    id: i32,
    title: String,
    content: String,
    created_at: i64,
    updated_at: i64,
}
async fn search(
    app_state: State<AppState>,
    Query(search_input_dto): Query<SearchInputDto>,
) -> impl IntoResponse {
    let mut select = tbl_article::Entity::find();
    if let Some(title) = search_input_dto.title {
        if !title.is_empty() {
            let like_pattern = format!("%{title}%");
            select = select.filter(tbl_article::Column::Title.like(like_pattern));
        }
    }
    if let Some(content) = search_input_dto.content {
        if !content.is_empty() {
            let like_pattern = format!("%{content}%");
            select = select.filter(tbl_article::Column::Content.like(like_pattern));
        }
    }
    let paginator = select
        .order_by_desc(tbl_article::Column::UpdatedAt)
        .paginate(&app_state.pg_conn, search_input_dto.size);
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
    let tbl_articles = match paginator.fetch_page(search_input_dto.page).await {
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
        articles.push(SearchOutputDto {
            id: tbl_article.id,
            title: tbl_article.title,
            content: tbl_article.content,
            created_at: tbl_article.created_at.and_utc().timestamp_millis(),
            updated_at: tbl_article.updated_at.and_utc().timestamp_millis(),
        });
    }
    (
        StatusCode::OK,
        [("code", "200"), ("msg", "ok")],
        Json(json!(
            {
            "page":{
              "size":search_input_dto.size,
              "total_elements":num_items,
              "total_pages":num_pages
            },
            "_embedded":{
                "article":articles
            }
           }
        )),
    )
}
