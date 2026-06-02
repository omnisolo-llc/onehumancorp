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

use std::sync::Arc;
use uuid::Uuid;

pub async fn offline_sync_handler(
    db: Arc<crate::db::DB>,
    hub: Arc<crate::hub::Hub>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if !tenant_id.is_empty() {
        let cache = crate::builder::edge::get_edge_cache();
        cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

        if let Ok(mut tx) = db.pool.begin().await {
            let mut commit_tx = true;
            for mutation in &payload.mutations {
                if let Some(product_id) = mutation.get("product_id").and_then(|v| v.as_str()) {
                    cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
                }

                // Process inventory modifications
                let event_type = mutation.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if event_type == "inventory_toggle" || event_type == "inventory_deduction" {
                    // product ID might be named "id" or "product_id" based on the payload format
                    let mut product_id_to_update = mutation.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    if product_id_to_update.starts_with("e2e-product-") {
                        // Keep as is for E2E toggle, though we might want to extract the real ID if needed.
                        // Assuming the DB uses these exact IDs in tests.
                    } else if product_id_to_update.is_empty() {
                        product_id_to_update = mutation.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
                    }

                    if !product_id_to_update.is_empty() {
                        let quantity: i32 = mutation.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;

                        // Decrement inventory, bounded at 0
                        let res = sqlx::query(
                            "UPDATE products SET inventory_count = GREATEST(inventory_count - $1, 0) WHERE id = $2 AND tenant_id = $3"
                        )
                        .bind(quantity)
                        .bind(product_id_to_update)
                        .bind(&tenant_id)
                        .execute(&mut *tx)
                        .await;

                        if res.is_err() {
                            tracing::error!("Failed to update product inventory: {:?}", res);
                            commit_tx = false;
                            break;
                        }

                        // Write to ledger
                        let entry_id = Uuid::new_v4().to_string();
                        let ledger_res = sqlx::query(
                            "INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change)
                             VALUES ($1, $2, $3, $4, $5)"
                        )
                        .bind(&entry_id)
                        .bind(&tenant_id)
                        .bind("operations")
                        .bind("inventory_deduction")
                        .bind(mutation)
                        .execute(&mut *tx)
                        .await;

                        if ledger_res.is_err() {
                            tracing::error!("Failed to write to ledger: {:?}", ledger_res);
                            commit_tx = false;
                            break;
                        }

                        // Publish event to the mesh
                        let _ = hub.clone().publish(::server_ohc::orchestration::Message {
                            id: Uuid::new_v4().to_string(),
                            from_agent: "system".to_string(),
                            to_agent: "operations".to_string(),
                            r#type: "inventory_deduction".to_string(),
                            meeting_id: "".to_string(),
                            occurred_at_unix: chrono::Utc::now().timestamp(),
                            content: "inventory_deduction".to_string(),
                        });
                    }
                }
            }

            if commit_tx {
                let _ = tx.commit().await;
            } else {
                let _ = tx.rollback().await;
            }
        }
    }
    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use std::sync::Arc;
    use crate::db::DB;
    use crate::hub::Hub;

    #[tokio::test]
    async fn test_offline_sync_handler() {
        let db = Arc::new(DB::new().await.unwrap());
        let tenant_id = "test_tenant".to_string();

        let (event_tx, _event_rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(event_tx, db.pool.clone()));

        // Setup mock data
        sqlx::query("INSERT INTO products (id, tenant_id, title, description, price, price_cents, currency, inventory_count) VALUES ('prod1', 'test_tenant', 'A', 'B', 10.0, 1000, 'USD', 5)")
            .execute(&db.pool)
            .await
            .unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://onehumancorp.com/tenant/test_tenant/service/test".parse().unwrap());

        let mutation = serde_json::json!({
            "type": "inventory_deduction",
            "id": "prod1",
            "quantity": 2
        });

        let payload = OfflineSyncRequest {
            mutations: vec![mutation],
        };

        let response = offline_sync_handler(db.clone(), hub.clone(), headers, Json(payload)).await;

        // Verify inventory was deducted
        let remaining_inventory: i32 = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = 'prod1'")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert_eq!(remaining_inventory, 3);

        // Verify ledger was updated
        let ledger_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ohc_universal_ledger WHERE tenant_id = 'test_tenant' AND action_type = 'inventory_deduction'")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert_eq!(ledger_count, 1);
    }
}
