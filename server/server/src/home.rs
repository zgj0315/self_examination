use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use chrono::NaiveDate;
use entity::{tbl_pdf_article, tbl_pdf_article_access_log};
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, Order, PaginatorTrait, QueryOrder, QuerySelect,
    prelude::Expr,
};
use serde::Serialize;
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

    #[derive(Serialize, FromQueryResult)]
    struct DailyAccessStat {
        day: NaiveDate,
        count: i64,
    }
    let daily_access_stats = match tbl_pdf_article_access_log::Entity::find()
        .select_only()
        .column_as(Expr::cust("DATE(created_at)"), "day")
        .column_as(tbl_pdf_article_access_log::Column::Id.count(), "count")
        .group_by(Expr::cust("DATE(created_at)"))
        .order_by(Expr::cust("DATE(created_at)"), Order::Asc)
        .limit(7)
        .into_model::<DailyAccessStat>()
        .all(&app_state.db_conn)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("tbl_pdf_article_access_log daily access stat err: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
        }
    };
    let mut daily_access_stat_output = Vec::new();
    for daily_access_stat in daily_access_stats {
        let day = daily_access_stat.day.format("%Y-%m-%d").to_string();
        daily_access_stat_output.push(json!({
            "day":day,
            "count":daily_access_stat.count
        }));
    }
    return (
        StatusCode::OK,
        Json(json!( {
                "pdf_article_count": pdf_article_count,
                "pdf_article_access_log_count": pdf_article_access_log_count,
                "daily_access_stats": daily_access_stat_output
        })),
    );
}
