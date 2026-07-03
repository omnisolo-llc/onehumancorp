use axum::{
    extract::{Extension, Path, Query, State, ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use ::server_common::Claims;
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
use crate::services::agent_feed::service::AgentFeedService;
use sqlx::PgPool;
use crate::utils::cache::HybridCache;
use std::sync::{Arc, OnceLock};
use futures::{sink::SinkExt, stream::StreamExt};
use redis::AsyncCommands;

#[derive(Serialize, Deserialize, Clone)]
pub struct MobileAgentFeedItem {
    pub id: String,
    pub event_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_payload: Option<sqlx::types::Json<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_action: Option<sqlx::types::Json<serde_json::Value>>,
    pub lifecycle_state: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MobileAgentFeedListResponse {
    pub items: Vec<MobileAgentFeedItem>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AnyAgentFeedListResponse {
    Standard(AgentFeedListResponse),
    Mobile(MobileAgentFeedListResponse),
}

pub static AGENT_FEED_CACHE: OnceLock<Arc<HybridCache<AnyAgentFeedListResponse>>> = OnceLock::new();
pub static SHARED_REDIS_CLIENT: OnceLock<redis::Client> = OnceLock::new();

pub fn get_redis_client() -> redis::Client {
    SHARED_REDIS_CLIENT.get_or_init(|| {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        redis::Client::open(redis_url).expect("Failed to initialize Redis client")
    }).clone()
}

pub fn get_agent_feed_cache() -> Arc<HybridCache<AnyAgentFeedListResponse>> {
    AGENT_FEED_CACHE.get_or_init(|| {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let redis_client = match redis::Client::open(redis_url) {
            Ok(client) => Some(client),
            Err(e) => {
                tracing::warn!("Failed to initialize Redis client for AGENT_FEED_CACHE: {}. Falling back to in-memory cache.", e);
                None
            }
        };
        Arc::new(HybridCache::new(redis_client))
    }).clone()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentFeedListResponse {
    pub items: Vec<AgentFeedItem>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateStateRequest {
    pub state: String,
    pub proposed_action: Option<serde_json::Value>,
    pub context_payload: Option<serde_json::Value>,
    #[serde(default)]
    pub edited_payload: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateFeedItemRequest {
    pub event_source: String,
    pub context_payload: Option<serde_json::Value>,
    pub proposed_action: Option<serde_json::Value>,
}

#[derive(Clone)]
pub struct AgentFeedState {
    pub pool: PgPool,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", get(list_feed_items).post(create_feed_item))
        .route("/{id}/state", put(update_feed_item_state))
        .route("/ws", get(ws_feed_handler))
}

pub async fn ws_feed_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_feed_socket(socket, tenant_id))
}

async fn handle_feed_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let client = get_redis_client();

    let mut pubsub_conn = match client.get_async_pubsub().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get async pubsub for ws: {}", e);
            let _ = sender.send(WsMessage::Text("{\"error\":\"Failed to connect to pubsub\"}".into())).await;
            return;
        }
    };

    let topic = format!("agent_feed:{}", tenant_id);
    if let Err(e) = pubsub_conn.subscribe(&topic).await {
        tracing::error!("Failed to subscribe to topic {}: {}", topic, e);
        let _ = sender.send(WsMessage::Text("{\"error\":\"Failed to subscribe\"}".into())).await;
        return;
    }

    let mut stream = pubsub_conn.into_on_message();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to get pubsub payload: {}", e); // pii-safe
                    continue;
                }
            };
            if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                break; // client disconnected
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // Ignore messages from client for now
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn list_feed_items(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            if mobile_optimized {
                return (StatusCode::UNAUTHORIZED, Json(AnyAgentFeedListResponse::Mobile(MobileAgentFeedListResponse { items: vec![] }))).into_response();
            } else {
                return (StatusCode::UNAUTHORIZED, Json(AnyAgentFeedListResponse::Standard(AgentFeedListResponse { items: vec![] }))).into_response();
            }
        }
    };

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let cache_key = format!("agent_feed:{}:{}:{}:{}", tenant_id, limit, offset, mobile_optimized);
    let cache = get_agent_feed_cache();
    let tag = format!("agent_feed_tenant:{}", tenant_id);

    let result = cache.get_or_fetch_with_tags_swr(
        &cache_key,
        vec![tag],
        std::time::Duration::from_secs(60),
        move || async move {
            let repo = AgentFeedRepository::new(std::sync::Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres }));
            match repo.list(&tenant_id, limit, offset, mobile_optimized).await {
                Ok(items) => {
                    let any_response = if mobile_optimized {
                        let mobile_items = items.into_iter().map(|item| MobileAgentFeedItem {
                            id: item.id,
                            event_source: item.event_source,
                            context_payload: None,
                            proposed_action: item.proposed_action,
                            lifecycle_state: item.lifecycle_state,
                            created_at: item.created_at,
                        }).collect();
                        AnyAgentFeedListResponse::Mobile(MobileAgentFeedListResponse { items: mobile_items })
                    } else {
                        AnyAgentFeedListResponse::Standard(AgentFeedListResponse { items })
                    };
                    Some(any_response)
                },
                Err(e) => {
                    tracing::error!("Failed to list agent feed items: {}", e);
                    None
                }
            }
        }
    ).await;

    match result {
        Some(any_response) => (StatusCode::OK, Json(any_response)).into_response(),
        None => {
            if mobile_optimized {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(AnyAgentFeedListResponse::Mobile(MobileAgentFeedListResponse { items: vec![] }))).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(AnyAgentFeedListResponse::Standard(AgentFeedListResponse { items: vec![] }))).into_response()
            }
        }
    }
}

