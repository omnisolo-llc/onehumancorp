use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct OfflineMutation {
    pub transaction_id: Option<String>,
    pub product_id: Option<String>,
    pub quantity_deducted: Option<i32>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub amount: Option<f64>,
    pub timestamp: Option<String>,
    pub idempotency_key: Option<String>,
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
        if let (Some(product_id), Some(quantity), Some(tx_id)) = (&mutation.product_id, mutation.quantity_deducted, &mutation.transaction_id) {
            cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

            let query = "
                UPDATE products
                SET inventory_count = GREATEST(0, inventory_count - $1)
                WHERE id = $2 AND tenant_id = $3
                RETURNING id
            ";

            let result = sqlx::query(query)
                .bind(quantity)
                .bind(product_id)
                .bind(&tenant_id)
                .fetch_optional(&db)
                .await;

            match result {
                Ok(Some(_)) => {
                    // Publish mesh event
                    let event = ::server_ohc::orchestration::TeammateMeshEvent {
                        action: "InventoryUpdated".to_string(),
                        agent_id: "system".to_string(),
                        status: "".to_string(),
                        msg_id: uuid::Uuid::new_v4().to_string(),
                        payload: serde_json::json!({
                            "product_id": product_id,
                            "transaction_id": tx_id,
                            "quantity_deducted": quantity,
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
        } else if mutation.event_type.as_deref() == Some("tap_to_pay") {
            tracing::info!("Processed tap_to_pay offline sync for tenant {}: amount {:?}", tenant_id, mutation.amount);
            // In a real app we'd publish an event to process the payment
            // For now, we accept it so it's cleared from the queue
            if let Some(id) = &mutation.id {
                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    action: "TapToPayOfflineSync".to_string(),
                    agent_id: "system".to_string(),
                    status: "".to_string(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                    payload: serde_json::json!({
                        "id": id,
                        "amount": mutation.amount,
                        "tenant_id": tenant_id
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish("mesh:payments:tap_to_pay_sync", event).await;
            }
        } else if mutation.event_type.as_deref() == Some("inventory_toggle") {
            tracing::info!("Processed inventory_toggle offline sync for tenant {}", tenant_id);
            // Accept the toggle sync event
            if let Some(id) = &mutation.id {
                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    action: "InventoryToggleOfflineSync".to_string(),
                    agent_id: "system".to_string(),
                    status: "".to_string(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                    payload: serde_json::json!({
                        "id": id,
                        "tenant_id": tenant_id
                    }).to_string().into_bytes(),
                };
                let _ = mesh.publish("mesh:inventory:toggle_sync", event).await;
            }
        } else {
            tracing::warn!("Unknown offline mutation format: {:?}", mutation);
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
                OfflineMutation {
                    transaction_id: Some("tx1".to_string()),
                    product_id: Some("prod-offline-1".to_string()),
                    quantity_deducted: Some(3),
                    id: None,
                    event_type: None,
                    amount: None,
                    timestamp: None,
                    idempotency_key: None,
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
                    transaction_id: Some("tx2".to_string()),
                    product_id: Some("prod-offline-1".to_string()),
                    quantity_deducted: Some(10),
                    id: None,
                    event_type: None,
                    amount: None,
                    timestamp: None,
                    idempotency_key: None,
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
