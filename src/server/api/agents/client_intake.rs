use axum::{
    extract::{State, Form, Extension},
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
    pub company_name: Option<String>,
    pub budget: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientIntakeResponse {
    pub success: bool,
    pub proposal_drafted: bool,
}

#[derive(Clone)]
pub struct ClientIntakeState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db_pool: Option<PgPool>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ClientIntakeState {
        orchestrator,
        db_pool: None,
    };
    Router::new()
        .route("/", post(handle_client_intake))
        .with_state(state)
}

pub fn router_with_db<S>(orchestrator: Arc<DepartmentOrchestrator>, db_pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ClientIntakeState {
        orchestrator,
        db_pool: Some(db_pool),
    };
    Router::new()
        .route("/", post(handle_client_intake))
        .with_state(state)
}

async fn handle_client_intake(
    State(state): State<ClientIntakeState>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Form(payload): Form<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = if auth_info.org_id.is_empty() {
        return StatusCode::UNAUTHORIZED.into_response();
    } else {
        auth_info.org_id.clone()
    };

    let intake_request_id = Uuid::new_v4().to_string();
    let proposal_id = Uuid::new_v4().to_string();

    let mut actual_customer_id = Uuid::new_v4().to_string();

    if let Some(pool) = &state.db_pool {
        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        if let Err(e) = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
            .execute(&mut *tx)
            .await
        {
            tracing::error!("Failed to set tenant context: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        // Upsert customer based on email
        let existing_customer = sqlx::query_as::<_, crate::domain::repository::models::Customer>(
            "SELECT * FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
        )
        .bind(&tenant_id)
        .bind(&payload.email)
        .fetch_optional(&mut *tx)
        .await;

        if let Ok(Some(cust)) = existing_customer {
            actual_customer_id = cust.id;
        } else {
            if let Err(e) = sqlx::query(
                "INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, $4)"
            )
            .bind(&actual_customer_id)
            .bind(&tenant_id)
            .bind(&payload.name)
            .bind(&payload.email)
            .execute(&mut *tx)
            .await {
                tracing::error!("Failed to insert customer: {}", e);
            }
        }

        let budget = payload.budget.unwrap_or(0);
        if let Err(e) = sqlx::query(
            "INSERT INTO b2b_intake_requests (id, tenant_id, customer_id, company_name, requirements, budget, status) VALUES ($1, $2, $3, $4, $5, $6, 'NEW')"
        )
        .bind(&intake_request_id)
        .bind(&tenant_id)
        .bind(&actual_customer_id)
        .bind(&payload.company_name)
        .bind(&payload.details)
        .bind(budget)
        .execute(&mut *tx)
        .await {
             tracing::error!("Failed to insert intake request: {}", e);
        }

        if let Err(e) = sqlx::query(
            "INSERT INTO b2b_proposals (id, tenant_id, intake_request_id, customer_id, status) VALUES ($1, $2, $3, $4, 'DRAFT')"
        )
        .bind(&proposal_id)
        .bind(&tenant_id)
        .bind(&intake_request_id)
        .bind(&actual_customer_id)
        .execute(&mut *tx)
        .await {
             tracing::error!("Failed to insert b2b_proposals: {}", e);
        }

        if let Err(e) = tx.commit().await {
            tracing::error!("Failed to commit transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let suggested_price = 1500.00;
    let service_name = "Custom Project Scope";

    let action_payload = serde_json::json!({
        "feature_type": "b2b_proposal_rag",
        "customer_inquiry": payload.details,
        "client_name": payload.name,
        "client_email": payload.email,
        "suggested_price": suggested_price,
        "proposal_id": proposal_id,
        "intake_request_id": intake_request_id
    });

    match state.orchestrator.execute_action(
        DepartmentType::Sales,
        format!("Draft RAG proposal for new B2B intake: {}", service_name),
        tenant_id,
        ActionRisk::DraftForReview,
        action_payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ClientIntakeResponse { success: true, proposal_drafted: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response(),
    }
}
