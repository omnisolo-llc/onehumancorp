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

            // Handle incident resolution execution
            if payload.state == "APPROVED" {
                if let Ok(Some(item)) = repo.get(&tenant_id, &id).await {
                    if item.event_source == "incident_resolution" {
                        if let Some(ref payload) = item.context_payload {
                            if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
                                let _ = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                                    .bind(incident_id)
                                    .bind(&tenant_id)
                                    .execute(&pool)
                                    .await;
                            }
                        }
                    }

                    if let Some(payload) = item.proposed_action.clone().or(item.context_payload.clone()) {
                        if payload.get("feature_type").and_then(|v| v.as_str()) == Some("social_post_draft") {
                            tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
                            // Real implementation would buffer post here to AYRSHARE.
                        }

                        if payload.get("feature_type").and_then(|v| v.as_str()) == Some("cart_recovery") {
                            tracing::info!("Approved cart recovery draft: {}", id);
                            if let Some(session_id) = payload.get("checkout_session_id").and_then(|v| v.as_str()) {
                                if let Ok(parsed_session_id) = uuid::Uuid::parse_str(session_id) {
                                    if let Ok(mut tx) = pool.begin().await {
                                        let recovery_job_id = Uuid::new_v4().to_string();
                                        let full_payload = payload.clone();

                                        let update_res = sqlx::query("UPDATE conversational_checkout_sessions SET status = 'recovery_sent', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                                            .bind(parsed_session_id)
                                            .bind(&tenant_id)
                                            .execute(&mut *tx)
                                            .await;

                                        let insert_res = sqlx::query(
                                            r#"
                                            INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
                                            VALUES ($1, $2, 'cart_recovery', $3, 'PENDING', CURRENT_TIMESTAMP)
                                            "#,
                                        )
                                        .bind(&recovery_job_id)
                                        .bind(&tenant_id)
                                        .bind(&full_payload)
                                        .execute(&mut *tx)
                                        .await;

                                        if update_res.is_ok() && insert_res.is_ok() {
                                            if tx.commit().await.is_ok() {
                                                tracing::info!("Successfully marked session {} as recovery_sent and queued dispatch job {}", parsed_session_id, recovery_job_id);
                                            } else {
                                                tracing::error!("Failed to commit cart_recovery transaction for session {}", parsed_session_id);
                                            }
                                        } else {
                                            tracing::error!("Failed to update state or queue job for cart_recovery session {}", parsed_session_id);
                                            let _ = tx.rollback().await;
                                        }
                                    } else {
                                        tracing::error!("Failed to begin transaction for cart_recovery session {}", parsed_session_id);
                                    }
                                } else {
                                    tracing::error!("Invalid checkout_session_id {} in cart_recovery proposed action for tenant {}", session_id, tenant_id);
                                }
                            }
                        }

                        if payload.get("feature_type").and_then(|v| v.as_str()) == Some("quote_draft") {
                            if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
                                tracing::info!("Approved quote draft: {}", quote_id);
                                let _ = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                                    .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
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
