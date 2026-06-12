use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message as WsMessage};
use futures_util::StreamExt;
use tokio::sync::broadcast;
use axum::{
    extract::{Extension, Path, Query, State},
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
use sqlx::PgPool;
use crate::utils::cache::HybridCache;
use std::sync::{Arc, OnceLock};


pub static FEED_BROADCAST: OnceLock<broadcast::Sender<AgentFeedItem>> = OnceLock::new();

pub fn get_feed_broadcast() -> broadcast::Sender<AgentFeedItem> {
    FEED_BROADCAST.get_or_init(|| {
        let (tx, _) = broadcast::channel(100);
        tx
    }).clone()
}

pub static AGENT_FEED_CACHE: OnceLock<Arc<HybridCache<AgentFeedListResponse>>> = OnceLock::new();

pub fn get_agent_feed_cache() -> Arc<HybridCache<AgentFeedListResponse>> {
    AGENT_FEED_CACHE.get_or_init(|| {
        let redis_client = if let Ok(url) = std::env::var("REDIS_URL") {
            match redis::Client::open(url.clone()) {
                Ok(client) => Some(client),
                Err(e) => {
                    tracing::warn!("Failed to initialize Redis client for AGENT_FEED_CACHE: {}. Falling back to in-memory cache.", e);
                    None
                }
            }
        } else {
            None
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
        .route("/ws", get(ws_handler))
}


#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    let auth_store = crate::auth::Store::new();
    let claims = match auth_store.validate_token(&query.token).await {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let tenant_id = match claims.organization_id {
        Some(org_id) => org_id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: String) {
    let mut rx = get_feed_broadcast().subscribe();

    // Check if we can use Redis
    let redis_client = crate::get_redis_client();
    if let Some(client) = redis_client {
        if let Ok(mut pubsub) = client.get_async_pubsub().await {
            let topic = format!("ohc:feed:{}", tenant_id);
            if pubsub.subscribe(&topic).await.is_ok() {
                let mut stream = pubsub.on_message();
                loop {
                    tokio::select! {
                        msg = stream.next() => {
                            if let Some(msg) = msg {
                                if let Ok(payload) = msg.get_payload::<String>() {
                                    if socket.send(WsMessage::Text(payload.into())).await.is_err() {
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        // Also handle incoming to keep connection alive
                        client_msg = socket.recv() => {
                            if client_msg.is_none() {
                                break;
                            }
                        }
                    }
                }
                return;
            }
        }
    }

    // Fallback to memory broadcast
    loop {
        tokio::select! {
            Ok(item) = rx.recv() => {
                if item.tenant_id == tenant_id {
                    if let Ok(payload) = serde_json::to_string(&item) {
                        if socket.send(WsMessage::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
            client_msg = socket.recv() => {
                if client_msg.is_none() {
                    break;
                }
            }
        }
    }
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
            if let Ok(mut items) = repo.list(&tenant_id_bg, limit, offset).await {
                if mobile_optimized {
                    for item in items.iter_mut() {
                        item.context_payload = None;
                        item.proposed_action = None;
                    }
                }
                let response = AgentFeedListResponse { items };
                let tag = format!("agent_feed_tenant:{}", tenant_id_bg);
                cache_bg.set_with_tags(&cache_key_bg, response, vec![tag], std::time::Duration::from_secs(60)).await;
            }
        });

        return (StatusCode::OK, Json(cached_resp)).into_response();
    }

    let repo = AgentFeedRepository::new(pool);

    match repo.list(&tenant_id, limit, offset).await {
        Ok(mut items) => {
            if mobile_optimized {
                for item in items.iter_mut() {
                    item.context_payload = None;
                    item.proposed_action = None;
                }
            }
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

            // Publish to websocket
            let _ = get_feed_broadcast().send(item.clone());

            if let Some(client) = crate::get_redis_client() {
                if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                    let topic = format!("ohc:feed:{}", tenant_id);
                    if let Ok(payload) = serde_json::to_string(&item) {
                        let _ = redis::cmd("PUBLISH")
                            .arg(&topic)
                            .arg(&payload)
                            .query_async::<()>(&mut conn)
                            .await;
                    }
                }
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

            // Handle incident resolution execution
            if payload.state == "APPROVED" {
                if let Ok(Some(item)) = repo.get(&tenant_id, &id).await {
                    if item.event_source == "incident_resolution" {
                        if let Some(payload) = item.context_payload {
                            if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
                                let _ = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                                    .bind(incident_id)
                                    .bind(&tenant_id)
                                    .execute(&pool)
                                    .await;
                            }
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
    use super::{get_agent_feed_cache, AgentFeedListResponse};

    #[tokio::test]
    async fn test_agent_feed_websocket_push() {
        use super::{get_feed_broadcast, AgentFeedItem};
        let broadcast = get_feed_broadcast();
        let mut rx = broadcast.subscribe();

        let item = AgentFeedItem {
            id: "test-id-123".to_string(),
            tenant_id: "test-tenant-123".to_string(),
            event_source: "test".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: None,
            updated_at: None,
        };

        // Send an item to the broadcast
        let _ = broadcast.send(item.clone());

        // Wait for it on the receiver
        if let Ok(received_item) = rx.recv().await {
            assert_eq!(received_item.id, "test-id-123");
            assert_eq!(received_item.tenant_id, "test-tenant-123");
        } else {
            panic!("Failed to receive broadcasted item");
        }
    }

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

        // Verify cache miss after invalidation
        // NOTE: there might be a short delay needed for tags to be invalidated in HybridCache depending on implementation,
        // but HybridCache tag invalidation is usually synchronous for local cache.
        let miss = cache.get(cache_key).await;
        assert!(miss.is_none());
    }
}
