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
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let mut current_depths = std::collections::HashMap::new();

        let mut query_str = String::from("INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after, organization_id) VALUES ");
        let mut values = Vec::new();

        for i in 0..jobs.len() {
            let base = i * 6;
            values.push(format!("(${}, ${}, ${}, ${}, 'QUEUED', ${}, ${})", base + 1, base + 2, base + 3, base + 4, base + 5, base + 6));
        }
        query_str.push_str(&values.join(", "));

        let mut query = sqlx::query(&query_str);

        let mut unique_tenants = std::collections::HashSet::new();
        for job in &jobs {
            unique_tenants.insert(job.tenant_id.clone());
        }

        let tenants_vec: Vec<String> = unique_tenants.into_iter().collect();
        if !tenants_vec.is_empty() {
            if let Ok(rows) = sqlx::query("SELECT organization_id, COUNT(*) FROM sub_agent_jobs WHERE organization_id = ANY($1) AND status = 'QUEUED' GROUP BY organization_id")
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
        for job in jobs {
            let depth = *current_depths.get(&job.tenant_id).unwrap_or(&0);

            let mut run_after = job.run_after;
            if depth > bursts_threshold {
                let delay_seconds = (depth - bursts_threshold) * 5;
                run_after = run_after + chrono::Duration::seconds(delay_seconds);
            }
            *current_depths.entry(job.tenant_id.clone()).or_insert(0) += 1;

            let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);
            query = query
                .bind(job.id)
                .bind(job.parent_task_id)
                .bind(job.agent_role)
                .bind(payload_json)
                .bind(run_after)
                .bind(job.tenant_id);
        }

        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sub_agent_jobs WHERE organization_id = $1 AND status = 'QUEUED'")
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
             VALUES ($1, $2, $3, $4, 'QUEUED', $5, $6)"
        )
        .bind(&job.id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(payload_json)
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

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let role_placeholders = roles.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(",");
        let query_str = format!(
            "UPDATE sub_agent_jobs SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM sub_agent_jobs
                 WHERE status = 'QUEUED' AND run_after <= CURRENT_TIMESTAMP AND agent_role IN ({})
                 ORDER BY run_after ASC, created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, organization_id",
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
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();

            let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());
            let latency = (chrono::Utc::now() - created_at).num_milliseconds() as f64 / 1000.0;
            ::server_telemetry::record_sub_agent_queue_delay(latency);

            let job = Job {
                id: row.get("id"),
                parent_task_id: row.try_get("parent_task_id").unwrap_or_default(),
                agent_role: row.try_get("agent_role").unwrap_or_default(),
                payload: payload_str,
                status: row.try_get("status").unwrap_or_default(),
                attempts: row.try_get("attempts").unwrap_or(0),
                max_attempts: row.try_get("max_attempts").unwrap_or(3),
                run_after: row.try_get("run_after").unwrap_or_else(|_| chrono::Utc::now()),
                locked_until: row.try_get("locked_until").unwrap_or(None),
                created_at,
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
                tenant_id: row.try_get("organization_id").unwrap_or_default(),
            };

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(Some(job));
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _reason: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT attempts, max_attempts FROM sub_agent_jobs WHERE id = $1 FOR UPDATE")
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
                sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', attempts = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(next_attempt)
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                // Exponential backoff
                let backoff_seconds = 1 << next_attempt;
                let new_run_after = chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds as i64);
                sqlx::query("UPDATE sub_agent_jobs SET status = 'QUEUED', attempts = $1, run_after = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3")
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
