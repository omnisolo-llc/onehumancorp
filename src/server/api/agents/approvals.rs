use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;
use ::server_utils::cache::HybridCache;

static APPROVALS_CACHE: OnceLock<HybridCache<Vec<ApprovalRequest>>> = OnceLock::new();
static ACTIVITY_CACHE: OnceLock<HybridCache<Vec<ApprovalRequest>>> = OnceLock::new();

#[derive(Serialize)]
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
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
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

    let cache_key = format!("approvals:{}:{:?}:{}", tenant_id, query.cursor, limit);
    let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_approvals) = cache.get(&cache_key).await {
        let next_cursor = if cached_approvals.len() == limit {
            cached_approvals.last().map(|a| a.id.clone())
        } else {
            None
        };
        return (StatusCode::OK, Json(ApprovalsResponse {
            pending_approvals: cached_approvals,
            next_cursor,
        })).into_response();
    }

    // Fetch from DB using cursor pagination
    let approvals = orchestrator.get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64).await;

    cache.set(&cache_key, approvals.clone(), std::time::Duration::from_secs(10)).await;

    let next_cursor = if approvals.len() == limit {
        approvals.last().map(|a| a.id.clone())
    } else {
        None
    };

    (StatusCode::OK, Json(ApprovalsResponse {
        pending_approvals: approvals,
        next_cursor,
    })).into_response()
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

    let cache_key = format!("activity:{}:{:?}:{}", tenant_id, query.cursor, limit);
    let cache = ACTIVITY_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_activities) = cache.get(&cache_key).await {
        let next_cursor = if cached_activities.len() == limit {
            cached_activities.last().map(|a| a.id.clone())
        } else {
            None
        };
        return (StatusCode::OK, Json(ApprovalsResponse {
            pending_approvals: cached_activities,
            next_cursor,
        })).into_response();
    }

    let activities = orchestrator.get_activity_feed(&tenant_id, query.cursor.clone(), limit as i64).await;

    cache.set(&cache_key, activities.clone(), std::time::Duration::from_secs(10)).await;

    let next_cursor = if activities.len() == limit {
        activities.last().map(|a| a.id.clone())
    } else {
        None
    };

    (StatusCode::OK, Json(ApprovalsResponse {
        pending_approvals: activities,
        next_cursor,
    })).into_response()
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

    match orchestrator.decide_approval(&id, &tenant_id, payload.approved).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}
// Support for AI Agent Department Architecture
