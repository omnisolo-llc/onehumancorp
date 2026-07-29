use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Extension},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::chat::service::ChatService;
use crate::db::DB;

#[derive(Deserialize)]
struct ClientMessage {
    action: String,
    conversation_id: String,
    content: Option<String>,
}

#[derive(Serialize)]
struct ServerMessage {
    action: String,
    message: Option<crate::services::chat::models::ChatMessage>,
    error: Option<String>,
}

pub async fn handle_chat_ws(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<server_common::Claims>,
    Extension(db): Extension<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) if !org_id.is_empty() => org_id.to_string(),
        _ => "default".to_string(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, db.pool.clone()))
}

async fn handle_socket(socket: WebSocket, tenant_id_str: String, pool: sqlx::PgPool) {
    let (mut sender, mut receiver) = socket.split();
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(id) => id,
        Err(_) => {
            let _ = sender.send(WsMessage::Close(None)).await;
            return;
        }
    };

    let chat_service = ChatService::new(pool);
    let pubsub_channel = format!("chat:tenant:{}", tenant_id);
    let redis_client_opt = crate::redis_pool::get_redis_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    // Redis PubSub subscription task
    let pubsub_task = async {
        if let Some(client) = redis_client_opt.clone() {
            if let Ok(mut conn) = client.get_async_pubsub().await {
                if conn.subscribe(&pubsub_channel).await.is_ok() {
                    let mut pubsub_stream = conn.on_message();
                    while let Some(msg) = pubsub_stream.next().await {
                        if let Ok(payload) = msg.get_payload::<String>() {
                            let _ = tx.send(payload).await;
                        }
                    }
                }
            }
        }
    };

    // Forward messages from Redis to the WebSocket
    let mut tx_socket = sender;
    let forward_task = async move {
        while let Some(msg) = rx.recv().await {
            if tx_socket.send(WsMessage::Text(msg.into())).await.is_err() {
                break;
            }
        }
    };

    // Receive messages from WebSocket
    let recv_task = async {
        let redis_client = redis_client_opt.clone();
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    if client_msg.action == "send_message" {
                        if let Some(content) = client_msg.content {
                            if let Ok(conv_id) = Uuid::parse_str(&client_msg.conversation_id) {
                                match chat_service.send_message(
                                    tenant_id,
                                    conv_id,
                                    "agent".to_string(), // Sender type
                                    None, // Sender ID (could map from claims)
                                    content,
                                ).await {
                                    Ok(chat_msg) => {
                                        // Broadcast the new message via Redis
                                        if let Some(client) = &redis_client {
                                            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                                let server_msg = ServerMessage {
                                                    action: "new_message".to_string(),
                                                    message: Some(chat_msg),
                                                    error: None,
                                                };
                                                if let Ok(json) = serde_json::to_string(&server_msg) {
                                                    let _: redis::RedisResult<()> = redis::cmd("PUBLISH")
                                                        .arg(&pubsub_channel)
                                                        .arg(&json)
                                                        .query_async(&mut conn).await;
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        tracing::error!("Failed to save chat message: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    tokio::select! {
        _ = pubsub_task => {},
        _ = forward_task => {},
        _ = recv_task => {},
    }
}
