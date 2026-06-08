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
use std::str::FromStr;

#[derive(Deserialize)]
pub struct DispatchActionRequest {
    pub action_type: String, // e.g. "DraftCustomerMessage", "UpdateInventory"
    pub action_description: String,
    pub payload: serde_json::Value,
    pub risk_level: String, // "LOW" or "HIGH"
}

#[derive(Serialize)]
pub struct DispatchActionResponse {
    pub success: bool,
    pub approval_request_id: Option<String>,
    pub error_message: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/dispatch", post(dispatch_action_endpoint))
        .with_state(orchestrator)
}

pub async fn dispatch_action_endpoint(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DispatchActionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DispatchActionResponse {
            success: false,
            approval_request_id: None,
            error_message: Some("Unauthorized".to_string())
        })).into_response(),
    };

    let risk = ActionRisk::from_str(&payload.risk_level).unwrap_or(ActionRisk::DraftForReview);

    // Determine target department based on action type (rudimentary routing)
    let target_dept = match payload.action_type.as_str() {
        "DraftCustomerMessage" => DepartmentType::CustomerSuccess,
        "UpdateInventory" => DepartmentType::Operations,
        _ => DepartmentType::Operations, // Fallback
    };

    let full_payload = serde_json::json!({
        "action_type": payload.action_type,
        "payload": payload.payload
    });

    match orchestrator.execute_action(
        target_dept,
        payload.action_description,
        tenant_id,
        risk.clone(),
        full_payload,
    ).await {
        Ok(approval_req) => {
            (StatusCode::OK, Json(DispatchActionResponse {
                success: true,
                approval_request_id: Some(approval_req.id),
                error_message: None
            })).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to dispatch action: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DispatchActionResponse {
                success: false,
                approval_request_id: None,
                error_message: Some(e)
            })).into_response()
        }
    }
}
