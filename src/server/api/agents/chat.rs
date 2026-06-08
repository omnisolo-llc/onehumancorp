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

    // Check if the message indicates a booking intent before semantic routing
    let message_lower = payload.message.to_lowercase();
    if message_lower.contains("book") || message_lower.contains("schedule") || message_lower.contains("appointment") || message_lower.contains("come look") || message_lower.contains("availability") {
        return handle_booking_intent(tenant_id, payload.message, state).await.into_response();
    }

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
        Ok(_) => (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(dept.to_string()) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
    }
}

async fn handle_booking_intent(
    tenant_id: String,
    message: String,
    state: ChatState,
) -> (StatusCode, Json<ChatResponse>) {
    let now = chrono::Utc::now();
    let tomorrow = now + chrono::Duration::days(1);
    let date_str = tomorrow.format("%Y-%m-%d").to_string();

    let product_id = "default_product"; // Extracted via LLM theoretically

    let description = format!(
        "Customer Assistant extracted booking intent for date {}. Checking availability...",
        date_str
    );

    let payload = serde_json::json!({
        "action": "check_availability",
        "date": date_str,
        "product_id": product_id,
        "original_message": message
    });

    // Check if the user is confirming a booking time, trigger checkout
    if message.to_lowercase().contains("yes") || message.to_lowercase().contains("perfect") || message.to_lowercase().contains("that works") || message.to_lowercase().contains("agree") {
        let checkout_description = format!(
            "Customer agreed to time. Creating Stripe checkout for {} deposit.",
            product_id
        );

        let checkout_payload = serde_json::json!({
            "action": "create_conversational_checkout",
            "product_id": product_id,
            "original_message": message,
            "amount_cents": 5000 // default $50 deposit
        });

        match state.orchestrator.execute_action(
            DepartmentType::Sales, // Checkout handled by sales/finance implicitly
            checkout_description,
            tenant_id.clone(),
            ActionRisk::AutoExecute,
            checkout_payload,
        ).await {
            Ok(_) => return (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(DepartmentType::Operations.to_string()) })),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None })),
        }
    }

    match state.orchestrator.execute_action(
        DepartmentType::Operations,
        description,
        tenant_id,
        ActionRisk::AutoExecute,
        payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(DepartmentType::Operations.to_string()) })),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None })),
    }
}
