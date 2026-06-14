use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::utils::cache::HybridCache;
use std::sync::OnceLock;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;

pub static APPROVALS_CACHE: OnceLock<HybridCache<ApprovalsResponse>> = OnceLock::new();


#[derive(Serialize, Deserialize, Clone)]
pub struct ApprovalsResponse {
    pub pending_approvals: Vec<ApprovalRequest>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct DecisionRequest {
    pub approved: bool,
    pub edited_payload: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct DecisionResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/activity", get(list_activity_feed))
        .route("/ledger", get(list_ledger_entries))
        .route("/simulate-smart-pricing", post(simulate_smart_pricing))
        .route("/simulate-quote-draft", post(simulate_quote_draft))
        .route("/simulate-stockout-reorder", post(simulate_stockout_reorder))
        .route("/stream", get(stream_agent_feed))
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
}

use axum::response::sse::{Event, Sse};
use tokio_stream::StreamExt;
use std::convert::Infallible;
use futures_util::stream::Stream;

async fn stream_agent_feed(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let redis_client_opt = crate::get_redis_client();
    let channel = format!("agent_feed:{}", tenant_id);

    let stream = tokio_stream::wrappers::BroadcastStream::new({
        let (tx, _rx) = tokio::sync::broadcast::channel::<String>(100);
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            if let Some(redis_client) = redis_client_opt {
                if let Ok(mut pubsub) = redis_client.get_async_pubsub().await {
                    if pubsub.subscribe(&channel).await.is_ok() {
                        let mut msg_stream = pubsub.into_on_message();
                        while let Some(msg) = tokio_stream::StreamExt::next(&mut msg_stream).await {
                            if let Ok(payload) = msg.get_payload::<String>() {
                                if tx_clone.send(payload).is_err() {
                                    // Receiver disconnected, stop the loop to prevent resource leak
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        });

        tx.subscribe()
    })
    .map(|msg| match msg {
        Ok(payload) => Ok(Event::default().event("message").data(payload)),
        Err(_) => Ok(Event::default().event("ping").data("connected")),
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

async fn simulate_stockout_reorder(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.simulate_stockout_restock_and_price(&tenant_id).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate stockout reorder: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_quote_draft(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    let payload = serde_json::json!({
        "feature_type": "quote_draft",
        "service": "Plumbing Fix",
        "customer_inquiry": "I need a quote for Plumbing Fix",
        "suggested_price": 250.0,
        "scope": "Plumbing Fix including labor and standard materials.",
        "suggested_time": "Tomorrow at 2 PM",
    });

    match orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::Sales,
        "Draft quote for Plumbing Fix".to_string(),
        tenant_id,
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate quote draft: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}

async fn simulate_smart_pricing(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.simulate_smart_pricing(&tenant_id).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to simulate smart pricing: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}


async fn list_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![], next_cursor: None })).into_response(),
    };

    let limit = query.limit.unwrap_or(20);
    let cache_key = format!("approvals:{}:{}:{}", tenant_id, query.cursor.as_deref().unwrap_or("none"), limit);
    let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (StatusCode::OK, Json(cached)).into_response();
        }

        let tenant_id_bg = tenant_id.clone();
        let cursor_bg = query.cursor.clone();
        let orchestrator_bg = orchestrator.clone();
        let cache_key_bg = cache_key.clone();
        tokio::spawn(async move {
            let approvals = orchestrator_bg.get_pending_approvals(&tenant_id_bg, cursor_bg, limit as i64).await;
            let next_cursor = if approvals.len() == limit {
                approvals.last().map(|a| a.id.clone())
            } else {
                None
            };
            if let Some(c) = APPROVALS_CACHE.get() {
                c.set(&cache_key_bg, ApprovalsResponse { pending_approvals: approvals, next_cursor }, std::time::Duration::from_secs(10)).await;
            }
        });

        return (StatusCode::OK, Json(cached)).into_response();
    }

    let approvals = orchestrator.get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64).await;

    let next_cursor = if approvals.len() == limit {
        approvals.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: approvals,
        next_cursor,
    };
    cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(10)).await;

    (StatusCode::OK, Json(response)).into_response()
}


async fn list_activity_feed(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![], next_cursor: None })).into_response(),
    };

    let limit = query.limit.unwrap_or(20);
    let cache_key = format!("activity_feed:{}:{}:{}", tenant_id, query.cursor.as_deref().unwrap_or("none"), limit);
    let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (StatusCode::OK, Json(cached)).into_response();
        }

        let tenant_id_bg = tenant_id.clone();
        let cursor_bg = query.cursor.clone();
        let orchestrator_bg = orchestrator.clone();
        let cache_key_bg = cache_key.clone();
        tokio::spawn(async move {
            let activities = orchestrator_bg.get_activity_feed(&tenant_id_bg, cursor_bg, limit as i64).await;
            let next_cursor = if activities.len() == limit {
                activities.last().map(|a| a.id.clone())
            } else {
                None
            };
            if let Some(c) = APPROVALS_CACHE.get() {
                c.set(&cache_key_bg, ApprovalsResponse { pending_approvals: activities, next_cursor }, std::time::Duration::from_secs(10)).await;
            }
        });

        return (StatusCode::OK, Json(cached)).into_response();
    }

    let activities = orchestrator.get_activity_feed(&tenant_id, query.cursor.clone(), limit as i64).await;

    let next_cursor = if activities.len() == limit {
        activities.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: activities,
        next_cursor,
    };
    cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(10)).await;

    (StatusCode::OK, Json(response)).into_response()
}

async fn decide_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.decide_approval(&id, &tenant_id, payload.approved, payload.edited_payload).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}
// Support for AI Agent Department Architecture


async fn list_ledger_entries(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "entries": [] }))).into_response(),
    };

    let limit = query.limit.unwrap_or(50);

    match orchestrator.get_ledger_entries(&tenant_id, limit as i64).await {
        Ok(entries) => (StatusCode::OK, Json(serde_json::json!({ "entries": entries }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_approvals_cache_initialization() {
        let tenant_id = "test_tenant";
        let cache_key = format!("approvals:{}:none:20", tenant_id);
        let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(None));

        let initial_val = cache.get(&cache_key).await;
        assert!(initial_val.is_none(), "Cache should be empty initially");

        let dummy_resp = ApprovalsResponse {
            pending_approvals: vec![],
            next_cursor: None,
        };

        cache.set(&cache_key, dummy_resp.clone(), std::time::Duration::from_secs(60)).await;

        let cached_val = cache.get(&cache_key).await;
        assert!(cached_val.is_some(), "Cache should hit after set");
    }
}
