#![allow(dead_code)]

use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use sqlx::{PgPool, SqlitePool, Row};
use prost::Message;

pub use crate::ohc::orchestration::Job;

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn enqueue(&self, job: Job) -> Result<(), String>;
    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String>;
    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String>;
    async fn requeue(&self, job: Job) -> Result<(), String>;
}

pub struct MemoryTaskQueue {
    jobs: DashMap<String, Job>,
}

impl MemoryTaskQueue {
    pub fn new() -> Self {
        MemoryTaskQueue {
            jobs: DashMap::new(),
        }
    }
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        self.jobs.insert(job.id.clone(), job);
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let now = chrono::Utc::now().timestamp();

        for mut entry in self.jobs.iter_mut() {
            let job = entry.value_mut();
            if job.status == "PENDING" && roles.contains(&job.agent_role) && job.run_after <= now && (job.locked_until == 0 || job.locked_until < now) {
                job.locked_until = now + 300; // Lock for 5 mins
                return Ok(Some(job.clone()));
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "COMPLETED".to_string();
            job.updated_at = chrono::Utc::now().timestamp();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, _reason: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "FAILED".to_string();
            job.updated_at = chrono::Utc::now().timestamp();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        self.jobs.insert(job.id.clone(), job);
        Ok(())
    }
}

pub struct PostgresTaskQueue {
    pool: Arc<PgPool>,
}

impl PostgresTaskQueue {
    pub fn new(pool: Arc<PgPool>) -> Self {
        PostgresTaskQueue { pool }
    }
}

#[async_trait]
impl TaskQueue for PostgresTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO shared_tasks (id, organization_id, parent_plan_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, protobuf_blob)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, to_timestamp($9::double precision), CASE WHEN $10 > 0 THEN to_timestamp($10::double precision) ELSE NULL END, to_timestamp($11::double precision), to_timestamp($12::double precision), $13)"
        )
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(&job.payload)
        .bind(&job.status)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.run_after)
        .bind(if job.locked_until > 0 { Some(job.locked_until) } else { None })
        .bind(job.created_at)
        .bind(job.updated_at)
        .bind(buf)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() {
            return Ok(None);
        }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let roles_placeholders = roles.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<_>>().join(", ");
        let query_str = format!(
            "SELECT id, protobuf_blob
             FROM shared_tasks
             WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < now()) AND run_after <= extract(epoch from now())::bigint AND agent_role IN ({}) AND protobuf_blob IS NOT NULL
             ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED",
            roles_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let row_opt = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            let id: String = row.get("id");
            let blob: Vec<u8> = row.get("protobuf_blob");

            sqlx::query("UPDATE shared_tasks SET locked_until = now() + interval '5 minutes' WHERE id = $1")
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            if let Ok(job) = Job::decode(&blob[..]) {
                return Ok(Some(job));
            }
        }

        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = now() WHERE id = $1 AND organization_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'FAILED', updated_at = now() WHERE id = $1 AND organization_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        sqlx::query(
            "UPDATE shared_tasks SET status = 'PENDING', attempts = $1, max_attempts = $2, run_after = to_timestamp($3::double precision), locked_until = NULL, updated_at = now(), protobuf_blob = $6 WHERE id = $4 AND organization_id = $5"
        )
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.run_after)
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(buf)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct SqliteTaskQueue {
    pool: Arc<SqlitePool>,
}

impl SqliteTaskQueue {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        SqliteTaskQueue { pool }
    }
}

