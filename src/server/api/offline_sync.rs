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

pub async fn offline_sync_handler(
    db: std::sync::Arc<crate::db::DB>,
    orchestrator: std::sync::Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if !tenant_id.is_empty() {
        let cache = crate::builder::edge::get_edge_cache();
        cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

        for mutation in &payload.mutations {
            let mutation_type = mutation.get("type").and_then(|v| v.as_str()).unwrap_or("");
            let id = mutation.get("id").and_then(|v| v.as_str()).or_else(|| mutation.get("product_id").and_then(|v| v.as_str()));

            if let Some(product_id) = id {
                cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

                if mutation_type == "inventory_toggle" {
                    // Deduct inventory to prevent negative counts and trigger eventual "Sold Out" states
                    let result = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query("UPDATE products SET inventory_count = GREATEST(inventory_count - 1, 0) WHERE id = $1 AND (tenant_id = $2 OR organization_id = $2) AND inventory_count > 0")
                                .bind(product_id)
                                .bind(&tenant_id)
                                .execute(&db.pool).await
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query("UPDATE products SET inventory_count = MAX(inventory_count - 1, 0) WHERE id = ? AND (tenant_id = ? OR organization_id = ?) AND inventory_count > 0")
                                .bind(product_id)
                                .bind(&tenant_id)
                                .bind(&tenant_id)
                                .execute(&db.pool).await
                        }
                    };

                    if let Ok(res) = result {
                        if res.rows_affected() > 0 {
                            let event = crate::orchestration::departments::types::DepartmentEvent {
                                id: uuid::Uuid::new_v4().to_string(),
                                tenant_id: tenant_id.clone(),
                                event_type: "tenant.order.fulfillment_ready".to_string(),
                                payload: serde_json::json!({
                                    "product_id": product_id,
                                    "action": "offline_sync_deduction"
                                }),
                            };
                            let _ = orchestrator.dispatch_event(event).await;
                        }
                    }
                }
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
    use std::sync::Arc;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_offline_sync_handler() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let orchestrator = Arc::new(crate::orchestration::departments::orchestrator::DepartmentOrchestrator::new(db.clone(), mesh));

        let tenant_id = "test-tenant-offline-sync".to_string();
        let product_id = "test-product-falafel".to_string();

        match &db.store {
            crate::db::DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool).await;

                let _ = sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ($1, $2, 'Test Product', 5) ON CONFLICT (id) DO UPDATE SET inventory_count = 5")
                    .bind(&product_id)
                    .bind(&tenant_id)
                    .execute(&db.pool).await;
            },
            crate::db::DbStore::Sqlite(_) => {
                let _ = sqlx::query("INSERT OR IGNORE INTO tenants (id, name, tier) VALUES (?, 'Test', 'starter')")
                    .bind(&tenant_id)
                    .execute(&db.pool).await;

                let _ = sqlx::query("INSERT OR REPLACE INTO products (id, tenant_id, title, inventory_count) VALUES (?, ?, 'Test Product', 5)")
                    .bind(&product_id)
                    .bind(&tenant_id)
                    .execute(&db.pool).await;
            }
        }

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", axum::http::HeaderValue::from_str(&format!("spiffe://onehumancorp.com/tenant/{}/user/123", tenant_id)).unwrap());

        let payload = OfflineSyncRequest {
            mutations: vec![
                serde_json::json!({
                    "id": product_id,
                    "type": "inventory_toggle"
                })
            ]
        };

        let response = offline_sync_handler(db.clone(), orchestrator.clone(), headers, Json(payload)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        // Check if inventory is deducted
        let count: i32 = match &db.store {
            crate::db::DbStore::Postgres => {
                let r: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = $1")
                    .bind(&product_id)
                    .fetch_one(&db.pool).await.unwrap();
                r.0
            },
            crate::db::DbStore::Sqlite(_) => {
                let r: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = ?")
                    .bind(&product_id)
                    .fetch_one(&db.pool).await.unwrap();
                r.0
            }
        };

        assert_eq!(count, 4, "Inventory should be deducted by 1");
    }
}
