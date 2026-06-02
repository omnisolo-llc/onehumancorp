use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use crate::hub::Hub;

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
    pub merged_count: usize,
    pub queued_count: usize,
    pub error: Option<String>,
}

pub async fn offline_sync_handler(
    db: Arc<DB>,
    hub: Arc<Hub>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        tracing::error!("Unauthorized offline sync request. No valid tenant ID.");
        return (
            StatusCode::UNAUTHORIZED,
            Json(OfflineSyncResponse {
                success: false,
                merged_count: 0,
                queued_count: 0,
                error: Some("Unauthorized. Valid SPIFFE ID required.".to_string()),
            }),
        );
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let mut merged_count = 0;

    for mutation in &payload.mutations {
        if let Some(product_id) = mutation.get("product_id").and_then(|v| v.as_str()) {
            cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

            // Simple CRDT / Sync logic for Tap-to-Pay Offline Inventory deductions
            // Determine if the mutation is a sale (e.g. quantity reduction)
            if let Some(action) = mutation.get("action").and_then(|v| v.as_str()) {
                if action == "sale" || action == "inventory_deduction" {
                    let qty_deducted = mutation.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1);

                    tracing::info!("Syncing offline sale for product {} (tenant {}): Deducting {}", product_id, tenant_id, qty_deducted);

                    let update_query = match db.store {
                        crate::db::DbStore::Postgres => "UPDATE products SET inventory_count = GREATEST(0, inventory_count - $1) WHERE id = $2 AND tenant_id = $3",
                        crate::db::DbStore::Sqlite(_) => "UPDATE products SET inventory_count = MAX(0, inventory_count - ?) WHERE id = ? AND tenant_id = ?",
                    };

                    let result = match db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query(update_query)
                                .bind(qty_deducted as i32)
                                .bind(product_id)
                                .bind(&tenant_id)
                                .execute(&db.pool)
                                .await
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query(update_query)
                                .bind(qty_deducted as i32)
                                .bind(product_id)
                                .bind(&tenant_id)
                                .execute(&db.pool)
                                .await
                        }
                    };

                    match result {
                        Ok(res) => {
                            if res.rows_affected() > 0 {
                                merged_count += 1;

                                // Emit NATS Event Mesh message via Teammate Mesh so AI operations department can pick it up
                                let payload_json = serde_json::json!({
                                    "tenant_id": tenant_id,
                                    "product_id": product_id,
                                    "action": "offline_inventory_sync",
                                    "qty_deducted": qty_deducted
                                }).to_string();

                                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                                    agent_id: "inventory_sync_service".to_string(),
                                    action: "INVENTORY_DEDUCTED".to_string(),
                                    status: "COMPLETED".to_string(),
                                    payload: payload_json.into_bytes(),
                                    msg_id: uuid::Uuid::new_v4().to_string(),
                                };

                                let _ = hub.publish_teammate_event("operations_dept".to_string(), event);
                            } else {
                                tracing::warn!("Product {} not found or not owned by tenant {} during offline sync.", product_id, tenant_id);
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to update inventory for product {}: {}", product_id, e);
                        }
                    }
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(OfflineSyncResponse {
            success: true,
            merged_count,
            queued_count: payload.mutations.len() - merged_count,
            error: None,
        }),
    )
}
