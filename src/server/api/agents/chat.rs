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
        None => return (StatusCode::UNAUTHORIZED, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
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

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.message.received".to_string(),
        payload: serde_json::json!({ "original_message": payload.message, "source": "chat" }),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(dept.to_string()) })).into_response(),
        Err(e) => {
            tracing::error!("Failed to dispatch event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None })).into_response()
        }
    }
}

