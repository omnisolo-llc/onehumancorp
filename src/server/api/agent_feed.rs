use super::ws_compression::{encode_json, negotiate};
use axum::{
    Json, Router,
    extract::{
        Extension, Query, State,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::domain::repository::agent_feed_repo::{AgentFeedItem, AgentFeedRepository};
use crate::services::agent_feed::service::AgentFeedService;
use crate::utils::cache::HybridCache;
use ::server_common::Claims;
use chrono::{DateTime, Utc};
use futures::{sink::SinkExt, stream::StreamExt};
use redis::AsyncCommands;
use sqlx::PgPool;
use std::collections::VecDeque;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::sync::{Mutex, Notify};

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

pub fn get_redis_client() -> Option<redis::Client> {
    crate::redis_pool::get_redis_client()
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

#[path = "agent_feed/decisions.rs"]
pub mod decisions;
pub use decisions::UpdateStateRequest;

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
        .route(
            "/api/v1/agent-feed",
            get(list_feed_items).post(create_feed_item),
        )
        .route(
            "/api/v1/agent-feed/",
            get(list_feed_items).post(create_feed_item),
        )
        .merge(decisions::router())
        .route("/api/v1/agent-feed/ws", get(ws_feed_handler))
}

pub async fn ws_feed_handler(
    ws: WebSocketUpgrade,
    claims: Option<Extension<Claims>>,
) -> impl IntoResponse {
    let tenant_id = match claims.and_then(|Extension(c)| c.organization_id) {
        Some(org_id) if !org_id.is_empty() => org_id,
        _ => "default".to_string(),
    };

    let (ws, gzip) = negotiate(ws);
    ws.on_upgrade(move |socket| handle_feed_socket(socket, tenant_id, gzip))
}

async fn handle_feed_socket(socket: WebSocket, tenant_id: String, gzip: bool) {
    let (mut sender, mut receiver) = socket.split();

    let client = match get_redis_client() {
        Some(c) => c,
        None => {
            tracing::warn!("Redis unavailable for agent feed ws");
            let _ = sender
                .send(WsMessage::Text(
                    "{\"error\":\"Failed to connect to pubsub\"}".into(),
                ))
                .await;
            return;
        }
    };

    let mut pubsub_conn = match client.get_async_pubsub().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get async pubsub for ws: {}", e);
            let _ = sender
                .send(WsMessage::Text(
                    "{\"error\":\"Failed to connect to pubsub\"}".into(),
                ))
                .await;
            return;
        }
    };

    let topic = format!("agent_feed:{}", tenant_id);
    if let Err(e) = pubsub_conn.subscribe(&topic).await {
        tracing::error!("Failed to subscribe to topic {}: {}", topic, e);
        let _ = sender
            .send(WsMessage::Text(
                "{\"error\":\"Failed to subscribe\"}".into(),
            ))
            .await;
        return;
    }

    let mut stream = pubsub_conn.into_on_message();

    // Bounded buffer with drop-oldest semantics (256 capacity)
    let buffer: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::with_capacity(256)));
    let notify = Arc::new(Notify::new());

    // Task 1: Redis subscriber → buffer (non-blocking, drops oldest on full)
    let buf_producer = buffer.clone();
    let notify_producer = notify.clone();
    let mut pubsub_task = tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(_) => {
                    tracing::error!("Failed to decode pubsub message");
                    continue;
                }
            };
            {
                let mut q = buf_producer.lock().await;
                if q.len() >= 256 {
                    q.pop_front(); // drop oldest
                }
                q.push_back(payload);
            }
            notify_producer.notify_one();
        }
    });

    // Task 2: Buffer → WebSocket with batching (50ms window, max 20)
    let buf_consumer = buffer.clone();
    let notify_consumer = notify.clone();
    let mut send_task = tokio::spawn(async move {
        let mut batch: Vec<String> = Vec::new();
        let mut tick = tokio::time::interval(Duration::from_millis(50));
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            // Drain all available messages from buffer first
            {
                let mut q = buf_consumer.lock().await;
                if batch.is_empty() && !q.is_empty() {
                    tick.reset();
                }
                while batch.len() < 20 {
                    let Some(msg) = q.pop_front() else { break };
                    batch.push(msg);
                }
            }

            if batch.len() >= 20 {
                if flush_batch(&mut sender, &mut batch, gzip).await.is_err() {
                    return;
                }
                continue;
            }

            if !batch.is_empty() {
                // We have messages but < 20; flush on timer
                tokio::select! {
                    _ = notify_consumer.notified() => {
                        // More messages may be available — loop back to drain
                    }
                    _ = tick.tick() => {
                        if flush_batch(&mut sender, &mut batch, gzip).await.is_err() { return; }
                    }
                    else => break,
                }
            } else {
                // No messages — wait for notification or timer
                tokio::select! {
                    _ = notify_consumer.notified() => {
                        // Loop back to drain
                    }
                    _ = tick.tick() => {
                        // Timer fired with no messages — nothing to do
                    }
                    else => break,
                }
            }
        }
        // Flush any remaining messages
        if !batch.is_empty() && flush_batch(&mut sender, &mut batch, gzip).await.is_err() {}
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // Ignore messages from client for now
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
            pubsub_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
            pubsub_task.abort();
        },
        _ = (&mut pubsub_task) => {
            send_task.abort();
            recv_task.abort();
        },
    };
}

