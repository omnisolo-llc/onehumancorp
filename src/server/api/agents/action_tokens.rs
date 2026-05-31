use axum::{
    extract::{State, Path, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Deserialize)]
pub struct DecisionRequest {
    pub approved: bool,
}

#[derive(Serialize)]
pub struct DecisionResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct TokenInfoResponse {
    pub id: String,
    pub tenant_id: String,
    pub approval_request_id: String,
    pub status: String,
    pub description: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/:token", get(get_token_info).post(decide_via_token))
        .with_state(orchestrator)
}

async fn get_token_info(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    match orchestrator.get_action_token(&token).await {
        Ok(t) => {
            if t.status != "PENDING" {
                return (StatusCode::BAD_REQUEST, axum::Json(TokenInfoResponse {
                    id: t.id,
                    tenant_id: t.tenant_id,
                    approval_request_id: t.approval_request_id,
                    status: t.status,
                    description: None,
                })).into_response();
            }

            // We could optionally fetch the approval request to get the description
            // For simplicity, we just return the token info
            (StatusCode::OK, axum::Json(TokenInfoResponse {
                id: t.id,
                tenant_id: t.tenant_id,
                approval_request_id: t.approval_request_id,
                status: t.status,
                description: None, // Can be extended to return the description
            })).into_response()
        },
        Err(_) => (StatusCode::NOT_FOUND, axum::Json(DecisionResponse { success: false })).into_response()
    }
}

async fn decide_via_token(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(token): Path<String>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let action_token = match orchestrator.get_action_token(&token).await {
        Ok(t) => t,
        Err(_) => return (StatusCode::NOT_FOUND, axum::Json(DecisionResponse { success: false })).into_response(),
    };

    if action_token.status != "PENDING" {
        return (StatusCode::BAD_REQUEST, axum::Json(DecisionResponse { success: false })).into_response();
    }

    if action_token.expires_at < chrono::Utc::now() {
        return (StatusCode::BAD_REQUEST, axum::Json(DecisionResponse { success: false })).into_response();
    }

    match orchestrator.decide_approval(&action_token.approval_request_id, &action_token.tenant_id, payload.approved).await {
        Ok(_) => {
            let _ = orchestrator.consume_action_token(&token).await;
            (StatusCode::OK, axum::Json(DecisionResponse { success: true })).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(DecisionResponse { success: false })).into_response(),
    }
}
