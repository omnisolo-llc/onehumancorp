use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::integrations::taxjar::client::{TaxJarClient, TaxRateRequest};

#[derive(Deserialize, Default)]
pub struct CalculateTaxPayload {
    pub from_country: Option<String>,
    pub from_zip: Option<String>,
    pub from_state: Option<String>,
    pub to_country: Option<String>,
    pub to_zip: Option<String>,
    pub to_state: Option<String>,
    pub amount: f64,
    pub shipping: f64,
}

#[derive(Deserialize)]
pub struct SyncOrderPayload {
    pub order_id: String,
    pub amount: f64,
    pub tax: f64,
    pub to_state: String,
}

pub async fn calculate_tax(
    State(pool): State<PgPool>,
    Json(payload): Json<CalculateTaxPayload>,
) -> impl IntoResponse {
    let api_key = std::env::var("TAXJAR_API_KEY").unwrap_or_else(|_| "mock_key".to_string());
    let client = TaxJarClient::new(api_key);

    let req = TaxRateRequest {
        from_country: payload.from_country.unwrap_or_default(),
        from_zip: payload.from_zip.unwrap_or_default(),
        from_state: payload.from_state.unwrap_or_default(),
        to_country: payload.to_country.unwrap_or_default(),
        to_zip: payload.to_zip.unwrap_or_default(),
        to_state: payload.to_state.unwrap_or_default(),
        amount: payload.amount,
        shipping: payload.shipping,
    };

    match client.calculate_tax(&req, &pool).await {
        Ok(res) => (StatusCode::OK, Json(serde_json::json!({ "success": true, "rate": res.rate, "amount_to_collect": res.amount_to_collect }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "success": false, "error": e }))),
    }
}

pub async fn sync_order(
    State(pool): State<PgPool>,
    Json(payload): Json<SyncOrderPayload>,
) -> impl IntoResponse {
    let api_key = std::env::var("TAXJAR_API_KEY").unwrap_or_else(|_| "mock_key".to_string());
    let client = TaxJarClient::new(api_key);

    match client.sync_order(&payload.order_id, payload.amount, payload.tax, &payload.to_state, &pool).await {
        Ok(msg) => (StatusCode::OK, Json(serde_json::json!({ "success": true, "message": msg }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "success": false, "error": e }))),
    }
}
