use axum::{
    extract::{Extension, Query, State, Json, ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    http::StatusCode,
    Router,
    routing::get,
};
use ::server_common::Claims;
use ::server_ohc::orchestration::{SyncMcpDeltasRequest, DeltaItem};
use ::server_ohc::orchestration::sync_service_server::SyncService;
use serde::Deserialize;
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use std::sync::OnceLock;

use std::sync::Arc;
use tokio::sync::Mutex;

static SYNC_BROADCAST: OnceLock<broadcast::Sender<String>> = OnceLock::new();
static REDIS_SUBSCRIBED: OnceLock<Arc<Mutex<bool>>> = OnceLock::new();

use axum::routing::post;

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/ws", get(ws_sync_handler))
}

pub fn router_with_pool<S: Clone + Send + Sync + 'static>() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/power_sync_pull", post(power_sync_pull_handler))
        .route("/power_sync_push", post(power_sync_push_handler))
}


async fn validate_token_and_get_tenant(pool: &sqlx::PgPool, headers: &axum::http::HeaderMap) -> Result<(String, String), axum::response::Response> {
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()),
    };

    let repo = std::sync::Arc::new(crate::auth::postgres_store::PgUserRepository::new(pool.clone()));
    let store = std::sync::Arc::new(crate::auth::Store::with_repo(repo));

    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()),
    };

    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let agent_id = claims.sub;

    Ok((tenant_id, agent_id))
}

pub async fn power_sync_pull_handler(
    State(pool): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(_payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let spiffe_id_str = match validate_token_and_get_tenant(&pool, &headers).await {
        Ok((tenant_id, agent_id)) => format!("spiffe://onehumancorp.io/org/{}/agent/{}", tenant_id, agent_id),
        Err(e) => return e,
    };
    let mut tonic_request = tonic::Request::new(::server_ohc::orchestration::PowerSyncPullRequest {});

    if let Ok(metadata_value) = spiffe_id_str.parse() {
        tonic_request.metadata_mut().insert("x-spiffe-id", metadata_value);
    }

    let service = crate::services::sync::service::MySyncService::new(pool);
    match service.power_sync_pull(tonic_request).await {
        Ok(resp) => {
            let inner = resp.into_inner();
            (StatusCode::OK, axum::Json(serde_json::from_str::<serde_json::Value>(&inner.payload).unwrap_or_else(|_| serde_json::json!([])))).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({
                "status": "error",
                "message": e.message(),
            }))).into_response()
        }
    }
}

pub async fn power_sync_push_handler(
    State(pool): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let spiffe_id_str = match validate_token_and_get_tenant(&pool, &headers).await {
        Ok((tenant_id, agent_id)) => format!("spiffe://onehumancorp.io/org/{}/agent/{}", tenant_id, agent_id),
        Err(e) => return e,
    };
    let payload_str = serde_json::to_string(&payload.get("payload").unwrap_or(&payload)).unwrap_or_else(|_| "[]".to_string());

    let mut tonic_request = tonic::Request::new(::server_ohc::orchestration::PowerSyncPushRequest {
        payload: payload_str,
    });

    if let Ok(metadata_value) = spiffe_id_str.parse() {
        tonic_request.metadata_mut().insert("x-spiffe-id", metadata_value);
    }

    let service = crate::services::sync::service::MySyncService::new(pool);
    match service.power_sync_push(tonic_request).await {
        Ok(resp) => {
            let inner = resp.into_inner();
            (StatusCode::OK, axum::Json(serde_json::json!({
                "status": inner.status,
            }))).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({
                "status": "error",
                "message": e.message(),
            }))).into_response()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct McpDeltasPayload {
    pub deltas: Vec<DeltaItemPayload>,
}

#[derive(serde::Deserialize)]
pub struct DeltaItemPayload {
    pub id: String,
    pub entity_id: String,
    pub data: String,
    pub updated_at: String,
}

pub async fn sync_mcp_deltas_handler(
    State(pool): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<McpDeltasPayload>,
) -> impl IntoResponse {
    let (tenant_id, agent_id) = match validate_token_and_get_tenant(&pool, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };
    let spiffe_id_str = format!("spiffe://onehumancorp.io/org/{}/agent/{}", tenant_id, agent_id);

    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({
            "status": "error",
            "message": "missing tenant identity in session",
            "synced_count": 0
        }))).into_response();
    }

    let mut tonic_request = tonic::Request::new(SyncMcpDeltasRequest {
        tenant_id: tenant_id.clone(),
        deltas: payload.deltas.into_iter().map(|d| DeltaItem {
            id: d.id,
            entity_id: d.entity_id,
            data: d.data,
            updated_at: d.updated_at,
        }).collect(),
    });

    if let Ok(metadata_value) = spiffe_id_str.parse() {
        tonic_request.metadata_mut().insert("x-spiffe-id", metadata_value);
    }

    let service = crate::services::sync::service::MySyncService::new(pool);
    match service.sync_mcp_deltas(tonic_request).await {
        Ok(resp) => {
            let inner = resp.into_inner();
            (StatusCode::OK, axum::Json(serde_json::json!({
                "status": inner.status,
                "message": inner.message,
                "synced_count": inner.synced_count
            }))).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({
                "status": "error",
                "message": e.message(),
                "synced_count": 0
            }))).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct WsQuery {
    pub topics: Option<String>,
}

