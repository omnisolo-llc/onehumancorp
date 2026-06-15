use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
    http::HeaderMap,
};
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message as MeshMessage};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SubscribeMessage {
    pub r#type: String, // "subscribe"
    pub topics: Vec<String>,
}

#[derive(Deserialize)]
pub struct WsAuthQuery {
    pub spiffe_id: Option<String>,
}

pub async fn client_sync_ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsAuthQuery>,
    State(transport): State<Arc<dyn MeshTransport>>,
) -> impl IntoResponse {
    let spiffe_id = query.spiffe_id.unwrap_or_default();

    // Validate SPIFFE ID and extract tenant_id securely
    let tenant_id = if spiffe_id.starts_with("spiffe://ohc/org/") {
        let parts: Vec<&str> = spiffe_id.split('/').collect();
        if parts.len() >= 5 {
            parts[4].to_string()
        } else {
            return axum::http::StatusCode::UNAUTHORIZED.into_response();
        }
    } else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };

    ws.on_upgrade(move |socket| handle_client_socket(socket, transport, tenant_id, spiffe_id))
}

async fn handle_client_socket(socket: WebSocket, transport: Arc<dyn MeshTransport>, tenant_id: String, _spiffe_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<MeshMessage>(100);

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // Send as JSON to web clients instead of protobuf
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(WsMessage::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    let transport_clone = transport.clone();
    let tenant_clone = tenant_id.clone();

    // Store cancellation functions for active subscriptions
    let mut active_subs = std::collections::HashMap::<String, Box<dyn Fn() + Send + Sync>>::new();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Text(text) = msg {
                if let Ok(sub_msg) = serde_json::from_str::<SubscribeMessage>(&text) {
                    if sub_msg.r#type == "subscribe" {
                        for topic in sub_msg.topics {
                            // Enforce multi-tenant authorization
                            let allowed_prefix = format!("{}:", tenant_clone);
                            if !topic.starts_with(&allowed_prefix) {
                                tracing::warn!("Unauthorized subscription attempt by {} to {}", tenant_clone, topic);
                                continue;
                            }

                            if !active_subs.contains_key(&topic) {
                                let tx_clone = tx.clone();
                                let handler = Box::new(move |msg: MeshMessage| {
                                    let _ = tx_clone.try_send(msg);
                                });

                                if let Ok(cancel) = transport_clone.subscribe(&topic, handler).await {
                                    active_subs.insert(topic.clone(), cancel);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Clean up subscriptions
        for (_, cancel) in active_subs.drain() {
            cancel();
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
    use axum::{routing::get, Router};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_client_sync_ws_handler() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let transport_clone = transport.clone();

        let app = Router::new()
            .route("/api/v1/sync/connect", get(client_sync_ws_handler))
            .with_state(transport);

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

        let ws_url = format!("ws://{}/api/v1/sync/connect?spiffe_id=spiffe://ohc/org/tenant_a/agent/client", addr);
        let request = axum::http::Request::builder()
            .uri(ws_url)
            .header("Host", addr.to_string())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
            .body(())
            .unwrap();

        let (mut ws_stream, _) = connect_async(request).await.expect("Failed to connect");

        // Subscribe to authorized topic
        let sub_msg = SubscribeMessage {
            r#type: "subscribe".to_string(),
            topics: vec!["tenant_a:inventory".to_string()],
        };
        ws_stream.send(TungsteniteMessage::Text(serde_json::to_string(&sub_msg).unwrap().into())).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // Publish to that topic
        let pub_msg = MeshMessage {
            agent_id: "system".to_string(),
            action: "tenant_a:inventory".to_string(),
            status: "ok".to_string(),
            payload: b"{\"product_id\":\"p1\",\"stock\":5}".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        transport_clone.publish("tenant_a:inventory", pub_msg).await.unwrap();

        // Receive message
        let msg = ws_stream.next().await.unwrap().unwrap();
        if let TungsteniteMessage::Text(text) = msg {
            let received: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(received["action"], "tenant_a:inventory");
        } else {
            panic!("Expected text message");
        }

        // Attempt to subscribe to unauthorized topic
        let unauthorized_msg = SubscribeMessage {
            r#type: "subscribe".to_string(),
            topics: vec!["tenant_b:orders".to_string()],
        };
        ws_stream.send(TungsteniteMessage::Text(serde_json::to_string(&unauthorized_msg).unwrap().into())).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // Publish to unauthorized topic
        let pub_msg2 = MeshMessage {
            agent_id: "system".to_string(),
            action: "tenant_b:orders".to_string(),
            status: "ok".to_string(),
            payload: b"{}".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        transport_clone.publish("tenant_b:orders", pub_msg2).await.unwrap();

        // Should not receive the unauthorized message.
        // We can test this by publishing another authorized message and ensuring it's the next one we receive.
        let pub_msg3 = MeshMessage {
            agent_id: "system".to_string(),
            action: "tenant_a:inventory".to_string(),
            status: "ok".to_string(),
            payload: b"{\"product_id\":\"p2\",\"stock\":0}".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        transport_clone.publish("tenant_a:inventory", pub_msg3).await.unwrap();

        let msg2 = ws_stream.next().await.unwrap().unwrap();
        if let TungsteniteMessage::Text(text) = msg2 {
            let received: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(received["action"], "tenant_a:inventory");
        } else {
            panic!("Expected text message");
        }
    }
}
