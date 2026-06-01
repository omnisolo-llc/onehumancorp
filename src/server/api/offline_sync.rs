use axum::{
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<Value>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
    pub synced_count: usize,
}

pub async fn offline_sync_handler(
    db: std::sync::Arc<crate::db::DB>,
    headers: HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    let mut tenant_id = "system".to_string();
    if let Some(tid) = headers.get("X-Tenant-ID") {
        if let Ok(tid_str) = tid.to_str() {
            tenant_id = tid_str.to_string();
        }
    }

    let mut synced_count = 0;

    for mutation in &payload.mutations {
        if let Some(mutation_type) = mutation.get("type").and_then(|v| v.as_str()) {
            if mutation_type == "pos_charge" {
                if let (Some(id), Some(amount)) = (
                    mutation.get("id").and_then(|v| v.as_str()),
                    mutation.get("amount").and_then(|v| v.as_f64()),
                ) {
                    let status = "completed"; // POS transactions are assumed successful offline

                    // Use idempotency logic to prevent double charging
                    match crate::db::DbStore::Postgres {
                        _ => {
                            let result = sqlx::query(
                                "INSERT INTO orders (id, tenant_id, total_amount, status) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING"
                            )
                            .bind(id)
                            .bind(&tenant_id)
                            .bind(amount)
                            .bind(status)
                            .execute(&db.pool)
                            .await;

                            if result.is_ok() {
                                synced_count += 1;
                            } else {
                                tracing::error!("Failed to sync POS transaction: {:?}", result.err());
                            }
                        }
                    }
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true, synced_count }),
    )
}
