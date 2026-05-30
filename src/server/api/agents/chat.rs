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
use crate::orchestration::departments::types::DepartmentType;
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
    pub queue: Arc<dyn crate::queue::TaskQueue>,
}

pub fn router<S>(state: ChatState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_chat))
        .with_state(state)
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
    State(state): State<ChatState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
    };

    let (dept, description, payload_json) = determine_routing(&payload.message);

    let job_payload = serde_json::json!({
        "type": "chat_action",
        "department": dept.to_string(),
        "description": description,
        "payload": payload_json
    });

    let job = crate::queue::Job {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        parent_task_id: "".to_string(),
        agent_role: "chat_agent".to_string(),
        payload: serde_json::to_string(&job_payload).unwrap_or_default(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let _ = state.queue.enqueue(job).await;

    (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(dept.to_string()) })).into_response()
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
