
fn apply_domain_logic<'a, 'c>(
    tx: &'a mut sqlx::Transaction<'c, sqlx::Postgres>,
    event: &'a SyncEvent,
    tenant_id: &'a str,
) -> futures::future::BoxFuture<'a, Result<(), sqlx::Error>> {
    Box::pin(async move {
        if event.entity_type == "order" && event.action_type == "UpdateStatus" {
            if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(status)
                    .bind(&event.entity_id)
                    .bind(&tenant_id)
                    .execute(&mut **tx)
                    .await?;
            }
        } else if event.entity_type == "appointment" && event.action_type == "UpdateStatus" {
            if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                let notes = event.payload.get("notes").and_then(|v| v.as_str());
                sqlx::query("UPDATE appointments SET status = $1, notes = COALESCE($2, notes), updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
                    .bind(status)
                    .bind(notes)
                    .bind(&event.entity_id)
                    .bind(&tenant_id)
                    .execute(&mut **tx)
                    .await?;

                if status == "Completed" {
                    let task_id = uuid::Uuid::new_v4().to_string();
                    let ai_payload = serde_json::json!({
                        "sync_event_id": event.id,
                        "entity_type": event.entity_type,
                        "entity_id": event.entity_id,
                        "action_type": event.action_type,
                        "message": "Offline job completed. Trigger downstream workflows like ETA SMS to next customer and invoice generation."
                    }).to_string();

                    let _ = sqlx::query(
                        "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                         VALUES ($1, $2, 'operations', 'job.completed', $3::jsonb, 'PENDING')"
                    )
                    .bind(&task_id)
                    .bind(&tenant_id)
                    .bind(&ai_payload)
                    .execute(&mut **tx)
                    .await?;
                }
            }
        } else if event.entity_type == "product" && event.action_type == "ToggleSoldOut" {
            if let Some(is_sold_out) = event.payload.get("is_sold_out").and_then(|v| v.as_bool()) {
                sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(is_sold_out)
                    .bind(&event.entity_id)
                    .bind(&tenant_id)
                    .execute(&mut **tx)
                    .await?;

                if let Some(client) = crate::get_redis_client() {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let invalidation_topic = "cache_invalidation_events";
                        let invalidation_payload = serde_json::json!({
                            "event": "product.updated",
                            "tags": [
                                format!("tenant-id:{}", tenant_id),
                                format!("entity:product:{}", event.entity_id)
                            ]
                        }).to_string();
                        let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                    }
                }

                let edge_cache = crate::builder::edge::get_edge_cache();
                edge_cache.invalidate_by_tag(&format!("entity:product:{}", event.entity_id)).await;
                edge_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

                let item_id_owned = event.entity_id.to_string();
                let tenant_id_owned = tenant_id.to_string();
                tokio::spawn(async move {
                    let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
                    cdn.invalidate_by_tag(&format!("entity:product:{}", item_id_owned)).await;
                    cdn.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_owned)).await;
                });
            }
        }
        Ok(())
    })
}

use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SyncEvent {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub base_version: i64,
    pub timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SyncEventsRequest {
    pub events: Vec<SyncEvent>,
}

#[derive(Serialize)]
pub struct SyncEventsResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
    pub failed_count: i32,
}


fn apply_domain_logic<'a, 'c>(
    tx: &'a mut sqlx::Transaction<'c, sqlx::Postgres>,
    event: &'a SyncEvent,
    tenant_id: &'a str,
) -> futures::future::BoxFuture<'a, Result<(), sqlx::Error>> {
    Box::pin(async move {
        if event.entity_type == "order" && event.action_type == "UpdateStatus" {
            if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(status)
                    .bind(&event.entity_id)
                    .bind(&tenant_id)
                    .execute(&mut **tx)
                    .await?;
            }
        } else if event.entity_type == "product" && event.action_type == "ToggleSoldOut" {
            if let Some(is_sold_out) = event.payload.get("is_sold_out").and_then(|v| v.as_bool()) {
                sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(is_sold_out)
                    .bind(&event.entity_id)
                    .bind(&tenant_id)
                    .execute(&mut **tx)
                    .await?;

                if let Some(client) = crate::get_redis_client() {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let invalidation_topic = "cache_invalidation_events";
                        let invalidation_payload = serde_json::json!({
                            "event": "product.updated",
                            "tags": [
                                format!("tenant-id:{}", tenant_id),
                                format!("entity:product:{}", event.entity_id)
                            ]
                        }).to_string();
                        let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                    }
                }

                let edge_cache = crate::builder::edge::get_edge_cache();
                edge_cache.invalidate_by_tag(&format!("entity:product:{}", event.entity_id)).await;
                edge_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

                let item_id_owned = event.entity_id.to_string();
                let tenant_id_owned = tenant_id.to_string();
                tokio::spawn(async move {
                    let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
                    cdn.invalidate_by_tag(&format!("entity:product:{}", item_id_owned)).await;
                    cdn.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_owned)).await;
                });
            }
        }
        Ok(())
    })
}

