use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::inventory::ledger::InventoryLedgerService;

#[derive(Deserialize, Debug, Clone)]
pub struct InventorySyncRequest {
    pub tenant_id: String,
    pub updates: Vec<InventoryUpdateItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InventoryUpdateItem {
    pub product_id: String,
    pub variant_id: Option<String>,
    pub delta: i32,
}

#[derive(Serialize)]
pub struct InventorySyncResponse {
    pub status: String,
    pub updated_count: usize,
}

pub async fn sync_inventory_handler(
    State(service): State<Arc<InventoryLedgerService>>,
    Json(payload): Json<InventorySyncRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut updated = 0;

    for update in &payload.updates {
        match service.apply_optimistic_update(&payload.tenant_id, &update.product_id, update.variant_id.clone(), update.delta).await {
            Ok(_) => updated += 1,
            Err(e) => {
                tracing::error!("Failed to apply optimistic update for product {}: {}", update.product_id, e);
                // Continue with other updates in batch even if one fails
            }
        }
    }

    Ok((
        StatusCode::OK,
        Json(InventorySyncResponse {
            status: "queued".to_string(),
            updated_count: updated,
        }),
    ))
}
