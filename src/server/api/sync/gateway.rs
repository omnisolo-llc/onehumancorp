use axum::{
    extract::{Extension, ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use futures::{sink::SinkExt, stream::StreamExt};
use redis::AsyncCommands;

pub fn router() -> Router {
    Router::new().route("/ws", get(ws_sync_handler))
}

#[derive(Deserialize)]
struct SyncRequest {
    action: String,
    topic: String,
}

pub async fn ws_sync_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_sync_socket(socket, tenant_id))
}

async fn handle_sync_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to open redis client: {}", e);
            return;
        }
    };

    let mut pubsub = match client.get_async_pubsub().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get async pubsub: {}", e);
            return;
        }
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(WsMessage::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                msg_opt = receiver.next() => {
                    match msg_opt {
                        Some(Ok(WsMessage::Text(t))) => {
                            if let Ok(req) = serde_json::from_str::<SyncRequest>(&t) {
                                // Authorize using exact match
                                let required_suffix = format!(":{}", tenant_id);
                                if !req.topic.ends_with(&required_suffix) {
                                    let _ = tx.send("{\"error\":\"Unauthorized topic\"}".to_string()).await;
                                    continue;
                                }

                                if req.action == "subscribe" {
                                    if let Err(e) = pubsub.subscribe(&req.topic).await {
                                        tracing::error!("Failed to subscribe to {}: {}", req.topic, e);
                                    } else {
                                        let _ = tx.send(format!("{{\"status\":\"subscribed\",\"topic\":\"{}\"}}", req.topic)).await;
                                    }
                                } else if req.action == "unsubscribe" {
                                    if let Err(e) = pubsub.unsubscribe(&req.topic).await {
                                        tracing::error!("Failed to unsubscribe from {}: {}", req.topic, e);
                                    } else {
                                        let _ = tx.send(format!("{{\"status\":\"unsubscribed\",\"topic\":\"{}\"}}", req.topic)).await;
                                    }
                                }
                            }
                        }
                        Some(Err(e)) => {
                            tracing::error!("WS recv error: {}", e);
                            break;
                        }
                        None => {
                            break;
                        }
                        _ => {}
                    }
                }
                msg_opt = pubsub.on_message().next() => {
                    if let Some(msg) = msg_opt {
                        let payload: String = match msg.get_payload() {
                            Ok(p) => p,
                            Err(_) => continue,
                        };
                        let topic = msg.get_channel_name().to_string();
                        let out = serde_json::json!({
                            "topic": topic,
                            "payload": payload
                        });
                        if tx.send(out.to_string()).await.is_err() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing::get, extract::Extension};
    use ::server_common::Claims;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio_tungstenite::connect_async;

    #[tokio::test]
    async fn test_ws_sync_handler() {
        let mut mock_claims = Claims::default();
        mock_claims.organization_id = Some("test_ws_tenant".to_string());

        let app = router().layer(Extension(mock_claims));

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        });

        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        if let Ok(client) = redis::Client::open(redis_url) {
            if client.get_connection().is_ok() {
                let ws_url = format!("ws://{}/ws", addr);
                let (mut ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");

                let sub_msg = "{\"action\":\"subscribe\",\"topic\":\"inventory:test_ws_tenant\"}";
                ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(sub_msg.to_string().into())).await.unwrap();

                let msg = tokio::time::timeout(std::time::Duration::from_secs(2), ws_stream.next())
                    .await
                    .expect("Timeout waiting for websocket message")
                    .expect("Stream closed early")
                    .expect("Error receiving message");
                assert!(msg.is_text());
                assert!(msg.to_text().unwrap().contains("subscribed"));

                // Test unauthorized topic
                let unauth_msg = "{\"action\":\"subscribe\",\"topic\":\"inventory:test_ws_tenant_other\"}";
                ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(unauth_msg.to_string().into())).await.unwrap();
                let msg2 = tokio::time::timeout(std::time::Duration::from_secs(2), ws_stream.next())
                    .await
                    .expect("Timeout waiting for websocket message")
                    .expect("Stream closed early")
                    .expect("Error receiving message");
                assert!(msg2.is_text());
                assert!(msg2.to_text().unwrap().contains("Unauthorized topic"));
            }
        }
    }
}
