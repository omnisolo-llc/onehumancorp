use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;
use futures::{stream::StreamExt};
use crate::db::DB;

#[derive(Deserialize)]
pub struct WsQuery {
    pub tenant_id: String,
}

pub async fn handle_ws_upgrade(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(_db): State<Arc<DB>>, // In a real scenario, use this to subscribe to redis pub/sub
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, query.tenant_id))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: String) {
    tracing::info!("Websocket connected for tenant: {}", tenant_id);

    if socket.send(WsMessage::Text(format!("Connected to Omnichannel Chat for {}", tenant_id).into())).await.is_err() {
        return;
    }

    while let Some(Ok(msg)) = socket.next().await {
        if let WsMessage::Text(text) = msg {
            if socket.send(WsMessage::Text(format!("Echo: {}", text).into())).await.is_err() {
                break;
            }
        }
    }

    tracing::info!("Websocket disconnected for tenant: {}", tenant_id);
}
