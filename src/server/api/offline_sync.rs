use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
}

pub async fn offline_sync_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if !tenant_id.is_empty() {
        let cache = crate::builder::edge::get_edge_cache();
        cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

        for mutation in &payload.mutations {
            if let Some(product_id) = mutation.get("product_id").and_then(|v| v.as_str()) {
                cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
            }
        }
    }
    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true }),
    )
}
