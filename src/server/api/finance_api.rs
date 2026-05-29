use axum::{
    extract::{State},
    routing::{post, get},
    Json, Router,
};
use std::sync::Arc;
use crate::services::finance::capital_engine::{CapitalEngine, CapitalAdvance};
use crate::db::DB;
use server_common::Claims;
use axum::extract::Extension;

pub fn finance_routes() -> Router<Arc<DB>> {
    Router::new()
        .route("/capital_advances", post(create_advance))
        .route("/capital_advances/repay", post(process_repayment))
        .route("/capital_advances/analyze", get(analyze_needs))
}

#[derive(serde::Deserialize)]
pub struct CreateAdvanceRequest {
    pub amount: f64,
    pub fee: f64,
    pub repayment_percentage: f64,
}

#[derive(serde::Deserialize)]
pub struct RepaymentRequest {
    pub transaction_amount: f64,
}

async fn create_advance(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateAdvanceRequest>,
) -> Result<Json<CapitalAdvance>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let engine = CapitalEngine::new(db.pool.clone());
    match engine.create_advance(&tenant_id, payload.amount, payload.fee, payload.repayment_percentage).await {
        Ok(advance) => Ok(Json(advance)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn process_repayment(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RepaymentRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let engine = CapitalEngine::new(db.pool.clone());
    match engine.process_repayment(&tenant_id, payload.transaction_amount).await {
        Ok(repaid) => Ok(Json(serde_json::json!({"repaid_amount": repaid}))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(serde::Deserialize)]
pub struct OfferAdvanceRequest {
    pub advance_id: String,
    pub amount: f64,
    pub fee: f64,
    pub repayment_percentage: f64,
}

async fn analyze_needs(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Option<CapitalAdvance>>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let engine = CapitalEngine::new(db.pool.clone());
    match engine.analyze_capital_needs(&tenant_id).await {
        Ok(advance_opt) => Ok(Json(advance_opt)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
