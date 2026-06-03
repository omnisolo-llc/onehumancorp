use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
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
                tracing::error!("Failed to begin transaction: {}", e);
                continue;
            }
        };

        let insert_tx_query = "
            INSERT INTO synced_transactions (tenant_id, transaction_id, product_id, timestamp)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (tenant_id, transaction_id) DO NOTHING
            RETURNING id
        ";

        let tx_result = sqlx::query(insert_tx_query)
            .bind(&tenant_id)
            .bind(&mutation.transaction_id)
            .bind(&mutation.product_id)
            .bind(mutation.timestamp.unwrap_or_else(|| chrono::Utc::now()))
            .fetch_optional(&mut *tx)
            .await;

        match tx_result {
            Ok(Some(_)) => {
                // Successfully recorded unique transaction
            }
            Ok(None) => {
                tracing::info!("Transaction {} already synced for tenant {}, skipping.", mutation.transaction_id, tenant_id);
                continue;
            }
            Err(e) => {
                tracing::error!("Failed to record transaction {}: {}", mutation.transaction_id, e);
                continue;
            }
        }

        let query = "
            UPDATE products
            SET inventory_count = inventory_count - $1
            WHERE id = $2 AND tenant_id = $3
            RETURNING inventory_count
        ";

        let result = sqlx::query_as::<_, (i32,)>(query)
            .bind(mutation.quantity_deducted)
            .bind(&mutation.product_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await;

        let commit_result = match result {
            Ok(Some((new_count,))) => {
                tx.commit().await.map(|_| Some(new_count))
            }
            Ok(None) => {
                let _ = tx.rollback().await;
                Ok(None)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(e)
            }
        };

        match commit_result {
            Ok(Some(new_count)) => {
                if new_count < 0 {
                    tracing::warn!("Inventory anomaly: Product {} dropped below 0 ({})", mutation.product_id, new_count);
                    let event = ::server_ohc::orchestration::TeammateMeshEvent {
                        action: "InventoryAnomaly".to_string(),
                        agent_id: "system".to_string(),
                        status: "".to_string(),
                        msg_id: uuid::Uuid::new_v4().to_string(),
                        payload: serde_json::json!({
                            "product_id": mutation.product_id,
                            "transaction_id": mutation.transaction_id,
                            "quantity_deducted": mutation.quantity_deducted,
                            "resulting_inventory": new_count,
                            "tenant_id": tenant_id
                        }).to_string().into_bytes(),
                    };
                    let _ = mesh.publish("mesh:inventory:anomaly", event).await;
                } else {
                    let event = ::server_ohc::orchestration::TeammateMeshEvent {
                        action: "InventoryUpdated".to_string(),
                        agent_id: "system".to_string(),
                        status: "".to_string(),
                        msg_id: uuid::Uuid::new_v4().to_string(),
                        payload: serde_json::json!({
                            "product_id": mutation.product_id,
                            "transaction_id": mutation.transaction_id,
                            "quantity_deducted": mutation.quantity_deducted,
                            "tenant_id": tenant_id
                        }).to_string().into_bytes(),
                    };
                    let _ = mesh.publish("mesh:inventory:updated", event).await;
                }
            }
            Ok(None) => {
                tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id);
            }
            Err(e) => {
                tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
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

        // Ensure synced_transactions table exists for the test
        sqlx::query("
            CREATE TABLE IF NOT EXISTS synced_transactions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                tenant_id VARCHAR NOT NULL,
                transaction_id VARCHAR NOT NULL,
                product_id VARCHAR NOT NULL,
                timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(tenant_id, transaction_id)
            )
        ").execute(&pool).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline', 'Offline Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-offline-1', 'tenant-offline', 'Test Prod', 5) ON CONFLICT DO NOTHING")
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

        // Test idempotency: submitting the exact same transaction should NOT deduct inventory again.
        let req_idempotent = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx1".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 3,
                    timestamp: None,
                },
            ],
        };

        let response_idempotent = offline_sync_handler(state.clone(), headers.clone(), Json(req_idempotent)).await.into_response();
        assert_eq!(response_idempotent.status(), StatusCode::OK);

        let count_idempotent: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count_idempotent.0, 2); // Inventory remains 2, not deducted again.

        // Test negative guard: dropping below zero should now be allowed to drop to -8 and trigger an anomaly.
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

        let response2 = offline_sync_handler(state, headers, Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, -8); // 2 - 10 = -8
    }
}
