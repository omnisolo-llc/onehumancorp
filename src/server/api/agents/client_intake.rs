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
pub struct ClientIntakeRequest {
    pub inquiry: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientIntakeResponse {
    pub success: bool,
    pub proposal_drafted: bool,
}

#[derive(Clone)]
pub struct ClientIntakeState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ClientIntakeState {
        orchestrator,
    };
    Router::new()
        .route("/", post(handle_client_intake))
        .with_state(state)
}

async fn handle_client_intake(
    State(state): State<ClientIntakeState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response(),
    };

    // Analyze the unstructured inquiry and extract parameters (mock logic for AI generation)
    // Create a drafted proposal
    let suggested_price = 1500.00;
    let service_name = "Custom Project Scope";

    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${}, including standard services.",
        payload.inquiry, suggested_price
    );

    let action_payload = serde_json::json!({
        "feature_type": "quote_draft",
        "customer_inquiry": payload.inquiry,
        "suggested_price": suggested_price,
        "scope": format!("{} with custom requirements.", service_name),
        "suggested_time": "Next Week",
        "generated_response": drafted_message,
        "service": service_name,
        "price": suggested_price,
    });

    match state.orchestrator.execute_action(
        DepartmentType::Sales,
        format!("Draft proposal for new intake: {}", service_name),
        tenant_id,
        ActionRisk::DraftForReview,
        action_payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ClientIntakeResponse { success: true, proposal_drafted: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response(),
    }
}
