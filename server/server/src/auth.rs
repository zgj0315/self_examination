use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{ConnectInfo, FromRequestParts, State},
    http::{StatusCode, request::Parts},
    response::IntoResponse,
    routing::post,
};
use entity::tbl_auth_user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::AppState;
pub fn routers(state: AppState) -> Router {
    Router::new().route("/login", post(login)).with_state(state)
}
#[derive(Deserialize, Debug, Validate)]
struct LoginInputDto {
    username: String,
    password: String,
}
async fn login(
    app_state: State<AppState>,
    Json(login_input_dto): Json<LoginInputDto>,
) -> impl IntoResponse {
    match tbl_auth_user::Entity::find()
        .filter(tbl_auth_user::Column::Username.eq(&login_input_dto.username))
        .one(&app_state.db_conn)
        .await
    {
        Ok(tbl_auth_user_op) => match tbl_auth_user_op {
            Some(tbl_auth_user) => {
                if tbl_auth_user.password.eq(&login_input_dto.password) {
                    (
                        StatusCode::OK,
                        [("code", "200"), ("msg", "ok")],
                        Json(json!({})),
                    )
                } else {
                    (
                        StatusCode::BAD_REQUEST,
                        [("code", "400"), ("msg", "user not exists")],
                        Json(json!({})),
                    )
                }
            }
            None => {
                log::warn!("user {} not exists", login_input_dto.username);
                (
                    StatusCode::BAD_REQUEST,
                    [("code", "400"), ("msg", "auth failed")],
                    Json(json!({})),
                )
            }
        },
        Err(e) => {
            log::error!("tbl_auth_user find err: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("code", "500"), ("msg", "tbl_auth_user find err")],
                Json(json!({})),
            )
        }
    }
}
pub struct RequireAuth;

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // let auth_header = parts
        //     .headers
        //     .get(header::AUTHORIZATION)
        //     .and_then(|value| value.to_str().ok());
        // log::info!("auth_header: {:?}", auth_header);

        match ConnectInfo::<SocketAddr>::from_request_parts(parts, state).await {
            Ok(ConnectInfo(socket_addr)) => {
                // log::info!("socket_addr: {:?}", socket_addr);
                log::info!("{} {} {}", socket_addr.ip(), parts.method, parts.uri);
            }
            Err(e) => {
                log::error!("get source ip err: {}", e);
            }
        }
        Ok(Self)
    }
}
