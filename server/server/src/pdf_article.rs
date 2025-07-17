use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{ConnectInfo, Multipart, Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::{get, patch},
};
use entity::{tbl_pdf_article, tbl_pdf_article_access_log};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::AppState;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/pdf_articles", get(query).post(create))
        .route(
            "/pdf_articles/{id}",
            patch(update).delete(delete).get(get_pdf_content),
        )
        .with_state(state)
}

#[derive(Deserialize, Debug, Validate)]
struct QueryInputDto {
    title: Option<String>,
    size: u64,
    page: u64,
}

#[derive(Serialize, Debug)]
struct QueryOutputDto {
    id: i32,
    title: String,
    access_count: u64,
    created_at: i64,
    updated_at: i64,
}
async fn query(
    app_state: State<AppState>,
    Query(query_input_dto): Query<QueryInputDto>,
) -> impl IntoResponse {
    let mut select = tbl_pdf_article::Entity::find();
    if let Some(title) = query_input_dto.title {
        if !title.is_empty() {
            let like_pattern = format!("%{title}%");
            select = select.filter(tbl_pdf_article::Column::Title.like(like_pattern));
        }
    }

    let paginator = select
        .order_by_desc(tbl_pdf_article::Column::CreatedAt)
        .paginate(&app_state.db_conn, query_input_dto.size);
    let num_pages = match paginator.num_pages().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("num_pages err: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
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
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!( {
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "msg": "pg connection err".to_string(),
                })),
            );
        }
    };
    let tbl_pdf_articles = match paginator.fetch_page(query_input_dto.page).await {
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
    let mut pdf_articles = Vec::new();
    for tbl_pdf_article in tbl_pdf_articles {
        let access_count = match tbl_pdf_article_access_log::Entity::find()
            .filter(tbl_pdf_article_access_log::Column::PdfArticleId.eq(tbl_pdf_article.id))
            .count(&app_state.db_conn)
            .await
        {
            Ok(count) => count,
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
        pdf_articles.push(QueryOutputDto {
            id: tbl_pdf_article.id,
            title: tbl_pdf_article.title,
            access_count,
            created_at: tbl_pdf_article.created_at.and_utc().timestamp_millis(),
            updated_at: tbl_pdf_article.updated_at.and_utc().timestamp_millis(),
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
                "pdf_article":pdf_articles
            }
           }
        )),
    )
}

async fn create(app_state: State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    let mut file_ids = Vec::new();
    while let Some(field) = match multipart.next_field().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("multipart.next_field err: {}", e);
            return (StatusCode::BAD_REQUEST, Json(json!({})));
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
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
                    }
                }
            } else {
                match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e_bytes) => {
                        log::error!("get field bytes err: {}", e_bytes);
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
                    }
                }
            };
            let tbl_pdf_article_am = tbl_pdf_article::ActiveModel {
                title: Set(file_name),
                pdf_content: Set(content_bytes),
                ..Default::default()
            };
            match tbl_pdf_article::Entity::insert(tbl_pdf_article_am)
                .exec(&app_state.db_conn)
                .await
            {
                Ok(insert_result) => {
                    file_ids.push(insert_result.last_insert_id);
                }
                Err(e) => {
                    log::error!("tbl_file insert err: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "file_ids":file_ids
        })),
    )
}

async fn update(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let tbl_pdf_article = match tbl_pdf_article::Entity::find_by_id(id)
        .one(&app_state.db_conn)
        .await
    {
        Ok(v_op) => match v_op {
            Some(v) => v,
            None => {
                log::warn!("tbl_pdf_article not find {}", id);
                return (StatusCode::BAD_REQUEST, Json(json!({})));
            }
        },
        Err(e) => {
            log::error!("tbl_pdf_article find err: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };
    let mut tbl_pdf_article_am = tbl_pdf_article.into_active_model();

    if let Some(field) = match multipart.next_field().await {
        Ok(v) => v,
        Err(e) => {
            log::error!("multipart.next_field err: {}", e);
            return (StatusCode::BAD_REQUEST, Json(json!({})));
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
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
                    }
                }
            } else {
                match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e_bytes) => {
                        log::error!("get field bytes err: {}", e_bytes);
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
                    }
                }
            };
            tbl_pdf_article_am.title = Set(file_name);
            tbl_pdf_article_am.pdf_content = Set(content_bytes);
            tbl_pdf_article_am.updated_at = Set(chrono::Utc::now().naive_utc());
            match tbl_pdf_article::Entity::update(tbl_pdf_article_am)
                .exec(&app_state.db_conn)
                .await
            {
                Ok(_) => {
                    return (StatusCode::OK, Json(json!({})));
                }
                Err(e) => {
                    log::error!("tbl_file insert err: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
                }
            }
        }
    }
    return (StatusCode::BAD_REQUEST, Json(json!({})));
}

async fn delete(Path(id): Path<i32>, State(app_state): State<AppState>) -> impl IntoResponse {
    match tbl_pdf_article::Entity::delete_by_id(id)
        .exec(&app_state.db_conn)
        .await
    {
        Ok(delete_result) => {
            if delete_result.rows_affected == 1 {
                log::info!("delete {id} success");
            } else {
                log::warn!(
                    "delete {id} success, affected row: {}",
                    delete_result.rows_affected
                );
            }
            (
                StatusCode::OK,
                [("code", "200"), ("msg", "ok")],
                Json(json!({})),
            )
        }
        Err(e) => {
            log::error!("delete db {id} err: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("code", "500"), ("msg", "delete db err")],
                Json(json!({})),
            )
        }
    }
}

async fn get_pdf_content(
    Path(id): Path<i32>,
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match tbl_pdf_article::Entity::find_by_id(id)
        .one(&app_state.db_conn)
        .await
    {
        Ok(tbl_pdf_article_op) => match tbl_pdf_article_op {
            Some(tbl_pdf_article) => {
                let user_agent = headers
                    .get("user-agent")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("Unknown");
                let tbl_pdf_article_access_log_am = tbl_pdf_article_access_log::ActiveModel {
                    pdf_article_id: Set(tbl_pdf_article.id),
                    src_ip: Set(socket_addr.ip().to_string()),
                    user_agent: Set(user_agent.to_string()),
                    ..Default::default()
                };
                if let Err(e) =
                    tbl_pdf_article_access_log::Entity::insert(tbl_pdf_article_access_log_am)
                        .exec(&app_state.db_conn)
                        .await
                {
                    log::error!("tbl_pdf_article_access_log insert err: {}", e);
                }
                let mut headers = HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/octet-stream"),
                );
                headers.insert(
                    header::CONTENT_DISPOSITION,
                    HeaderValue::from_str(&format!(
                        "attachment; filename=\"{}\"",
                        tbl_pdf_article.title
                    ))
                    .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
                );
                (StatusCode::OK, headers, tbl_pdf_article.pdf_content).into_response()
            }
            None => {
                log::warn!("not find file_id: {}", id);
                (StatusCode::BAD_REQUEST, Json(json!({}))).into_response()
            }
        },
        Err(e) => {
            log::error!("find file_id: {}, err: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response()
        }
    }
}
