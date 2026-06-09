use axum::{
    extract::{Extension, State, Path, Query},
    response::{IntoResponse, sse::{Event, Sse}},
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::convert::Infallible;
use futures_util::stream::{self, Stream};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;


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
        .route("/ledger", get(list_ledger_entries))
        .route("/stream", get(stream_approvals))
        .route("/simulate-smart-pricing", post(simulate_smart_pricing))
        .route("/simulate-quote-draft", post(simulate_quote_draft))
        .route("/simulate-stockout-reorder", post(simulate_stockout_reorder))
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
}

async fn stream_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized".into_response()),
    };

    let mesh = orchestrator.mesh();
    let topic = format!("tenant:{}:agent_approvals", tenant_id);
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    let mut subscriber = mesh.subscribe(&topic).await.unwrap();

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                msg_opt = subscriber.next() => {
                    if let Some(msg) = msg_opt {
                        if let Ok(payload) = msg.get_payload() {
                            let event = Event::default().data(payload);
                            if tx_clone.send(Ok::<_, Infallible>(event)).await.is_err() {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                _ = tx_clone.closed() => {
                    tracing::debug!("Client disconnected, cleaning up SSE stream for tenant {}", tenant_id);
                    let _ = subscriber.unsub().await;
                    break;
                }
            }
        }
    });

    Sse::new(ReceiverStream::new(rx))
        .keep_alive(axum::response::sse::KeepAlive::new())
        .into_response()
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

    // Fetch from DB using cursor pagination
    let approvals = orchestrator.get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64).await;

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

    let activities = orchestrator.get_activity_feed(&tenant_id, query.cursor.clone(), limit as i64).await;

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
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt; // for `oneshot`
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

    #[tokio::test]
    async fn test_stream_approvals_unauthorized() {
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/ohc").await.unwrap();
        let mesh = Arc::new(crate::orchestration::mesh::TeammateMesh::new("redis://localhost").await.unwrap());
        let orchestrator = Arc::new(DepartmentOrchestrator::new(pool, mesh));

        let app = router(orchestrator);

        let req = Request::builder()
            .uri("/stream")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