#[async_trait]
impl TaskQueue for SqliteTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        let locked_until = if job.locked_until > 0 { Some(job.locked_until) } else { None };

        sqlx::query(
            "INSERT INTO shared_tasks (id, organization_id, parent_plan_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at, protobuf_blob)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime(?, 'unixepoch'), CASE WHEN ? IS NOT NULL THEN datetime(?, 'unixepoch') ELSE NULL END, datetime(?, 'unixepoch'), datetime(?, 'unixepoch'), ?)"
        )
        .bind(&job.id)
        .bind(&job.tenant_id)
        .bind(&job.parent_task_id)
        .bind(&job.agent_role)
        .bind(&job.payload)
        .bind(&job.status)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.run_after)
        .bind(locked_until)
        .bind(locked_until)
        .bind(job.created_at)
        .bind(job.updated_at)
        .bind(buf)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() {
            return Ok(None);
        }

        let roles_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query_str = format!(
            "SELECT id, protobuf_blob
             FROM shared_tasks
             WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < datetime('now')) AND run_after <= strftime('%s', 'now') AND agent_role IN ({}) AND protobuf_blob IS NOT NULL
             ORDER BY created_at ASC LIMIT 1",
            roles_placeholders
        );

        let mut query = sqlx::query(&query_str);
        for role in &roles {
            query = query.bind(role);
        }

        let row: Option<sqlx::sqlite::SqliteRow> = query
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let blob: Vec<u8> = row.get("protobuf_blob");

            sqlx::query("UPDATE shared_tasks SET locked_until = datetime('now', '+5 minutes') WHERE id = ?")
                .bind(&id)
                .execute(&*self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if let Ok(job) = Job::decode(&blob[..]) {
                return Ok(Some(job));
            }
        }

        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = datetime('now') WHERE id = ? AND organization_id = ?")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, _reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE shared_tasks SET status = 'FAILED', updated_at = datetime('now') WHERE id = ? AND organization_id = ?")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        sqlx::query(
            "UPDATE shared_tasks SET status = 'PENDING', attempts = ?, max_attempts = ?, run_after = datetime(?, 'unixepoch'), locked_until = NULL, updated_at = datetime('now'), protobuf_blob = ? WHERE id = ? AND organization_id = ?"
        )
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.run_after)
        .bind(buf)
        .bind(&job.id)
        .bind(&job.tenant_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct RedisTaskQueue {
    client: redis::Client,
    queue_name: String,
}

impl RedisTaskQueue {
    pub fn new(redis_url: &str, queue_name: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            queue_name: queue_name.to_string(),
        })
    }
}

#[async_trait]
impl TaskQueue for RedisTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        let mut buf = Vec::new();
        job.encode(&mut buf).map_err(|e| e.to_string())?;

        let _: () = redis::cmd("RPUSH")
            .arg(&self.queue_name)
            .arg(buf)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        let result: Option<(String, Vec<u8>)> = redis::cmd("BLPOP")
            .arg(&self.queue_name)
            .arg(1)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if let Some((_, payload)) = result {
            if let Ok(job) = Job::decode(&payload[..]) {
                if roles.contains(&job.agent_role) {
                    return Ok(Some(job));
                } else {
                    let _ = self.enqueue(job).await;
                }
            }
        }
        Ok(None)
    }

    async fn complete(&self, _job_id: &str, _tenant_id: &str) -> Result<(), String> {
        Ok(())
    }

    async fn fail(&self, _job_id: &str, _tenant_id: &str, _reason: &str) -> Result<(), String> {
        Ok(())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        self.enqueue(job).await
    }
}

#[async_trait]
pub trait TaskJobHandler: Send + Sync {
    async fn handle_job(&self, job: Job) -> Result<(), String>;
}

pub struct QueueManager {
    queue: Arc<dyn TaskQueue>,
    roles: Vec<String>,
    handler: Arc<dyn TaskJobHandler>,
    cancel_tx: tokio::sync::broadcast::Sender<()>,
}

impl QueueManager {
    pub fn new(queue: Arc<dyn TaskQueue>, roles: Vec<String>, handler: Arc<dyn TaskJobHandler>) -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(1);
        Self {
            queue,
            roles,
            handler,
            cancel_tx: tx,
        }
    }

    pub fn start(&self) {
        let queue = self.queue.clone();
        let roles = self.roles.clone();
        let handler = self.handler.clone();
        let mut rx = self.cancel_tx.subscribe();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = rx.recv() => {
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(500)) => {
                        if let Ok(Some(job)) = queue.dequeue(roles.clone()).await {
                            let job_id = job.id.clone();
                            let tenant_id = job.tenant_id.clone();
                            if let Err(e) = handler.handle_job(job.clone()).await {
                                tracing::error!("Failed to handle job {}: {}", job_id, e);
                                if job.attempts < job.max_attempts {
                                    let mut new_job = job.clone();
                                    new_job.attempts += 1;
                                    new_job.run_after = chrono::Utc::now().timestamp() + (2_i64.pow(new_job.attempts as u32) * 60);
                                    let _ = queue.requeue(new_job).await;
                                } else {
                                    let _ = queue.fail(&job_id, &tenant_id, &e).await;
                                }
                            } else {
                                let _ = queue.complete(&job_id, &tenant_id).await;
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        let _ = self.cancel_tx.send(());
    }
}
