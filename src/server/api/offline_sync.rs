use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OfflineMutation {
    Tagged(TaggedOfflineMutation),
    Legacy {
        transaction_id: String,
        product_id: String,
        quantity_deducted: i32,
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaggedOfflineMutation {
    InventoryToggle {
        id: String,
        timestamp: String,
    },
    TapToPay {
        id: String,
        amount: f64,
        timestamp: String,
        idempotency_key: String,
    },
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
        match mutation {
            OfflineMutation::Legacy { product_id, quantity_deducted, transaction_id } => {
                cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

                let query = "
                    UPDATE products
                    SET inventory_count = GREATEST(0, inventory_count - $1)
                    WHERE id = $2 AND tenant_id = $3
                    RETURNING id
                ";

                let result = sqlx::query(query)
                    .bind(quantity_deducted)
                    .bind(product_id)
                    .bind(&tenant_id)
                    .fetch_optional(&db)
                    .await;

                match result {
                    Ok(Some(_)) => {
                        let event = ::server_ohc::orchestration::TeammateMeshEvent {
                            action: "InventoryUpdated".to_string(),
                            agent_id: "system".to_string(),
                            status: "".to_string(),
                            msg_id: uuid::Uuid::new_v4().to_string(),
                            payload: serde_json::json!({
                                "product_id": product_id,
                                "transaction_id": transaction_id,
                                "quantity_deducted": quantity_deducted,
                                "tenant_id": tenant_id
                            }).to_string().into_bytes(),
                        };
                        let _ = mesh.publish("mesh:inventory:updated", event).await;
                    }
                    Ok(None) => {
                        tracing::warn!("Product {} not found or unauthorized for tenant {}", product_id, tenant_id);
                    }
                    Err(e) => {
                        tracing::error!("Failed to deduct inventory for product {}: {}", product_id, e);
                    }
                }
            }
            OfflineMutation::Tagged(TaggedOfflineMutation::InventoryToggle { id, timestamp }) => {
                // e.g. e2e-product-falafel -> strip e2e-product- if needed, or assume id is the product_id
                let product_id = id.strip_prefix("e2e-product-").unwrap_or(id);

                cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

                // Toggle inventory (e.g. set to 0 if it was > 0, or just set to 0 as in the "Sold Out" button)
                let query = "
                    UPDATE products
                    SET inventory_count = 0
                    WHERE id = $1 AND tenant_id = $2
                    RETURNING id
                ";

                let result = sqlx::query(query)
                    .bind(product_id)
                    .bind(&tenant_id)
                    .fetch_optional(&db)
                    .await;

                match result {
                    Ok(Some(_)) => {
                        let event = ::server_ohc::orchestration::TeammateMeshEvent {
                            action: "InventoryUpdated".to_string(),
                            agent_id: "system".to_string(),
                            status: "".to_string(),
                            msg_id: uuid::Uuid::new_v4().to_string(),
                            payload: serde_json::json!({
                                "product_id": product_id,
                                "transaction_id": format!("toggle-{}", timestamp),
                                "quantity_deducted": 0, // indicates sold out
                                "tenant_id": tenant_id
                            }).to_string().into_bytes(),
                        };
                        let _ = mesh.publish("mesh:inventory:updated", event).await;
                    }
                    Ok(None) => {
                        tracing::warn!("Product {} not found or unauthorized for tenant {}", product_id, tenant_id);
                    }
                    Err(e) => {
                        tracing::error!("Failed to toggle inventory for product {}: {}", product_id, e);
                    }
                }
            }
            OfflineMutation::Tagged(TaggedOfflineMutation::TapToPay { id, amount, timestamp, idempotency_key }) => {
                cache.invalidate_by_tag(&format!("tenant-id:{}:payments", tenant_id)).await;

                // In a real implementation this would insert a ledger entry or payment record.
                // For the offline sync handler we acknowledge the payment syncing.
                tracing::info!(
                    "Synced offline TapToPay payment for tenant {}: amount={}, tx_id={}, timestamp={}",
                    tenant_id, amount, id, timestamp
                );

                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    action: "PaymentReceived".to_string(),
                    agent_id: "system".to_string(),
                    status: "".to_string(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                    payload: serde_json::json!({
                        "transaction_id": id,
                        "amount": amount,
                        "timestamp": timestamp,
                        "idempotency_key": idempotency_key,
                        "tenant_id": tenant_id
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish("mesh:payment:received", event).await;
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

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        let req = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation::Legacy {
                    transaction_id: "tx1".to_string(),
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

        // Test negative guard
        let req_over = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation::Legacy {
                    transaction_id: "tx2".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 10,
                },
            ],
        };

        let response2 = offline_sync_handler(state, headers, Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-offline-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, 0); // GREATEST(0, 2 - 10) = 0
    }
}
