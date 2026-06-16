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
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<OfflineMutation>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    pub success: bool,
    pub failed_count: i32,
}

pub async fn crdt_sync_handler(
    State((db, mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} CRDT mutations for edge sync.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(OfflineSyncResponse { success: false, failed_count: 0 }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let mut futures: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>>> = Vec::new();
    for mutation in payload.mutations {
        let db_clone = db.clone();
        let tenant_id_clone = tenant_id.clone();

        futures.push(Box::pin(async move {
            let mut db_tx = db_clone.begin().await.map_err(|e| e.to_string())?;

            let crdt_payload_str = mutation.payload.unwrap_or_else(|| "{}".to_string());

            // Insert into crdt_deltas
            let q1 = "INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
                      VALUES ($1, $2, $3, $4, $5, $6)
                      ON CONFLICT(tenant_id, id) DO UPDATE SET
                      data = excluded.data, updated_at = excluded.updated_at, synced_to_cloud = $6
                      WHERE crdt_deltas.updated_at < excluded.updated_at";

            sqlx::query(q1)
                .bind(&tenant_id_clone)
                .bind(&mutation.transaction_id)
                .bind(&mutation.product_id)
                .bind(&crdt_payload_str)
                .bind(chrono::Utc::now().to_rfc3339())
                .bind(true)
                .execute(&mut *db_tx)
                .await
                .map_err(|e| e.to_string())?;

            // Insert into mcp_sync_deltas
            let q2 = "INSERT INTO mcp_sync_deltas (tenant_id, id, entity_type, entity_id, payload, updated_at)
                      VALUES ($1, $2, $3, $4, $5, $6)
                      ON CONFLICT (tenant_id, id) DO UPDATE SET
                      payload = excluded.payload, updated_at = excluded.updated_at
                      WHERE mcp_sync_deltas.updated_at < excluded.updated_at";

            sqlx::query(q2)
                .bind(&tenant_id_clone)
                .bind(&mutation.transaction_id)
                .bind("crdt_mutation")
                .bind(&mutation.product_id)
                .bind(&crdt_payload_str)
                .bind(chrono::Utc::now().timestamp_millis())
                .execute(&mut *db_tx)
                .await
                .map_err(|e| e.to_string())?;

            // Logic to check for conflict, for instance negative inventory count if it's an inventory crdt update
            if let Ok(crdt_json) = serde_json::from_str::<serde_json::Value>(&crdt_payload_str) {
                if let Some(qty_deducted) = crdt_json.get("quantity_deducted").and_then(|v| v.as_i64()) {
                    let q3 = "UPDATE products SET inventory_count = inventory_count - $1 WHERE id = $2 AND tenant_id = $3 RETURNING inventory_count";
                    if let Ok(Some(row)) = sqlx::query(q3)
                        .bind(qty_deducted)
                        .bind(&mutation.product_id)
                        .bind(&tenant_id_clone)
                        .fetch_optional(&mut *db_tx)
                        .await
                    {
                        use sqlx::Row;
                        let inv_count: i32 = row.get("inventory_count");
                        if inv_count < 0 {
                            // Enqueue task for operations agent
                            let job_id = uuid::Uuid::new_v4().to_string();
                            let job_payload = serde_json::json!({
                                "conflict_type": "oversold_inventory",
                                "product_id": mutation.product_id,
                                "shortage": -inv_count
                            }).to_string();
                            let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'agent_task', $3, 'PENDING')")
                                .bind(&job_id)
                                .bind(&tenant_id_clone)
                                .bind(job_payload)
                                .execute(&mut *db_tx)
                                .await;
                        }
                    }
                }
            }

            db_tx.commit().await.map_err(|e| e.to_string())?;
            Ok(())
        }));
    }

    let results = futures::future::join_all(futures).await;
    let failed_count = results.into_iter().filter(|r| r.is_err()).count() as i32;

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true, failed_count }),
    ).into_response()
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
            Json(OfflineSyncResponse { success: false, failed_count: 0 }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let mut futures: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>>> = Vec::new();
    for mutation in &payload.mutations {
        let mutation = mutation.clone();
        let cache_clone = cache.clone();
        let tenant_id_clone = tenant_id.clone();
        let db_clone = db.clone();
        let mesh_clone = mesh.clone();

        if mutation.mutation_type.as_deref() == Some("draft_quote") {
            futures.push(Box::pin(async move {
                let mut db_tx = db_clone.begin().await.unwrap();
                let _ = sqlx::query(
                    "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                     VALUES ($1, $2, 'sales', 'tenant.omnichannel.message.received', $3::jsonb, 'PENDING')"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id_clone)
                .bind(serde_json::json!({
                    "source": "offline_app",
                    "message": mutation.payload.unwrap_or_default()
                }).to_string())
                .execute(&mut *db_tx)
                .await;
                db_tx.commit().await.unwrap();
                Ok(())
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>>);
            continue;
        }

        if mutation.mutation_type.as_deref() == Some("agent_intent") {
            futures.push(Box::pin(async move {
                let mut db_tx = db_clone.begin().await.map_err(|e| e.to_string())?;
                let job_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'agent_intent', $3::jsonb)"
                )
                .bind(&job_id)
                .bind(&tenant_id_clone)
                .bind(mutation.payload.unwrap_or_else(|| "{}".to_string()))
                .execute(&mut *db_tx)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to enqueue agent intent: {}", e);
                    e.to_string()
                })?;
                db_tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>>);
            continue;
        }

        futures.push(Box::pin(async move {
            cache_clone.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

            let mut db_tx = db_clone.begin().await.unwrap();

            let query = "SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE";
            let current_stock = sqlx::query(query)
                .bind(&mutation.product_id)
                .bind(&tenant_id_clone)
                .fetch_optional(&mut *db_tx)
                .await;

            match current_stock {
                Ok(Some(row)) => {
                    let stock: i32 = sqlx::Row::get(&row, "inventory_count");
                    let mut is_conflict = false;
                    if stock < mutation.quantity_deducted {
                        is_conflict = true;
                    }

                    let new_stock = std::cmp::max(0, stock - mutation.quantity_deducted);

                    let _ = sqlx::query("UPDATE products SET inventory_count = $1, available_quantity = GREATEST(0, available_quantity - $4) WHERE id = $2 AND tenant_id = $3")
                        .bind(new_stock)
                        .bind(&mutation.product_id)
                        .bind(&tenant_id_clone)
                        .bind(mutation.quantity_deducted)
                        .execute(&mut *db_tx)
                        .await;

                    if is_conflict {
                        let ai_task_id = uuid::Uuid::new_v4().to_string();
                        let ai_payload = serde_json::json!({
                            "transaction_id": mutation.transaction_id,
                            "product_id": mutation.product_id,
                            "expected_stock": mutation.quantity_deducted,
                            "actual_stock": stock,
                            "message": format!("Heads up! A pop-up sale overlapped with an online order for {}. Operations has drafted an email to the online customer.", mutation.product_id)
                        }).to_string();

                        let _ = sqlx::query(
                            "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                             VALUES ($1, $2, 'operations', 'InventoryConflictEvent', $3::jsonb, 'PENDING')"
                        )
                        .bind(&ai_task_id)
                        .bind(&tenant_id_clone)
                        .bind(&ai_payload)
                        .execute(&mut *db_tx)
                        .await;

                        // Refined TerminalSession data schema update for offline-sync reconciliation
                        // Add this conflict to the session's pending_reconciliation array
                        let conflict_payload = serde_json::json!({
                            "transaction_id": mutation.transaction_id,
                            "product_id": mutation.product_id,
                            "shortage": mutation.quantity_deducted - stock,
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }).to_string();

                        let _ = sqlx::query(
                            "UPDATE pos_terminal_sessions
                             SET sync_status = 'CONFLICTS_PENDING',
                                 pending_reconciliation = pending_reconciliation || $1::jsonb
                             WHERE tenant_id = $2
                             AND device_id = (SELECT client_id FROM pos_offline_transactions WHERE id = $3)"
                        )
                        .bind(serde_json::json!([serde_json::from_str::<serde_json::Value>(&conflict_payload).unwrap()]))
                        .bind(&tenant_id_clone)
                        .bind(&mutation.transaction_id)
                        .execute(&mut *db_tx)
                        .await;
                    }

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
                    .execute(&mut *db_tx)
                    .await;

                    if let Err(e) = job_res {
                        tracing::error!("Failed to enqueue offline_pos_sync job: {}", e);
                    }

                    // Publish to Redis Pub/Sub for Real-Time Sync Engine
                    let redis_client_opt = crate::get_redis_client();
                    let redis_tenant_id = tenant_id_clone.clone();
                    let redis_product_id = mutation.product_id.clone();
                    tokio::spawn(async move {
                        if let Some(client) = redis_client_opt {
                            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                let topic = format!("inventory:{}", redis_tenant_id);
                                let payload = serde_json::json!({
                                    "event": "inventory_updated",
                                    "product_id": redis_product_id,
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                }).to_string();
                                let _: () = redis::cmd("PUBLISH").arg(topic.trim()).arg(payload).query_async(&mut conn).await.unwrap_or(());
                            }
                        }
                    });

                    db_tx.commit().await.unwrap();

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
                    Ok(())
                }
                Ok(None) => {
                    tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id_clone);
                    Err("Product not found or unauthorized".to_string())
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("Failed to deduct inventory for product ");
                    tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                    Err("Database error".to_string())
                }
            }
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>>);
    }
    let results = futures::future::join_all(futures).await;

    let failed_count = results.into_iter().filter(|r| r.is_err()).count() as i32;

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true, failed_count }),
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
        let pool = PgPoolOptions::new().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap();
        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool, mesh));

        let req = OfflineSyncRequest { mutations: vec![] };
        let headers = HeaderMap::new();

        let response = offline_sync_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_crdt_sync_handler() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }
        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline', 'Offline Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-crdt-1', 'tenant-offline', 'CRDT Prod', 5) ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        let req = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx-crdt-1".to_string(),
                    product_id: "prod-crdt-1".to_string(),
                    quantity_deducted: 2,
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    mutation_type: Some("crdt_delta".to_string()),
                    payload: Some(serde_json::json!({"quantity_deducted": 2}).to_string()),
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline/agent/x".parse().unwrap());

        let response = crdt_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let row: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-crdt-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 3);

        // Test oversold condition (conflict resolution)
        let req2 = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx-crdt-2".to_string(),
                    product_id: "prod-crdt-1".to_string(),
                    quantity_deducted: 5,
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    mutation_type: Some("crdt_delta".to_string()),
                    payload: Some(serde_json::json!({"quantity_deducted": 5}).to_string()),
                },
            ],
        };
        let response2 = crdt_sync_handler(state, headers, Json(req2)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let row2: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = 'prod-crdt-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row2.0, -2);
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
                    currency: Some("USD".to_string()), mutation_type: None, payload: None,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let req_over = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx2".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 10,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()), mutation_type: None, payload: None,
                },
            ],
        };

        let response2 = offline_sync_handler(state.clone(), headers.clone(), Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        // Test with a non-existent product which should fail
        let req_fail = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    transaction_id: "tx3".to_string(),
                    product_id: "prod-offline-nonexistent".to_string(),
                    quantity_deducted: 1,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()), mutation_type: None, payload: None,
                },
            ],
        };

        let response_fail = offline_sync_handler(state, headers, Json(req_fail)).await.into_response();
        assert_eq!(response_fail.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response_fail.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["success"], true);
        assert_eq!(body_json["failed_count"], 1);
    }
}