fn get_redis_client() -> redis::Client {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    redis::Client::open(redis_url).expect("Invalid Redis URL")
}

async fn ensure_redis_subscription() {
    let subscribed = REDIS_SUBSCRIBED.get_or_init(|| Arc::new(Mutex::new(false)));
    let mut is_sub = subscribed.lock().await;
    if *is_sub {
        return;
    }
    *is_sub = true;

    tokio::spawn(async move {
        let client = get_redis_client();
        let mut pubsub_conn = match client.get_async_pubsub().await {
            Ok(conn) => conn,
            Err(e) => {
                tracing::error!("Failed to get async pubsub for sync ws: {}", e);
                return;
            }
        };

        if let Err(e) = pubsub_conn.psubscribe("*").await {
             tracing::error!("Failed to psubscribe: {}", e);
             return;
        }

        let mut pubsub_stream = pubsub_conn.on_message();
        let tx = SYNC_BROADCAST.get_or_init(|| {
            let (tx, _) = broadcast::channel(1000);
            tx
        });

        while let Some(msg) = pubsub_stream.next().await {
            let channel_name = msg.get_channel_name().to_string();
            if channel_name.starts_with("inventory:") || channel_name.starts_with("orders:") || channel_name.starts_with("tenant_events:") {
                if let Ok(payload) = msg.get_payload::<String>() {
                    let wrapped_msg = serde_json::json!({
                        "channel": channel_name,
                        "payload": payload
                    }).to_string();
                    let _ = tx.send(wrapped_msg);
                }
            }
        }
    });
}

pub async fn ws_sync_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let topics = query.topics.unwrap_or_else(|| "default".to_string())
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    ws.on_upgrade(move |socket| handle_sync_socket(socket, tenant_id, topics))
}

async fn handle_sync_socket(socket: WebSocket, tenant_id: String, topics: Vec<String>) {
    ensure_redis_subscription().await;

    let (mut sender, mut receiver) = socket.split();

    let tx = SYNC_BROADCAST.get_or_init(|| {
        let (tx, _) = broadcast::channel(1000);
        tx
    });
    let mut rx = tx.subscribe();

    let target_channels: Vec<String> = topics.iter().map(|t| format!("{}:{}", t, tenant_id)).collect();

    loop {
        tokio::select! {
            msg_res = rx.recv() => {
                match msg_res {
                    Ok(msg_str) => {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&msg_str) {
                            if let Some(channel) = parsed.get("channel").and_then(|c| c.as_str()) {
                                if target_channels.contains(&channel.to_string()) {
                                    if let Some(payload) = parsed.get("payload").and_then(|p| p.as_str()) {
                                        if let Err(e) = sender.send(WsMessage::Text(payload.to_string().into())).await {
                                            tracing::error!("Failed to send sync message to client: {}", e);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Client lagged, ignore and continue
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            client_msg = receiver.next() => {
                match client_msg {
                    Some(Ok(msg)) => {
                        if let WsMessage::Close(_) = msg {
                            break;
                        }
                        if let WsMessage::Ping(data) = msg {
                            if let Err(e) = sender.send(WsMessage::Pong(data)).await {
                                tracing::error!("Failed to send pong: {}", e);
                                break;
                            }
                        }
                    }
                    Some(Err(e)) => {
                        tracing::error!("Error receiving from client ws: {}", e);
                        break;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
}
