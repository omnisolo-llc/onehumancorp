use axum::{
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct FetchRatesRequest {
    pub orderId: String,
    pub weight: String,
    pub dimensions: String,
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
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    let weight_f64 = payload.weight.parse::<f64>().unwrap_or(1.0);

    let registry = crate::integrations::registry::IntegrationsRegistry::new();

    match registry.fetch_rates("shippo", weight_f64, &payload.dimensions).await {
        Ok(rates) => (StatusCode::OK, Json(RatesResponse { rates })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn purchase_label(
    Json(payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    let registry = crate::integrations::registry::IntegrationsRegistry::new();


    let address_to = match std::env::var("SHIPPO_ADDRESS_TO_JSON") {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or(serde_json::json!({})),
        Err(_) => serde_json::json!({}),
    };

    let validation_result = registry.validate_address("shippo", &address_to).await;
    if let Ok(validation) = validation_result {
        if !validation.is_valid {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Address validation failed",
                    "messages": validation.messages,
                })),
            )
                .into_response();
        }
    }

    match registry.purchase_label("shippo", &payload.rateId).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}
