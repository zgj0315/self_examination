use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::{StatusCode, header, request::Parts},
};

pub struct RequireAuth;

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        log::info!("auth_header: {:?}", auth_header);

        let uri = parts.uri.to_string();
        log::info!("uri: {uri}");

        match ConnectInfo::<SocketAddr>::from_request_parts(parts, state).await {
            Ok(ConnectInfo(socket_addr)) => {
                log::info!("socket_addr: {:?}", socket_addr);
            }
            Err(e) => {
                log::error!("get source ip err: {}", e);
            }
        }
        Ok(Self)
    }
}
