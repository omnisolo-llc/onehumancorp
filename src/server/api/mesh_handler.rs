use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse, Json,
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


#[derive(serde::Serialize, Deserialize)]
pub struct BroadcastRequest {
    pub agent_id: String,
    pub channel: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

pub async fn mesh_broadcast_handler(
    State(transport): State<Arc<dyn MeshTransport>>,
    Json(req): Json<BroadcastRequest>,
) -> impl IntoResponse {
    let payload_json = match serde_json::to_string(&req) {
        Ok(s) => s,
        Err(e) => return (axum::http::StatusCode::BAD_REQUEST, format!("Invalid OHC-SIP payload: {}", e)).into_response(),
    };

    let msg = MeshMessage {
        topic: req.channel.clone(),
        payload: payload_json.into_bytes(),
    };

    match transport.publish(&req.channel, msg).await {
        Ok(_) => (axum::http::StatusCode::OK, "Broadcasted successfully").into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn mesh_ws_handler(
    ws: WebSocketUpgrade,
    State(transport): State<Arc<dyn MeshTransport>>,
    Query(query): Query<ConnectQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, transport, query.channel))
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
            eprintln!("Failed to subscribe to mesh transport: {}", e);
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
                eprintln!("Failed to encode mesh message to protobuf");
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
        let test_payload = serde_json::json!({
            "agent_id": "test_agent",
            "channel": "test_chan",
            "event_type": "TEST_EVENT",
            "data": {"foo": "bar"}
        });
        let test_msg = MeshMessage {
            topic: "test_chan".to_string(),
            payload: serde_json::to_vec(&test_payload).unwrap(),
        };
        let mut buf = Vec::new();
        test_msg.encode(&mut buf).unwrap();
        let b64 = base64::engine::general_purpose::STANDARD.encode(&buf);
        ws_stream.send(TungsteniteMessage::Text(b64.into())).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Test receiving a message from server to client (subscribe)
        let srv_payload = serde_json::json!({
            "agent_id": "srv_agent",
            "channel": "test_chan",
            "event_type": "SRV_EVENT",
            "data": {"baz": "qux"}
        });
        let srv_msg = MeshMessage {
            topic: "test_chan".to_string(),
            payload: serde_json::to_vec(&srv_payload).unwrap(),
        };
        transport_clone.publish("test_chan", srv_msg.clone()).await.unwrap();

        let mut found = false;
        for _ in 0..2 {
            if let Some(Ok(msg)) = ws_stream.next().await {
                if let TungsteniteMessage::Text(text) = msg {
                    let buf = base64::engine::general_purpose::STANDARD.decode(&text).unwrap();
                    let received_mesh_msg: MeshMessage = prost::Message::decode(&buf[..]).unwrap();
                    let received_json: serde_json::Value = serde_json::from_slice(&received_mesh_msg.payload).unwrap();
                    if received_json["agent_id"] == "srv_agent" {
                        assert_eq!(received_mesh_msg.topic, "test_chan");
                        found = true;
                        break;
                    }
                }
            }
        }
        assert!(found, "Did not receive the srv_agent message");
    }

    #[tokio::test]
    async fn test_mesh_broadcast_handler_compliance() {
        use axum::routing::post;
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let app = Router::new()
            .route("/api/mesh/broadcast", post(mesh_broadcast_handler))
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

        let client = reqwest::Client::new();
        let url = format!("http://{}/api/mesh/broadcast", addr);

        // Test compliant payload
        let compliant_payload = serde_json::json!({
            "agent_id": "test_agent",
            "channel": "test_chan",
            "event_type": "TEST_EVENT",
            "data": {"foo": "bar"}
        });
        let resp = client.post(&url).json(&compliant_payload).send().await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::OK);

        // Test non-compliant payload (missing agent_id)
        let non_compliant_payload = serde_json::json!({
            "channel": "test_chan",
            "event_type": "TEST_EVENT",
            "data": {"foo": "bar"}
        });
        let resp = client.post(&url).json(&non_compliant_payload).send().await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::UNPROCESSABLE_ENTITY); // Axum returns 422 for Json deserialization failure
    }
}
