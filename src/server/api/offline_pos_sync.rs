use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct OfflinePosSyncRequest {
    pub mutations: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct OfflinePosSyncResponse {
    pub success: bool,
    pub processed: usize,
}

pub async fn offline_pos_sync_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflinePosSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline pos mutations for sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        tracing::warn!("Unauthenticated offline POS sync attempt");
        return (
            StatusCode::UNAUTHORIZED,
            Json(OfflinePosSyncResponse { success: false, processed: 0 }),
        );
    }

    let pool = crate::db::get_pool();

    match crate::services::sync::offline_pos::process_offline_pos_queue(&pool, &tenant_id, &payload.mutations).await {
        Ok(processed) => {
            tracing::info!("Successfully processed {} offline pos mutations for tenant {}", processed, tenant_id);
            (
                StatusCode::OK,
                Json(OfflinePosSyncResponse { success: true, processed }),
            )
        }
        Err(e) => {
            tracing::error!("Failed to process offline pos queue: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(OfflinePosSyncResponse { success: false, processed: 0 }),
            )
        }
    }
}
