use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{ActionRisk, ApprovalRequest, DepartmentType};
use crate::orchestration::router::{SemanticRouter, SemanticRoutingRequest};
use ::server_common::Claims;
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub success: bool,
    pub department_assigned: Option<String>,
    pub approval: Option<ApprovalRequest>,
}

#[derive(Clone)]
pub struct ChatState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub semantic_router: Arc<SemanticRouter>,
}

pub fn router<S>(
    orchestrator: Arc<DepartmentOrchestrator>,
    semantic_router: Arc<SemanticRouter>,
) -> Router<S>
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
    headers: HeaderMap,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) if !org_id.trim().is_empty() => org_id.to_string(),
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ChatResponse {
                    success: false,
                    department_assigned: None,
                    approval: None,
                }),
            )
                .into_response();
        }
    };

    let expected_user = headers
        .get_all("x-ohc-expected-user")
        .iter()
        .collect::<Vec<_>>();
    let expected_tenant = headers
        .get_all("x-ohc-expected-tenant")
        .iter()
        .collect::<Vec<_>>();
    if (!expected_user.is_empty() || !expected_tenant.is_empty())
        && (expected_user.len() != 1
            || expected_tenant.len() != 1
            || expected_user[0].to_str().ok() != Some(claims.sub.as_str())
            || expected_tenant[0].to_str().ok() != Some(tenant_id.as_str()))
    {
        return (
            StatusCode::CONFLICT,
            Json(ChatResponse {
                success: false,
                department_assigned: None,
                approval: None,
            }),
        )
            .into_response();
    }
    if payload.message.trim().is_empty() || payload.message.chars().count() > 16000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ChatResponse {
                success: false,
                department_assigned: None,
                approval: None,
            }),
        )
            .into_response();
    }

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

    match state
        .orchestrator
        .execute_action(
            dept,
            description,
            tenant_id,
            ActionRisk::DraftForReview,
            payload_json,
        )
        .await
    {
        Ok(approval) => (
            StatusCode::OK,
            Json(ChatResponse {
                success: true,
                department_assigned: Some(dept.to_string()),
                approval: Some(approval),
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ChatResponse {
                success: false,
                department_assigned: None,
                approval: None,
            }),
        )
            .into_response(),
    }
}

#[cfg(test)]
#[path = "chat_test.rs"]
mod tests;
