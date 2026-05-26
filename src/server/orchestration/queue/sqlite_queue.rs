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

        let mut query_str = String::from("INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after, organization_id) VALUES ");
        let mut values = Vec::new();

        for _ in 0..jobs.len() {
            values.push("(?, ?, ?, ?, 'QUEUED', ?, ?)");
        }
        query_str.push_str(&values.join(", "));

        let mut query = sqlx::query(&query_str);

        let mut unique_tenants = std::collections::HashSet::new();
        for job in &jobs {
            unique_tenants.insert(job.tenant_id.clone());
        }

        if !unique_tenants.is_empty() {
            let placeholders: Vec<_> = unique_tenants.iter().map(|_| "?").collect();
            let count_query = format!("SELECT organization_id, COUNT(*) FROM sub_agent_jobs WHERE organization_id IN ({}) AND status = 'QUEUED' GROUP BY organization_id", placeholders.join(","));

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
        for job in jobs {
            let depth = *current_depths.get(&job.tenant_id).unwrap_or(&0);

            let mut run_after = job.run_after;
            if depth > bursts_threshold {
                let delay_seconds = (depth - bursts_threshold) * 5;
                run_after = run_after + chrono::Duration::seconds(delay_seconds);
            }
            *current_depths.entry(job.tenant_id.clone()).or_insert(0) += 1;

            query = query
                .bind(job.id)
                .bind(job.parent_task_id)
                .bind(job.agent_role)
                .bind(job.payload)
                .bind(run_after)
                .bind(job.tenant_id);
        }

        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        ::server_telemetry::record_queue_length_sync(1, ::server_telemetry::get_deployment_mode());
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sub_agent_jobs WHERE organization_id = ? AND status = 'QUEUED'")
            .bind(&job.tenant_id)
            .fetch_one(&*self.pool)
            .await
            .unwrap_or((0,));

        let mut run_after = job.run_after;
        let bursts_threshold = 10;
        if count_row.0 > bursts_threshold {
            let delay_seconds = (count_row.0 - bursts_threshold) * 5;
            run_after = run_after + chrono::Duration::seconds(delay_seconds);
        }

        sqlx::query(
            "INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after, organization_id)
             VALUES (?, ?, ?, ?, 'QUEUED', ?, ?)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(&job.payload)
        .bind(run_after)
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
            "SELECT id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, organization_id
             FROM sub_agent_jobs
             WHERE status = 'QUEUED' AND run_after <= CURRENT_TIMESTAMP AND agent_role IN ({})
             ORDER BY run_after ASC, created_at ASC
             LIMIT 1",
            role_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let start_poll = std::time::Instant::now();
        let job_opt: Option<sqlx::sqlite::SqliteRow> = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

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
                agent_role: row.get("agent_role"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                run_after: row.try_get("run_after").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at,
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
                tenant_id: row.try_get("organization_id").unwrap_or_default(),
            };

            sqlx::query("UPDATE sub_agent_jobs SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
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
        let row = sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING updated_at, run_after")
            .bind(job_id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            ::server_telemetry::record_queue_length_sync(-1, ::server_telemetry::get_deployment_mode());
            use sqlx::Row;
            let updated: chrono::DateTime<chrono::Utc> = r.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now());
            let run_after: chrono::DateTime<chrono::Utc> = r.try_get("run_after").unwrap_or_else(|_| chrono::Utc::now());
            let latency = (updated - run_after).num_milliseconds() as f64 / 1000.0;
            ::server_telemetry::record_task_processing_latency(::server_telemetry::get_deployment_mode(), latency);
        }
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT attempts, max_attempts FROM sub_agent_jobs WHERE id = ?")
            .bind(job_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_attempts: i32 = r.try_get("attempts").unwrap_or(0);
            let max_attempts: i32 = r.try_get("max_attempts").unwrap_or(3);
            let next_attempt = current_attempts + 1;

            if next_attempt >= max_attempts {
                // Poison pill
                sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', attempts = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(next_attempt)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_attempt;
                let new_run_after = chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64);
                sqlx::query("UPDATE sub_agent_jobs SET status = 'QUEUED', attempts = ?, run_after = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(next_attempt)
                    .bind(new_run_after)
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
