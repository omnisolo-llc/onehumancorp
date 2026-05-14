use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ShippoLabelPayload {
    pub order_id: String,
    pub address_to: String,
}

#[derive(Debug, Serialize)]
pub struct ShippoLabelResponse {
    pub status: String,
    pub tracking_number: String,
}

pub async fn handle_shippo_label(
    Json(payload): Json<ShippoLabelPayload>,
) -> Json<ShippoLabelResponse> {
    tracing::info!("Received Shippo label generation request: {:?}", payload);

    // In a real implementation, we would call the Shippo API to generate a label
    // and email the tracking number to the customer.

    Json(ShippoLabelResponse {
        status: "ok".to_string(),
        tracking_number: format!("shippo_mock_tracking_{}", payload.order_id),
    })
}
