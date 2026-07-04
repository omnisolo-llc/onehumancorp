use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub timestamp: Option<String>,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>, // amount in cents
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
    pub client_mutation_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<OfflineMutation>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_reconciliation: Option<Vec<serde_json::Value>>,
    pub success: bool,
    pub failed_count: i32,
}


async fn validate_token_and_get_tenant(pool: &sqlx::PgPool, headers: &axum::http::HeaderMap) -> Result<(String, String), axum::response::Response> {
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()),
    };

    let repo = std::sync::Arc::new(crate::auth::postgres_store::PgUserRepository::new(pool.clone()));
    let store = std::sync::Arc::new(crate::auth::Store::with_repo(repo));

    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()),
    };

    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let agent_id = claims.sub;

    Ok((tenant_id, agent_id))
}

pub async fn offline_sync_handler(
    State((db, mesh)): State<(sqlx::PgPool, Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} offline mutations for edge sync.", payload.mutations.len()); // pii-safe

    let (tenant_id, _) = match validate_token_and_get_tenant(&db, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(OfflineSyncResponse { success: false, failed_count: 0, pending_reconciliation: None }),
        ).into_response();
    }

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let mut futures: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<serde_json::Value>, String>> + Send>>> = Vec::new();
    for mutation in &payload.mutations {
        let mutation = mutation.clone();
        let cache_clone = cache.clone();
        let tenant_id_clone = tenant_id.clone();
        let db_clone = db.clone();
        let mesh_clone = mesh.clone();

        if mutation.mutation_type.as_deref() == Some("draft_quote") {
            futures.push(Box::pin(async move {
                let mut db_tx = db_clone.begin().await.unwrap();

                if let Some(ref mutation_id) = mutation.client_mutation_id {
                    let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                        .bind(mutation_id)
                        .bind(&tenant_id_clone)
                        .fetch_one(&mut *db_tx)
                        .await
                        .unwrap_or((0,));

                    if exists.0 > 0 {
                        let redacted_mutation_id = ::server_telemetry::redact_interface_pii(serde_json::Value::String(mutation_id.clone()));
                        tracing::info!("Idempotency key hit for client_mutation_id: {}, skipping.", redacted_mutation_id.as_str().unwrap_or("")); // pii-safe
                        let _ = db_tx.rollback().await;
                        return Ok(None);
                    }

                    let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                        .bind(mutation_id)
                        .bind(&tenant_id_clone)
                        .execute(&mut *db_tx)
                        .await;
                }
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
                Ok(None)
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<serde_json::Value>, String>> + Send>>);
            continue;
        }

        if mutation.mutation_type.as_deref() == Some("agent_intent") {
            futures.push(Box::pin(async move {
                let mut db_tx = db_clone.begin().await.map_err(|e| e.to_string())?;

                if let Some(ref mutation_id) = mutation.client_mutation_id {
                    let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                        .bind(mutation_id)
                        .bind(&tenant_id_clone)
                        .fetch_one(&mut *db_tx)
                        .await
                        .unwrap_or((0,));

                    if exists.0 > 0 {
                        let redacted_mutation_id = ::server_telemetry::redact_interface_pii(serde_json::Value::String(mutation_id.clone()));
                        tracing::info!("Idempotency key hit for client_mutation_id: {}, skipping.", redacted_mutation_id.as_str().unwrap_or("")); // pii-safe
                        let _ = db_tx.rollback().await;
                        return Ok(None);
                    }

                    let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                        .bind(mutation_id)
                        .bind(&tenant_id_clone)
                        .execute(&mut *db_tx)
                        .await;
                }
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
                Ok(None)
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<serde_json::Value>, String>> + Send>>);
            continue;
        }

        futures.push(Box::pin(async move {
            cache_clone.invalidate_by_tag(&format!("entity:product:{}", mutation.product_id)).await;

            let locker: Box<dyn crate::orchestration::locks::DistributedLock> = if crate::is_standalone_runtime() {
                Box::new(crate::orchestration::locks::StandaloneLock::new())
            } else {
                if let Some(client) = crate::get_redis_client() {
                    Box::new(crate::orchestration::locks::RedisLock::new(client))
                } else {
                    Box::new(crate::orchestration::locks::StandaloneLock::new())
                }
            };
            let _lock_guard = match locker.acquire_resource(&tenant_id_clone, "inventory", &mutation.product_id).await {
                Ok(guard) => guard,
                Err(_) => {
                    tracing::warn!("Failed to acquire lock for offline sync reconciliation: inventory:{}", mutation.product_id);
                    return Err("Failed to acquire lock".to_string());
                }
            };


            let mut db_tx = db_clone.begin().await.unwrap();

            if let Some(ref mutation_id) = mutation.client_mutation_id {
                let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                    .bind(mutation_id)
                    .bind(&tenant_id_clone)
                    .fetch_one(&mut *db_tx)
                    .await
                    .unwrap_or((0,));

                if exists.0 > 0 {
                    let redacted_mutation_id = ::server_telemetry::redact_interface_pii(serde_json::Value::String(mutation_id.clone()));
                        tracing::info!("Idempotency key hit for client_mutation_id: {}, skipping.", redacted_mutation_id.as_str().unwrap_or("")); // pii-safe
                    let _ = db_tx.rollback().await;
                    return Ok(None);
                }

                let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                    .bind(mutation_id)
                    .bind(&tenant_id_clone)
                    .execute(&mut *db_tx)
                    .await;
            }

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
                    let mutation_ts = mutation.timestamp.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

                    let _ = sqlx::query("UPDATE products SET pn_counter_n = pn_counter_n + $4, inventory_count = GREATEST(0, pn_counter_p - (pn_counter_n + $4)), available_quantity = GREATEST(0, available_quantity - $4) WHERE id = $2 AND tenant_id = $3")
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
                             VALUES ($1, $2, 'operations', 'inventory.sync.conflict', $3::jsonb, 'PENDING')"
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
                            "timestamp": mutation_ts
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
                                    "timestamp": mutation_ts
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
                    if is_conflict {
                        let conflict_payload_val = serde_json::json!({
                            "transaction_id": mutation.transaction_id,
                            "product_id": mutation.product_id,
                            "shortage": mutation.quantity_deducted - stock,
                            "timestamp": mutation.timestamp.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339())
                        });
                        Ok(Some(conflict_payload_val))
                    } else {
                        Ok(None)
                    }
                }
                Ok(None) => {
                    tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id_clone); // pii-safe // pii-safe
                    Err("Product not found or unauthorized".to_string())
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("[bug] Failed to deduct inventory for product ");
                    tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                    Err("Database error".to_string())
                }
            }
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<serde_json::Value>, String>> + Send>>);
    }
    let results = futures::future::join_all(futures).await;

    let failed_count = results.iter().filter(|r| r.is_err()).count() as i32;
    let mut pending_reconciliation = Vec::new();
    for res in results {
        if let Ok(Some(conflict)) = res {
            pending_reconciliation.push(conflict);
        }
    }

    let pending_reconciliation = if pending_reconciliation.is_empty() {
        None
    } else {
        Some(pending_reconciliation)
    };

    (
        StatusCode::OK,
        Json(OfflineSyncResponse { success: true, failed_count, pending_reconciliation }),
    ).into_response()
}


#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SyncEvent {
    pub id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub base_version: i64,
}

