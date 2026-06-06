use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OfflineMutation {
    pub mutation_id: String,
    pub mutation_type: String, // "INVENTORY_DEDUCT", "TOGGLE_SOLD_OUT", "UPDATE_ORDER_STATUS"
    pub product_id: Option<String>,
    pub quantity_deducted: Option<i32>,
    pub amount: Option<i64>, // amount in cents
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub order_id: Option<String>,
    pub status: Option<String>,
    pub timestamp: String, // ISO8601
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<OfflineMutation>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
    pub processed_ids: Vec<String>,
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
            Json(OfflineSyncResponse { success: false, processed_ids: vec![] }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let mut processed_ids = Vec::new();

    for mutation in &payload.mutations {
        let mutation_id = &mutation.mutation_id;

        match mutation.mutation_type.as_str() {
            "INVENTORY_DEDUCT" => {
                if let (Some(product_id), Some(quantity)) = (&mutation.product_id, mutation.quantity_deducted) {
                    cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

                    let query = "
                        UPDATE products
                        SET inventory_count = GREATEST(0, inventory_count - $1),
                            updated_at = GREATEST(updated_at, $4::timestamptz)
                        WHERE id = $2 AND tenant_id = $3
                        RETURNING id
                    ";

                    let result = sqlx::query(query)
                        .bind(quantity)
                        .bind(product_id)
                        .bind(&tenant_id)
                        .bind(&mutation.timestamp)
                        .fetch_optional(&db)
                        .await;

                    if let Ok(Some(_)) = result {
                        enqueue_job(&db, &tenant_id, "offline_inventory_sync", mutation).await;
                        publish_event(&mesh, "InventoryUpdated", mutation, &tenant_id).await;
                        processed_ids.push(mutation_id.clone());
                    }
                }
            },
            "TOGGLE_SOLD_OUT" => {
                if let (Some(product_id), Some(metadata)) = (&mutation.product_id, &mutation.metadata) {
                    let is_sold_out = metadata["is_sold_out"].as_bool().unwrap_or(false);
                    let inventory_count = if is_sold_out { 0 } else { 10 }; // Simplified logic

                    let query = "
                        UPDATE products
                        SET inventory_count = $1,
                            updated_at = GREATEST(updated_at, $4::timestamptz)
                        WHERE id = $2 AND tenant_id = $3
                        RETURNING id
                    ";

                    let result = sqlx::query(query)
                        .bind(inventory_count)
                        .bind(product_id)
                        .bind(&tenant_id)
                        .bind(&mutation.timestamp)
                        .fetch_optional(&db)
                        .await;

                    if let Ok(Some(_)) = result {
                        enqueue_job(&db, &tenant_id, "offline_status_sync", mutation).await;
                        publish_event(&mesh, "ProductStatusUpdated", mutation, &tenant_id).await;
                        processed_ids.push(mutation_id.clone());
                    }
                }
            },
            "UPDATE_ORDER_STATUS" => {
                if let (Some(order_id), Some(status)) = (&mutation.order_id, &mutation.status) {
                    let query = "
                        UPDATE orders
                        SET status = $1,
                            updated_at = GREATEST(updated_at, $4::timestamptz)
                        WHERE id = $2 AND tenant_id = $3
                        RETURNING id
                    ";

                    let result = sqlx::query(query)
                        .bind(status)
                        .bind(order_id)
                        .bind(&tenant_id)
                        .bind(&mutation.timestamp)
                        .fetch_optional(&db)
                        .await;

                    if let Ok(Some(_)) = result {
                        enqueue_job(&db, &tenant_id, "offline_order_sync", mutation).await;
                        publish_event(&mesh, "OrderStatusUpdated", mutation, &tenant_id).await;
                        processed_ids.push(mutation_id.clone());
                    }
                }
            },
            _ => {
                tracing::warn!("Unknown mutation type: {}", mutation.mutation_type);
            }
        }
    }

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true, processed_ids }),
    ).into_response()
}

async fn enqueue_job(db: &sqlx::PgPool, tenant_id: &str, job_type: &str, mutation: &OfflineMutation) {
    let job_id = uuid::Uuid::new_v4().to_string();
    let job_payload = serde_json::to_string(mutation).unwrap_or_default();

    let job_res = sqlx::query(
        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
         VALUES ($1, $2, $3, $4::jsonb)"
    )
    .bind(&job_id)
    .bind(tenant_id)
    .bind(job_type)
    .bind(&job_payload)
    .execute(db)
    .await;

    if let Err(e) = job_res {
        tracing::error!("Failed to enqueue {} job: {}", job_type, e);
    }
}

async fn publish_event(mesh: &Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>, action: &str, mutation: &OfflineMutation, tenant_id: &str) {
    let event = ::server_ohc::orchestration::TeammateMeshEvent {
        action: action.to_string(),
        agent_id: "system".to_string(),
        status: "".to_string(),
        msg_id: uuid::Uuid::new_v4().to_string(),
        payload: serde_json::json!({
            "mutation_id": mutation.mutation_id,
            "type": mutation.mutation_type,
            "tenant_id": tenant_id,
            "metadata": mutation.metadata
        }).to_string().into_bytes(),
    };
    let _ = mesh.publish(&format!("mesh:sync:{}", action.to_lowercase()), event).await;
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
    async fn test_offline_sync_lww_conflict_resolution() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-lww', 'LWW Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let initial_ts = "2024-01-01T12:00:00Z";
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count, updated_at) VALUES ('prod-lww', 'tenant-lww', 'LWW Prod', 10, $1::timestamptz) ON CONFLICT DO NOTHING")
            .bind(initial_ts)
            .execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        // 1. Send an OLD mutation
        let old_ts = "2023-12-31T12:00:00Z";
        let req_old = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    mutation_id: "m1".to_string(),
                    mutation_type: "INVENTORY_DEDUCT".to_string(),
                    product_id: Some("prod-lww".to_string()),
                    quantity_deducted: Some(1),
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    order_id: None,
                    status: None,
                    timestamp: old_ts.to_string(),
                    metadata: None,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-lww/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req_old)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let prod: (i32, chrono::DateTime<chrono::Utc>) = sqlx::query_as("SELECT inventory_count, updated_at FROM products WHERE id = 'prod-lww'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(prod.0, 9);
        assert!(prod.1.timestamp() >= chrono::DateTime::parse_from_rfc3339(initial_ts).unwrap().timestamp());

        // 2. Send a NEW mutation
        let new_ts = "2024-02-01T12:00:00Z";
        let req_new = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    mutation_id: "m2".to_string(),
                    mutation_type: "INVENTORY_DEDUCT".to_string(),
                    product_id: Some("prod-lww".to_string()),
                    quantity_deducted: Some(2),
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    order_id: None,
                    status: None,
                    timestamp: new_ts.to_string(),
                    metadata: None,
                },
            ],
        };

        let response2 = offline_sync_handler(state.clone(), headers.clone(), Json(req_new)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let prod2: (i32, chrono::DateTime<chrono::Utc>) = sqlx::query_as("SELECT inventory_count, updated_at FROM products WHERE id = 'prod-lww'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(prod2.0, 7);
        assert_eq!(prod2.1.timestamp(), chrono::DateTime::parse_from_rfc3339(new_ts).unwrap().timestamp());
    }
}
