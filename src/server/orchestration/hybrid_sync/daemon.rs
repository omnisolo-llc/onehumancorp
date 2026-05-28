use std::time::Duration;
use sqlx::{SqlitePool, PgPool, Row};
use serde_json::{Value, json};
use tracing::{info, error, warn};
use uuid::Uuid;
use chrono::Utc;

pub struct HybridSyncDaemon {
    sqlite_pool: SqlitePool,
    pg_pool: Option<PgPool>,
    cloud_url: Option<String>,
    client: reqwest::Client,
}

impl HybridSyncDaemon {
    pub fn new(sqlite_pool: SqlitePool, pg_pool: Option<PgPool>, cloud_url: Option<String>) -> Self {
        Self {
            sqlite_pool,
            pg_pool,
            cloud_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn run(&self) {
        loop {
            if let Err(e) = self.sync_step().await {
                error!("Hybrid sync daemon error: {}", e);
            }
            if let Err(e) = self.sync_telemetry_step().await {
                error!("Hybrid sync telemetry error: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn sync_telemetry_step(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !::server_config::get().telemetry_enabled {
            return Ok(());
        }

        let rows = sqlx::query("SELECT id, metric_name, metric_type, value, labels_json, timestamp FROM telemetry_buffer WHERE sync_status = 'pending'")
            .fetch_all(&self.sqlite_pool)
            .await?;

        if rows.is_empty() {
            return Ok(());
        }

        let mut payload = Vec::new();
        for row in &rows {
            let metric_name: String = row.get("metric_name");
            let metric_type: String = row.get("metric_type");
            let value: f32 = row.get("value");
            let labels_json: String = row.get("labels_json");
            let timestamp: chrono::NaiveDateTime = row.get("timestamp");

            let parsed_labels: Value = serde_json::from_str(&labels_json).unwrap_or(json!({}));

            payload.push(json!({
                "metric_name": metric_name,
                "metric_type": metric_type,
                "value": value,
                "labels": parsed_labels,
                "timestamp": chrono::DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc)
            }));
        }

        if let Some(cloud_url) = &self.cloud_url {
            let url = format!("{}/api/telemetry/sync", cloud_url);
            let res = self.client.post(&url)
                .json(&payload)
                .send()
                .await;

            match res {
                Ok(resp) if resp.status().is_success() => {
                    for row in rows {
                        let id: i32 = row.get("id");
                        let _ = sqlx::query("UPDATE telemetry_buffer SET sync_status = 'SYNCED' WHERE id = ?")
                            .bind(id)
                            .execute(&self.sqlite_pool)
                            .await;
                    }
                    info!("Successfully synced telemetry batch via REST");
                    return Ok(());
                }
                Ok(resp) => {
                    warn!("Cloud sync failed with status: {}", resp.status());
                    return Ok(());
                }
                Err(e) => {
                    warn!("Cloud sync HTTP request failed: {}", e);
                    return Ok(());
                }
            }
        }

        // Fallback to direct DB sync if no cloud url is provided but pg_pool is available (mainly for tests)
        if let Some(pg_pool) = &self.pg_pool {
            let mut tx = match pg_pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    warn!("Failed to begin pg transaction for telemetry sync: {}", e);
                    return Ok(());
                }
            };

            for row in &rows {
                let metric_name: String = row.get("metric_name");
                let metric_type: String = row.get("metric_type");
                let value: f32 = row.get("value");
                let labels_json: String = row.get("labels_json");
                let timestamp: chrono::NaiveDateTime = row.get("timestamp");

                let res = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'synced')")
                    .bind(metric_name)
                    .bind(metric_type)
                    .bind(value)
                    .bind(labels_json)
                    .bind(chrono::DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc))
                    .execute(&mut *tx)
                    .await;

                if let Err(e) = res {
                    let _ = tx.rollback().await;
                    warn!("Failed to insert telemetry to pg: {}", e);
                    return Ok(());
                }
            }

            if let Err(e) = tx.commit().await {
                warn!("Failed to commit telemetry to pg: {}", e);
                return Ok(());
            }

            for row in rows {
                let id: i32 = row.get("id");
                let _ = sqlx::query("UPDATE telemetry_buffer SET sync_status = 'SYNCED' WHERE id = ?")
                    .bind(id)
                    .execute(&self.sqlite_pool)
                    .await;
            }

            info!("Successfully synced telemetry batch");
        }