async fn flush_batch(
    sender: &mut futures::stream::SplitSink<WebSocket, WsMessage>,
    batch: &mut Vec<String>,
    gzip: bool,
) -> Result<(), axum::Error> {
    if let Some(payload) = super::ws_batch::take_batch(batch) {
        sender.send(encode_json(payload, gzip)).await?;
    }
    Ok(())
}

pub async fn list_feed_items(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            if mobile_optimized {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(AnyAgentFeedListResponse::Mobile(
                        MobileAgentFeedListResponse { items: vec![] },
                    )),
                )
                    .into_response();
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(AnyAgentFeedListResponse::Standard(AgentFeedListResponse {
                        items: vec![],
                    })),
                )
                    .into_response();
            }
        }
    };

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Approval lists must reflect the canonical store on every request.
    // Tag invalidation cannot make process-local SWR snapshots authoritative.
    let repo = AgentFeedRepository::new(std::sync::Arc::new(crate::db::DB {
        pool: pool.clone(),
        store: crate::db::DbStore::Postgres,
    }));
    let result = match repo.list(&tenant_id, limit, offset, mobile_optimized).await {
        Ok(items) => {
            let any_response = if mobile_optimized {
                let mobile_items = items
                    .into_iter()
                    .map(|item| MobileAgentFeedItem {
                        id: item.id,
                        event_source: item.event_source,
                        context_payload: None,
                        proposed_action: None,
                        lifecycle_state: item.lifecycle_state,
                        created_at: item.created_at,
                    })
                    .collect();
                AnyAgentFeedListResponse::Mobile(MobileAgentFeedListResponse {
                    items: mobile_items,
                })
            } else {
                AnyAgentFeedListResponse::Standard(AgentFeedListResponse { items })
            };
            Some(any_response)
        }
        Err(e) => {
            tracing::error!("Failed to list agent feed items: {}", e);
            None
        }
    };

    match result {
        Some(any_response) => (
            StatusCode::OK,
            [("cache-control", "no-store")],
            Json(any_response),
        )
            .into_response(),
        None => {
            if mobile_optimized {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [("cache-control", "no-store")],
                    Json(AnyAgentFeedListResponse::Mobile(
                        MobileAgentFeedListResponse { items: vec![] },
                    )),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [("cache-control", "no-store")],
                    Json(AnyAgentFeedListResponse::Standard(AgentFeedListResponse {
                        items: vec![],
                    })),
                )
                    .into_response()
            }
        }
    }
}