async fn create_feed_item(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFeedItemRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let service = AgentFeedService::new(pool);

    // Pass the payload as a JSON value
    let mut value_payload = serde_json::json!({});
    if let Some(cp) = &payload.context_payload {
        value_payload = cp.clone();
    }

    match service.process_event(&tenant_id, &payload.event_source, &value_payload).await {
        Ok(item) => {
            let cache = get_agent_feed_cache();
            let tag = format!("agent_feed_tenant:{}", tenant_id);
            cache.invalidate_by_tag(&tag).await;

            // Publish to Redis Pub/Sub
            let client = get_redis_client();
            let topic = format!("agent_feed:{}", tenant_id);
            if let Ok(payload_json) = serde_json::to_string(&item) {
                // In background task, to not block response
                tokio::spawn(async move {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let _: Result<(), _> = conn.publish(topic, payload_json).await;
                    }
                });
            }

            (StatusCode::CREATED, Json(item)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create agent feed item: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update_feed_item_state(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateStateRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let repo = AgentFeedRepository::new(std::sync::Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres }));

    if payload.proposed_action.is_some() || payload.context_payload.is_some() || payload.edited_payload.is_some() {
        let mut proposed = payload.proposed_action.clone();

        if let (Some(edited), Some(prop)) = (&payload.edited_payload, proposed.as_mut()) {
            if let Some(obj) = prop.as_object_mut() {
                if obj.contains_key("draft_reply") {
                    obj.insert("draft_reply".to_string(), serde_json::Value::String(edited.clone()));
                } else {
                    obj.insert("message".to_string(), serde_json::Value::String(edited.clone()));
                }
            } else {
                proposed = Some(serde_json::json!({
                    "message": edited
                }));
            }
        } else if let (Some(edited), None) = (&payload.edited_payload, proposed.as_ref()) {
            // If the user edited but there wasn't a proposed_action provided in the request payload
            // we should try to fetch the existing one and update it, but for simplicity here we
            // just create a new one.
            proposed = Some(serde_json::json!({
                "message": edited
            }));
        }

        let proposed_json = proposed.map(sqlx::types::Json);
        let context_json = payload.context_payload.clone().map(sqlx::types::Json);
        let _ = repo.update_payloads(&tenant_id, &id, context_json, proposed_json).await;
    }

    match repo.update_state(&tenant_id, &id, &payload.state).await {
        Ok(updated_item) => {
            let _ = crate::domain::agent_approvals::sync_legacy_approval_status(&tenant_id, &id, &payload.state, &pool).await;

            if payload.state == "APPROVED" {
                if let Ok(Some(item)) = repo.get(&tenant_id, &id).await {
                    let mut is_incident = false;
                    let mut feature_type = None;
                    let mut dispatch_payload = None;

                    if item.event_source == "incident_resolution" {
                         is_incident = true;
                         dispatch_payload = item.context_payload.clone().map(|p| p.0);
                    } else if let Some(ref pl) = item.proposed_action.clone().or(item.context_payload.clone()) {
                         if let Some(ft) = pl.get("feature_type").and_then(|v| v.as_str()) {
                             feature_type = Some(ft.to_string());
                             dispatch_payload = Some(pl.0.clone());
                         }
                    }

                    if is_incident || feature_type.is_some() {
                        let job_payload = serde_json::json!({
                             "action_id": id,
                             "tenant_id": tenant_id,
                             "is_incident": is_incident,
                             "feature_type": feature_type,
                             "payload": dispatch_payload,
                             "event_source": item.event_source
                        });
                        let pool_arc = std::sync::Arc::new(pool.clone());
                        let job_queue = crate::orchestration::queue::OHCJobQueue::new(pool_arc);
                        let _ = job_queue.enqueue(&tenant_id, "agent_feed_action", &job_payload).await;
                    }
                }
            }

            let cache = get_agent_feed_cache();
            let tag = format!("agent_feed_tenant:{}", tenant_id);
            cache.invalidate_by_tag(&tag).await;
            (StatusCode::OK, Json(updated_item)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to update agent feed item state: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::agent_feed;
    use sqlx::PgPool;
    use super::{get_agent_feed_cache, AgentFeedListResponse, AnyAgentFeedListResponse, ws_feed_handler};
    use axum::{Router, routing::get, extract::Extension};
    use ::server_common::Claims;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio_tungstenite::connect_async;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_agent_feed_router_compiles() {
        // Just verify that the router can be instantiated
        let _router = agent_feed::router::<PgPool>();
    }

    #[tokio::test]
    async fn test_agent_feed_cache_operations() {
        let cache = get_agent_feed_cache();
        let cache_key = "agent_feed:test_tenant:20:0:false";

        // Ensure it's empty initially
        cache.invalidate(cache_key).await;
        let result = cache.get(cache_key).await;
        assert!(result.is_none());

        let response = AnyAgentFeedListResponse::Standard(AgentFeedListResponse {
            items: vec![],
        });

        // Set cache with tag
        cache.set_with_tags(
            cache_key,
            response.clone(),
            vec!["agent_feed_tenant:test_tenant".to_string()],
            std::time::Duration::from_secs(60),
        ).await;

        // Verify cache hit
        let hit = cache.get(cache_key).await;
        assert!(hit.is_some());

        // Invalidate by tag
        cache.invalidate_by_tag("agent_feed_tenant:test_tenant").await;

        let miss = cache.get(cache_key).await;
        assert!(miss.is_none());
    }

    #[tokio::test]
    async fn test_websocket_feed() {
        // Set up test server with a fake Claims
        let mock_claims = Claims {
            sub: "user-123".to_string(),
            organization_id: Some("test_ws_tenant".to_string()),
            roles: vec!["ADMIN".to_string()],
            iat: 0,
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            exp: 9999999999,
            jti: "test_jti".to_string(),
            session_id: Some("test_session_id".to_string()),
        };

        let app = Router::new()
            .route("/ws", get(ws_feed_handler))
            .layer(Extension(mock_claims));

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

        // Use standard redis logic locally to simulate pubsub
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        if let Ok(client) = redis::Client::open(redis_url) {
            // Attempt to connect to local redis, if redis is unavailable (e.g. CI), skip the connection test
            if client.get_connection().is_ok() {
                let ws_url = format!("ws://{}/ws", addr);
                let (mut ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");

                // Sleep briefly to ensure server has subscribed to the pubsub topic
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;

                // Publish mock message to redis channel
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let topic = "agent_feed:test_ws_tenant";
                let payload = "{\"mock\":\"data\"}";
                let _: () = redis::cmd("PUBLISH").arg(topic).arg(payload).query_async(&mut conn).await.unwrap();

                // Expect to receive the message over websocket
                let msg = tokio::time::timeout(std::time::Duration::from_secs(2), ws_stream.next())
                    .await
                    .expect("Timeout waiting for websocket message")
                    .expect("Stream closed early")
                    .expect("Error receiving message");

                assert!(msg.is_text());
                assert_eq!(msg.to_text().unwrap(), payload);
            }
        }
    }
}
