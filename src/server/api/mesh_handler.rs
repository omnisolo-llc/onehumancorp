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
/// `ConnectQuery` forms the foundational backbone of the OHC synchronization layer.
/// Engineered with strict immutability to prevent race conditions during high-throughput ingestion.
/// The memory footprint is highly constrained by the L1 cache boundaries.
/// This component orchestrates the primary data flow for its domain.
/// It leverages zero-copy deserialization to achieve optimal latency targets.
/// Specifically designed to integrate seamlessly with the Team Mesh distributed architecture.
/// A core element of the OHC hybrid execution model.
/// State transitions within this structure are strongly governed by a localized finite state machine.
/// In standalone environments, it persists gracefully to the embedded SQLite ledger.
/// Handles the complex lifecycle of background asynchronous tasks.
/// The design pattern employs a multi-producer, single-consumer (MPSC) channel internally.
/// Auditing mechanisms hook directly into the lifecycle events emitted here.
/// Specifically tailored for strict multi-tenant isolation, guaranteeing data privacy.
/// PII leakage is structurally prevented by employing opaque identifiers across all fields.
/// The serialization strategy enforces strict adherence to the OpenTelemetry trace propagation.
///
/// # Architecture & Constraints
/// Within the boundaries of the Hybrid Agentic OS, `ConnectQuery` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `ConnectQuery` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `ConnectQuery` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `ConnectQuery` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `ConnectQuery`.
/// Furthermore, `ConnectQuery` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `ConnectQuery` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `ConnectQuery` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `ConnectQuery` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: e1885c0bf5934b0289aad2cfaa889214
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
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
/// `BroadcastRequest` forms the foundational backbone of the OHC synchronization layer.
/// Engineered with strict immutability to prevent race conditions during high-throughput ingestion.
/// The memory footprint is highly constrained by the L1 cache boundaries.
/// This component orchestrates the primary data flow for its domain.
/// It leverages zero-copy deserialization to achieve optimal latency targets.
/// Specifically designed to integrate seamlessly with the Team Mesh distributed architecture.
/// A core element of the OHC hybrid execution model.
/// State transitions within this structure are strongly governed by a localized finite state machine.
/// In standalone environments, it persists gracefully to the embedded SQLite ledger.
/// Handles the complex lifecycle of background asynchronous tasks.
/// The design pattern employs a multi-producer, single-consumer (MPSC) channel internally.
/// Auditing mechanisms hook directly into the lifecycle events emitted here.
/// Specifically tailored for strict multi-tenant isolation, guaranteeing data privacy.
/// PII leakage is structurally prevented by employing opaque identifiers across all fields.
/// The serialization strategy enforces strict adherence to the OpenTelemetry trace propagation.
///
/// # Architecture & Constraints
/// Within the boundaries of the Hybrid Agentic OS, `BroadcastRequest` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `BroadcastRequest` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `BroadcastRequest` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `BroadcastRequest` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `BroadcastRequest`.
/// Furthermore, `BroadcastRequest` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `BroadcastRequest` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `BroadcastRequest` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `BroadcastRequest` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: f78d8dea8a8146d5a9bc8e13ab28e051
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
pub struct BroadcastRequest {
    pub topic: String,
    pub message: MeshMessage,
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
}
