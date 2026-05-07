use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
};
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message as MeshMessage};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use serde::Deserialize;
use prost::Message as ProstMessage;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[derive(Deserialize)]
pub struct ConnectQuery {
    pub channel: String,
}

pub async fn mesh_ws_handler(
    ws: WebSocketUpgrade,
    State(transport): State<Arc<dyn MeshTransport>>,
    Query(query): Query<ConnectQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, transport, query.channel))
}

#[derive(serde::Deserialize)]
pub struct BroadcastRequest {
    pub topic: String,
    pub message: MeshMessage,
}

#[derive(serde::Deserialize)]
pub struct V1BroadcastRequest {
    pub agent_id: String,
    pub channel: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

pub async fn broadcast_v1_handler(
    State(transport): State<Arc<dyn MeshTransport>>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let payload_val: serde_json::Value = match serde_json::from_slice(&body_bytes) {
        Ok(v) => v,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(serde_json::json!({ "error": "Invalid JSON payload" }))).into_response(),
    };

    if payload_val.get("action").is_some() || payload_val.get("status").is_some() {
        return (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(serde_json::json!({ "error": "Deprecated key found in payload" }))).into_response();
    }

    let payload: V1BroadcastRequest = match serde_json::from_value(payload_val) {
        Ok(v) => v,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(serde_json::json!({ "error": "Missing required OHC-SIP field" }))).into_response(),
    };

    if payload.agent_id.is_empty() || payload.channel.is_empty() || payload.event_type.is_empty() || payload.data.is_null() {
        return (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(serde_json::json!({ "error": "OHC-SIP fields cannot be empty strings or null" }))).into_response();
    }

    let mesh_msg = MeshMessage {
        agent_id: payload.agent_id,
        action: payload.channel.clone(),
        status: "ok".to_string(),
        payload: payload.data.to_string().into_bytes(),
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    match transport.publish(&payload.channel, mesh_msg).await {
        Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            let error_res = serde_json::json!({ "error": e.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
        }
    }
}

pub async fn broadcast_handler(
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<BroadcastRequest>,
) -> impl IntoResponse {
    match transport.publish(&payload.topic, payload.message.into()).await {
        Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            let error_res = serde_json::json!({ "error": e.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
        }
    }
}

async fn handle_socket(socket: WebSocket, transport: Arc<dyn MeshTransport>, channel: String) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::channel::<MeshMessage>(100);

    let handler = Box::new(move |msg: MeshMessage| {
        let _ = tx.try_send(msg);
    });

    let cancel = match transport.subscribe(&channel, handler).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to subscribe to mesh transport: {}", e);
            return;
        }
    };

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let mut buf = Vec::new();
            if msg.encode(&mut buf).is_ok() {
                let text = STANDARD.encode(&buf);
                if sender.send(WsMessage::Text(text.into())).await.is_err() {
                    break;
                }
            } else {
                tracing::error!("Failed to encode mesh message to protobuf");
            }
        }
    });

    let transport_clone = transport.clone();
    let channel_clone = channel.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Text(text) = msg {
                if let Ok(buf) = STANDARD.decode(text.as_str()) {
                    if let Ok(mesh_msg) = MeshMessage::decode(&buf[..]) {
                        let _ = transport_clone.publish(&channel_clone, mesh_msg).await;
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    cancel();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        routing::get,
        Router,
    };
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

    #[tokio::test]
    async fn test_mesh_ws_handler() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let transport_clone = transport.clone();

        let app = Router::new()
            .route("/api/v1/mesh/connect", get(mesh_ws_handler))
            .route("/api/mesh/broadcast", axum::routing::post(broadcast_v1_handler))
            .route("/api/mesh/v2/broadcast", axum::routing::post(broadcast_handler))
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

        let ws_url = format!("ws://{}/api/v1/mesh/connect?channel=test_chan", addr);
        let (mut ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");

        // Test sending a message from client to server (publish)
        let test_msg = MeshMessage {
            agent_id: "test".to_string(),
            action: "test_chan".to_string(),
            status: "ok".to_string(),
            payload: b"ws_test".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        let mut buf = Vec::new();
        test_msg.encode(&mut buf).unwrap();
        let b64 = base64::engine::general_purpose::STANDARD.encode(&buf);
        ws_stream.send(TungsteniteMessage::Text(b64.into())).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Test receiving a message from server to client (subscribe)
        let srv_msg = crate::ohc::orchestration::TeammateMeshEvent {
            agent_id: "test".to_string(),
            action: "test_chan".to_string(),
            status: "ok".to_string(),
            payload: b"srv_test".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        transport_clone.publish("test_chan", srv_msg.clone()).await.unwrap();

        let mut found = false;
        for _ in 0..2 {
            if let Some(Ok(msg)) = ws_stream.next().await {
                if let TungsteniteMessage::Text(text) = msg {
                    let buf = base64::engine::general_purpose::STANDARD.decode(&text).unwrap();
                    let received_mesh_msg: MeshMessage = prost::Message::decode(&buf[..]).unwrap();
                    if received_mesh_msg.payload == b"srv_test" {
                        assert_eq!(received_mesh_msg.action, "test_chan");
                        found = true;
                        break;
                    }
                }
            }
        }
        assert!(found, "Did not receive the srv_test message");
    }

    #[tokio::test]
    async fn test_broadcast_v1_validation() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let app = Router::new()
            .route("/api/mesh/broadcast", axum::routing::post(broadcast_v1_handler))
            .with_state(transport);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let url = format!("http://{}/api/mesh/broadcast", addr);

        // Valid payload
        let valid_payload = serde_json::json!({
            "agent_id": "123",
            "channel": "mesh:tasks",
            "event_type": "TASK",
            "data": {"foo": "bar"}
        });
        let res = client.post(&url).json(&valid_payload).send().await.unwrap();
        assert_eq!(res.status(), 200);

        // Missing agent_id
        let missing_payload = serde_json::json!({
            "channel": "mesh:tasks",
            "event_type": "TASK",
            "data": {"foo": "bar"}
        });
        let res = client.post(&url).json(&missing_payload).send().await.unwrap();
        assert_eq!(res.status(), 400); // 400 Bad Request explicit rejection

        // Empty string payload
        let empty_payload = serde_json::json!({
            "agent_id": "",
            "channel": "mesh:tasks",
            "event_type": "TASK",
            "data": {"foo": "bar"}
        });
        let res = client.post(&url).json(&empty_payload).send().await.unwrap();
        assert_eq!(res.status(), 400); // Because our handler checks for is_empty()

        // Deprecated keys
        let deprecated_payload = serde_json::json!({
            "agent_id": "123",
            "channel": "mesh:tasks",
            "event_type": "TASK",
            "data": {"foo": "bar"},
            "action": "do_something"
        });
        let res = client.post(&url).json(&deprecated_payload).send().await.unwrap();
        assert_eq!(res.status(), 400);
    }
}
