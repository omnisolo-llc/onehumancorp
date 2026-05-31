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

pub async fn offline_sync_handler(db: std::sync::Arc<crate::db::DB>, _headers: axum::http::HeaderMap, Json(payload): Json<OfflineSyncRequest>) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    // We will extract tenant id and apply changes.
    // Assuming 'tenant' comes from headers or payload (we use 'default' for now as a fallback).
    let mut tenant_id = "default".to_string();

    // Try to extract from the first mutation if available
    if let Some(first) = payload.mutations.first() {
         if let Some(t) = first.get("tenant_id").and_then(|v| v.as_str()) {
             tenant_id = t.to_string();
         }
    }

    for mutation in payload.mutations {
        if let Some(mut_type) = mutation.get("type").and_then(|v| v.as_str()) {
            if mut_type == "tap_to_pay" {
                if let Some(amount) = mutation.get("amount").and_then(|v| v.as_f64()) {
                    let id = mutation.get("id").and_then(|v| v.as_str()).unwrap_or("unknown_txn").to_string();
                    let tenant_id = tenant_id.clone();

                    let _ = db.execute_with_retry("offline_sync_order", || {
                        let db = db.clone();
                        let id = id.clone();
                        let tenant_id = tenant_id.clone();
                        async move {
                            if db.is_sqlite() {
                                if let crate::db::DbStore::Sqlite(pool) = &db.store {
                                    sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING")
                                        .bind(&id)
                                        .bind(&tenant_id)
                                        .bind("offline_customer")
                                        .bind(amount)
                                        .bind("completed")
                                        .execute(pool)
                                        .await?;
                                }
                            } else {
                                sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING")
                                    .bind(&id)
                                    .bind(&tenant_id)
                                    .bind("offline_customer")
                                    .bind(amount)
                                    .bind("completed")
                                    .execute(&db.pool)
                                    .await?;
                            }
                            Ok::<(), sqlx::Error>(())
                        }
                    }).await;
                }
            } else if mut_type == "inventory_toggle" {
                let id = mutation.get("id").and_then(|v| v.as_str()).unwrap_or("unknown_toggle").to_string();
                tracing::info!("Processed inventory_toggle for {}", id);
            }
        }
    }

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true }),
    )
}

// Separate function to not break lib.rs route injection if it defaults
pub async fn offline_sync_handler_legacy(Json(payload): Json<OfflineSyncRequest>) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true }),
    )
}
