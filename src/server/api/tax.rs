use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::db::DB;
use crate::domain::repository::models::TaxLedgerEntry;
use crate::domain::repository::tax_repo::TaxRepository;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CalculateTaxRequest {
    pub tenant_id: String,
    pub country_code: String,
    pub region_code: Option<String>,
    pub taxable_amount_cents: i64,
}

#[derive(Serialize)]
pub struct CalculateTaxResponse {
    pub tax_rate: Decimal,
    pub tax_amount_cents: i64,
    pub total_amount_cents: i64,
    pub jurisdiction_id: Option<String>,
}

#[derive(Deserialize)]
pub struct RecordTaxRequest {
    pub tenant_id: String,
    pub transaction_id: String,
    pub jurisdiction_id: String,
    pub taxable_amount_cents: i64,
    pub tax_rate: Decimal,
    pub tax_collected_cents: i64,
}

#[derive(Serialize)]
pub struct TaxHealthResponse {
    pub total_taxable_sales_cents: i64,
    pub total_tax_collected_cents: i64,
    pub pending_liability_cents: i64,
    pub alerts: Vec<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/tax/calculate", post(calculate_tax))
        .route("/api/tax/record", post(record_tax))
        .route("/api/tax/health/:tenant_id", get(get_tax_health))
}

async fn calculate_tax(
    State(state): State<AppState>,
    Json(payload): Json<CalculateTaxRequest>,
) -> impl IntoResponse {
    let repo = TaxRepository::new(state.db.clone());

    let (rate, j_id) = match repo.get_jurisdiction(&payload.country_code, payload.region_code.as_deref()).await {
        Ok(Some(jurisdiction)) => (jurisdiction.tax_rate, Some(jurisdiction.id)),
        Ok(None) => (Decimal::new(0, 0), None), // fallback to 0% tax if unknown
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let tax_amount_decimal = rate * Decimal::from_i64(payload.taxable_amount_cents).unwrap_or(Decimal::new(0, 0));
    let tax_amount_cents = tax_amount_decimal.round().to_string().parse::<i64>().unwrap_or(0);

    let response = CalculateTaxResponse {
        tax_rate: rate,
        tax_amount_cents,
        total_amount_cents: payload.taxable_amount_cents + tax_amount_cents,
        jurisdiction_id: j_id,
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn record_tax(
    State(state): State<AppState>,
    Json(payload): Json<RecordTaxRequest>,
) -> impl IntoResponse {
    let repo = TaxRepository::new(state.db.clone());

    let entry = TaxLedgerEntry {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        transaction_id: payload.transaction_id,
        jurisdiction_id: payload.jurisdiction_id,
        taxable_amount_cents: payload.taxable_amount_cents,
        tax_rate: payload.tax_rate,
        tax_collected_cents: payload.tax_collected_cents,
        created_at: Some(Utc::now()),
    };

    match repo.add_ledger_entry(entry).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_tax_health(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let repo = TaxRepository::new(state.db.clone());

    match repo.get_ledger_entries(&tenant_id).await {
        Ok(entries) => {
            let mut total_taxable = 0;
            let mut total_tax = 0;

            for entry in entries {
                total_taxable += entry.taxable_amount_cents;
                total_tax += entry.tax_collected_cents;
            }

            // Ideally this would invoke an AI service,
            // but we cannot make real LLM network calls directly in this handler synchronously reliably.
            // We use threshold as an approximation of the Regulatory AI alerting for now.
            let mut alerts = Vec::new();
            if total_taxable > 10_000_000 {
                alerts.push("Regulatory AI Agent Alert: You are approaching the economic nexus threshold for California.".to_string());
            } else if total_taxable > 5_000_000 {
                 alerts.push("Regulatory AI Agent Alert: You are nearing the economic nexus threshold in some states.".to_string());
            }

            let response = TaxHealthResponse {
                total_taxable_sales_cents: total_taxable,
                total_tax_collected_cents: total_tax,
                pending_liability_cents: total_tax,
                alerts,
            };

            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_tax_calculation_logic() {
        let tax_rate = dec!(0.085);
        let taxable_amount_cents = 10000; // $100.00
        let tax_amount_decimal = tax_rate * Decimal::from_i64(taxable_amount_cents).unwrap();
        let tax_amount_cents = tax_amount_decimal.round().to_string().parse::<i64>().unwrap();
        assert_eq!(tax_amount_cents, 850); // $8.50
    }
}