pub async fn sync_events_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncEventsRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SyncEventsResponse { success: false, applied_count: 0, conflict_count: 0, failed_count: 0 }),
        ).into_response();
    }

    let mut applied_count = 0;
    let mut conflict_count = 0;
    let mut failed_count = 0;

    for event in &payload.events {
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
            "SELECT COUNT(*) FROM sync_events WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&event.id)
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

        // Insert into sync_events as PENDING
        let insert_res = sqlx::query(
            "INSERT INTO sync_events (id, tenant_id, entity_type, entity_id, action_type, payload, base_version, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'PENDING')"
        )
        .bind(&event.id)
        .bind(&tenant_id)
        .bind(&event.entity_type)
        .bind(&event.entity_id)
        .bind(&event.action_type)
        .bind(&event.payload)
        .bind(event.base_version)
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_res {
            tracing::error!("Failed to insert sync event: {}", e);
            let _ = tx.rollback().await;
            failed_count += 1;
            continue;
        }

        // Check version
        let current_version_res: Result<Option<(i64,)>, sqlx::Error> = sqlx::query_as(
            "SELECT current_version FROM entity_versions WHERE tenant_id = $1 AND entity_type = $2 AND entity_id = $3 FOR UPDATE"
        )
        .bind(&tenant_id)
        .bind(&event.entity_type)
        .bind(&event.entity_id)
        .fetch_optional(&mut *tx)
        .await;

        match current_version_res {
            Ok(Some((current_version,))) => {
                if current_version > event.base_version {
                    // Conflict
                    let conflict_id = uuid::Uuid::new_v4().to_string();
                    let _ = sqlx::query(
                        "INSERT INTO conflict_queue (id, tenant_id, sync_event_id, entity_type, entity_id, action_type, payload, base_version, current_version, status)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'UNRESOLVED')"
                    )
                    .bind(&conflict_id)
                    .bind(&tenant_id)
                    .bind(&event.id)
                    .bind(&event.entity_type)
                    .bind(&event.entity_id)
                    .bind(&event.action_type)
                    .bind(&event.payload)
                    .bind(event.base_version)
                    .bind(current_version)
                    .execute(&mut *tx)
                    .await;

                    let _ = sqlx::query("UPDATE sync_events SET status = 'CONFLICT' WHERE id = $1 AND tenant_id = $2")
                        .bind(&event.id)
                        .bind(&tenant_id)
                        .execute(&mut *tx)
                        .await;

                    // Route to operations agent by inserting into department_tasks
                    let task_id = uuid::Uuid::new_v4().to_string();
                    let ai_payload = serde_json::json!({
                        "sync_event_id": event.id,
                        "entity_type": event.entity_type,
                        "entity_id": event.entity_id,
                        "action_type": event.action_type,
                        "base_version": event.base_version,
                        "current_version": current_version,
                        "message": "A data synchronization conflict occurred while offline. Please review and resolve."
                    }).to_string();

                    let _ = sqlx::query(
                        "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                         VALUES ($1, $2, 'operations', 'sync.event.conflict', $3::jsonb, 'PENDING')"
                    )
                    .bind(&task_id)
                    .bind(&tenant_id)
                    .bind(&ai_payload)
                    .execute(&mut *tx)
                    .await;

                    conflict_count += 1;
                } else {
                    // Apply update
                    let new_version = current_version + 1;
                    let _ = sqlx::query("UPDATE entity_versions SET current_version = $1 WHERE tenant_id = $2 AND entity_type = $3 AND entity_id = $4")
                        .bind(new_version)
                        .bind(&tenant_id)
                        .bind(&event.entity_type)
                        .bind(&event.entity_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE sync_events SET status = 'APPLIED' WHERE id = $1 AND tenant_id = $2")
                        .bind(&event.id)
                        .bind(&tenant_id)
                        .execute(&mut *tx)
                        .await;

                    // DOMAIN LOGIC: Actually apply the changes
                    if event.entity_type == "order" && event.action_type == "UpdateStatus" {
                        if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                            let _ = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                                .bind(status)
                                .bind(&event.entity_id)
                                .bind(&tenant_id)
                                .execute(&mut *tx)
                                .await;
                        }
                    } else if event.entity_type == "appointment" && event.action_type == "UpdateStatus" {
                        if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                            let notes = event.payload.get("notes").and_then(|v| v.as_str());
                            let _ = sqlx::query("UPDATE appointments SET status = $1, notes = COALESCE($2, notes), updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
                                .bind(status)
                                .bind(notes)
                                .bind(&event.entity_id)
                                .bind(&tenant_id)
                                .execute(&mut *tx)
                                .await;

                            if status == "Completed" {
                                let task_id = uuid::Uuid::new_v4().to_string();
                                let ai_payload = serde_json::json!({
                                    "sync_event_id": event.id,
                                    "entity_type": event.entity_type,
                                    "entity_id": event.entity_id,
                                    "action_type": event.action_type,
                                    "message": "Offline job completed. Trigger downstream workflows like ETA SMS to next customer and invoice generation."
                                }).to_string();

                                let _ = sqlx::query(
                                    "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                                     VALUES ($1, $2, 'operations', 'job.completed', $3::jsonb, 'PENDING')"
                                )
                                .bind(&task_id)
                                .bind(&tenant_id)
                                .bind(&ai_payload)
                                .execute(&mut *tx)
                                .await;
                            }
                        }
                    } else if event.entity_type == "appointment" && event.action_type == "UpdateStatus" {
                        if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                            let notes = event.payload.get("notes").and_then(|v| v.as_str());
                            let _ = sqlx::query("UPDATE appointments SET status = $1, notes = COALESCE($2, notes), updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
                                .bind(status)
                                .bind(notes)
                                .bind(&event.entity_id)
                                .bind(&tenant_id)
                                .execute(&mut *tx)
                                .await;

                            if status == "Completed" {
                                let task_id = uuid::Uuid::new_v4().to_string();
                                let ai_payload = serde_json::json!({
                                    "sync_event_id": event.id,
                                    "entity_type": event.entity_type,
                                    "entity_id": event.entity_id,
                                    "action_type": event.action_type,
                                    "message": "Offline job completed. Trigger downstream workflows like ETA SMS to next customer and invoice generation."
                                }).to_string();

                                let _ = sqlx::query(
                                    "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                                     VALUES ($1, $2, 'operations', 'job.completed', $3::jsonb, 'PENDING')"
                                )
                                .bind(&task_id)
                                .bind(&tenant_id)
                            let tenant_id_owned = tenant_id.to_string();
                            tokio::spawn(async move {
                                let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
                                cdn.invalidate_by_tag(&format!("entity:product:{}", item_id_owned)).await;
                                cdn.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_owned)).await;
                            });

                    }

                    applied_count += 1;
                }
            }
            Ok(None) => {
                // No current version, so this is creating the entity or it's the first event for it.
                let new_version = event.base_version + 1;
                let _ = sqlx::query(
                    "INSERT INTO entity_versions (tenant_id, entity_type, entity_id, current_version) VALUES ($1, $2, $3, $4)"
                )
                .bind(&tenant_id)
                .bind(&event.entity_type)
                .bind(&event.entity_id)
                .bind(new_version)
                .execute(&mut *tx)
                .await;

                let _ = sqlx::query("UPDATE sync_events SET status = 'APPLIED' WHERE id = $1 AND tenant_id = $2")
                    .bind(&event.id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await;

                    // DOMAIN LOGIC: Actually apply the changes
                    if event.entity_type == "order" && event.action_type == "UpdateStatus" {
                        if let Some(status) = event.payload.get("status").and_then(|v| v.as_str()) {
                            let _ = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                                .bind(status)
                                .bind(&event.entity_id)
                                .bind(&tenant_id)
                                .execute(&mut *tx)
                                .await;
                        }
                    } else if event.entity_type == "product" && event.action_type == "ToggleSoldOut" {
                        if let Some(is_sold_out) = event.payload.get("is_sold_out").and_then(|v| v.as_bool()) {
                            let _ = sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2 AND tenant_id = $3")
                                .bind(is_sold_out)
                                .bind(&event.entity_id)
                                .bind(&tenant_id)
                                .execute(&mut *tx)
                                .await;
                        }
                            if let Some(client) = crate::get_redis_client() {
                                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                    let invalidation_topic = "cache_invalidation_events";
                                    let invalidation_payload = serde_json::json!({
                                        "event": "product.updated",
                                        "tags": [
                                            format!("tenant-id:{}", tenant_id),
                                            format!("entity:product:{}", event.entity_id)
                                        ]
                                    }).to_string();
                                    let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                                }
                            }
                            let edge_cache = crate::builder::edge::get_edge_cache();
                            edge_cache.invalidate_by_tag(&format!("entity:product:{}", event.entity_id)).await;
                            edge_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

                            let item_id_owned = event.entity_id.to_string();
                            let tenant_id_owned = tenant_id.to_string();
                            tokio::spawn(async move {
                                let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
                                cdn.invalidate_by_tag(&format!("entity:product:{}", item_id_owned)).await;
                                cdn.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_owned)).await;
                            });

                    }

                applied_count += 1;
            }
            Err(e) => {
                tracing::error!("Failed to check entity version: {}", e);
                let _ = tx.rollback().await;
                failed_count += 1;
                continue;
            }
        }

        if let Err(e) = tx.commit().await {
            tracing::error!("Failed to commit transaction: {}", e);
            // It could be that another thread updated it concurrently. We rollback implicitly.
            failed_count += 1;
            // Since we updated counts earlier, we need to revert the count
            if applied_count > 0 && conflict_count == 0 {
                applied_count -= 1;
            } else if conflict_count > 0 {
                conflict_count -= 1;
            }
        }
    }

    (
        StatusCode::OK,
        Json(SyncEventsResponse { success: true, applied_count, conflict_count, failed_count }),
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_sync_events_unauthorized() {
        let pool = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap();
        let state = State(pool);

        let req = SyncEventsRequest { events: vec![] };
        let headers = HeaderMap::new();

        let response = sync_events_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_sync_events_application() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-sync', 'Sync Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        sqlx::query("DELETE FROM sync_events WHERE tenant_id = 'tenant-sync'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM conflict_queue WHERE tenant_id = 'tenant-sync'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM entity_versions WHERE tenant_id = 'tenant-sync'").execute(&pool).await.unwrap();

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

        let state = State(pool.clone());
        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc.local/org/tenant-sync/agent/x".parse().unwrap());

        // Test 1: Successful Application (No existing version)
        let req1 = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "evt-1".to_string(),
                    entity_type: "booking".to_string(),
                    entity_id: "book-1".to_string(),
                    action_type: "UpdateStatus".to_string(),
                    payload: serde_json::json!({"status": "COMPLETED"}),
                    base_version: 1,
                    timestamp: None,
                }
            ],
        };

        let res1 = sync_events_handler(state.clone(), headers.clone(), Json(req1)).await.into_response();
        assert_eq!(res1.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(res1.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["applied_count"], 1);
        assert_eq!(body_json["conflict_count"], 0);

        // Verify entity version is updated
        let (ver,): (i64,) = sqlx::query_as("SELECT current_version FROM entity_versions WHERE entity_id = 'book-1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(ver, 2);

        // Test 2: Idempotent Retry (Same ID)
        let req2 = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "evt-1".to_string(),
                    entity_type: "booking".to_string(),
                    entity_id: "book-1".to_string(),
                    action_type: "UpdateStatus".to_string(),
                    payload: serde_json::json!({"status": "COMPLETED"}),
                    base_version: 1,
                    timestamp: None,
                }
            ],
        };

        let res2 = sync_events_handler(state.clone(), headers.clone(), Json(req2)).await.into_response();
        let body_bytes2 = axum::body::to_bytes(res2.into_body(), usize::MAX).await.unwrap();
        let body_json2: serde_json::Value = serde_json::from_slice(&body_bytes2).unwrap();
        assert_eq!(body_json2["applied_count"], 1); // Should count as applied

        // Test 3: Conflict Routing
        let req3 = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "evt-2".to_string(),
                    entity_type: "booking".to_string(),
                    entity_id: "book-1".to_string(),
                    action_type: "CancelBooking".to_string(),
                    payload: serde_json::json!({"status": "CANCELLED"}),
                    base_version: 1, // The DB version is now 2, so this should conflict
                    timestamp: None,
                }
            ],
        };

        let res3 = sync_events_handler(state.clone(), headers.clone(), Json(req3)).await.into_response();
        let body_bytes3 = axum::body::to_bytes(res3.into_body(), usize::MAX).await.unwrap();
        let body_json3: serde_json::Value = serde_json::from_slice(&body_bytes3).unwrap();
        assert_eq!(body_json3["applied_count"], 0);
        assert_eq!(body_json3["conflict_count"], 1);

        // Verify conflict_queue insertion
        let (status,): (String,) = sqlx::query_as("SELECT status FROM sync_events WHERE id = 'evt-2'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(status, "CONFLICT");

        let (q_status, q_base, q_current): (String, i64, i64) = sqlx::query_as("SELECT status, base_version, current_version FROM conflict_queue WHERE sync_event_id = 'evt-2'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(q_status, "UNRESOLVED");
        assert_eq!(q_base, 1);
        assert_eq!(q_current, 2);
    }
}
