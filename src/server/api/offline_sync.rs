use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub timestamp: String,
    pub product_id: String,
    pub quantity_deducted: i32,
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

        // Insert into ledger to prevent double processing of same transaction_id
        let insert_ledger = "
            INSERT INTO offline_sync_ledger (transaction_id, tenant_id, product_id, quantity_deducted)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (transaction_id) DO NOTHING
            RETURNING transaction_id
        ";
        let ledger_result = sqlx::query(insert_ledger)
            .bind(&mutation.transaction_id)
            .bind(&tenant_id)
            .bind(&mutation.product_id)
            .bind(mutation.quantity_deducted)
            .fetch_optional(&db)
            .await;

        match ledger_result {
            Ok(Some(_)) => {
                // Successfully inserted, proceed
            }
            Ok(None) => {
                tracing::info!("Transaction {} already processed for tenant {}", mutation.transaction_id, tenant_id);
                continue;
            }
            Err(e) => {
                tracing::error!("Database error checking ledger for transaction {}: {}", mutation.transaction_id, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(OfflineSyncResponse { success: false }),
                ).into_response();
            }
        }

        let query = "
            UPDATE products
            SET inventory_count = inventory_count - $1
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, inventory_count
        ";

        let result: Result<Option<(String, i32)>, sqlx::Error> = sqlx::query_as(query)
            .bind(mutation.quantity_deducted)
            .bind(&mutation.product_id)
            .bind(&tenant_id)
            .fetch_optional(&db)
            .await;

        match result {
            Ok(Some((_, new_inventory))) => {
                if new_inventory < 0 {
                    // Anomaly detected
                    let anomaly_event = ::server_ohc::orchestration::TeammateMeshEvent {
                        action: "InventoryAnomaly".to_string(),
                        agent_id: "system".to_string(),
                        status: "".to_string(),
                        msg_id: uuid::Uuid::new_v4().to_string(),
                        payload: serde_json::json!({
                            "product_id": mutation.product_id,
                            "transaction_id": mutation.transaction_id,
                            "quantity_deducted": mutation.quantity_deducted,
                            "new_inventory": new_inventory,
                            "timestamp": mutation.timestamp,
                            "tenant_id": tenant_id
                        }).to_string().into_bytes(),
                    };
                    let _ = mesh.publish("mesh:inventory:anomaly", anomaly_event).await;
                }

                // Publish mesh event
                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    action: "InventoryUpdated".to_string(),
                    agent_id: "system".to_string(),
                    status: "".to_string(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                    payload: serde_json::json!({
                        "product_id": mutation.product_id,
                        "transaction_id": mutation.transaction_id,
                        "timestamp": mutation.timestamp,
                        "quantity_deducted": mutation.quantity_deducted,
                        "tenant_id": tenant_id
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish("mesh:inventory:updated", event).await;
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

        // Setup test data
        sqlx::query("CREATE TABLE IF NOT EXISTS offline_sync_ledger (transaction_id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, quantity_deducted INT);")
            .execute(&pool).await.unwrap();
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
                    timestamp: "2024-01-01T00:00:00Z".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 3,
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

        let req_over = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx2".to_string(),
                    timestamp: "2024-01-01T00:00:00Z".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 10,
                },
            ],
        };

        let response2 = offline_sync_handler(state.clone(), headers.clone(), Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, -8); // 2 - 10 = -8

        // Test idempotency
        let req_idempotent = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx2".to_string(),
                    timestamp: "2024-01-01T00:00:00Z".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 5,
                },
            ],
        };

        let response_idempotent = offline_sync_handler(state, headers, Json(req_idempotent)).await.into_response();
        assert_eq!(response_idempotent.status(), StatusCode::OK);

        let count_idempotent: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count_idempotent.0, -8); // unchanged since tx2 already processed

    }
}