#[derive(Deserialize, Debug)]
#[derive(Clone)]
pub struct SyncEventsRequest {
    pub events: Vec<SyncEvent>,
}

#[derive(Serialize)]
pub struct SyncEventsResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
}

pub async fn sync_events_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncEventsRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} generic sync events.", payload.events.len()); // pii-safe

    let (tenant_id, _) = match validate_token_and_get_tenant(&db, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SyncEventsResponse { success: false, applied_count: 0, conflict_count: 0 }),
        ).into_response();
    }

    let mut futures = Vec::new();
    for event in payload.events {
        let db_clone = db.clone();
        let tenant_id_clone = tenant_id.clone();
        futures.push(tokio::spawn(async move {
            let mut tx = match db_clone.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction for sync event {}: {}", event.id, e);
                    return ("failed", 1);
                }
            };

            // Idempotency check
            let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sync_events WHERE id = $1 AND tenant_id = $2")
                .bind(&event.id)
                .bind(&tenant_id_clone)
                .fetch_one(&mut *tx)
                .await
                .unwrap_or((0,));

            if exists.0 > 0 {
                let _ = tx.rollback().await;
                return ("applied", 1); // Already processed
            }

            // Version checking against test_sync_entities (demonstrative entity)
            let mut is_conflict = false;
            let mut current_version = 1;

            if event.entity_type == "test_sync_entity" {
                let row_res: Result<(i64,), sqlx::Error> = sqlx::query_as("SELECT version FROM test_sync_entities WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                    .bind(&event.entity_id)
                    .bind(&tenant_id_clone)
                    .fetch_one(&mut *tx)
                    .await;

                match row_res {
                    Ok((ver,)) => {
                        current_version = ver;
                        if current_version > event.base_version {
                            is_conflict = true;
                        }
                    }
                    Err(sqlx::Error::RowNotFound) => {
                        // Entity does not exist, so version is essentially 1 (or 0)
                        // Let's create it with version 1
                        let _ = sqlx::query("INSERT INTO test_sync_entities (id, tenant_id, version) VALUES ($1, $2, 1)")
                            .bind(&event.entity_id)
                            .bind(&tenant_id_clone)
                            .execute(&mut *tx)
                            .await;
                    }
                    Err(e) => {
                        tracing::error!("Failed to fetch test_sync_entities version for sync event {}: {}", event.id, e);
                        let _ = tx.rollback().await;
                        return ("failed", 1);
                    }
                }
            }

            if is_conflict {
                let res1 = sqlx::query(
                    "INSERT INTO sync_conflict_queue (id, tenant_id, event_id, entity_id, entity_type, base_version, current_version, payload) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id_clone)
                .bind(&event.id)
                .bind(&event.entity_id)
                .bind(&event.entity_type)
                .bind(event.base_version)
                .bind(current_version)
                .bind(&event.payload)
                .execute(&mut *tx)
                .await;

                let res2 = sqlx::query(
                    "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'sync_conflict_alert', $3::jsonb, 'PENDING')"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id_clone)
                .bind(serde_json::json!({
                    "event_id": event.id,
                    "entity_id": event.entity_id,
                    "entity_type": event.entity_type,
                    "message": "A data synchronization conflict occurred and requires Operations review."
                }).to_string())
                .execute(&mut *tx)
                .await;

                if res1.is_ok() && res2.is_ok() {
                    if tx.commit().await.is_ok() {
                        return ("conflict", 1);
                    }
                } else {
                    let _ = tx.rollback().await;
                }
                return ("failed", 1);
            } else {
                let res1 = sqlx::query(
                    "INSERT INTO sync_events (id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4)"
                )
                .bind(&event.id)
                .bind(&tenant_id_clone)
                .bind(&event.action_type)
                .bind(event.payload.to_string())
                .execute(&mut *tx)
                .await;

                let mut res2 = Ok(sqlx::postgres::PgQueryResult::default());
                if event.entity_type == "test_sync_entity" {
                    res2 = sqlx::query("UPDATE test_sync_entities SET version = version + 1 WHERE id = $1 AND tenant_id = $2")
                        .bind(&event.entity_id)
                        .bind(&tenant_id_clone)
                        .execute(&mut *tx)
                        .await;
                }

                if res1.is_ok() && res2.is_ok() {
                    if tx.commit().await.is_ok() {
                        return ("applied", 1);
                    }
                } else {
                    let _ = tx.rollback().await;
                }
                return ("failed", 1);
            }
        }));
    }

    use futures::future::join_all;
    let results = join_all(futures).await;

    let mut applied_count = 0;
    let mut conflict_count = 0;

    for res in results {
        if let Ok((status, count)) = res {
            match status {
                "applied" => applied_count += count,
                "conflict" => conflict_count += count,
                _ => {}
            }
        }
    }

    (
        StatusCode::OK,
        Json(SyncEventsResponse { success: true, applied_count, conflict_count }),
    ).into_response()
}


