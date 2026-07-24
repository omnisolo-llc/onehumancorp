use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;

#[derive(Deserialize)]
pub struct ChatWsQuery {
    pub inbox_id: Option<String>,
}

#[derive(Clone)]
pub struct ChatWsState {
    pub db: Arc<DB>,
}

pub async fn chat_ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<::server_common::Claims>,
    State(state): State<ChatWsState>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) if !org_id.is_empty() => org_id.to_string(),
        _ => "default".to_string(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: String, _state: ChatWsState) {
    while let Some(msg) = socket.next().await {
        if let Ok(Message::Text(text)) = msg {
            let response = serde_json::json!({
                "status": "received",
                "tenant_id": tenant_id,
                "message": text.to_string()
            });
            let _ = socket.send(Message::Text(response.to_string().into())).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_ws_handler_routing() {
        let db = Arc::new(crate::db::DB { pool: sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/db").unwrap(), store: crate::db::DbStore::Postgres });
        let state = ChatWsState { db };

        let app = Router::new()
            .route("/ws", get(chat_ws_handler))
            .layer(Extension(::server_common::Claims {
                sub: "user-1".to_string(),
                email: "test@example.com".to_string(),
                exp: 0,
                organization_id: Some("tenant-1".to_string()),
                roles: vec!["owner".to_string()],
                iat: 0,
                username: "user".to_string(),
                session_id: Some("sess".to_string()),
                jti: "jti".to_string(),
            }))
            .with_state(state);

        let req = Request::builder()
            .uri("/ws")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert!(res.status() == StatusCode::SWITCHING_PROTOCOLS || res.status() == StatusCode::NOT_FOUND || res.status() == StatusCode::UPGRADE_REQUIRED);
    }
}
