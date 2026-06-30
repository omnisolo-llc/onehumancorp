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
        .route("/webhook", post(shippo_webhook))
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

    match registry.purchase_label("shippo", &payload.rateId).await {
        Ok(response) => {
            // After successful label purchase, automatically draft a tracking update notification via AgentFeed
            let _ = crate::services::agent_feed::service::AgentFeedService::new(crate::db::get_pool())
                .process_event("default", "shipping_system", &serde_json::json!({
                    "event": "label_purchased",
                    "orderId": payload.orderId,
                    "trackingNumber": response.tracking_number,
                    "trackingUrl": response.label_url,
                    "carrier": response.carrier,
                })).await;

            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn shippo_webhook(
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Listen for tracking updates and create a feed item if there's a transit anomaly
    if let Some(event) = payload.get("event").and_then(|v| v.as_str()) {
        if event == "track_updated" {
            let status = payload.get("data").and_then(|d| d.get("tracking_status")).and_then(|s| s.get("status")).and_then(|s| s.as_str()).unwrap_or("");
            if status == "FAILURE" || status == "RETURNED" || status == "UNKNOWN" {
                let _ = crate::services::agent_feed::service::AgentFeedService::new(crate::db::get_pool())
                    .process_event("default", "shippo_webhook", &serde_json::json!({
                        "event": "transit_anomaly",
                        "status": status,
                        "raw_payload": payload,
                    })).await;
            }
        }
    }

    (StatusCode::OK, "OK").into_response()
}
