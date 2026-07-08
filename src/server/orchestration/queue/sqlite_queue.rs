use super::queue::{Job, TaskQueue};
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use std::sync::Arc;

pub struct SQLiteTaskQueue {
    pool: Arc<SqlitePool>,
    mu: tokio::sync::Mutex<()>,
}

impl SQLiteTaskQueue {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool, mu: tokio::sync::Mutex::new(()) }
    }
}

#[async_trait]
impl TaskQueue for SQLiteTaskQueue {
async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        ::server_telemetry::record_queue_length_sync(jobs.len() as i32, ::server_telemetry::get_deployment_mode());
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let mut current_depths = std::collections::HashMap::new();
        let mut unique_tenants = std::collections::HashSet::new();
        for job in &jobs {
            unique_tenants.insert(job.tenant_id.clone());
        }

        if !unique_tenants.is_empty() {
            let placeholders: Vec<_> = unique_tenants.iter().map(|_| "?").collect();
            let count_query = format!("SELECT tenant_id, COUNT(*) FROM ohc_job_queue WHERE tenant_id IN ({}) AND status = 'PENDING' GROUP BY tenant_id", placeholders.join(","));

            let mut q = sqlx::query(&count_query);
            for tenant in &unique_tenants {
                q = q.bind(tenant);
            }

            if let Ok(rows) = q.fetch_all(&mut *tx).await {
                for row in rows {
                    let org_id: String = row.try_get(0).unwrap_or_default();
                    let count: i64 = row.try_get(1).unwrap_or(0);
                    current_depths.insert(org_id, count);
                }
            }
        }

        let bursts_threshold = 10;

        // SQLite has a parameter limit (often 32766 or 999). We use a conservative chunk size of 100.
        // 6 parameters per job * 100 = 600 parameters < 999.
        for chunk in jobs.chunks(100) {
            let mut query_builder = sqlx::QueryBuilder::new("INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id) ");
            query_builder.push_values(chunk, |mut b, job| {
                let depth = *current_depths.get(&job.tenant_id).unwrap_or(&0);
                let mut next_retry_at = job.next_retry_at;
                if depth > bursts_threshold {
                    let delay_seconds = (depth - bursts_threshold) * 5;
                    next_retry_at = next_retry_at + chrono::Duration::seconds(delay_seconds);
                }

                b.push_bind(job.id.clone())
                 .push_bind(job.parent_task_id.clone())
                 .push_bind(job.job_type.clone())
                 .push_bind(job.payload.clone())
                 .push_bind("PENDING")
                 .push_bind(next_retry_at)
                 .push_bind(job.tenant_id.clone());
            });

            let query = query_builder.build();
            query.execute(&mut *tx).await.map_err(|e| e.to_string())?;

            for job in chunk {
                *current_depths.entry(job.tenant_id.clone()).or_insert(0) += 1;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        ::server_telemetry::record_queue_length_sync(1, ::server_telemetry::get_deployment_mode());
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = ? AND status = 'PENDING'")
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
             VALUES (?, ?, ?, ?, 'PENDING', ?, ?)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.job_type)
        .bind(&job.payload)
        .bind(next_retry_at)
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

        let lock_result = self.mu.try_lock();
        let _lock = match lock_result {
            Ok(guard) => guard,
            Err(_) => {
                let pg_pool = crate::db::get_pool();
                let _ = crate::telemetry::record_sqlite_lock_contention(&pg_pool, "PollTasks").await;
                self.mu.lock().await
            }
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let role_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, parent_task_id, job_type, payload, status, retry_count, max_retries, next_retry_at, locked_until, created_at, updated_at, tenant_id
             FROM ohc_job_queue
             WHERE status = 'PENDING' AND next_retry_at <= ? AND job_type IN ({})
             ORDER BY next_retry_at ASC, created_at ASC
             LIMIT 1",
            role_placeholders
        );

        let mut query = sqlx::query(&query_str);
        query = query.bind(chrono::Utc::now());
        for role in &roles {
            query = query.bind(role);
        }

        let start_poll = std::time::Instant::now();
        let job_opt: Option<sqlx::sqlite::SqliteRow> = tokio::time::timeout(std::time::Duration::from_secs(60), query.fetch_optional(&mut *tx)).await.map_err(|_| "Timeout fetching job from queue".to_string())?.map_err(|e| e.to_string())?;

        if start_poll.elapsed() > std::time::Duration::from_millis(100) {
            ::server_telemetry::record_task_claim_contention(::server_telemetry::get_deployment_mode());
        }

        if let Some(row) = job_opt {
            ::server_telemetry::record_queue_length_sync(-1, ::server_telemetry::get_deployment_mode());
            let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());
            let latency = (chrono::Utc::now() - created_at).num_milliseconds() as f64 / 1000.0;
            ::server_telemetry::record_sub_agent_queue_delay(latency, ::server_telemetry::get_deployment_mode());

            let job = Job {
                id: row.get("id"),
                parent_task_id: row.get("parent_task_id"),
                job_type: row.get("job_type"),
                payload: row.get("payload"),
                status: row.get("status"),
                retry_count: row.get("retry_count"),
                max_retries: row.get("max_retries"),
                next_retry_at: row.try_get("next_retry_at").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at,
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
                tenant_id: row.try_get("tenant_id").unwrap_or_default(),
            };

            sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(&job.id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        let row = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING updated_at, next_retry_at")
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

        let row = sqlx::query("SELECT retry_count, max_retries, tenant_id, payload FROM ohc_job_queue WHERE id = ?")
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
                let payload: String = r.try_get("payload").unwrap_or_else(|_| String::from("{}"));
                sqlx::query("INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(job_id)
                    .bind(&tenant_id)
                    .bind("job_failed")
                    .bind("job_queue")
                    .bind(&payload)
                    .bind(_reason)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("UPDATE agents SET status = 'PAUSED' WHERE tenant_id = ? AND status != 'PAUSED'")
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', retry_count = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(next_attempt)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_attempt;
                let new_next_retry_at = chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64);
                sqlx::query("UPDATE ohc_job_queue SET status = 'PENDING', retry_count = ?, next_retry_at = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
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
