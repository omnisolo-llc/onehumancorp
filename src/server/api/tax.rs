use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use crate::services::tax::service::TaxComputationEngine;

#[derive(Clone)]
pub struct TaxAppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CalculateTaxRequest {
    pub tenant_id: String,
    pub transaction_id: String,
    pub amount: f64,
    pub country_code: String,
    pub state_code: Option<String>,
    pub zip_code: Option<String>,
    pub product_category: Option<String>,
}

#[derive(Serialize)]
pub struct CalculateTaxResponse {
    pub tax_amount: f64,
    pub jurisdiction_id: String,
}

#[derive(Serialize)]
pub struct ComplianceReportResponse {
    pub alerts: Option<String>,
    pub liabilities_summary: serde_json::Value,
}

pub fn router() -> Router<TaxAppState> {
    Router::new()
        .route("/api/tax/calculate", post(calculate_tax))
        .route("/api/tax/compliance/:tenant_id", get(get_compliance_report))
}

async fn calculate_tax(
    State(state): State<TaxAppState>,
    Json(payload): Json<CalculateTaxRequest>,
) -> impl IntoResponse {
    let engine = TaxComputationEngine::new(state.db);
    match engine.calculate_tax(
        &payload.tenant_id,
        &payload.transaction_id,
        payload.amount,
        &payload.country_code,
        payload.state_code.as_deref(),
        payload.zip_code.as_deref(),
        payload.product_category.as_deref(),
    ).await {
        Ok(entry) => {
            let resp = CalculateTaxResponse {
                tax_amount: entry.tax_amount,
                jurisdiction_id: entry.jurisdiction_id,
            };
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_compliance_report(
    State(state): State<TaxAppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let engine = TaxComputationEngine::new(state.db);

    // Evaluate thresholds
    let alerts = match engine.evaluate_compliance_thresholds(&tenant_id).await {
        Ok(a) => a,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    // Get ledgers for summary
    let ledgers = match engine.get_tenant_tax_ledgers(&tenant_id).await {
        Ok(l) => l,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let mut total = 0.0;
    for entry in &ledgers {
        total += entry.tax_amount;
    }

    let summary = serde_json::json!({
        "total_tax_liability": total,
        "entry_count": ledgers.len()
    });

    let resp = ComplianceReportResponse {
        alerts,
        liabilities_summary: summary,
    };

    (StatusCode::OK, Json(resp)).into_response()
}
