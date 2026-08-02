use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::api::unified_ws::get_broadcast_tx;

#[derive(Deserialize)]
pub struct WsQuery {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
}

#[derive(Deserialize)]
pub struct ClientMessage {
    pub content: String,
    pub sender_id: Option<Uuid>,
}

use server_common::Claims;
use axum::extract::Extension;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&query.tenant_id.to_string()) {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, db, query))
}

async fn handle_socket(socket: WebSocket, db: Arc<DB>, query: WsQuery) {
    let (mut sender, mut receiver) = socket.split();
    let service = ChatService::new(db.pool.clone());
    let mut rx = get_broadcast_tx().subscribe();
    let broadcast_tx = get_broadcast_tx();

    // Create topic for this conversation
    let topic = format!("chat:{}:{}", query.tenant_id, query.conversation_id);

    let mut send_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    // Check if it's our envelope
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&msg) {
                        if let Some(msg_topic) = v.get("topic").and_then(|t| t.as_str()) {
                            if msg_topic == topic {
                                if let Some(data) = v.get("data") {
                                    let _ = sender.send(Message::Text(data.to_string())).await;
                                }
                            }
                        }
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    continue;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    });

    let topic_clone = format!("chat:{}:{}", query.tenant_id, query.conversation_id);
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg_res) = receiver.next().await {
            let Ok(Message::Text(text)) = msg_res else { continue; };
            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                if let Ok(saved_msg) = service.send_message(
                    query.tenant_id,
                    query.conversation_id,
                    "contact".to_string(),
                    client_msg.sender_id,
                    client_msg.content,
                ).await {
                    let envelope = serde_json::json!({
                        "channel": "chat",
                        "topic": topic_clone,
                        "seq": 0,
                        "data": saved_msg,
                        "ts": chrono::Utc::now().timestamp_millis()
                    });
                    let _ = broadcast_tx.send(envelope.to_string());
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
