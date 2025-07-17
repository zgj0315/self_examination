use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use entity::{tbl_pdf_article, tbl_pdf_article_access_log};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::AppState;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/pdf_article_access_logs", get(query))
        .with_state(state)
}

#[derive(Deserialize, Debug, Validate)]
struct QueryInputDto {
    src_ip: Option<String>,
    user_agent: Option<String>,
    size: u64,
    page: u64,
}

#[derive(Serialize, Debug)]
struct QueryOutputDto {
    id: i32,
    article_id: i32,
    article_title: String,
    src_ip: String,
    user_agent: String,
    created_at: i64,
}
async fn query(
    app_state: State<AppState>,
    Query(query_input_dto): Query<QueryInputDto>,
) -> impl IntoResponse {
    let mut select = tbl_pdf_article_access_log::Entity::find();
    if let Some(src_ip) = query_input_dto.src_ip {
        if !src_ip.is_empty() {
            let like_pattern = format!("%{src_ip}%");
            select = select.filter(tbl_pdf_article_access_log::Column::SrcIp.like(like_pattern));
        }
    }
    if let Some(user_agent) = query_input_dto.user_agent {
        if !user_agent.is_empty() {
            let like_pattern = format!("%{user_agent}%");
            select =
                select.filter(tbl_pdf_article_access_log::Column::UserAgent.like(like_pattern));
        }
    }
    let paginator = select
        .order_by_desc(tbl_pdf_article_access_log::Column::CreatedAt)
        .paginate(&app_state.db_conn, query_input_dto.size);
    let num_pages = match paginator.num_pages().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("num_pages err: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };
    let num_items = match paginator.num_items().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("num_items err: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!( {
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "msg": "pg connection err".to_string(),
                })),
            );
        }
    };
    let tbl_pdf_article_access_logs = match paginator.fetch_page(query_input_dto.page).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("fetch_page err: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!( {
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "msg": "pg content err".to_string(),
                })),
            );
        }
    };
    let mut pdf_article_access_logs = Vec::new();
    for tbl_pdf_article_access_log in tbl_pdf_article_access_logs {
        let title =
            match tbl_pdf_article::Entity::find_by_id(tbl_pdf_article_access_log.pdf_article_id)
                .select_only()
                .column(tbl_pdf_article::Column::Title)
                .into_tuple::<String>()
                .one(&app_state.db_conn)
                .await
            {
                Ok(title_op) => match title_op {
                    Some(v) => v,
                    None => {
                        log::error!(
                            "not find title, pdf_article_id: {}",
                            tbl_pdf_article_access_log.pdf_article_id
                        );
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!( {
                                    "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                                    "msg": "not find title".to_string(),
                            })),
                        );
                    }
                },
                Err(e) => {
                    log::error!("tbl_pdf_article_access_log count err: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!( {
                                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                                "msg": "pg connection err".to_string(),
                        })),
                    );
                }
            };
        pdf_article_access_logs.push(QueryOutputDto {
            id: tbl_pdf_article_access_log.id,
            article_id: tbl_pdf_article_access_log.pdf_article_id,
            article_title: title,
            src_ip: tbl_pdf_article_access_log.src_ip,
            user_agent: tbl_pdf_article_access_log.user_agent,
            created_at: tbl_pdf_article_access_log
                .created_at
                .and_utc()
                .timestamp_millis(),
        });
    }
    (
        StatusCode::OK,
        Json(json!(
            {
            "page":{
              "size":query_input_dto.size,
              "total_elements":num_items,
              "total_pages":num_pages
            },
            "_embedded":{
                "pdf_article_access_log":pdf_article_access_logs
            }
           }
        )),
    )
}
