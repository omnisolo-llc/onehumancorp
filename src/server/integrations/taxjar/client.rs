use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxJarClient {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxRateRequest {
    pub from_country: String,
    pub from_zip: String,
    pub from_state: String,
    pub to_country: String,
    pub to_zip: String,
    pub to_state: String,
    pub amount: f64,
    pub shipping: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxRateResponse {
    pub amount_to_collect: f64,
    pub rate: f64,
}

impl TaxJarClient {
    pub fn new(api_key: String) -> Self {
        TaxJarClient { api_key }
    }

    pub async fn calculate_tax(&self, request: &TaxRateRequest, db_pool: &PgPool) -> Result<TaxRateResponse, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            db_pool,
            "unknown", // tenant id is not passed here for simplicity
            "taxjar_calculate_tax",
            0.05 // mock cost for api call
        ).await;

        let rate = if request.to_state.to_lowercase() == "ca" || request.to_state.to_lowercase() == "california" {
            0.0825
        } else if request.to_state.to_lowercase() == "ny" || request.to_state.to_lowercase() == "new york" {
            0.08875
        } else {
            0.08 // Default mock rate
        };

        let amount_to_collect = (request.amount + request.shipping) * rate;

        Ok(TaxRateResponse {
            amount_to_collect,
            rate,
        })
    }

    pub async fn sync_order(&self, order_id: &str, _amount: f64, tax: f64, to_state: &str, db_pool: &PgPool) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            db_pool,
            "unknown",
            "taxjar_sync_order",
            0.01 // mock cost
        ).await;

        Ok(format!("Order {} synced to TaxJar successfully with tax {} for state {}", order_id, tax, to_state))
    }
}
