use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use crate::orchestration::router::{SemanticRouter, SemanticRoutingRequest};
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub success: bool,
    pub department_assigned: Option<String>,
    pub draft_action: Option<crate::orchestration::departments::types::ApprovalRequest>,
}

#[derive(Clone)]
pub struct ChatState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub semantic_router: Arc<SemanticRouter>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>, semantic_router: Arc<SemanticRouter>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ChatState {
        orchestrator,
        semantic_router,
    };
    Router::new()
        .route("/", post(handle_chat))
        .with_state(state)
}

async fn handle_chat(
    State(state): State<ChatState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ChatResponse { success: false, department_assigned: None, draft_action: None })).into_response(),
    };

    let req = SemanticRoutingRequest {
        tenant_id: tenant_id.clone(),
        prompt: payload.message.clone(),
        embedding: None,
    };

    let dept = match state.semantic_router.route(&req) {
        Ok(res) => res.target_department,
        Err(_) => DepartmentType::Operations, // Fallback
    };

    let description = format!("Task routed via semantic gateway to {:?}", dept);
    let payload_json = serde_json::json!({ "original_request": payload.message, "action": "semantic_routed_task" });

    match state.orchestrator.execute_action(
        dept.clone(),
        description,
        tenant_id,
        ActionRisk::DraftForReview,
        payload_json,
    ).await {
        Ok(req) => (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(dept.to_string()), draft_action: Some(req) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None, draft_action: None })).into_response(),
    }
}

