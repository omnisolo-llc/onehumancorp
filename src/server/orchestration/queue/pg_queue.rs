use super::queue::{Job, TaskQueue};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;

pub struct PgTaskQueue {
    pool: Arc<PgPool>,
}

impl PgTaskQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskQueue for PgTaskQueue {
async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        ::server_telemetry::record_queue_length_sync(jobs.len() as i32, ::server_telemetry::get_deployment_mode());
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let mut current_depths = std::collections::HashMap::new();

        let mut unique_tenants = std::collections::HashSet::new();
        for job in &jobs {
            unique_tenants.insert(job.tenant_id.clone());
        }

        let tenants_vec: Vec<String> = unique_tenants.into_iter().collect();
        if !tenants_vec.is_empty() {
            if let Ok(rows) = sqlx::query("SELECT tenant_id, COUNT(*) FROM ohc_job_queue WHERE tenant_id = ANY($1) AND status = 'PENDING' GROUP BY tenant_id")
                .bind(&tenants_vec)
                .fetch_all(&mut *tx)
                .await
            {
                for row in rows {
                    let org_id: String = row.try_get(0).unwrap_or_default();
                    let count: i64 = row.try_get(1).unwrap_or(0);
                    current_depths.insert(org_id, count);
                }
            }
        }

        let bursts_threshold = 10;

        // Chunk jobs to avoid Postgres parameter limits (65535 parameters max)
        // We have 6 parameters per insert, so safe max is ~10,000. We use 5,000 for safety.
        for chunk in jobs.chunks(5000) {
            let mut query_builder = sqlx::QueryBuilder::new("INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id) ");
            query_builder.push_values(chunk, |mut b, job| {
                let depth = *current_depths.get(&job.tenant_id).unwrap_or(&0);
                let mut next_retry_at = job.next_retry_at;
                if depth > bursts_threshold {
                    let delay_seconds = (depth - bursts_threshold) * 5;
                    next_retry_at = next_retry_at + chrono::Duration::seconds(delay_seconds);
                }

                // Note: Rust doesn't allow mutation here easily due to ownership,
                // but depth is only an estimate anyway so it's acceptable.
                // We could collect all modified next_retry_ats prior to this step if exact counting was crucial.

                let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

                b.push_bind(job.id.clone())
                 .push_bind(job.parent_task_id.clone())
                 .push_bind(job.job_type.clone())
                 .push_bind(payload_json)
                 .push_bind("PENDING")
                 .push_bind(next_retry_at.to_rfc3339())
                 .push_bind(job.tenant_id.clone());
            });

            let query = query_builder.build();
            query.execute(&mut *tx).await.map_err(|e| e.to_string())?;

            // Update depths for subsequent chunks
            for job in chunk {
                *current_depths.entry(job.tenant_id.clone()).or_insert(0) += 1;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        ::server_telemetry::record_queue_length_sync(1, ::server_telemetry::get_deployment_mode());
        let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = $1 AND status = 'PENDING'")
            .bind(&job.tenant_id)
            .fetch_one(&*self.pool)
            .await
            .unwrap_or((0,));

        let mut next_retry_at = job.next_retry_at;
        let bursts_threshold = 10;
        if count_row.0 > bursts_threshold {
            let delay_seconds = (count_row.0 - bursts_threshold) * 5;
            next_retry_at = next_retry_at + chrono::Duration::seconds(delay_seconds);
        }

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id)
             VALUES ($1, $2, $3, $4, 'PENDING', $5, $6)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.job_type)
        .bind(payload_json)
        .bind(next_retry_at.to_rfc3339())
        .bind(&job.tenant_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>, _estimated_vram: i64, _estimated_tokens: i64) -> Result<Option<Job>, String> {
        if roles.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let role_placeholders = roles.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM ohc_job_queue
                 WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type IN ({})
                 ORDER BY next_retry_at ASC, created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING id, parent_task_id, job_type, payload, status, retry_count, max_retries, next_retry_at, locked_until, created_at, updated_at, tenant_id",
            role_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let start_poll = std::time::Instant::now();
        let job_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if start_poll.elapsed() > std::time::Duration::from_millis(100) {
            ::server_telemetry::record_task_claim_contention(::server_telemetry::get_deployment_mode());
        }

        if let Some(row) = job_opt {
            ::server_telemetry::record_queue_length_sync(-1, ::server_telemetry::get_deployment_mode());
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();

            let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());
            let latency = (chrono::Utc::now() - created_at).num_milliseconds() as f64 / 1000.0;
            ::server_telemetry::record_sub_agent_queue_delay(latency, ::server_telemetry::get_deployment_mode());

            let job = Job {
                id: row.get("id"),
                parent_task_id: row.try_get("parent_task_id").unwrap_or_default(),
                job_type: row.try_get("job_type").unwrap_or_default(),
                payload: payload_str,
                status: row.try_get("status").unwrap_or_default(),
                retry_count: row.try_get("retry_count").unwrap_or(0),
                max_retries: row.try_get("max_retries").unwrap_or(3),
                next_retry_at: row.try_get("next_retry_at").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at,
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
                tenant_id: row.try_get("tenant_id").unwrap_or_default(),
            };

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        let row = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 RETURNING updated_at, next_retry_at")
            .bind(job_id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            ::server_telemetry::record_queue_length_sync(-1, ::server_telemetry::get_deployment_mode());
            use sqlx::Row;
            let updated: chrono::DateTime<chrono::Utc> = r.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now());
            let next_retry_at: chrono::DateTime<chrono::Utc> = r.try_get("next_retry_at").unwrap_or_else(|_| chrono::Utc::now());
            let latency = (updated - next_retry_at).num_milliseconds() as f64 / 1000.0;
            ::server_telemetry::record_task_processing_latency(::server_telemetry::get_deployment_mode(), latency);
        }
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT retry_count, max_retries, tenant_id, payload FROM ohc_job_queue WHERE id = $1 FOR UPDATE")
            .bind(job_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_retry_count: i32 = r.try_get("retry_count").unwrap_or(0);
            let max_retries: i32 = r.try_get("max_retries").unwrap_or(3);
            let next_attempt = current_retry_count + 1;

            if next_attempt >= max_retries {
                // Poison pill
                let tenant_id: String = r.try_get("tenant_id").unwrap_or_default();
                let payload: serde_json::Value = r.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));
                let payload_str = serde_json::to_string(&payload).unwrap_or_default();
                sqlx::query("INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(uuid::Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .bind("job_failed")
                    .bind("job_queue")
                    .bind(&payload_str)
                    .bind(_reason)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', retry_count = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(next_attempt)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_attempt;
                let new_next_retry_at = (chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64)).to_rfc3339();
                sqlx::query("UPDATE ohc_job_queue SET status = 'PENDING', retry_count = $1, next_retry_at = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
                    .bind(next_attempt)
                    .bind(new_next_retry_at)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