#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_sync_events_success() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-sync-1', 'Sync Test Tenant 1') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM sync_events WHERE tenant_id = 'tenant-sync-1'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM test_sync_entities WHERE tenant_id = 'tenant-sync-1'").execute(&pool).await.unwrap();

        let req = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "se1".to_string(),
                    entity_id: "test-ent-1".to_string(),
                    entity_type: "test_sync_entity".to_string(),
                    action_type: "update_status".to_string(),
                    payload: serde_json::json!({"status": "completed"}),
                    base_version: 1,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-sync-1/agent/x".parse().unwrap());

        let response = sync_events_handler(State(pool.clone()), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["success"], true);
        assert_eq!(body_json["applied_count"], 1);
        assert_eq!(body_json["conflict_count"], 0);

        let (ver,): (i64,) = sqlx::query_as("SELECT version FROM test_sync_entities WHERE id = 'test-ent-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(ver, 2);
    }

    #[tokio::test]
    async fn test_sync_events_idempotent() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-sync-2', 'Sync Test Tenant 2') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM sync_events WHERE tenant_id = 'tenant-sync-2'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM test_sync_entities WHERE tenant_id = 'tenant-sync-2'").execute(&pool).await.unwrap();

        let req = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "se2".to_string(),
                    entity_id: "test-ent-2".to_string(),
                    entity_type: "test_sync_entity".to_string(),
                    action_type: "update_status".to_string(),
                    payload: serde_json::json!({"status": "completed"}),
                    base_version: 1,
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-sync-2/agent/x".parse().unwrap());

        // First call
        let response1 = sync_events_handler(State(pool.clone()), headers.clone(), Json(req.clone())).await.into_response();
        assert_eq!(response1.status(), StatusCode::OK);

        // Second call
        let response2 = sync_events_handler(State(pool.clone()), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response2.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["success"], true);
        assert_eq!(body_json["applied_count"], 0);
        assert_eq!(body_json["conflict_count"], 0);
    }

    #[tokio::test]
    async fn test_sync_events_conflict() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-sync-3', 'Sync Test Tenant 3') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM sync_events WHERE tenant_id = 'tenant-sync-3'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM test_sync_entities WHERE tenant_id = 'tenant-sync-3'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM sync_conflict_queue WHERE tenant_id = 'tenant-sync-3'").execute(&pool).await.unwrap();

        // Preset entity with version 5
        sqlx::query("INSERT INTO test_sync_entities (id, tenant_id, version) VALUES ('test-ent-3', 'tenant-sync-3', 5)")
            .execute(&pool).await.unwrap();

        let req = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "se3".to_string(),
                    entity_id: "test-ent-3".to_string(),
                    entity_type: "test_sync_entity".to_string(),
                    action_type: "update_status".to_string(),
                    payload: serde_json::json!({"status": "completed"}),
                    base_version: 1, // Conflict! DB has 5
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-sync-3/agent/x".parse().unwrap());

        let response = sync_events_handler(State(pool.clone()), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["success"], true);
        assert_eq!(body_json["applied_count"], 0);
        assert_eq!(body_json["conflict_count"], 1);

        let (c_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sync_conflict_queue WHERE event_id = 'se3'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(c_count, 1);
    }
    use super::*;
    use axum::http::HeaderMap;
    use ohc_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport};
    #[allow(unused_imports)]
    use sqlx::postgres::PgPoolOptions;


    #[tokio::test]
    async fn test_offline_sync_unauthorized() {
        let pool = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap();
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

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

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
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                    transaction_id: "tx1".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 3,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()), mutation_type: None, payload: None, client_mutation_id: Some("mut1".to_string()),
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
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                    transaction_id: "tx2".to_string(),
                    product_id: "prod-offline-1".to_string(),
                    quantity_deducted: 10,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()), mutation_type: None, payload: None, client_mutation_id: Some("mut2".to_string()),
                },
            ],
        };

        let response2 = offline_sync_handler(state.clone(), headers.clone(), Json(req_over)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        // Test with a non-existent product which should fail
        let req_fail = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                    transaction_id: "tx3".to_string(),
                    product_id: "prod-offline-nonexistent".to_string(),
                    quantity_deducted: 1,
                    amount: Some(1000),
                    payment_method: None,
                    payment_intent_id: None,
                    currency: Some("USD".to_string()), mutation_type: None, payload: None, client_mutation_id: Some("mut3".to_string()),
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

    #[tokio::test]
    async fn test_offline_sync_field_service_mutations() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-field-service', 'Field Service Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS applied_client_mutations (
                client_mutation_id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS department_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                department TEXT NOT NULL,
                event_type TEXT NOT NULL,
                payload JSONB,
                status TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let state = State((pool.clone(), mesh.clone()));

        let req = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                    transaction_id: "tx-quote-1".to_string(),
                    product_id: "".to_string(),
                    quantity_deducted: 0,
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    mutation_type: Some("draft_quote".to_string()),
                    payload: Some(serde_json::json!({
                        "customer_name": "John Doe",
                        "customer_email": "john@example.com",
                        "total_amount": 5000,
                        "description": "Pipe fixing"
                    }).to_string()),
                    client_mutation_id: Some("mut-quote-1".to_string()),
                },
                OfflineMutation {
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                    transaction_id: "tx-intent-1".to_string(),
                    product_id: "".to_string(),
                    quantity_deducted: 0,
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    mutation_type: Some("agent_intent".to_string()),
                    payload: Some(serde_json::json!({
                        "intent": "update_booking_status",
                        "booking_id": "booking1",
                        "status": "COMPLETED"
                    }).to_string()),
                    client_mutation_id: Some("mut-intent-1".to_string()),
                },
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-field-service/agent/x".parse().unwrap());

        let response = offline_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["success"], true);
        assert_eq!(body_json["failed_count"], 0);

        // Verify that the idempotency check worked by repeating the same request
        let req_duplicate = OfflineSyncRequest {
            mutations: vec![
                OfflineMutation {
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                    transaction_id: "tx-quote-1".to_string(),
                    product_id: "".to_string(),
                    quantity_deducted: 0,
                    amount: None,
                    payment_method: None,
                    payment_intent_id: None,
                    currency: None,
                    mutation_type: Some("draft_quote".to_string()),
                    payload: Some(serde_json::json!({
                        "customer_name": "John Doe",
                        "customer_email": "john@example.com",
                        "total_amount": 5000,
                        "description": "Pipe fixing"
                    }).to_string()),
                    client_mutation_id: Some("mut-quote-1".to_string()),
                },
            ],
        };
        let response_dup = offline_sync_handler(state.clone(), headers.clone(), Json(req_duplicate)).await.into_response();
        assert_eq!(response_dup.status(), StatusCode::OK); // Dup gets skipped but doesn't fail
    }
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct OperationIntent {
    pub id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub timestamp: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct OperationIntentRequest {
    pub intents: Vec<OperationIntent>,
}

#[derive(serde::Serialize)]
pub struct OperationIntentResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
    pub failed_count: i32,
}

