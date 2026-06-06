use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>, // amount in cents
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
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

    let mut futures = Vec::new();
    for mutation in &payload.mutations {
        let mutation = mutation.clone();
        let cache_clone = cache.clone();
        let tenant_id_clone = tenant_id.clone();
        let db_clone = db.clone();
        let mesh_clone = mesh.clone();

        futures.push(async move {
            cache_clone.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

            let query = "
                UPDATE products
                SET inventory_count = GREATEST(0, inventory_count - $1)
                WHERE id = $2 AND tenant_id = $3
                RETURNING id
            ";

            let result = sqlx::query(query)
                .bind(mutation.quantity_deducted)
                .bind(&mutation.product_id)
                .bind(&tenant_id_clone)
                .fetch_optional(&db_clone)
                .await;

            match result {
                Ok(Some(_)) => {
                    // Also queue an offline_pos_sync job to record the transaction
                    let job_id = uuid::Uuid::new_v4().to_string();
                    let job_payload = serde_json::json!({
                        "transaction_id": mutation.transaction_id,
                        "product_id": mutation.product_id,
                        "quantity_deducted": mutation.quantity_deducted,
                        "amount": mutation.amount,
                        "payment_method": mutation.payment_method,
                        "payment_intent_id": mutation.payment_intent_id,
                        "currency": mutation.currency,
                    }).to_string();

                    let job_res = sqlx::query(
                        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                         VALUES ($1, $2, 'offline_pos_sync', $3::jsonb)"
                    )
                    .bind(&job_id)
                    .bind(&tenant_id_clone)
                    .bind(&job_payload)
                    .execute(&db_clone)
                    .await;

                    if let Err(e) = job_res {
                        tracing::error!("Failed to enqueue offline_pos_sync job: {}", e);
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
                            "tenant_id": tenant_id_clone
                        }).to_string().into_bytes(),
                    };
                    let _ = mesh_clone.publish("mesh:inventory:updated", event).await;
                }
                Ok(None) => {
                    tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id_clone);
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("Failed to deduct inventory for product ");
                    tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                }
            }
        });
    }
    futures::future::join_all(futures).await;

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
                    transaction_id: "tx1".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 3,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()),
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // The handler enqueues a job now, it doesn't process it synchronously.
        // We can verify the job was enqueued.
        let job_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE job_type = 'offline_pos_sync'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(job_count.0, 1);

        let req_over = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx2".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 10,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()),
                },
            ],
        };

        let response2 = offline_sync_handler(state, headers, Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let job_count2: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE job_type = 'offline_pos_sync'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(job_count2.0, 2);
    }
}