pub async fn create_feed_item(
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

    match service
        .process_event(&tenant_id, &payload.event_source, &value_payload)
        .await
    {
        Ok(item) => {
            crate::invalidate_agent_feed_caches(&tenant_id).await;

            // Publish to Redis Pub/Sub
            if let Some(client) = get_redis_client() {
                let topic = format!("agent_feed:{}", tenant_id);
                if let Ok(payload_json) = serde_json::to_string(&item) {
                    // In background task, to not block response
                    tokio::spawn(async move {
                        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                            let _: Result<(), _> = conn.publish(topic, payload_json).await;
                        }
                    });
                }
            }

            (StatusCode::CREATED, Json(item)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create agent feed item: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AgentFeedListResponse, AnyAgentFeedListResponse, get_agent_feed_cache, ws_feed_handler,
    };
    use crate::api::agent_feed;
    use ::server_common::Claims;
    use axum::{Router, extract::Extension, routing::get};
    use futures::StreamExt;
    use sqlx::PgPool;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio_tungstenite::connect_async;

    #[tokio::test]
    async fn test_agent_feed_router_compiles() {
        // Just verify that the router can be instantiated
        let _router = agent_feed::router::<PgPool>();
    }

    // Exercise the mounted list route after claims extraction. This is a
    // repository-failure contract, not evidence of authentication success.
    async fn assert_cached_list_fails_closed(mobile_optimized: bool) {
        use axum::body::{Body, to_bytes};
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let tenant_id = format!("feed-closed-pool-{}", uuid::Uuid::new_v4());
        let cache_key = format!("agent_feed:{tenant_id}:20:0:{mobile_optimized}");
        let cache = get_agent_feed_cache();
        let item = super::AgentFeedItem {
            id: format!("{tenant_id}-action"),
            tenant_id: tenant_id.clone(),
            event_source: "fixture".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: None,
            updated_at: None,
        };
        let cached = if mobile_optimized {
            AnyAgentFeedListResponse::Mobile(super::MobileAgentFeedListResponse {
                items: vec![super::MobileAgentFeedItem {
                    id: item.id,
                    event_source: item.event_source,
                    context_payload: None,
                    proposed_action: None,
                    lifecycle_state: item.lifecycle_state,
                    created_at: item.created_at,
                }],
            })
        } else {
            AnyAgentFeedListResponse::Standard(AgentFeedListResponse { items: vec![item] })
        };
        cache
            .set_with_tags(
                &cache_key,
                cached,
                vec![format!("agent_feed_tenant:{tenant_id}")],
                std::time::Duration::from_secs(60),
            )
            .await;
        let warm =
            serde_json::to_value(cache.get(&cache_key).await.expect("warm list entry")).unwrap();
        assert_eq!(warm["items"][0]["lifecycle_state"], "PENDING_APPROVAL");

        // A closed lazy pool fails immediately and never opens a DB connection.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused:unused@127.0.0.1:1/closed_fixture")
            .unwrap();
        pool.close().await;
        assert!(pool.is_closed());
        let claims = Claims {
            sub: "closed-pool-owner".to_string(),
            organization_id: Some(tenant_id),
            roles: vec!["ADMIN".to_string()],
            iat: 0,
            username: "fixture".to_string(),
            email: "fixture@example.test".to_string(),
            exp: 9999999999,
            jti: "closed-pool-fixture".to_string(),
            session_id: Some("closed-pool-session".to_string()),
        };
        let response = agent_feed::router::<PgPool>()
            .with_state(pool)
            .layer(Extension(claims))
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/api/v1/agent-feed?limit=20&offset=0&mobile_optimized={mobile_optimized}"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let cache_control = response.headers().get("cache-control").cloned();
        let body = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
        cache.invalidate(&cache_key).await;

        assert_eq!(
            status,
            StatusCode::INTERNAL_SERVER_ERROR,
            "a cached list must not conceal an unavailable canonical store"
        );
        assert_eq!(payload, serde_json::json!({ "items": [] }));
        assert_eq!(cache_control.unwrap().to_str().unwrap(), "no-store");
    }

    #[tokio::test]
    async fn test_agent_feed_standard_list_fails_closed_with_warm_cache() {
        assert_cached_list_fails_closed(false).await;
    }

    #[tokio::test]
    async fn test_agent_feed_mobile_list_fails_closed_with_warm_cache() {
        assert_cached_list_fails_closed(true).await;
    }

    #[tokio::test]
    async fn test_agent_feed_cache_operations() {
        let cache = get_agent_feed_cache();
        let cache_key = "agent_feed:test_tenant:20:0:false";

        // Ensure it's empty initially
        cache.invalidate(cache_key).await;
        let result = cache.get(cache_key).await;
        assert!(result.is_none());

        let response = AnyAgentFeedListResponse::Standard(AgentFeedListResponse { items: vec![] });

        // Set cache with tag
        cache
            .set_with_tags(
                cache_key,
                response.clone(),
                vec!["agent_feed_tenant:test_tenant".to_string()],
                std::time::Duration::from_secs(60),
            )
            .await;

        // Verify cache hit
        let hit = cache.get(cache_key).await;
        assert!(hit.is_some());

        // Invalidate by tag
        cache
            .invalidate_by_tag("agent_feed_tenant:test_tenant")
            .await;

        let miss = cache.get(cache_key).await;
        assert!(miss.is_none());
    }

    #[tokio::test]
    async fn test_websocket_feed() {
        if crate::redis_pool::get_redis_client().is_none() {
            return;
        }

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
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
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
                let _: () = redis::cmd("PUBLISH")
                    .arg(topic)
                    .arg(payload)
                    .query_async(&mut conn)
                    .await
                    .unwrap();

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

    #[tokio::test]
    async fn test_websocket_feed_batching() {
        if crate::redis_pool::get_redis_client().is_none() {
            return;
        }

        let mock_claims = Claims {
            sub: "user-456".to_string(),
            organization_id: Some("test_batch_tenant".to_string()),
            roles: vec!["ADMIN".to_string()],
            iat: 0,
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            exp: 9999999999,
            jti: "test_jti_batch".to_string(),
            session_id: Some("test_session_id_batch".to_string()),
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

        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        if let Ok(client) = redis::Client::open(redis_url)
            && client.get_connection().is_ok()
        {
            let ws_url = format!("ws://{}/ws", addr);
            let (mut ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");

            tokio::time::sleep(std::time::Duration::from_millis(200)).await;

            let mut conn = client.get_multiplexed_async_connection().await.unwrap();
            let topic = "agent_feed:test_batch_tenant";

            // Publish 5 messages rapidly — they should be batched
            for i in 0..5 {
                let payload = format!("{{\"seq\":{}}}", i);
                let _: () = redis::cmd("PUBLISH")
                    .arg(topic)
                    .arg(payload)
                    .query_async(&mut conn)
                    .await
                    .unwrap();
            }

            let msg = tokio::time::timeout(std::time::Duration::from_secs(3), ws_stream.next())
                .await
                .expect("Timeout waiting for batch")
                .expect("Stream closed")
                .expect("Error receiving message");

            assert!(msg.is_text());
            let text = msg.to_text().unwrap();
            let parsed: serde_json::Value = serde_json::from_str(text).expect("Invalid JSON");
            assert_eq!(parsed["type"], "batch");
            let items = parsed["items"].as_array().expect("items not an array");
            assert_eq!(items.len(), 5);
            for (i, item) in items.iter().enumerate() {
                assert_eq!(*item, serde_json::json!({"seq": i}));
            }
        }
    }
}