pub async fn operation_intents_handler(
    axum::extract::State(db): axum::extract::State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    axum::Json(payload): axum::Json<OperationIntentRequest>,
) -> impl axum::response::IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            axum::http::StatusCode::UNAUTHORIZED,
            axum::Json(OperationIntentResponse { success: false, applied_count: 0, conflict_count: 0, failed_count: 0 }),
        ).into_response();
    }

    let mut applied_count = 0;
    let conflict_count = 0;
    let mut failed_count = 0;

    for intent in &payload.intents {
        let mut tx = match db.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                failed_count += 1;
                continue;
            }
        };

        // Check idempotency (does the event id already exist?)
        let exists: Result<(i64,), sqlx::Error> = sqlx::query_as(
            "SELECT COUNT(*) FROM operation_intents WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&intent.id)
        .bind(&tenant_id)
        .fetch_one(&mut *tx)
        .await;

        if let Ok((count,)) = exists {
            if count > 0 {
                // Already processed
                let _ = tx.rollback().await;
                applied_count += 1; // It was previously applied (idempotency)
                continue;
            }
        }

        // Insert into operation_intents
        let insert_res = sqlx::query(
            "INSERT INTO operation_intents (id, tenant_id, action_type, payload, status)
             VALUES ($1, $2, $3, $4, 'SYNCED')"
        )
        .bind(&intent.id)
        .bind(&tenant_id)
        .bind(&intent.action_type)
        .bind(&intent.payload)
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_res {
            tracing::error!("Failed to insert operation intent: {}", e);
            let _ = tx.rollback().await;
            failed_count += 1;
            continue;
        }

        if let Err(e) = tx.commit().await {
            tracing::error!("Failed to commit transaction: {}", e);
            failed_count += 1;
            continue;
        }

        // For now, assume it's applied successfully if there is no specific conflict logic mapped.
        applied_count += 1;
    }

    (
        axum::http::StatusCode::OK,
        axum::Json(OperationIntentResponse { success: true, applied_count, conflict_count, failed_count }),
    ).into_response()
}
