use axum::{
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
    extract::Extension,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct FetchRatesRequest {
    pub orderId: String,
    pub weight: String,
    pub dimensions: String,
    pub address: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PurchaseLabelRequest {
    pub orderId: String,
    pub rateId: String,
}

#[derive(Serialize)]
pub struct RatesResponse {
    pub rates: Vec<crate::integrations::shippo::client::ShippoRate>,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/rates", post(fetch_rates))
        .route("/label", post(purchase_label))
}

async fn fetch_rates(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    let _tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let weight_f64 = payload.weight.parse::<f64>().unwrap_or(1.0);

    let registry = crate::integrations::registry::IntegrationsRegistry::new();

    // Attempt to connect default Shippo sub-account if missing (Mocked sub-account provisioning)
    // Normally we'd look up the tenant's sub-account token from DB
    let api_key = std::env::var("SHIPPO_API_TOKEN").unwrap_or_else(|_| "mock_token".to_string());
    let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
        integration_id: "shippo".to_string(),
        base_url: "https://api.goshippo.com".to_string(),
        bot_token: "".to_string(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: api_key,
        from_phone: "".to_string(),
    };
    let _ = registry.connect("shippo", "https://api.goshippo.com", creds);

    if let Some(address) = &payload.address {
        let is_valid = registry.shippo_validate_address(address).await.unwrap_or(true);
        if !is_valid {
            tracing::warn!("Invalid shipping address provided to Shippo");
        }
    }

    match registry.fetch_rates("shippo", weight_f64, &payload.dimensions, payload.address).await {
        Ok(rates) => (StatusCode::OK, Json(RatesResponse { rates })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn purchase_label(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    let _tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    let registry = crate::integrations::registry::IntegrationsRegistry::new();

    // Attempt to connect default Shippo sub-account if missing
    let api_key = std::env::var("SHIPPO_API_TOKEN").unwrap_or_else(|_| "mock_token".to_string());
    let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
        integration_id: "shippo".to_string(),
        base_url: "https://api.goshippo.com".to_string(),
        bot_token: "".to_string(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: api_key,
        from_phone: "".to_string(),
    };
    let _ = registry.connect("shippo", "https://api.goshippo.com", creds);

    match registry.purchase_label("shippo", &payload.rateId).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}
