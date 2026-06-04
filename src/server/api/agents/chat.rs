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

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_chat))
        .with_state(orchestrator)
}

pub fn determine_routing(msg: &str) -> (DepartmentType, String, serde_json::Value) {
    let lower_msg = msg.to_lowercase();
    if lower_msg.contains("refund") {
        (
            DepartmentType::Operations,
            "Process refund request from team chat".to_string(),
            serde_json::json!({ "original_request": msg, "action": "refund" })
        )
    } else if lower_msg.contains("post") || lower_msg.contains("newsletter") || lower_msg.contains("campaign") || lower_msg.contains("promote") {
        (
            DepartmentType::Marketing,
            "Draft marketing content from team chat".to_string(),
            serde_json::json!({ "original_request": msg, "action": "create_content" })
        )
    } else if lower_msg.contains("quote") || lower_msg.contains("lead") || lower_msg.contains("discount") || lower_msg.contains("pricing") {
        (
            DepartmentType::Sales,
            "Draft sales response/quote from team chat".to_string(),
            serde_json::json!({ "original_request": msg, "action": "draft_quote" })
        )
    } else {
        (
            DepartmentType::Operations, // Fallback
            "General task assignment from team chat".to_string(),
            serde_json::json!({ "original_request": msg, "action": "general_task" })
        )
    }
}

async fn handle_chat(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
    };

    let (dept, description, payload_json) = determine_routing(&payload.message);

    match orchestrator.execute_action(
        dept.clone(),
        description,
        tenant_id,
        ActionRisk::DraftForReview,
        payload_json,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(dept.to_string()) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_routing_refund() {
        let (dept, _, _) = determine_routing("Please refund order 123");
        assert_eq!(dept, DepartmentType::Operations);
    }

    #[test]
    fn test_chat_routing_marketing() {
        let (dept, _, _) = determine_routing("Draft a new newsletter for mothers day");
        assert_eq!(dept, DepartmentType::Marketing);
    }

    #[test]
    fn test_chat_routing_sales() {
        let (dept, _, _) = determine_routing("Give me a quote for roofing");
        assert_eq!(dept, DepartmentType::Sales);
    }

    #[test]
    fn test_chat_routing_fallback() {
        let (dept, _, _) = determine_routing("I need help with general stuff");
        assert_eq!(dept, DepartmentType::Operations);
    }
}
