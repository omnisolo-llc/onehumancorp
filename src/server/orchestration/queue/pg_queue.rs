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
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let mut ids = Vec::with_capacity(jobs.len());
        let mut parent_task_ids = Vec::with_capacity(jobs.len());
        let mut job_types = Vec::with_capacity(jobs.len());
        let mut payloads = Vec::with_capacity(jobs.len());
        let mut statuses = Vec::with_capacity(jobs.len());
        let mut next_retry_ats = Vec::with_capacity(jobs.len());
        let mut tenant_ids = Vec::with_capacity(jobs.len());

        for job in &jobs {
            let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

            ids.push(job.id.clone());
            parent_task_ids.push(job.parent_task_id.clone());
            job_types.push(job.job_type.clone());
            payloads.push(payload_json);
            statuses.push("PENDING".to_string());
            next_retry_ats.push(job.next_retry_at);
            tenant_ids.push(job.tenant_id.clone());
        }

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id)
             SELECT unnest.id, unnest.parent_task_id, unnest.job_type, unnest.payload, unnest.status, unnest.next_retry_at, unnest.tenant_id FROM UNNEST($1::text[], $2::text[], $3::text[], $4::jsonb[], $5::text[], $6::timestamptz[], $7::text[]) AS unnest(id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id)"
        )
        .bind(&ids)
        .bind(&parent_task_ids)
        .bind(&job_types)
        .bind(&payloads)
        .bind(&statuses)
        .bind(&next_retry_ats)
        .bind(&tenant_ids)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await.map_err(|e| e.to_string())?;
        ::server_telemetry::record_queue_length_sync(1, ::server_telemetry::get_deployment_mode());
        let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id)
             VALUES ($1, $2, $3, $4, 'PENDING', $5, $6)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.job_type)
        .bind(payload_json)
        .bind(job.next_retry_at)
        .bind(&job.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>, _estimated_vram: i64, _estimated_tokens: i64) -> Result<Option<Job>, String> {
        if roles.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

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
        let job_opt = tokio::time::timeout(std::time::Duration::from_secs(60), query.fetch_optional(&mut *tx)).await.map_err(|_| "Timeout fetching job from queue".to_string())?.map_err(|e| e.to_string())?;

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
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;
        let row = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 RETURNING updated_at, next_retry_at")
            .bind(job_id)
            .fetch_optional(&mut *tx)
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
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn cleanup_stale_jobs(&self) -> Result<u64, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        let query_str = "UPDATE ohc_job_queue SET status = 'PENDING', retry_count = retry_count + 1, updated_at = CURRENT_TIMESTAMP WHERE status = 'PROCESSING' AND updated_at < CURRENT_TIMESTAMP - INTERVAL '1 hour'";

        let result = sqlx::query(query_str)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let query_str = "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT gen_random_uuid()::text, tenant_id, 'job_failed', 'job_queue', COALESCE(payload::text, '{}'), '[cleanup] Stagnant backlog item stuck in PENDING for > 24 hours' FROM ohc_job_queue WHERE status = 'PENDING' AND updated_at < CURRENT_TIMESTAMP - INTERVAL '24 hours'";

        sqlx::query(query_str)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let query_str = "DELETE FROM ohc_job_queue WHERE status = 'PENDING' AND updated_at < CURRENT_TIMESTAMP - INTERVAL '24 hours'";

        let stagnant_result = sqlx::query(query_str)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(result.rows_affected() + stagnant_result.rows_affected())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

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
                    .bind(job_id)
                    .bind(&tenant_id)
                    .bind("job_failed")
                    .bind("job_queue")
                    .bind(&payload_str)
                    .bind(_reason)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("UPDATE agents SET status = 'PAUSED' WHERE tenant_id = $1 AND status != 'PAUSED'")
                    .bind(&tenant_id)
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
                let new_next_retry_at = chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64);
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
