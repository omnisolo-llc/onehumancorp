use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use server_ohc::orchestration::MeshEvent;

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryRequest {
    pub tenant_id: String,
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct UpdateInventoryResponse {
    pub success: bool,
}

pub async fn update_inventory(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<UpdateInventoryRequest>,
) -> axum::response::Result<Json<UpdateInventoryResponse>> {

    // Simulate updating DB

    // Publish inventory status change event to mesh
    let event = MeshEvent {
        event_id: uuid::Uuid::new_v4().to_string(),
        topic: "mesh:inventory:status_changed".to_string(),
        payload: serde_json::to_vec(&serde_json::json!({
            "tenant_id": payload.tenant_id,
            "product_id": payload.product_id,
            "quantity": payload.quantity,
        })).unwrap_or_default(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    if let Err(e) = hub.publish_mesh_event(event) {
        tracing::error!("Failed to publish inventory status changed event: {}", e);
    }

    Ok(Json(UpdateInventoryResponse { success: true }))
}
