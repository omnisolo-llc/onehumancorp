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
use crate::domain::repository::models::{TaxObligation, VirtualEnvelope};
use crate::domain::repository::finance_repo::FinanceRepository;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CreateTaxObligationRequest {
    pub tenant_id: String,
    pub transaction_id: String,
    pub tax_type: String,
    pub amount: f64,
    pub jurisdiction: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateVirtualEnvelopeRequest {
    pub tenant_id: String,
    pub name: String,
    pub target_amount: Option<f64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/finance/tax_obligations/{tenant_id}", get(get_tax_obligations))
        .route("/api/finance/tax_obligations", post(create_tax_obligation))
        .route("/api/finance/virtual_envelopes/{tenant_id}", get(get_virtual_envelopes))
        .route("/api/finance/virtual_envelopes", post(create_virtual_envelope))
}

async fn get_tax_obligations(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let repo = FinanceRepository::new(state.db);
    match repo.get_tax_obligations(&tenant_id).await {
        Ok(obligations) => (StatusCode::OK, Json(obligations)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn create_tax_obligation(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaxObligationRequest>,
) -> impl IntoResponse {
    let repo = FinanceRepository::new(state.db);
    let obligation = TaxObligation {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        transaction_id: payload.transaction_id,
        tax_type: payload.tax_type,
        amount: payload.amount,
        jurisdiction: payload.jurisdiction,
        status: "PENDING".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    match repo.create_tax_obligation(obligation.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(obligation)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_virtual_envelopes(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let repo = FinanceRepository::new(state.db);
    match repo.get_virtual_envelopes(&tenant_id).await {
        Ok(envelopes) => (StatusCode::OK, Json(envelopes)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn create_virtual_envelope(
    State(state): State<AppState>,
    Json(payload): Json<CreateVirtualEnvelopeRequest>,
) -> impl IntoResponse {
    let repo = FinanceRepository::new(state.db);
    let envelope = VirtualEnvelope {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        name: payload.name,
        balance: 0.0,
        target_amount: payload.target_amount,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    match repo.create_virtual_envelope(envelope.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(envelope)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
