use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
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
        // Idempotency check: insert into synced_transactions
        let mut tx = match db.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to start transaction: {}", e);
                continue;
            }
        };

        // Try to insert the transaction
        let insert_query = "
            INSERT INTO synced_transactions (tenant_id, transaction_id, product_id, quantity_deducted, timestamp)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, transaction_id) DO NOTHING
            RETURNING id
        ";

        let ts: Option<chrono::DateTime<chrono::Utc>> = mutation.timestamp.as_ref().and_then(|ts| ts.parse().ok());

        let insert_result = sqlx::query(insert_query)
            .bind(&tenant_id)
            .bind(&mutation.transaction_id)
            .bind(&mutation.product_id)
            .bind(mutation.quantity_deducted)
            .bind(ts)
            .fetch_optional(&mut *tx)
            .await;

        match insert_result {
            Ok(Some(_)) => {
                // Successfully inserted, meaning this is a new transaction
            }
            Ok(None) => {
                // Conflict occurred, transaction already processed
                tracing::info!("Transaction {} already processed for tenant {}", mutation.transaction_id, tenant_id);
                let _ = tx.commit().await;
                continue;
            }
            Err(e) => {
                tracing::error!("Failed to insert into synced_transactions for tx {}: {}", mutation.transaction_id, e);
                let _ = tx.rollback().await;
                continue;
            }
        }

        cache.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

        let query = "
            UPDATE products
            SET inventory_count = inventory_count - $1
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, inventory_count
        ";

        let result = sqlx::query_as::<_, (uuid::Uuid, i32)>(query)
            .bind(mutation.quantity_deducted)
            .bind(&mutation.product_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await;

        match result {
            Ok(Some((_, new_inventory_count))) => {
                if let Err(e) = tx.commit().await {
                    tracing::error!("Failed to commit transaction for product {}: {}", mutation.product_id, e);
                    continue;
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
                        "quantity_deducted": mutation.quantity_deducted,
                        "tenant_id": tenant_id
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish("mesh:inventory:updated", event).await;

                // Check for oversell anomaly
                if new_inventory_count < 0 {
                    tracing::warn!("Oversell detected for product {}! New inventory: {}", mutation.product_id, new_inventory_count);
                    let anomaly_event = ::server_ohc::orchestration::TeammateMeshEvent {
                        action: "InventoryAnomaly".to_string(),
                        agent_id: "system".to_string(),
                        status: "Oversell".to_string(),
                        msg_id: uuid::Uuid::new_v4().to_string(),
                        payload: serde_json::json!({
                            "product_id": mutation.product_id,
                            "transaction_id": mutation.transaction_id,
                            "quantity_deducted": mutation.quantity_deducted,
                            "current_inventory": new_inventory_count,
                            "tenant_id": tenant_id
                        }).to_string().into_bytes(),
                    };
                    let _ = mesh.publish("mesh:inventory:anomaly", anomaly_event).await;
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
    use std::sync::atomic::{AtomicUsize, Ordering};


    // A mock transport that counts published anomalies
    struct MockMeshTransport {
        inner: InProcessTransport,
        anomaly_count: Arc<AtomicUsize>,
    }

    impl MockMeshTransport {
        fn new() -> Self {
            Self {
                inner: InProcessTransport::new(),
                anomaly_count: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn get_anomaly_count(&self) -> usize {
            self.anomaly_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait::async_trait]
    impl MeshTransport for MockMeshTransport {
        async fn publish(&self, topic: &str, event: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
            if topic == "mesh:inventory:anomaly" {
                self.anomaly_count.fetch_add(1, Ordering::SeqCst);
            }
            self.inner.publish(topic, event).await
        }

        async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(::server_ohc::orchestration::TeammateMeshEvent) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            self.inner.subscribe(topic, handler).await
        }

        async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
            self.inner.acquire_lock(resource, owner, ttl_seconds).await
        }

        async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
            self.inner.release_lock(resource, owner).await
        }

        async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
            self.inner.register_presence(agent_id, status, ttl_seconds).await
        }

        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
            self.inner.get_active_agents().await
        }
    }

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
        sqlx::query("CREATE TABLE IF NOT EXISTS synced_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id VARCHAR NOT NULL,
            transaction_id VARCHAR NOT NULL,
            product_id VARCHAR NOT NULL,
            quantity_deducted INT NOT NULL,
            timestamp TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (tenant_id, transaction_id)
        )").execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline', 'Offline Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-offline-1', 'tenant-offline', 'Test Prod', 5) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let mesh = Arc::new(MockMeshTransport::new());
        let state = State((pool.clone(), mesh.clone() as Arc<dyn MeshTransport>));

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
        assert_eq!(mesh.get_anomaly_count(), 0);

        // Test negative inventory and anomaly publishing
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
        assert_eq!(mesh.get_anomaly_count(), 1);
    }

    #[tokio::test]
    async fn test_offline_sync_idempotency() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("CREATE TABLE IF NOT EXISTS synced_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tenant_id VARCHAR NOT NULL,
            transaction_id VARCHAR NOT NULL,
            product_id VARCHAR NOT NULL,
            quantity_deducted INT NOT NULL,
            timestamp TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (tenant_id, transaction_id)
        )").execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline-idem', 'Offline Idem Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-idem-1', 'tenant-offline-idem', 'Test Prod Idem', 10) ON CONFLICT (id) DO UPDATE SET inventory_count = 10")
            .execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));



        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline-idem/agent/x".parse().unwrap());

        // First call
        let response1 = offline_sync_handler(state.clone(), headers.clone(), Json(OfflineSyncRequest {
            mutations: vec![OfflineMutation {
                transaction_id: "tx-idem-1".to_string(),
                product_id: "prod-idem-1".to_string(),
                quantity_deducted: 4,
                timestamp: None,
            }],
        })).await.into_response();
        assert_eq!(response1.status(), StatusCode::OK);

        // Second call with same tx
        let response2 = offline_sync_handler(state.clone(), headers.clone(), Json(OfflineSyncRequest {
            mutations: vec![OfflineMutation {
                transaction_id: "tx-idem-1".to_string(),
                product_id: "prod-idem-1".to_string(),
                quantity_deducted: 4,
                timestamp: None,
            }],
        })).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-idem-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 6); // 10 - 4 = 6. Should not deduct twice!
    }
}