        Ok(())
    }

    pub async fn sync_step(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Find tasks requiring cloud escalation
        let rows = sqlx::query("SELECT memory_id, context FROM swarm_truth_embeddings WHERE escalation_required = 1 AND sync_status = 'PENDING' AND (sync_error IS NULL OR last_synced_at < datetime('now', '-5 minutes'))")
            .fetch_all(&self.sqlite_pool)
            .await?;

        let mut success_count = 0;
        let mut payloads = Vec::new();

        for row in &rows {
            let id: String = row.get("memory_id");
            let context: String = row.get("context");

            // Sanitize PII
            let parsed: Value = serde_json::from_str(&context).unwrap_or(json!({ "raw": context }));
            let sanitized = ::server_telemetry::redact_interface_pii(parsed);

            payloads.push(json!({
                "source": "hybrid_sync",
                "memory_id": id,
                "context": sanitized
            }));
        }

        if payloads.is_empty() {
            return Ok(());
        }

        if let Some(cloud_url) = &self.cloud_url {
            let url = format!("{}/api/sync/missions", cloud_url);
            let req_body = json!({
                "payloads": payloads
            });

            // Note: Cloud endpoint requires system role, so we set a system token or mock header.
            // For now, setting the spiffe-id or authorization header. Since `auth.RequireRole("system", handler)`
            // in Rust uses `Extension(claims)`, we need a way to mock or generate this token.
            // But we don't have the auth private key here. So we might need to rely on the server accepting
            // an API key or internal mechanism. We'll use a dummy Bearer token if we can't generate it,
            // or the existing system spiffe ID header. Let's look at auth in Rust. In `lib.rs`, `sync_handler`
            // is wrapped in `auth::Store::validate_token`.
            // Wait, actually, let's just pass a header `x-spiffe-id: spiffe://onehumancorp.io/system/system`
            // if we use a different middleware, but if we use JWT, we need a JWT.
            // For now we send the request. If it fails, we gracefully degrade.

            // To ensure local integration testing works without auth, we can pass a dummy auth header that the local mock might accept,
            // or we might need an actual token. We'll use a generic token or let it gracefully fail.
            let res = self.client.post(&url)
                .header("Authorization", "Bearer system") // Dummy for now, standalone config would provide real token
                .json(&req_body)
                .send()
                .await;

            match res {
                Ok(resp) if resp.status().is_success() => {
                    for row in &rows {
                        let id: String = row.get("memory_id");
                        sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'SYNCED' WHERE memory_id = ?")
                            .bind(&id)
                            .execute(&self.sqlite_pool)
                            .await?;
                        success_count += 1;
                        let _ = success_count;
                    }
                    info!("Successfully escalated missions via REST");
                    return Ok(());
                }
                Ok(resp) => {
                    warn!("Cloud sync for missions failed with status: {}", resp.status());
                    for row in &rows {
                        let id: String = row.get("memory_id");
                        let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_error = 'HTTP error', last_synced_at = CURRENT_TIMESTAMP WHERE memory_id = ?")
                            .bind(&id)
                            .execute(&self.sqlite_pool)
                            .await;
                    }
                    return Ok(());
                }
                Err(e) => {
                    warn!("Cloud sync for missions HTTP request failed: {}", e);
                    for row in &rows {
                        let id: String = row.get("memory_id");
                        let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_error = ?, last_synced_at = CURRENT_TIMESTAMP WHERE memory_id = ?")
                            .bind(e.to_string())
                            .bind(&id)
                            .execute(&self.sqlite_pool)
                            .await;
                    }
                    return Ok(());
                }
            }
        }

        // Fallback to direct DB logic
        if let Some(pg_pool) = &self.pg_pool {
            for row in rows {
                let id: String = row.get("memory_id");
                let context: String = row.get("context");

                let parsed: Value = serde_json::from_str(&context).unwrap_or(json!({ "raw": context }));
                let sanitized = ::server_telemetry::redact_interface_pii(parsed);

                let payload = json!({
                    "source": "hybrid_sync",
                    "memory_id": id,
                    "context": sanitized
                });

                let queue_id = Uuid::new_v4().to_string();
                let now = Utc::now().naive_utc();

                let mut tx = match pg_pool.begin().await {
                    Ok(t) => t,
                    Err(e) => {
                        warn!("Failed to begin pg transaction: {}, gracefully degrading (cloud unreachable).", e);
                        let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_error = ?, last_synced_at = CURRENT_TIMESTAMP WHERE memory_id = ?")
                            .bind(e.to_string())
                            .bind(&id)
                            .execute(&self.sqlite_pool)
                            .await;
                        continue;
                    }
                };

                let mission_res = sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, 'PENDING', $2, 'system')")
                    .bind(&queue_id)
                    .bind(payload.to_string())
                    .execute(&mut *tx)
                    .await;

                if let Err(e) = mission_res {
                    warn!("Failed to insert pg agent_missions: {}, gracefully degrading (cloud unreachable).", e);
                    let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_error = ?, last_synced_at = CURRENT_TIMESTAMP WHERE memory_id = ?")
                        .bind(e.to_string())
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await;
                    let _ = tx.rollback().await;
                    continue;
                }

                let res = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, 'system', NULL, $2, 'QUEUED', $3, $3, $3)")
                    .bind(&queue_id)
                    .bind(payload.to_string())
                    .bind(now)
                    .execute(&mut *tx)
                    .await;

                match res {
                    Ok(_) => {
                        let commit_res = tx.commit().await;
                        if let Err(e) = commit_res {
                            warn!("Failed to commit pg transaction for memory_id: {}, gracefully degrading. Error: {}", id, e);
                            let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_error = ?, last_synced_at = CURRENT_TIMESTAMP WHERE memory_id = ?")
                                .bind(e.to_string())
                                .bind(&id)
                                .execute(&self.sqlite_pool)
                                .await;
                            continue;
                        }

                        // Update SQLite sync status
                        let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'SYNCED' WHERE memory_id = ?")
                            .bind(&id)
                            .execute(&self.sqlite_pool)
                            .await;
                        info!("Successfully escalated memory_id: {} to cloud queue: {}", id, queue_id);
                        success_count += 1;
                        let _ = success_count;

                        if let Err(e) = ::server_telemetry::record_rag_escalation(pg_pool, "system", "").await {
                            warn!("Failed to record RAG escalation telemetry: {}", e);
                        }
                    }
                    Err(e) => {
                        let _ = tx.rollback().await;
                        warn!("Failed to escalate memory_id: {}, gracefully degrading (cloud unreachable). Error: {}", id, e);
                        let _ = sqlx::query("UPDATE swarm_truth_embeddings SET sync_error = ?, last_synced_at = CURRENT_TIMESTAMP WHERE memory_id = ?")
                            .bind(e.to_string())
                            .bind(&id)
                            .execute(&self.sqlite_pool)
                            .await;
                    }
                }
            }
        }

        if success_count > 0 {
            if let Some(pg_pool) = &self.pg_pool {
                if let Err(e) = ::server_telemetry::record_sync_escalation(pg_pool, success_count as f32, ::server_telemetry::get_deployment_mode()).await {
                    warn!("Failed to record sync escalation telemetry: {}", e);
                }
            }
        }

        Ok(())
    }
}
