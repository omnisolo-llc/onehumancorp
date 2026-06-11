use axum::{
    extract::{State, Extension},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use server_common::Claims;

#[derive(Deserialize)]
pub struct InitiateReturnRequest {
    pub order_id: String,
    pub product_id: String,
    pub amount_cents: i64,
}

#[derive(Serialize)]
pub struct ReturnResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/initiate", post(initiate_return))
        .with_state(orchestrator)
}

async fn initiate_return(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<InitiateReturnRequest>,
) -> axum::response::Result<Json<ReturnResponse>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();

    let event_payload = serde_json::json!({
        "feature_type": "return_requested",
        "order_id": payload.order_id,
        "product_id": payload.product_id,
        "amount_cents": payload.amount_cents,
        "action": "Return & Refund",
    });

    let _ = orchestrator.execute_action(
        DepartmentType::Operations,
        format!("Return requested for Order #{}. Please review and approve restock & refund.", payload.order_id),
        tenant_id,
        ActionRisk::DraftForReview,
        event_payload,
    ).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ReturnResponse { success: true }))
}
