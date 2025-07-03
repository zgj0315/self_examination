use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::AppState;

pub async fn search(
    _app_state: State<AppState>,
    // Query(search_input_dto): Query<SearchInputDto>,
) -> impl IntoResponse {
    let articles: Vec<String> = Vec::new();
    (
        StatusCode::OK,
        [("code", "200"), ("msg", "ok")],
        Json(json!(
            {
            "page":{
              "size":20_u64,
              "total_elements":55_u64,
              "total_pages":3_u64
            },
            "_embedded":{
                "article":articles
            }
           }
        )),
    )
}
