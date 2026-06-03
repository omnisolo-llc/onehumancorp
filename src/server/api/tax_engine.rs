use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use utoipa::ToSchema;
use sqlx::types::BigDecimal;
use std::str::FromStr;

use crate::domain::repository::tax_engine_repo::{TaxEngineRepository, TaxLedgerEntry};

// gRPC Server Implementation
use crate::proto::ohc::tax::v1::{
    tax_engine_service_server::TaxEngineService,
    CalculateTaxRequest as GrpcCalculateTaxRequest,
    CalculateTaxResponse as GrpcCalculateTaxResponse,
    TaxLedgerEntry as GrpcTaxLedgerEntry,
    NexusThresholdAlertRequest,
    NexusThresholdAlertResponse,
    NexusThresholdAlert,
};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct TaxEngineState {
    pub repo: TaxEngineRepository,
}

impl TaxEngineState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: TaxEngineRepository::new(pool),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CalculateTaxRequest {
    pub tenant_id: String,
    pub buyer_country_code: String,
    pub buyer_region_code: Option<String>,
    pub taxable_amount: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CalculateTaxResponse {
    pub tax_amount: f64,
    pub tax_rate: f64,
    pub jurisdiction_id: String,
}

pub async fn calculate_tax(
    State(state): State<Arc<TaxEngineState>>,
    Json(payload): Json<CalculateTaxRequest>,
) -> Result<Json<CalculateTaxResponse>, axum::http::StatusCode> {

    // Look up the jurisdiction from the repo
    let jurisdiction = match state.repo.get_jurisdiction(&payload.buyer_country_code, payload.buyer_region_code.as_deref()).await {
        Ok(Some(j)) => j,
        Ok(None) => return Err(axum::http::StatusCode::NOT_FOUND),
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    let tax_rate: f64 = jurisdiction.tax_rate.to_string().parse().unwrap_or(0.0);
    // Standard rounding to 2 decimal places for financial calculations
    let tax_amount = (payload.taxable_amount * tax_rate * 100.0).round() / 100.0;

    Ok(Json(CalculateTaxResponse {
        tax_amount,
        tax_rate,
        jurisdiction_id: jurisdiction.id,
    }))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecordLedgerRequest {
    pub id: String,
    pub tenant_id: String,
    pub order_id: String,
    pub jurisdiction_id: String,
    pub taxable_amount: f64,
    pub tax_amount: f64,
    pub tax_rate: f64,
}

pub async fn record_tax_ledger(
    State(state): State<Arc<TaxEngineState>>,
    Json(payload): Json<RecordLedgerRequest>,
) -> Result<Json<()>, axum::http::StatusCode> {

    let entry = TaxLedgerEntry {
        id: payload.id,
        tenant_id: payload.tenant_id,
        order_id: payload.order_id,
        jurisdiction_id: payload.jurisdiction_id,
        taxable_amount: BigDecimal::from_str(&payload.taxable_amount.to_string()).unwrap_or_default(),
        tax_amount: BigDecimal::from_str(&payload.tax_amount.to_string()).unwrap_or_default(),
        tax_rate: BigDecimal::from_str(&payload.tax_rate.to_string()).unwrap_or_default(),
        status: "COLLECTED".to_string(),
    };

    match state.repo.record_ledger_entry(&entry).await {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

use axum::{routing::post, Router};

pub fn router() -> Router<Arc<TaxEngineState>> {
    Router::new()
        .route("/calculate", post(calculate_tax))
        .route("/ledger", post(record_tax_ledger))
}

#[tonic::async_trait]
impl TaxEngineService for TaxEngineState {
    async fn calculate_tax(
        &self,
        request: Request<GrpcCalculateTaxRequest>,
    ) -> Result<Response<GrpcCalculateTaxResponse>, Status> {
        let req = request.into_inner();

        let jurisdiction = match self.repo.get_jurisdiction(&req.buyer_location_code, None).await {
            Ok(Some(j)) => j,
            Ok(None) => return Err(Status::not_found("Jurisdiction not found")),
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let tax_rate: f64 = jurisdiction.tax_rate.to_string().parse().unwrap_or(0.0);
        let tax_amount = (req.taxable_amount * tax_rate * 100.0).round() / 100.0;

        Ok(Response::new(GrpcCalculateTaxResponse {
            tax_amount,
            tax_rate,
            jurisdiction_id: jurisdiction.id,
        }))
    }

    async fn record_tax_ledger(
        &self,
        request: Request<GrpcTaxLedgerEntry>,
    ) -> Result<Response<GrpcTaxLedgerEntry>, Status> {
        let req = request.into_inner();

        let entry = TaxLedgerEntry {
            id: req.id.clone(),
            tenant_id: req.tenant_id.clone(),
            order_id: req.order_id.clone(),
            jurisdiction_id: req.jurisdiction_id.clone(),
            taxable_amount: BigDecimal::from_str(&req.taxable_amount.to_string()).unwrap_or_default(),
            tax_amount: BigDecimal::from_str(&req.tax_amount.to_string()).unwrap_or_default(),
            tax_rate: BigDecimal::from_str(&req.tax_rate.to_string()).unwrap_or_default(),
            status: req.status.clone(),
        };

        self.repo.record_ledger_entry(&entry).await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(req))
    }

    async fn get_nexus_alerts(
        &self,
        request: Request<NexusThresholdAlertRequest>,
    ) -> Result<Response<NexusThresholdAlertResponse>, Status> {
        let req = request.into_inner();

        let thresholds = self.repo.get_nexus_thresholds(&req.tenant_id).await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut alerts = Vec::new();
        for threshold in thresholds {
            let current: f64 = threshold.current_volume.to_string().parse().unwrap_or(0.0);
            let limit: f64 = threshold.threshold_volume.to_string().parse().unwrap_or(1.0);
            let ratio = current / limit;

            if ratio >= 0.8 {
                alerts.push(NexusThresholdAlert {
                    tenant_id: threshold.tenant_id.clone(),
                    jurisdiction_id: threshold.jurisdiction_id.clone(),
                    current_volume: current,
                    threshold_volume: limit,
                    alert_message: format!(
                        "You are nearing the economic nexus for jurisdiction {}. You have reached {:.1}% of the threshold.",
                        threshold.jurisdiction_id,
                        ratio * 100.0
                    ),
                });
            }
        }

        Ok(Response::new(NexusThresholdAlertResponse {
            alerts,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_calculation_rounding() {
        let taxable_amount = 100.0;
        let tax_rate = 0.0825; // 8.25%

        let tax_amount = (taxable_amount * tax_rate * 100.0).round() / 100.0;
        assert_eq!(tax_amount, 8.25);

        let complex_amount = 100.55;
        let complex_tax = (complex_amount * tax_rate * 100.0).round() / 100.0;
        assert_eq!(complex_tax, 8.30); // 8.295375 rounds up to 8.30
    }
}
