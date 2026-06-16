use axum::{
    extract::{Extension, Path, Query, State, ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use ::server_common::Claims;
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
use crate::domain::action::{
    router::ActionRouter,
    sre_handler::SreHandler,
    marketing_handler::MarketingHandler,
    inbox_handler::InboxHandler,
    sales_handler::SalesHandler,
    ActionIntent,
};
use sqlx::PgPool;
use crate::utils::cache::HybridCache;
use std::sync::{Arc, OnceLock};
use futures::{sink::SinkExt, stream::StreamExt};
use redis::AsyncCommands;

pub static AGENT_FEED_CACHE: OnceLock<Arc<HybridCache<AgentFeedListResponse>>> = OnceLock::new();
pub static SHARED_REDIS_CLIENT: OnceLock<redis::Client> = OnceLock::new();

pub fn get_redis_client() -> redis::Client {
    SHARED_REDIS_CLIENT.get_or_init(|| {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        redis::Client::open(redis_url).expect("Failed to initialize Redis client")
    }).clone()
}

pub fn get_agent_feed_cache() -> Arc<HybridCache<AgentFeedListResponse>> {
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
                    tracing::error!("Failed to get pubsub payload: {}", e);
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
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(AgentFeedListResponse { items: vec![] })).into_response(),
    };

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let cache_key = format!("agent_feed:{}:{}:{}:{}", tenant_id, limit, offset, mobile_optimized);
    let cache = get_agent_feed_cache();

    if let Some((cached_resp, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (StatusCode::OK, Json(cached_resp)).into_response();
        }

        let pool_bg = pool.clone();
        let tenant_id_bg = tenant_id.clone();
        let cache_bg = cache.clone();
        let cache_key_bg = cache_key.clone();

        tokio::spawn(async move {
            let repo = AgentFeedRepository::new(pool_bg);
            if let Ok(items) = repo.list(&tenant_id_bg, limit, offset, mobile_optimized).await {
                let response = AgentFeedListResponse { items };
                let tag = format!("agent_feed_tenant:{}", tenant_id_bg);
                cache_bg.set_with_tags(&cache_key_bg, response, vec![tag], std::time::Duration::from_secs(60)).await;
            }
        });

        return (StatusCode::OK, Json(cached_resp)).into_response();
    }

    let repo = AgentFeedRepository::new(pool);

    match repo.list(&tenant_id, limit, offset, mobile_optimized).await {
        Ok(items) => {
            let response = AgentFeedListResponse { items };
            let tag = format!("agent_feed_tenant:{}", tenant_id);
            cache.set_with_tags(&cache_key, response.clone(), vec![tag], std::time::Duration::from_secs(60)).await;
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to list agent feed items: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(AgentFeedListResponse { items: vec![] })).into_response()
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

    let repo = AgentFeedRepository::new(pool);

    let item = AgentFeedItem {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_source: payload.event_source,
        context_payload: payload.context_payload.map(sqlx::types::Json),
        proposed_action: payload.proposed_action.map(sqlx::types::Json),
        lifecycle_state: "PENDING_APPROVAL".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    match repo.create(item.clone()).await {
        Ok(_) => {
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

    let repo = AgentFeedRepository::new(pool.clone());

    match repo.update_state(&tenant_id, &id, &payload.state).await {
        Ok(updated_item) => {
            // Trigger legacy execution by synchronizing the agent_approvals table
            if payload.state == "APPROVED" || payload.state == "REJECTED" || payload.state == "DISMISSED" {
                let legacy_status = if payload.state == "APPROVED" { "APPROVED" } else { "REJECTED" };
                let _ = sqlx::query("UPDATE agent_approvals SET status = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(legacy_status)
                    .bind(&id)
                    .bind(&tenant_id)
                    .execute(&pool)
                    .await;
            }

            // Handle execution dynamically using Operations Manager protocol
            if payload.state == "APPROVED" {
                if let Ok(Some(item)) = repo.get(&tenant_id, &id).await {
                    let mut payload = item.proposed_action.clone().or_else(|| item.context_payload.clone()).unwrap_or_else(|| sqlx::types::Json(serde_json::json!({})));

                    let mut feature_type = payload.get("feature_type").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    if feature_type.is_empty() && item.event_source == "incident_resolution" {
                        feature_type = "incident_resolution".to_string();
                    }

                    if !feature_type.is_empty() {
                        let intent = ActionIntent {
                            feature_type: feature_type.clone(),
                            action: payload.get("action").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            resource_type: payload.get("resource_type").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            resource_id: payload.get("resource_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            payload: payload.0,
                        };

                        let router = ActionRouter::new()
                            .register("incident_resolution", Box::new(SreHandler))
                            .register("social_post_draft", Box::new(MarketingHandler))
                            .register("ambassador_reply", Box::new(InboxHandler))
                            .register("instagram_dm", Box::new(InboxHandler))
                            .register("quote_draft", Box::new(SalesHandler));

                        if let Err(e) = router.dispatch(&pool, &tenant_id, &intent).await {
                            tracing::error!("Failed to execute action intent for feed item {}: {}", id, e);
                        }
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
    use super::{get_agent_feed_cache, AgentFeedListResponse, ws_feed_handler};
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
        let cache_key = "agent_feed:test_tenant:20:0";

        // Ensure it's empty initially
        cache.invalidate(cache_key).await;
        let result = cache.get(cache_key).await;
        assert!(result.is_none());

        let response = AgentFeedListResponse {
            items: vec![],
        };

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
