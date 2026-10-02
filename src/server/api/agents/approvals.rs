use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use crate::utils::cache::HybridCache;
use ::server_common::Claims;
use axum::{
    Json, Router,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::OnceLock;

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
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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
        .route("/{id}", post(decide_approval))
        .route_layer(axum::middleware::from_fn(
            crate::api::fixture_boundary::reject_retired_fixture_paths,
        ))
        .with_state(orchestrator)
}

async fn list_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApprovalsResponse {
                    pending_approvals: vec![],
                    next_cursor: None,
                }),
            )
                .into_response();
        }
    };

    let limit = query.limit.unwrap_or(20);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let cache_key = format!(
        "approvals:{}:{}:{}:{}",
        tenant_id,
        query.cursor.as_deref().unwrap_or("none"),
        limit,
        mobile_optimized
    );
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
            let mut approvals = orchestrator_bg
                .get_pending_approvals(&tenant_id_bg, cursor_bg, limit as i64)
                .await;
            if mobile_optimized {
                for a in &mut approvals {
                    a.payload = None;
                }
            }
            let next_cursor = if approvals.len() == limit {
                approvals.last().map(|a| a.id.clone())
            } else {
                None
            };
            if let Some(c) = APPROVALS_CACHE.get() {
                c.set(
                    &cache_key_bg,
                    ApprovalsResponse {
                        pending_approvals: approvals,
                        next_cursor,
                    },
                    std::time::Duration::from_secs(10),
                )
                .await;
            }
        });

        return (StatusCode::OK, Json(cached)).into_response();
    }

    let mut approvals = orchestrator
        .get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64)
        .await;
    if mobile_optimized {
        for a in &mut approvals {
            a.payload = None;
        }
    }

    let next_cursor = if approvals.len() == limit {
        approvals.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: approvals,
        next_cursor,
    };
    cache
        .set(
            &cache_key,
            response.clone(),
            std::time::Duration::from_secs(10),
        )
        .await;

    (StatusCode::OK, Json(response)).into_response()
}

async fn list_activity_feed(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApprovalsResponse {
                    pending_approvals: vec![],
                    next_cursor: None,
                }),
            )
                .into_response();
        }
    };

    let limit = query.limit.unwrap_or(20);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let cache_key = format!(
        "activity_feed:{}:{}:{}:{}",
        tenant_id,
        query.cursor.as_deref().unwrap_or("none"),
        limit,
        mobile_optimized
    );
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
            let mut activities = orchestrator_bg
                .get_activity_feed(&tenant_id_bg, cursor_bg, limit as i64)
                .await;
            if mobile_optimized {
                for a in &mut activities {
                    a.payload = None;
                }
            }
            let next_cursor = if activities.len() == limit {
                activities.last().map(|a| a.id.clone())
            } else {
                None
            };
            if let Some(c) = APPROVALS_CACHE.get() {
                c.set(
                    &cache_key_bg,
                    ApprovalsResponse {
                        pending_approvals: activities,
                        next_cursor,
                    },
                    std::time::Duration::from_secs(10),
                )
                .await;
            }
        });

        return (StatusCode::OK, Json(cached)).into_response();
    }

    let mut activities = orchestrator
        .get_activity_feed(&tenant_id, query.cursor.clone(), limit as i64)
        .await;
    if mobile_optimized {
        for a in &mut activities {
            a.payload = None;
        }
    }

    let next_cursor = if activities.len() == limit {
        activities.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: activities,
        next_cursor,
    };
    cache
        .set(
            &cache_key,
            response.clone(),
            std::time::Duration::from_secs(10),
        )
        .await;

    (StatusCode::OK, Json(response)).into_response()
}

async fn decide_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return (
            StatusCode::FORBIDDEN,
            Json(DecisionResponse { success: false }),
        )
            .into_response();
    }
    if id.len() > 255
        || payload
            .edited_payload
            .as_ref()
            .is_some_and(|value| value.to_string().len() > 65536)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(DecisionResponse { success: false }),
        )
            .into_response();
    }
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(DecisionResponse { success: false }),
            )
                .into_response();
        }
    };

    match orchestrator
        .decide_approval(&id, &tenant_id, payload.approved, payload.edited_payload)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DecisionResponse { success: false }),
        )
            .into_response(),
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
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "entries": [] })),
            )
                .into_response();
        }
    };

    let limit = query.limit.unwrap_or(50);

    match orchestrator
        .get_ledger_entries(&tenant_id, limit as i64)
        .await
    {
        Ok(entries) => (
            StatusCode::OK,
            Json(serde_json::json!({ "entries": entries })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_approvals_cache_initialization() {
        let tenant_id = "test_tenant";
        let cache_key = format!("approvals:{}:none:20:false", tenant_id);
        let cache = APPROVALS_CACHE.get_or_init(|| HybridCache::new(None));

        let initial_val = cache.get(&cache_key).await;
        assert!(initial_val.is_none(), "Cache should be empty initially");

        let dummy_resp = ApprovalsResponse {
            pending_approvals: vec![],
            next_cursor: None,
        };

        cache
            .set(
                &cache_key,
                dummy_resp.clone(),
                std::time::Duration::from_secs(60),
            )
            .await;

        let cached_val = cache.get(&cache_key).await;
        assert!(cached_val.is_some(), "Cache should hit after set");
    }
}
