use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
#[derive(Clone)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
#[derive(Clone)]
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
                tracing::error!("Failed to begin transaction for mutation {}: {}", mutation.transaction_id, e);
                continue;
            }
        };

        // Insert into offline_sync_transactions to ensure idempotency
        let insert_ledger = "
            INSERT INTO offline_sync_transactions (transaction_id, tenant_id, product_id, quantity_deducted)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (transaction_id) DO NOTHING
            RETURNING transaction_id
        ";

        let ledger_result = sqlx::query(insert_ledger)
            .bind(&mutation.transaction_id)
            .bind(&tenant_id)
            .bind(&mutation.product_id)
            .bind(mutation.quantity_deducted)
            .fetch_optional(&mut *tx)
            .await;

        if let Ok(Some(_)) = ledger_result {
            let query = "
                UPDATE products
                SET inventory_count = inventory_count - $1
                WHERE id = $2 AND tenant_id = $3
                RETURNING id, inventory_count
            ";

            let result = sqlx::query_as::<_, (String, i32)>(query)
                .bind(mutation.quantity_deducted)
                .bind(&mutation.product_id)
                .bind(&tenant_id)
                .fetch_optional(&mut *tx)
                .await;

            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction for mutation {}: {}", mutation.transaction_id, e);
                continue;
            }

            match result {
                Ok(Some((_, inventory_count))) => {
                    let action = if inventory_count < 0 {
                        "Anomaly".to_string()
                    } else {
                        "InventoryUpdated".to_string()
                    };
                    let topic = if inventory_count < 0 {
                        "mesh:inventory:anomaly"
                    } else {
                        "mesh:inventory:updated"
                    };

                    let event = ::server_ohc::orchestration::TeammateMeshEvent {
                        action,
                        agent_id: "system".to_string(),
                        status: "".to_string(),
                        msg_id: uuid::Uuid::new_v4().to_string(),
                        payload: serde_json::json!({
                            "product_id": mutation.product_id,
                            "transaction_id": mutation.transaction_id,
                            "quantity_deducted": mutation.quantity_deducted,
                            "timestamp": mutation.timestamp,
                            "tenant_id": tenant_id,
                            "inventory_count": inventory_count
                        }).to_string().into_bytes(),
                    };
                    let _ = mesh.publish(topic, event).await;
                }
                Ok(None) => {
                    tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id);
                }
                Err(e) => {
                    tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                }
            }
        } else if let Ok(None) = ledger_result {
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction (skip) for mutation {}: {}", mutation.transaction_id, e);
            }
            tracing::info!("Skipped mutation {} as it was already processed.", mutation.transaction_id);
        } else if let Err(e) = ledger_result {
            tracing::error!("Failed to insert into ledger for mutation {}: {}", mutation.transaction_id, e);
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

        let response = offline_sync_handler(state, headers, Json(req.clone())).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_offline_sync_success_idempotency_and_anomaly_event() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("CREATE TABLE IF NOT EXISTS offline_sync_transactions (transaction_id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, quantity_deducted INT NOT NULL, synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP)")
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
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 3,
                    timestamp: None,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req.clone())).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 2); // 5 - 3 = 2

        // Test idempotency
        let response_idem = offline_sync_handler(state.clone(), headers.clone(), Json(req.clone())).await.into_response();
        assert_eq!(response_idem.status(), StatusCode::OK);

        let count_idem: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count_idem.0, 2); // Should remain 2


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

        let response2 = offline_sync_handler(state, headers, Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, -8); // 2 - 10 = -8
    }
}
