use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub timestamp: i64,
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

    let mut has_error = false;

    for mutation in &payload.mutations {
        cache.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

        let mut tx = match db.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                has_error = true;
                continue;
            }
        };

        // Try to insert the transaction into the synced_transactions table to prevent double-counting
        let insert_query = "
            INSERT INTO synced_transactions (id, tenant_id, product_id, timestamp)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO NOTHING
        ";
        let insert_result = sqlx::query(insert_query)
            .bind(&mutation.transaction_id)
            .bind(&tenant_id)
            .bind(&mutation.product_id)
            .bind(mutation.timestamp)
            .execute(&mut *tx)
            .await;

        if let Ok(result) = insert_result {
            if result.rows_affected() == 0 {
                tracing::info!("Transaction {} already processed, skipping.", mutation.transaction_id);
                let _ = tx.rollback().await;
                continue;
            }
        } else {
            tracing::error!("Failed to insert transaction {}: {:?}", mutation.transaction_id, insert_result);
            has_error = true;
            let _ = tx.rollback().await;
            continue;
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

        match result {
            Ok(Some((new_count,))) => {
                if let Err(e) = tx.commit().await {
                    tracing::error!("Failed to commit transaction for mutation {}: {}", mutation.transaction_id, e);
                    has_error = true;
                    continue;
                }

                let action_name = if new_count < 0 { "InventoryAnomaly" } else { "InventoryUpdated" };
                let topic_name = if new_count < 0 { "mesh:inventory:anomaly" } else { "mesh:inventory:updated" };

                // Publish mesh event
                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    action: action_name.to_string(),
                    agent_id: "system".to_string(),
                    status: "".to_string(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                    payload: serde_json::json!({
                        "product_id": mutation.product_id,
                        "transaction_id": mutation.transaction_id,
                        "quantity_deducted": mutation.quantity_deducted,
                        "inventory_count": new_count,
                        "tenant_id": tenant_id
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish(topic_name, event).await;

                if new_count < 0 {
                    tracing::warn!("Oversell anomaly detected for product {}: inventory count is {}", mutation.product_id, new_count);
                }
            }
            Ok(None) => {
                tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id);
                let _ = tx.rollback().await;
            }
            Err(e) => {
                tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                has_error = true;
                let _ = tx.rollback().await;
            }
        }
    }

    if has_error {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(OfflineSyncResponse { success: false }),
        ).into_response();
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
        sqlx::query("CREATE TABLE IF NOT EXISTS synced_transactions (id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, timestamp BIGINT)").execute(&pool).await.unwrap();
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
                    quantity_deducted: 3, timestamp: 1672531200,
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
                    quantity_deducted: 10, timestamp: 1672531201,
                },
            ],
        };

        let response2 = offline_sync_handler(state, headers, Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, -8); // Allowed to drop below zero
    }

    #[tokio::test]
    async fn test_offline_sync_duplicate_mutation_ignored() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("CREATE TABLE IF NOT EXISTS synced_transactions (id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, timestamp BIGINT)").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline-dup', 'Offline Test Tenant Dup') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-offline-dup-1', 'tenant-offline-dup', 'Test Prod Dup', 10) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        let req = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx-dup-1".to_string(),
                    product_id: "prod-offline-dup-1".to_string(),
                    quantity_deducted: 2,
                    timestamp: 1672531200,
                },
                OfflineMutation {
                    transaction_id: "tx-dup-1".to_string(),
                    product_id: "prod-offline-dup-1".to_string(),
                    quantity_deducted: 2,
                    timestamp: 1672531200,
                },
            ],
        };

        let mut headers = axum::http::HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline-dup/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-dup-1'")
            .fetch_one(&pool).await.unwrap();
        // Since it's duplicate, it should only be deducted once: 10 - 2 = 8.
        assert_eq!(count.0, 8);
    }
}
