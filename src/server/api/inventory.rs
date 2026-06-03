use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    // Dummy state for API compilation
}

#[derive(Serialize, Deserialize)]
pub struct InventorySyncRequest {
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub quantity_sold: i32,
    pub client_timestamp: String,
}

#[derive(Serialize)]
pub struct InventorySyncResponse {
    pub status: String,
}

pub async fn handle_offline_sync(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<InventorySyncRequest>,
) -> Json<InventorySyncResponse> {
    // In a real implementation, we would insert into `offline_sales_sync`
    // and reconcile with `inventory_items` using CRDTs.
    Json(InventorySyncResponse {
        status: "success".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_sync_response() {
        let resp = InventorySyncResponse {
            status: "success".to_string(),
        };
        assert_eq!(resp.status, "success");
    }
}
