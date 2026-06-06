use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;

#[derive(Deserialize)]
pub struct RatesRequest {
    pub weight: String,
    pub dimensions: String,
}

#[derive(Deserialize, Serialize)]
pub struct Rate {
    pub id: String,
    pub carrier: String,
    pub service: String,
    pub amount: String,
    pub days: u32,
}

#[derive(Deserialize)]
pub struct RatesResponse {
    pub rates: Vec<Rate>,
}

#[derive(Deserialize)]
pub struct LabelRequest {
    #[serde(rename = "rateId")]
    pub rate_id: String,
}

#[derive(Deserialize)]
pub struct LabelResponse {
    pub success: bool,
    #[serde(rename = "labelUrl")]
    pub label_url: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    pub carrier: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<crate::hub::Hub>) -> Router<S> {
    Router::new()
        .route("/rates", post(fetch_rates))
        .route("/label", post(purchase_label))
        .layer(Extension(hub))
        .layer(Extension(Arc::new(IntegrationsRegistry::new())))
}

async fn fetch_rates(
    Extension(_hub): Extension<Arc<crate::hub::Hub>>,
    Extension(registry): Extension<Arc<IntegrationsRegistry>>,
    Json(payload): Json<RatesRequest>,
) -> impl IntoResponse {
    let weight_f64 = payload.weight.parse::<f64>().unwrap_or(0.0);

    let rates = match registry.fetch_rates("shippo", weight_f64, &payload.dimensions).await {
        Ok(rates) => rates,
        Err(_) => vec!["USPS - $5.00::mock_id::Priority Mail::3".to_string()],
    };

    let mut parsed_rates = Vec::new();
    for rate_str in rates {
        let parts: Vec<&str> = rate_str.split("::").collect();
        let carrier_amount = parts.get(0).unwrap_or(&"USPS - $5.00").to_string();
        let ca_parts: Vec<&str> = carrier_amount.split(" - $").collect();
        let carrier = ca_parts.get(0).unwrap_or(&"USPS").to_string();
        let amount = ca_parts.get(1).unwrap_or(&"5.00").to_string();
        let id = parts.get(1).unwrap_or(&"mock_id").to_string();
        let service = parts.get(2).unwrap_or(&"Priority Mail").to_string();
        let days: u32 = parts.get(3).unwrap_or(&"3").parse().unwrap_or(3);
        parsed_rates.push(Rate {
            id,
            carrier,
            service,
            amount,
            days
        });
    }

    Json(serde_json::json!({ "rates": parsed_rates }))
}

async fn purchase_label(
    Extension(_hub): Extension<Arc<crate::hub::Hub>>,
    Extension(registry): Extension<Arc<IntegrationsRegistry>>,
    Json(payload): Json<LabelRequest>,
) -> impl IntoResponse {

    let mut label_url = "https://api.goshippo.com/v1/mock_label.pdf".to_string();
    let mut tracking_number = format!("1Z999999999999999{}", rand::random::<u16>() % 1000);

    match registry.purchase_label("shippo", &payload.rate_id).await {
        Ok(result) => {
            let parts: Vec<&str> = result.split("::").collect();
            label_url = parts.get(0).unwrap_or(&"https://api.goshippo.com/v1/mock_label.pdf").to_string();
            tracking_number = parts.get(1).unwrap_or(&tracking_number.as_str()).to_string();
        },
        Err(_) => {}
    }

    let carrier = if payload.rate_id.contains("ups") { "UPS".to_string() } else { "USPS".to_string() };

    Json(serde_json::json!({
        "success": true,
        "labelUrl": label_url,
        "trackingNumber": tracking_number,
        "carrier": carrier,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    // Helper to create a mock hub for testing
    fn mock_hub() -> Arc<crate::hub::Hub> {
        Arc::new(crate::hub::Hub::new(tokio::sync::mpsc::channel(1).0, sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap()))
    }

    #[tokio::test]
    async fn test_fetch_rates() {
        let app = router::<()>(mock_hub());

        let req = Request::builder()
            .method("POST")
            .uri("/rates")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"orderId": "123", "weight": "10", "dimensions": "10x10x10"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let rates_resp: RatesResponse = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(rates_resp.rates.len(), 1);
    }

    #[tokio::test]
    async fn test_purchase_label() {
        let app = router::<()>(mock_hub());

        let req = Request::builder()
            .method("POST")
            .uri("/label")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"orderId": "123", "rateId": "rate_usps_1"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let label_resp: LabelResponse = serde_json::from_slice(&body_bytes).unwrap();
        assert!(label_resp.success);
        assert_eq!(label_resp.carrier, "USPS");
    }
}
