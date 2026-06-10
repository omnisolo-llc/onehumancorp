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

    let cache_key = format!("agent_feed:{}:{}:{}", tenant_id, limit, offset);
    let cache = get_agent_feed_cache();

    if let Some(cached_resp) = cache.get(&cache_key).await {
        return (StatusCode::OK, Json(cached_resp)).into_response();
    }

    let repo = AgentFeedRepository::new(pool);

    match repo.list(&tenant_id, limit, offset).await {
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
    use axum::{


    };
    use crate::api::agent_feed;
    use sqlx::PgPool;
    use super::{get_agent_feed_cache, AgentFeedListResponse};
    use crate::domain::repository::agent_feed_repo::AgentFeedItem;

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
