use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};

#[derive(Serialize, Deserialize)]
pub struct IntakeRequestPayload {
    pub description: String,
    pub company_name: String,
    pub contact_email: String,
    pub budget_cents: Option<i64>,
    pub timeline: Option<String>,
    pub tenant_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProposalPayload {
    pub intake_request_id: String,
    pub b2b_client_id: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/intake", post(create_intake_request))
        .route("/proposals", post(create_proposal))
        .route("/proposals/{id}", get(get_proposal))
        .route("/proposals/{id}/approve", post(approve_proposal))
}

async fn create_intake_request(
    Json(payload): Json<IntakeRequestPayload>,
) -> impl IntoResponse {
    let client_id = Uuid::new_v4().to_string();
    let intake_id = Uuid::new_v4().to_string();
    let tenant_id = payload.tenant_id.unwrap_or_else(|| "default".to_string());

    // Create an intake request and trigger the agent
    let suggested_price = payload.budget_cents.unwrap_or(150000) as f64 / 100.0;

    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${}, including standard services.",
        payload.description, suggested_price
    );

    // In a real app we'd save this to DB here. For the agent to pick it up, we fire an action.
    // However, since we mock orchestrator injection here for simplicity, we rely on the client_intake.rs
    // or webhook.rs for actual agent dispatch.

    (StatusCode::CREATED, Json(serde_json::json!({ "id": intake_id, "b2b_client_id": client_id }))).into_response()
}

async fn create_proposal(
    Json(_payload): Json<ProposalPayload>,
) -> impl IntoResponse {
    let proposal_id = Uuid::new_v4().to_string();
    (StatusCode::CREATED, Json(serde_json::json!({ "id": proposal_id }))).into_response()
}

async fn get_proposal(
    Path(id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "id": id,
        "tenant_id": "default",
        "intake_request_id": "intake_id",
        "b2b_client_id": "client_id",
        "status": "DRAFT",
        "total_amount_cents": 150000,
        "required_deposit_cents": 50000,
        "checkout_url": null,
        "client_name": "ACME Corp",
        "project_scope": "Website Redesign",
        "timeline": "4 Weeks"
    }))).into_response()
}

async fn approve_proposal(
    Path(_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "approved", "checkout_url": "https://buy.stripe.com/test_mock" }))).into_response()
}
