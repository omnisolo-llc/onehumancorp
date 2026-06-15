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
use axum::{
    extract::Request,
    middleware::Next,
    body::Body,
};

pub async fn validation_middleware(
    req: Request,
    next: Next,
) -> Result<axum::response::Response, axum::response::Response> {
    let (parts, body) = req.into_parts();
    // Use a sensible limit of 5MB for the payload to prevent memory exhaustion DoS
    let bytes = match axum::body::to_bytes(body, 5 * 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => {
            let error_res = serde_json::json!({ "error": "failed to read body or body too large" });
            return Err((axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response());
        }
    };

    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        if let Some(obj) = json.as_object() {
            let required_keys = ["agent_id", "channel", "event_type", "data"];
            let mut missing_keys = Vec::new();

            // Check for presence of required keys
            for key in required_keys.iter() {
                if !obj.contains_key(*key) {
                    missing_keys.push(*key);
                }
            }

            if !missing_keys.is_empty() {
                let error_res = serde_json::json!({ "error": format!("missing required keys: {:?}", missing_keys) });
                return Err((axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response());
            }

            // Check for EXACTLY four keys to prevent any additional payload fields, including deprecated ones
            if obj.len() != 4 {
                let error_res = serde_json::json!({ "error": "payload must contain exactly four root-level keys: agent_id, channel, event_type, data" });
                return Err((axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response());
            }
        } else {
            let error_res = serde_json::json!({ "error": "json payload must be an object" });
            return Err((axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response());
        }
    } else {
        let error_res = serde_json::json!({ "error": "invalid json payload" });
        return Err((axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response());
    }

    let req = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(req).await)
}

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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DirectRequest {
    pub target_agent_id: String,
    pub message: MeshMessage,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MailboxRequest {
    pub mailbox_id: String,
    pub message: MeshMessage,
}

fn check_spiffe_auth(headers: &HeaderMap) -> Result<(String, String, String), axum::response::Response> {
    let spiffe_id = headers.get("x-spiffe-id")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    if spiffe_id.is_empty() {
        let error_res = serde_json::json!({ "error": "unauthorized" });
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::response::Json(error_res)).into_response());
    }

    match ::server_auth::parse_spiffe_id(spiffe_id) {
        Ok((org_id, agent_id)) => Ok((org_id, agent_id, spiffe_id.to_string())),
        Err(_) => {
            let error_res = serde_json::json!({ "error": "unauthorized" });
            Err((axum::http::StatusCode::UNAUTHORIZED, axum::response::Json(error_res)).into_response())
        }
    }
}


/// Helper method to format responses
fn publish_response(result: Result<(), String>) -> axum::response::Response {
    match result {
        Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            let error_res = serde_json::json!({ "error": e.to_string() });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
        }
    }
}

/// Handler to broadcast messages via HTTP
pub async fn orchestration_broadcast_handler(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<BroadcastRequest>,
) -> impl IntoResponse {
    let (org_id, _agent_id, _spiffe_id) = match check_spiffe_auth(&headers) {
        Ok(res) => res,
        Err(err) => return err,
    };

    let isolated_topic = format!("tenant:{}:{}", org_id, payload.topic);
    publish_response(transport.publish(&isolated_topic, payload.message.into()).await)
}

/// Handler for WebSockets to stream orchestration tasks
pub async fn orchestration_tasks_stream_handler(
    ws: WebSocketUpgrade,
    State(transport): State<Arc<dyn MeshTransport>>,
    Query(query): Query<ConnectQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, transport, query.channel))
}

/// Broadcast handler for general mesh communication
pub async fn broadcast_handler(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    axum::Json(payload): axum::Json<BroadcastRequest>,
) -> impl IntoResponse {
    let (org_id, _agent_id, _spiffe_id) = match check_spiffe_auth(&headers) {
        Ok(res) => res,
        Err(err) => return err,
    };

