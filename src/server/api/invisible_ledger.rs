use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::db::DB;
use crate::domain::repository::models::{PaymentRoutingRule, TransactionGroup};
use crate::domain::repository::invisible_ledger_repo::InvisibleLedgerRepository;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CreateRoutingRuleRequest {
    pub tenant_id: String,
    pub product_service_id: String,
    pub split_percentage: f64,
    pub destination_party_id: String,
}

#[derive(Deserialize)]
pub struct CreateTransactionGroupRequest {
    pub tenant_id: String,
    pub reference_type: String,
    pub reference_id: String,
    pub total_amount: f64,
    pub source_party_id: String,
    pub product_service_id: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let state = AppState { db };
    Router::new()
        .route("/api/invisible_ledger/routing_rules", post(create_routing_rule))
        .route("/api/invisible_ledger/transaction_groups", post(create_transaction_group))
        .route("/api/invisible_ledger/balances/{tenant_id}", get(get_tenant_balances))
        .with_state(state)
}

async fn create_routing_rule(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoutingRuleRequest>,
) -> impl IntoResponse {
    let repo = InvisibleLedgerRepository::new(state.db);

    let rule = PaymentRoutingRule {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        product_service_id: payload.product_service_id,
        split_percentage: payload.split_percentage,
        destination_party_id: payload.destination_party_id,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    match repo.create_routing_rule(rule.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(rule)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn create_transaction_group(
    State(state): State<AppState>,
    Json(payload): Json<CreateTransactionGroupRequest>,
) -> impl IntoResponse {
    let repo = InvisibleLedgerRepository::new(state.db);

    let group = TransactionGroup {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        reference_type: payload.reference_type,
        reference_id: payload.reference_id,
        status: Some("PENDING".to_string()),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    match repo.record_transaction_group(group.clone(), payload.total_amount, &payload.source_party_id, &payload.product_service_id).await {
        Ok(_) => (StatusCode::CREATED, Json(group)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_tenant_balances(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let repo = InvisibleLedgerRepository::new(state.db);
    match repo.get_tenant_balances(&tenant_id).await {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
