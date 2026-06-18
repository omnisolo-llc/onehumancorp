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

#[derive(Deserialize)]
struct LLMQuoteDraft {
    service_name: String,
    suggested_price: f64,
    drafted_message: String,
}

async fn handle_client_intake(
    State(state): State<ClientIntakeState>,
    Query(query): Query<TenantQuery>,
    Form(payload): Form<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());

    let mut services_context = String::new();
    let heuristics_res = sqlx::query("SELECT service_category, base_rate_cents FROM pricing_heuristics WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&state.orchestrator.db().pool)
        .await;

    if let Ok(heuristics) = heuristics_res {
        use sqlx::Row;
        for h in heuristics {
            let category: String = h.get("service_category");
            let rate_cents: i64 = h.get("base_rate_cents");
            services_context.push_str(&format!("- {}: ${:.2}\n", category, (rate_cents as f64) / 100.0));
        }
    }

    if services_context.is_empty() {
        services_context.push_str("- Custom Project Scope: $1500.00\n");
    }

    let prompt = format!(
        "You are an expert sales agent for a service-based agency. A potential client has submitted an inquiry.\n\
        Client Name: {}\n\
        Client Email: {}\n\
        Inquiry Details: {}\n\
        \n\
        Here is the catalog of services and base prices for this agency:\n\
        {}\n\
        \n\
        Analyze the inquiry and generate a professional proposal draft. Match the inquiry to the most relevant service from the catalog, determine a suggested price, and draft a short, friendly cover letter response.\n\
        Output your response as a valid JSON object with EXACTLY these fields:\n\
        {{\n\
            \"service_name\": \"The matched service name\",\n\
            \"suggested_price\": The numerical price (e.g. 1500.00),\n\
            \"drafted_message\": \"The drafted response message to the client\"\n\
        }}\n\
        Do NOT include markdown formatting like ```json or any other text.",
        payload.name, payload.email, payload.details, services_context
    );

    let llm_client = crate::minimax::LocalLLMClient::new();

    let (service_name, suggested_price, drafted_message) = match llm_client.reason(&prompt).await {
        Ok(response) => {
            let cleaned = response.trim().trim_start_matches("```json").trim_end_matches("```").trim();
            match serde_json::from_str::<LLMQuoteDraft>(&cleaned) {
                Ok(draft) => (draft.service_name, draft.suggested_price, draft.drafted_message),
                Err(e) => {
                    tracing::error!("Failed to parse LLM JSON: {}. Fallback to heuristics.", e);
                    fallback_intake(&payload, &tenant_id, &state).await
                }
            }
        },
        Err(e) => {
            tracing::error!("LLM request failed: {}. Fallback to heuristics.", e);
            fallback_intake(&payload, &tenant_id, &state).await
        }
    };

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
        tenant_id.clone(),
        ActionRisk::DraftForReview,
        action_payload.clone(),
    ).await {
        Ok(_) => {
            let feed_repo = crate::domain::repository::agent_feed_repo::AgentFeedRepository::new(state.orchestrator.db().pool.clone());
            let feed_item = crate::domain::repository::agent_feed_repo::AgentFeedItem {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: tenant_id,
                event_source: "sales_agent".to_string(),
                context_payload: None,
                proposed_action: Some(sqlx::types::Json(action_payload)),
                lifecycle_state: "PENDING_APPROVAL".to_string(),
                created_at: Some(chrono::Utc::now()),
                updated_at: Some(chrono::Utc::now()),
            };
            if let Err(e) = feed_repo.create(feed_item).await {
                 tracing::error!("Failed to push to agent feed: {}", e);
            }
            (StatusCode::OK, Json(ClientIntakeResponse { success: true, proposal_drafted: true })).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response(),
    }
}

async fn fallback_intake(payload: &ClientIntakeRequest, tenant_id: &str, state: &ClientIntakeState) -> (String, f64, String) {
    let mut suggested_price = 1500.00;
    let mut service_name = "Custom Project Scope".to_string();

    let heuristics_res = sqlx::query("SELECT service_category, base_rate_cents FROM pricing_heuristics WHERE tenant_id = $1")
        .bind(tenant_id)
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

    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${:.2}, including standard services.",
        payload.details, suggested_price
    );

    (service_name, suggested_price, drafted_message)
}
