use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use serde::Deserialize;

pub struct ChatAppState {
    // Shared broadcast channel for all WebSockets. In a real application,
    // this would be structured per-tenant to ensure strict isolation,
    // or messages would be filtered before transmission.
    pub tx: broadcast::Sender<(String, String)>, // (tenant_id, JSON_payload)
}

#[derive(Deserialize)]
pub struct WsAuth {
    token: Option<String>,
}

pub fn chat_ws_routes(state: Arc<ChatAppState>) -> Router {
    Router::new()
        .route("/ws/:tenant_id", get(chat_ws_handler))
        .with_state(state)
}

pub async fn chat_ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<String>,
    Query(auth): Query<WsAuth>,
    State(state): State<Arc<ChatAppState>>,
) -> impl IntoResponse {
    let token = auth.token.unwrap_or_default();
    if token.is_empty() {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from("Unauthorized: Missing token in query params"))
            .unwrap()
            .into_response();
    }

    let auth_store = crate::auth::Store::new();
    let claims = match auth_store.validate_token(&token).await {
        Ok(claims) => claims,
        Err(_) => {
            return axum::response::Response::builder()
                .status(axum::http::StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("Unauthorized: Invalid token"))
                .unwrap()
                .into_response();
        }
    };

    let authorized_tenant = claims.organization_id.unwrap_or_default();
    if authorized_tenant != tenant_id {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::FORBIDDEN)
            .body(axum::body::Body::from("Forbidden: Token not authorized for this tenant"))
            .unwrap()
            .into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: String, state: Arc<ChatAppState>) {
    let mut rx = state.tx.subscribe();

    tracing::info!("Chat WebSocket connection established for tenant: {}", tenant_id);

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok((msg_tenant_id, text)) => {
                        // Strict tenant isolation checking
                        if msg_tenant_id == tenant_id {
                            if socket.send(Message::Text(text)).await.is_err() {
                                tracing::warn!("Client disconnected");
                                break;
                            }
                        }
                    }
                    Err(RecvError::Lagged(skipped)) => {
                        tracing::warn!("Client connection lagged, skipped {} messages", skipped);
                        // Do not break the loop on lagged error, just continue
                    }
                    Err(RecvError::Closed) => {
                        tracing::error!("Broadcast channel closed");
                        break;
                    }
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        tracing::info!("Received message from client: {}", text);
                        // Process incoming message
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::info!("Client disconnected normally");
                        break;
                    }
                    Some(Err(e)) => {
                        tracing::error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
