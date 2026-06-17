use axum::{
    extract::{State, Query, Form},
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
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TenantQuery {
    pub tenant: Option<String>,
}

#[derive(Deserialize)]
pub struct ClientIntakeRequest {
    pub name: String,
    pub email: String,
    pub details: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientIntakeResponse {
    pub success: bool,
    pub proposal_drafted: bool,
}

#[derive(Clone)]
pub struct ClientIntakeState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub pool: PgPool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>, pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ClientIntakeState {
        orchestrator,
        pool,
    };
    Router::new()
        .route("/", post(handle_client_intake))
        .with_state(state)
}

async fn handle_client_intake(
    State(state): State<ClientIntakeState>,
    Query(query): Query<TenantQuery>,
    Form(payload): Form<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());

    let suggested_price = 450.00;
    let suggested_price_cents = (suggested_price * 100.0) as i64;
    let service_name = "Living Room Painting";
    let scope_text = format!("{} based on your standard rate.", service_name);

    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${}.",
        payload.details, suggested_price
    );

    let proposal_id = Uuid::new_v4();

    let res = sqlx::query(
        "INSERT INTO proposals (id, tenant_id, status, scope, total_amount_cents, required_deposit_cents, created_at, updated_at) VALUES ($1, $2, 'DRAFT', $3, $4, $5, NOW(), NOW())"
    )
    .bind(proposal_id)
    .bind(&tenant_id)
    .bind(&scope_text)
    .bind(suggested_price_cents)
    .bind(suggested_price_cents / 2) // 50% deposit default
    .execute(&state.pool)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to insert drafted proposal: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response();
    }

    let action_payload = serde_json::json!({
        "feature_type": "quote_draft",
        "proposal_id": proposal_id.to_string(),
        "customer_inquiry": payload.details,
        "client_name": payload.name,
        "client_email": payload.email,
        "suggested_price": suggested_price,
        "scope": scope_text,
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
