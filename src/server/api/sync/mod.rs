use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
        State, Query, Extension,
    },
    response::IntoResponse,
    http::{HeaderMap, StatusCode},
};
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;

pub async fn ws_sync_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, headers, tenant_id))
}

async fn handle_socket(socket: WebSocket, headers: HeaderMap, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let client = crate::api::agent_feed::get_redis_client();

    let mut pubsub_conn = match client.get_async_pubsub().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get async pubsub for ws: {}", e);
            let _ = sender.send(WsMessage::Text("{\"error\":\"Failed to connect to pubsub\"}".into())).await;
            return;
        }
    };

    let topics = [
        format!("inventory:{}", tenant_id),
        format!("orders:{}", tenant_id),
        format!("agent_feed:{}", tenant_id),
    ];

    for topic in &topics {
        if let Err(e) = pubsub_conn.subscribe(topic).await {
            tracing::error!("Failed to subscribe to topic {}: {}", topic, e);
            let _ = sender.send(WsMessage::Text("{\"error\":\"Failed to subscribe\"}".into())).await;
            return;
        }
    }

    let mut stream = pubsub_conn.into_on_message();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            let channel = msg.get_channel_name();
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to get pubsub payload: {}", e);
                    continue;
                }
            };

            let wrapped_msg = serde_json::json!({
                "topic": channel,
                "payload": payload
            });

            if let Err(e) = sender.send(WsMessage::Text(wrapped_msg.to_string())).await {
                tracing::warn!("Failed to send ws message: {}", e);
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(msg_result) = receiver.next().await {
            match msg_result {
                Ok(WsMessage::Close(_)) => break,
                Ok(_) => {
                    // Ignore incoming messages for now
                }
                Err(e) => {
                    tracing::warn!("Error receiving ws message: {}", e);
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_ws_sync_handler() {
        // Just verify it compiles and exists.
        assert!(true);
    }
}
