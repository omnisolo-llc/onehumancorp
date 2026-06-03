use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    #[serde(default)]
    pub timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<OfflineMutation>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
}

pub async fn offline_sync_handler(
    State((db, mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(OfflineSyncResponse { success: false }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    for mutation in &payload.mutations {
        cache.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

        let mut tx = match db.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction for offline sync: {}", e);
                continue;
            }
        };

        let parsed_timestamp: Option<DateTime<Utc>> = mutation.timestamp.as_ref()
            .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
            .map(|t| t.with_timezone(&Utc));

        let insert_ledger_query = "
            INSERT INTO synced_transactions (transaction_id, tenant_id, product_id, quantity_deducted, timestamp)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (transaction_id) DO NOTHING
            RETURNING transaction_id
        ";

        let ledger_result = sqlx::query(insert_ledger_query)
            .bind(&mutation.transaction_id)
            .bind(&tenant_id)
            .bind(&mutation.product_id)
            .bind(mutation.quantity_deducted)
            .bind(parsed_timestamp)
            .fetch_optional(&mut *tx)
            .await;

        match ledger_result {
            Ok(Some(_)) => {}
            Ok(None) => {
                tracing::info!("Transaction {} already processed, skipping.", mutation.transaction_id);
                let _ = tx.commit().await;
                continue;
            }
            Err(e) => {
                tracing::error!("Failed to insert into synced_transactions ledger for {}: {}", mutation.transaction_id, e);
                let _ = tx.rollback().await;
                continue;
            }
        }

        let update_query = "
            UPDATE products
            SET inventory_count = inventory_count - $1
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, inventory_count
        ";

        let update_result = sqlx::query_as::<_, (String, i32)>(update_query)
            .bind(mutation.quantity_deducted)
            .bind(&mutation.product_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await;

        match update_result {
            Ok(Some((_, new_inventory))) => {
                let (action, event_action) = if new_inventory < 0 {
                    ("mesh:inventory:anomaly", "AnomalyDetected")
                } else {
                    ("mesh:inventory:updated", "InventoryUpdated")
                };

                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    action: event_action.to_string(),
                    agent_id: "system".to_string(),
                    status: "".to_string(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                    payload: serde_json::json!({
                        "product_id": mutation.product_id,
                        "transaction_id": mutation.transaction_id,
                        "quantity_deducted": mutation.quantity_deducted,
                        "tenant_id": tenant_id,
                        "new_inventory": new_inventory,
                        "timestamp": mutation.timestamp
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish(action, event).await;

                if let Err(e) = tx.commit().await {
                    tracing::error!("Failed to commit transaction for offline sync {}: {}", mutation.transaction_id, e);
                }
            }
            Ok(None) => {
                tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id);
                let _ = tx.rollback().await;
            }
            Err(e) => {
                tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                let _ = tx.rollback().await;
            }
        }
    }

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true }),
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use ohc_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport};
    use sqlx::postgres::PgPoolOptions;


    #[tokio::test]
    async fn test_offline_sync_unauthorized() {
        let pool = PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool, mesh));

        let req = OfflineSyncRequest { mutations: vec![] };
        let headers = HeaderMap::new();

        let response = offline_sync_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_offline_sync_success_and_negative_guard() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline', 'Offline Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-offline-1', 'tenant-offline', 'Test Prod', 5) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS synced_transactions (transaction_id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, product_id TEXT NOT NULL, quantity_deducted INT NOT NULL, timestamp TIMESTAMPTZ, synced_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        sqlx::query("DELETE FROM synced_transactions WHERE tenant_id = 'tenant-offline'")
            .execute(&pool).await.unwrap();

        // Reset inventory to 5 in case the test has already run
        sqlx::query("UPDATE products SET inventory_count = 5 WHERE id = 'prod-offline-1' AND tenant_id = 'tenant-offline'")
            .execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        let req = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx1".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 3,
                    timestamp: None,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 2); // 5 - 3 = 2

        // Test negative guard
        let req_over = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx2".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 10,
                    timestamp: None,
                },
            ],
        };

        let response2 = offline_sync_handler(state.clone(), headers.clone(), Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, -8); // 2 - 10 = -8

        // Restore for test idempotency
        sqlx::query("DELETE FROM synced_transactions WHERE tenant_id = 'tenant-offline'")
            .execute(&pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_offline_sync_idempotency() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline-idem', 'Offline Test Tenant Idem') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-offline-idem', 'tenant-offline-idem', 'Test Prod', 5) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS synced_transactions (transaction_id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, product_id TEXT NOT NULL, quantity_deducted INT NOT NULL, timestamp TIMESTAMPTZ, synced_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        sqlx::query("DELETE FROM synced_transactions WHERE tenant_id = 'tenant-offline-idem'")
            .execute(&pool).await.unwrap();

        sqlx::query("UPDATE products SET inventory_count = 5 WHERE id = 'prod-offline-idem' AND tenant_id = 'tenant-offline-idem'")
            .execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        let req = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx-idem-1".to_string(),
                    product_id: "prod-offline-idem".to_string(),
                    quantity_deducted: 3,
                    timestamp: None,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline-idem/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-idem'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 2); // 5 - 3 = 2

        // Send same transaction id again
        let req2 = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx-idem-1".to_string(),
                    product_id: "prod-offline-idem".to_string(),
                    quantity_deducted: 3,
                    timestamp: None,
                },
            ],
        };

        let response2 = offline_sync_handler(state, headers, Json(req2)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-idem'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, 2); // Should still be 2, not deducted again

        // Clean up
        sqlx::query("DELETE FROM synced_transactions WHERE tenant_id = 'tenant-offline-idem'")
            .execute(&pool).await.unwrap();
    }
}
