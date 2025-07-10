use crate::AppState;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
};
use entity::tbl_article;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

pub fn routers(state: AppState) -> Router {
    Router::new()
        .route("/articles", get(query).post(create))
        .route("/articles/{id}", patch(update).delete(delete))
        .with_state(state)
}

#[derive(Deserialize, Debug, Validate)]
struct QueryInputDto {
    title: Option<String>,
    content: Option<String>,
    size: u64,
    page: u64,
}

#[derive(Serialize, Debug)]
struct QueryOutputDto {
    id: i32,
    title: String,
    content: String,
    created_at: i64,
    updated_at: i64,
}
async fn query(
    app_state: State<AppState>,
    Query(query_input_dto): Query<QueryInputDto>,
) -> impl IntoResponse {
    let mut select = tbl_article::Entity::find();
    if let Some(title) = query_input_dto.title {
        if !title.is_empty() {
            let like_pattern = format!("%{title}%");
            select = select.filter(tbl_article::Column::Title.like(like_pattern));
        }
    }
    if let Some(content) = query_input_dto.content {
        if !content.is_empty() {
            let like_pattern = format!("%{content}%");
            select = select.filter(tbl_article::Column::Content.like(like_pattern));
        }
    }
    let paginator = select
        .order_by_desc(tbl_article::Column::UpdatedAt)
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
            title: tbl_article.title.chars().take(10).collect(),
            content: tbl_article.content.chars().take(10).collect(),
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
              "size":query_input_dto.size,
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

#[derive(Deserialize, Debug, Validate)]
struct CreateInputDto {
    title: String,
    content: String,
}
async fn create(
    app_state: State<AppState>,
    Json(create_input_dto): Json<CreateInputDto>,
) -> impl IntoResponse {
    let tbl_article_am = tbl_article::ActiveModel {
        title: Set(create_input_dto.title),
        content: Set(create_input_dto.content),
        ..Default::default()
    };
    match tbl_article::Entity::insert(tbl_article_am)
        .exec(&app_state.db_conn)
        .await
    {
        Ok(insert_result) => {
            let artile_id = insert_result.last_insert_id;
            (
                StatusCode::OK,
                [("code", "200"), ("msg", "ok")],
                Json(json!({
                    "artile_id":artile_id
                })),
            )
        }
        Err(e) => {
            log::error!("tbl_article insert err: {}", e);
            (
                StatusCode::OK,
                [("code", "500"), ("msg", "insert pg err")],
                Json(json!({})),
            )
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
struct UpdateInputDto {
    title: Option<String>,
    content: Option<String>,
}
#[derive(Serialize, Debug)]
struct UpdateOutputDto {
    id: i32,
    title: String,
    content: String,
    created_at: i64,
    updated_at: i64,
}
async fn update(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(update_input_dto): Json<UpdateInputDto>,
) -> impl IntoResponse {
    let tbl_article = match tbl_article::Entity::find_by_id(id)
        .one(&app_state.db_conn)
        .await
    {
        Ok(tbl_article_opt) => match tbl_article_opt {
            Some(model) => model,
            None => {
                log::error!("not found: {id}",);
                return (
                    StatusCode::BAD_REQUEST,
                    [("code", "400"), ("msg", "not found")],
                    Json(json!({})),
                );
            }
        },
        Err(e) => {
            log::error!("find by id {id} err: {e}");
            return (
                StatusCode::BAD_REQUEST,
                [("code", "400"), ("msg", "find by id {id} err")],
                Json(json!({})),
            );
        }
    };
    let mut tbl_article_am = tbl_article.into_active_model();
    if let Some(title) = update_input_dto.title {
        tbl_article_am.title = Set(title);
    }
    if let Some(content) = update_input_dto.content {
        tbl_article_am.content = Set(content);
    }
    tbl_article_am.updated_at = Set(chrono::Utc::now().naive_utc());
    match tbl_article::Entity::update(tbl_article_am)
        .exec(&app_state.db_conn)
        .await
    {
        Ok(model) => {
            let update_output_dto = UpdateOutputDto {
                id,
                title: model.title,
                content: model.content,
                created_at: model.created_at.and_utc().timestamp_millis(),
                updated_at: model.updated_at.and_utc().timestamp_millis(),
            };
            let json = match serde_json::to_string(&update_output_dto) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("update_output_dto to_string() {id} err: {e}");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        [("code", "500"), ("msg", "update_output_dto to_string()")],
                        Json(json!({})),
                    );
                }
            };
            (
                StatusCode::OK,
                [("code", "200"), ("msg", "ok")],
                Json(sea_orm::JsonValue::String(json)),
            )
        }
        Err(e) => {
            log::error!("update db {id} err: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("code", "500"), ("msg", "update db err")],
                Json(json!({})),
            )
        }
    }
}

async fn delete(Path(id): Path<i32>, State(app_state): State<AppState>) -> impl IntoResponse {
    match tbl_article::Entity::delete_by_id(id)
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
