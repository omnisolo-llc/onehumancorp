use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
    http::HeaderMap,
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
    pub agent_id: String,
    pub action: String,
    pub status: String,
    pub payload: Vec<u8>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DirectRequest {
    pub target_agent_id: String,
    pub agent_id: String,
    pub action: String,
    pub status: String,
    pub payload: Vec<u8>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MailboxRequest {
    pub mailbox_id: String,
    pub agent_id: String,
    pub action: String,
    pub status: String,
    pub payload: Vec<u8>,
}

fn check_spiffe_auth(headers: &HeaderMap) -> Result<String, axum::response::Response> {
    let spiffe_id = headers.get("x-spiffe-id")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    if spiffe_id.is_empty() {
        let error_res = serde_json::json!({ "error": "unauthorized" });
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::response::Json(error_res)).into_response());
    }

    let parts: Vec<&str> = spiffe_id.split('/').collect();
    if parts.len() < 7 || parts[2] != "ohc" || parts[3] != "org" || parts[5] != "agent" || parts[4] != "system" {
        let error_res = serde_json::json!({ "error": "unauthorized" });
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::response::Json(error_res)).into_response());
    }

    Ok(spiffe_id.to_string())
}

pub async fn orchestration_broadcast_handler(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<BroadcastRequest>,
) -> impl IntoResponse {
    if let Err(err_response) = check_spiffe_auth(&headers) {
        return err_response;
    }

    let msg = MeshMessage {
        agent_id: payload.agent_id,
        action: payload.action,
        status: payload.status,
        payload: payload.payload,
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    match transport.publish(&payload.topic, msg.into()).await {
        Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            let error_res = serde_json::json!({ "error": e.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
        }
    }
}

pub async fn orchestration_tasks_stream_handler(
    ws: WebSocketUpgrade,
    State(transport): State<Arc<dyn MeshTransport>>,
    Query(query): Query<ConnectQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, transport, query.channel))
}

pub async fn broadcast_handler(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<BroadcastRequest>,
) -> impl IntoResponse {
    if let Err(err_response) = check_spiffe_auth(&headers) {
        return err_response;
    }

    let msg = MeshMessage {
        agent_id: payload.agent_id,
        action: payload.action,
        status: payload.status,
        payload: payload.payload,
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    match transport.publish(&payload.topic, msg.into()).await {
        Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            let error_res = serde_json::json!({ "error": e.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
        }
    }
}

pub async fn direct_handler(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<DirectRequest>,
) -> impl IntoResponse {
    if let Err(err_response) = check_spiffe_auth(&headers) {
        return err_response;
    }

    let msg = MeshMessage {
        agent_id: payload.agent_id,
        action: payload.action,
        status: payload.status,
        payload: payload.payload,
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    let topic = format!("mesh:direct:{}", payload.target_agent_id);
    match transport.publish(&topic, msg.into()).await {
        Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            let error_res = serde_json::json!({ "error": e.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
        }
    }
}

pub async fn mailbox_handler(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<MailboxRequest>,
) -> impl IntoResponse {
    if let Err(err_response) = check_spiffe_auth(&headers) {
        return err_response;
    }

    let msg = MeshMessage {
        agent_id: payload.agent_id,
        action: payload.action,
        status: payload.status,
        payload: payload.payload,
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    let topic = format!("mesh:mailbox:{}", payload.mailbox_id);
    match transport.publish(&topic, msg.into()).await {
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
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

    #[tokio::test]
    async fn test_mesh_ws_handler() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let transport_clone = transport.clone();

        let app = Router::new()
            .route("/api/v1/mesh/connect", get(mesh_ws_handler))
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
        let srv_msg = ::server_ohc::orchestration::TeammateMeshEvent {
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
    async fn test_mesh_direct_handler() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());

        let app = Router::new()
            .route("/api/mesh/v2/direct", axum::routing::post(direct_handler))
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
        let url = format!("http://{}/api/mesh/v2/direct", addr);

        let req_body = DirectRequest {
            target_agent_id: "agent-1".to_string(),
            agent_id: "test".to_string(),
            action: "test_action".to_string(),
            status: "ok".to_string(),
            payload: b"ws_test".to_vec(),
        };

        // Missing x-spiffe-id header
        let res = client.post(&url).json(&req_body).send().await.unwrap();
        assert_eq!(res.status(), 401);

        // With x-spiffe-id header
        let res = client.post(&url).header("x-spiffe-id", "spiffe://ohc/org/system/agent/agent-1").json(&req_body).send().await.unwrap();
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn test_mesh_mailbox_handler() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());

        let app = Router::new()
            .route("/api/mesh/v2/mailbox", axum::routing::post(mailbox_handler))
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
        let url = format!("http://{}/api/mesh/v2/mailbox", addr);

        let req_body = MailboxRequest {
            mailbox_id: "mailbox-1".to_string(),
            agent_id: "test".to_string(),
            action: "test_action".to_string(),
            status: "ok".to_string(),
            payload: b"ws_test".to_vec(),
        };

        // Missing x-spiffe-id header
        let res = client.post(&url).json(&req_body).send().await.unwrap();
        assert_eq!(res.status(), 401);

        // With x-spiffe-id header
        let res = client.post(&url).header("x-spiffe-id", "spiffe://ohc/org/system/agent/agent-1").json(&req_body).send().await.unwrap();
        assert_eq!(res.status(), 200);
    }
}
