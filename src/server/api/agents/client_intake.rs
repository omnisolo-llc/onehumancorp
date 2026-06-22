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
    Query(query): Query<TenantQuery>,
    Form(payload): Form<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());

<<<<<<< HEAD
    // Discovery: Dynamic Quoting Logic
    // In a real system, we would use an LLM here to match the inquiry against the pricing heuristics.
    // For this implementation, we'll perform a keyword-based heuristic lookup to fulfill the requirement.

    let mut suggested_price = 1500.00;
    let mut service_name = "Custom Project Scope".to_string();

    // Attempt to find a matching heuristic for the tenant
    // Using non-macro query to avoid Bazel/Cargo env issues in CI
    let heuristics_res = sqlx::query("SELECT service_category, base_rate_cents FROM pricing_heuristics WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&state.orchestrator.db().pool)
        .await;

    if let Ok(heuristics) = heuristics_res {
        use sqlx::Row;
        for h in heuristics {
            let category: String = h.get("service_category");
            let rate_cents: i64 = h.get("base_rate_cents");
            if payload.details.to_lowercase().contains(&category.to_lowercase()) {
                suggested_price = (rate_cents as f64) / 100.0;
                service_name = category;
                break;
=======
    let lead_id = uuid::Uuid::new_v4().to_string();
    let customer_id = uuid::Uuid::new_v4(); // Generate a new UUID for the customer if one doesn't exist

    // Insert service lead
    match &state.orchestrator.db().store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES ($1, $2, $3, $4, 'web_form', 'new', NOW(), NOW())"
            )
            .bind(&lead_id)
            .bind(&tenant_id)
            .bind(customer_id)
            .bind(&payload.details)
            .execute(&state.orchestrator.db().pool).await;

            if let Err(e) = res {
                tracing::error!("Failed to insert service lead: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response();
            }

            let job_id = uuid::Uuid::new_v4().to_string();
            let job_payload = serde_json::json!({
                "lead_id": lead_id,
                "name": payload.name,
                "email": payload.email,
            });

            let job_res = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'LeadReceived', $3, 'PENDING', NOW())"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(job_payload)
            .execute(&state.orchestrator.db().pool).await;

            if let Err(e) = job_res {
                tracing::error!("Failed to enqueue lead received job: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response();
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
             let res = sqlx::query(
                "INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES (?, ?, ?, ?, 'web_form', 'new', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&lead_id)
            .bind(&tenant_id)
            .bind(customer_id.to_string())
            .bind(&payload.details)
            .execute(pool).await;

            if let Err(e) = res {
                tracing::error!("Failed to insert service lead: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response();
            }

            let job_id = uuid::Uuid::new_v4().to_string();
            let job_payload = serde_json::json!({
                "lead_id": lead_id,
                "name": payload.name,
                "email": payload.email,
            });

            let job_res = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'LeadReceived', ?, 'PENDING', CURRENT_TIMESTAMP)"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(job_payload.to_string())
            .execute(pool).await;

            if let Err(e) = job_res {
                tracing::error!("Failed to enqueue lead received job: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response();
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
            }
        }
    }

<<<<<<< HEAD
    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${:.2}, including standard services.",
        payload.details, suggested_price
    );

    let action_payload = serde_json::json!({
        "feature_type": "quote_draft",
        "customer_inquiry": payload.details,
        "client_name": payload.name,
        "client_email": payload.email,
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
=======
    (StatusCode::OK, Json(ClientIntakeResponse { success: true, proposal_drafted: true })).into_response()
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
}
