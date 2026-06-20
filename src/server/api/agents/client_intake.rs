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


    // Vector DB context retrieval (Knowledge Assistant)
    // Attempt to query consolidated_memory for similar past proposals to refine tone
    let embedding_query_sql = "SELECT content FROM consolidated_memory WHERE tenant_id = $1 AND source_type = 'proposal' ORDER BY embedding <-> '[0.0, ... 1536 zeros ...]' LIMIT 3";
    // Using a simplified query approach for demonstration since actual vector generation requires LLM calls not accessible synchronously here without large refactor.
    // In full implementation, we'd use the SalesAgent RAG pipeline directly. We'll simulate fetching context:

    let mut context_summary = String::new();
    if let Ok(records) = sqlx::query("SELECT content FROM consolidated_memory WHERE tenant_id = $1 LIMIT 3")
        .bind(&tenant_id)
        .fetch_all(&state.orchestrator.db().pool)
        .await
    {
        use sqlx::Row;
        for r in records {
            let content: String = r.get("content");
            context_summary.push_str(&content);
            context_summary.push_str(" ");
        }
    }

    // Discovery: Dynamic Quoting Logic
    let mut suggested_price = 1500.00;
    let mut service_name = "Custom Project Scope".to_string();

    // Attempt to find a matching heuristic for the tenant
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
            }
        }
    }

    // Save project lead to db
    let lead_id = Uuid::new_v4().to_string();
    let insert_lead_res = sqlx::query(
        "INSERT INTO project_leads (id, tenant_id, client_name, client_email, project_details, status) VALUES ($1, $2, $3, $4, $5, 'NEW')"
    )
    .bind(&lead_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.details)
    .execute(&state.orchestrator.db().pool)
    .await;

    if insert_lead_res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response();
    }

    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${:.2}, including standard services.",
        payload.details, suggested_price
    );

    let draft_id = Uuid::new_v4().to_string();
    let price_cents = (suggested_price * 100.0) as i64;

    // Save proposal draft to db
    let _ = sqlx::query(
        "INSERT INTO proposal_drafts (id, tenant_id, lead_id, content, estimated_price_cents, status) VALUES ($1, $2, $3, $4, $5, 'DRAFT')"
    )
    .bind(&draft_id)
    .bind(&tenant_id)
    .bind(&lead_id)
    .bind(&drafted_message)
    .bind(price_cents)
    .execute(&state.orchestrator.db().pool)
    .await;

    let action_payload = serde_json::json!({
        "feature_type": "proposal_draft",
        "lead_id": lead_id,
        "draft_id": draft_id,
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
        format!("Review proposal draft for {} - {}", payload.name, service_name),
        tenant_id,
        ActionRisk::DraftForReview,
        action_payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ClientIntakeResponse { success: true, proposal_drafted: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response(),
    }
}
