#![allow(dead_code)]


use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use sqlx::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Job {
    pub id: String,
    pub tenant_id: String,
    pub parent_task_id: String,
    pub agent_role: String,
    pub payload: String,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub run_after: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
        for mut job_ref in self.jobs.iter_mut() {
            if job_ref.status == "PENDING" && roles.contains(&job_ref.agent_role) {
                job_ref.status = "IN_PROGRESS".to_string();
                job_ref.updated_at = Utc::now();
                return Ok(Some(job_ref.clone()));
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
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "FAILED".to_string();
            job.payload = format!("{} (Reason: {})", job.payload, reason);
            job.updated_at = Utc::now();
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
    pool: sqlx::PgPool,
}

impl PostgresTaskQueue {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PostgresTaskQueue { pool }
    }
}

#[async_trait]
impl TaskQueue for PostgresTaskQueue {
    async fn requeue(&self, job: Job) -> Result<(), String> {
        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();

        sqlx::query("UPDATE sub_agent_jobs SET status = 'PENDING', payload = $3, run_after = $4 WHERE id = $1 AND tenant_id = $2")
            .bind(&job.id)
            .bind(&job.tenant_id)
            .bind(new_payload)
            .bind(job.run_after)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let run_after = job.run_after;
        
        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["agent_role"] = serde_json::Value::String(job.agent_role.clone());
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
        
        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();
        
        let org_id = if job.tenant_id.is_empty() {
            payload_map["tenant_id"].as_str().unwrap_or("").to_string()
        } else {
            job.tenant_id.clone()
        };
        
        sqlx::query("INSERT INTO sub_agent_jobs (id, tenant_id, parent_task_id, payload, status, run_after) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(job.id)
            .bind(org_id)
            .bind(job.parent_task_id)
            .bind(new_payload)
            .bind("PENDING")
            .bind(run_after)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() { return Ok(None); }
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let row = sqlx::query("UPDATE sub_agent_jobs SET status = 'RUNNING' WHERE id = (SELECT id FROM sub_agent_jobs WHERE status = 'PENDING' AND run_after <= CURRENT_TIMESTAMP AND payload::json->>'agent_role' = ANY($1) ORDER BY run_after ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_task_id, payload, status, run_after")
            .bind(&roles)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
            
        if let Some(row) = row {
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let parent_task_id: String = row.get("parent_task_id");
            let payload: String = row.get("payload");
            let status: String = row.get("status");
            let scheduled_at: DateTime<Utc> = row.get("run_after");
            
            let mut j = Job {
                id,
                tenant_id: tenant_id,
                parent_task_id,
                agent_role: String::new(),
                payload: payload.clone(),
                status,
                attempts: 0,
                max_attempts: 3,
                run_after: scheduled_at,
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            let payload_map: serde_json::Value = serde_json::from_str(&payload).unwrap_or_else(|_| serde_json::json!({}));
            if let Some(role) = payload_map["agent_role"].as_str() {
                j.agent_role = role.to_string();
            }
            if let Some(attempts) = payload_map["attempts"].as_i64() {
                j.attempts = attempts as i32;
            }
            if let Some(max_attempts) = payload_map["max_attempts"].as_i64() {
                j.max_attempts = max_attempts as i32;
            }
            
            j.attempts += 1;
            
            Ok(Some(j))
        } else {
            Ok(None)
        }
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', completed_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String> {
        let error_payload = serde_json::to_string(&serde_json::json!({"error": reason}))
            .unwrap_or_else(|_| "{}".to_string());
        sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', payload = COALESCE(payload::jsonb, '{}'::jsonb) || $2::jsonb WHERE id = $1 AND tenant_id = $3")
            .bind(job_id)
            .bind(error_payload)
            .bind(tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }
}

#[async_trait]
pub trait TaskJobHandler: Send + Sync {
    async fn handle(&self, job: Job) -> Result<(), String>;
}

pub struct Worker {
    queue: Arc<dyn TaskQueue>,
    roles: Vec<String>,
    handler: Arc<dyn TaskJobHandler>,
}

impl Worker {
    pub fn new(queue: Arc<dyn TaskQueue>, roles: Vec<String>, handler: Arc<dyn TaskJobHandler>) -> Self {
        Worker { queue, roles, handler }
    }

    pub async fn start(&self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match self.queue.dequeue(self.roles.clone()).await {
                        Ok(Some(job)) => {
                            println!("Worker processing job: {}", job.id);
                            let handle_res = tokio::time::timeout(tokio::time::Duration::from_secs(60), self.handler.handle(job.clone())).await;
                            let handler_res = match handle_res {
                                Ok(Ok(())) => Ok(()),
                                Ok(Err(e)) => Err(e),
                                Err(_) => Err("Timeout executing job".to_string()),
                            };
                            match handler_res {
                                Ok(_) => {
                                    println!("Worker successfully processed job: {}", job.id);
                                    let _ = self.queue.complete(&job.id, &job.tenant_id).await;
                                }
                                Err(e) => {
                                    println!("Worker failed to process job: {}, error: {}", job.id, e);
                                    if job.attempts < job.max_attempts {
                                        let mut retry_job = job.clone();
                                        retry_job.attempts += 1;
                                        retry_job.status = "PENDING".to_string();
                                        retry_job.run_after = chrono::Utc::now() + chrono::Duration::seconds(5);
                                        let _ = self.queue.requeue(retry_job).await;
                                    } else {
                                        let _ = self.queue.fail(&job.id, &job.tenant_id, &e).await;
                                    }
                                }
                            }
                        }
                        Ok(None) => {
                            // No job available
                        }
                        Err(e) => {
                            println!("Worker failed to dequeue job: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    println!("Worker shutting down");
                    break;
                }
            }
        }
    }
}

#[async_trait]
pub trait JobQueue: Send + Sync {
    async fn push(&self, topic: &str, payload: Vec<u8>) -> Result<(), String>;
    async fn pop(&self, topic: &str) -> Result<Vec<u8>, String>;
}

pub struct InMemJobQueue {
    topics: DashMap<String, (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>)>,
}

impl InMemJobQueue {
    pub fn new() -> Self {
        InMemJobQueue {
            topics: DashMap::new(),
        }
    }

    fn get_or_create_topic(&self, topic: &str) -> (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>) {
        if let Some(t) = self.topics.get(topic) {
            return t.value().clone();
        }
        
        let (tx, rx) = mpsc::channel(10000);
        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        let t = (tx, rx);
        self.topics.insert(topic.to_string(), t.clone());
        t
    }
}

#[async_trait]
impl JobQueue for InMemJobQueue {
    async fn push(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        let (tx, _) = self.get_or_create_topic(topic);
        tx.send(payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn pop(&self, topic: &str) -> Result<Vec<u8>, String> {
        let (_, rx) = self.get_or_create_topic(topic);
        let mut rx = rx.lock().await;
        rx.recv().await.ok_or_else(|| "channel closed".to_string())
    }
}

#[async_trait]
pub trait JobPayloadHandler: Send + Sync {
    async fn handle(&self, payload: Vec<u8>) -> Result<(), String>;
}

pub struct WorkerPool {
    queue: Arc<dyn JobQueue>,
    topic: String,
    handler: Arc<dyn JobPayloadHandler>,
    workers: usize,
}

impl WorkerPool {
    pub fn new(queue: Arc<dyn JobQueue>, topic: String, workers: usize, handler: Arc<dyn JobPayloadHandler>) -> Self {
        WorkerPool { queue, topic, handler, workers }
    }

    pub async fn start(&self, shutdown_rx: tokio::sync::broadcast::Sender<()>) {
        for i in 0..self.workers {
            let queue = self.queue.clone();
            let topic = self.topic.clone();
            let handler = self.handler.clone();
            let mut rx = shutdown_rx.subscribe();
            
            tokio::spawn(async move {
                println!("Worker {} starting", i);
                loop {
                    tokio::select! {
                        res = queue.pop(&topic) => {
                            match res {
                                Ok(payload) => {
                                    println!("Worker {} processing job", i);
                                    if let Err(e) = handler.handle(payload).await {
                                        println!("Worker {} handler failed: {}", i, e);
                                    }
                                }
                                Err(e) => {
                                    println!("Worker {} failed to pop: {}", i, e);
                                }
                            }
                        }
                        _ = rx.recv() => {
                            println!("Worker {} shutting down", i);
                            break;
                        }
                    }
                }
            });
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubAgentJob {
    pub id: String,
    pub tenant_id: String,
    pub parent_task_id: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub worker_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct QueueManager {
    pool: sqlx::PgPool,
}

impl QueueManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        QueueManager { pool }
    }

    pub async fn enqueue(&self, job: SubAgentJob) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&job.payload).unwrap_or_default();
        
        sqlx::query("INSERT INTO sub_agent_jobs (id, tenant_id, parent_task_id, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(job.id)
            .bind(job.tenant_id)
            .bind(job.parent_task_id)
            .bind(payload_str)
            .bind("PENDING")
            .bind(job.created_at)
            .bind(job.updated_at)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn poll(&self, worker_id: &str) -> Result<Option<SubAgentJob>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await?;
        let row = sqlx::query("UPDATE sub_agent_jobs SET status = 'RUNNING', worker_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = (SELECT id FROM sub_agent_jobs WHERE status = 'PENDING' AND (run_after IS NULL OR run_after <= CURRENT_TIMESTAMP) ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_task_id, payload, status, worker_id, created_at, updated_at")
            .bind(worker_id)
            .fetch_optional(&mut *tx)
            .await?;
        tx.commit().await?;
            
        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            
            Ok(Some(SubAgentJob {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_task_id: row.get("parent_task_id"),
                payload,
                status: row.get("status"),
                worker_id: row.get("worker_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn mark_completed(&self, job_id: &str, tenant_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }


    pub async fn requeue(&self, job_id: &str, tenant_id: &str, payload: serde_json::Value) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        // Since SubAgentJob's polling uses `status = 'PENDING'`, and some implementations might not filter by scheduled_at,
        // we can still add a simple delay by using tokio::time::sleep here or rely on the caller to backoff,
        // or actually update the scheduled_at column if the poll query respects it.
        // Wait, QueueManager::poll does: `SELECT id FROM sub_agent_jobs WHERE status = 'PENDING' ORDER BY created_at ASC`
        // It does NOT use `scheduled_at`!
        // To implement a true backoff, we need to add `AND (run_after IS NULL OR run_after <= CURRENT_TIMESTAMP)`.

        // Update the row.
        sqlx::query("UPDATE sub_agent_jobs SET status = 'PENDING', payload = $3, updated_at = CURRENT_TIMESTAMP, run_after = CURRENT_TIMESTAMP + INTERVAL '5 seconds' WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .bind(payload_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn mark_failed(&self, job_id: &str, _reason: &str, tenant_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(job_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn start_polling<F, Fut>(&self, worker_id: &str, interval: Duration, handler: F, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>)
    where
        F: Fn(SubAgentJob) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), String>> + Send + 'static,
    {
        let mut interval = tokio::time::interval(interval);
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    loop {
                        match self.poll(worker_id).await {
                            Ok(Some(job)) => {
                                println!("QueueManager dispatched job: {}", job.id);
                                let mut attempts = job.payload.get("attempts").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                                let max_attempts = job.payload.get("max_attempts").and_then(|v| v.as_i64()).unwrap_or(3) as i32;
                                attempts += 1;
                                let handle_res = tokio::time::timeout(tokio::time::Duration::from_secs(60), handler(job.clone())).await;
                                let handler_res = match handle_res {
                                    Ok(Ok(())) => Ok(()),
                                    Ok(Err(e)) => Err(e),
                                    Err(_) => Err("Timeout executing job".to_string()),
                                };
                                match handler_res {
                                    Ok(_) => {
                                        println!("Job handler succeeded: {}", job.id);
                                        let _ = self.mark_completed(&job.id, &job.tenant_id).await;
                                    }
                                    Err(e) => {
                                        println!("Job handler failed: {}, error: {}", job.id, e);
                                        if attempts < max_attempts {
                                            let mut retry_job = job.clone();
                                            retry_job.payload["attempts"] = serde_json::json!(attempts);
                                            retry_job.payload["max_attempts"] = serde_json::json!(max_attempts);
                                            let _ = self.requeue(&job.id, &job.tenant_id, retry_job.payload).await;
                                        } else {
                                            let _ = self.mark_failed(&job.id, &e, &job.tenant_id).await;
                                        }
                                    }
                                }
                            }
                            Ok(None) => {
                                break;
                            }
                            Err(e) => {
                                println!("Failed to poll queue: {}", e);
                                break;
                            }
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    println!("QueueManager polling shutting down");
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedTaskModel {
    pub id: String,
    pub tenant_id: String,
    pub parent_id: Option<String>,
    pub epic_id: Option<String>,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub payload: serde_json::Value,
    pub dependencies: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct TaskQueueService {
    pool: sqlx::PgPool,
}

impl TaskQueueService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        TaskQueueService { pool }
    }

    pub async fn push_task(&self, task: SharedTaskModel) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&task.payload).unwrap_or_default();
        let deps_str = serde_json::to_string(&task.dependencies).unwrap_or_else(|_| "[]".to_string());
        
        sqlx::query("INSERT INTO shared_tasks (id, parent_id, epic_id, title, status, assigned_agent, payload, tenant_id, dependencies) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb)")
            .bind(task.id)
            .bind(task.parent_id)
            .bind(task.epic_id)
            .bind(task.title)
            .bind("PENDING")
            .bind(task.assigned_agent)
            .bind(payload_str)
            .bind(task.tenant_id)
            .bind(deps_str)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn claim_task(&self, agent_id: &str) -> Result<Option<SharedTaskModel>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await?;
        let row = sqlx::query("UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent = $1 WHERE id = (SELECT st.id FROM shared_tasks st WHERE st.status = 'PENDING' AND (st.assigned_agent IS NULL OR st.assigned_agent = $1) AND NOT EXISTS (SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep_id JOIN shared_tasks parent ON parent.id::text = dep_id WHERE parent.status != 'COMPLETED') ORDER BY st.created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_id, epic_id, title, status, assigned_agent, payload, dependencies::text AS dependencies, created_at, updated_at")
            .bind(agent_id)
            .fetch_optional(&mut *tx)
            .await?;
        tx.commit().await?;
            
        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let deps_str: String = row.get("dependencies");
            let dependencies: serde_json::Value = serde_json::from_str(&deps_str).unwrap_or_else(|_| serde_json::json!([]));
            
            Ok(Some(SharedTaskModel {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                dependencies,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }



    pub async fn fail_task(&self, task_id: &str, reason: &str) -> Result<(), sqlx::Error> {
        let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap_or_else(|_| "{}".to_string());
        // We could merge this better using jsonb operators or just save status
        sqlx::query("UPDATE shared_tasks SET status = 'FAILED', payload = COALESCE(payload, '{}'::jsonb) || $2::jsonb, updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(task_id)
            .bind(payload_update)
            .execute(&self.pool)
            .await?;

        Ok(())
    }


    pub async fn get_completed_tasks(&self, limit: i64) -> Result<Vec<SharedTaskModel>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, tenant_id, parent_id, epic_id, title, status, assigned_agent, payload, dependencies::text AS dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'COMPLETED' LIMIT $1")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
            
        let mut tasks = Vec::new();
        for row in rows {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let deps_str: String = row.get("dependencies");
            let dependencies: serde_json::Value = serde_json::from_str(&deps_str).unwrap_or_else(|_| serde_json::json!([]));
            
            tasks.push(SharedTaskModel {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                dependencies,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(tasks)
    }
}

pub struct SqliteTaskQueue {
    pool: sqlx::SqlitePool,
}

impl SqliteTaskQueue {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        SqliteTaskQueue { pool }
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sub_agent_jobs (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                parent_task_id TEXT,
                agent_role TEXT NOT NULL,
                payload BLOB NOT NULL,
                status TEXT NOT NULL DEFAULT 'QUEUED',
                attempts INTEGER DEFAULT 0,
                max_attempts INTEGER DEFAULT 3,
                run_after TEXT,
                locked_until TEXT,
                created_at TEXT,
                updated_at TEXT
            );"
        ).execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl TaskQueue for SqliteTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        sqlx::query("INSERT INTO sub_agent_jobs (id, tenant_id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(job.id)
            .bind(job.tenant_id)
            .bind(job.parent_task_id)
            .bind(job.agent_role)
            .bind(job.payload.as_bytes())
            .bind("PENDING")
            .bind(job.attempts)
            .bind(job.max_attempts)
            .bind(job.run_after.to_rfc3339())
            .bind(job.locked_until.map(|d| d.to_rfc3339()))
            .bind(job.created_at.to_rfc3339())
            .bind(job.updated_at.to_rfc3339())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        if roles.is_empty() { return Ok(None); }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let role_placeholders = roles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, tenant_id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at FROM sub_agent_jobs WHERE status = 'PENDING' AND (run_after IS NULL OR run_after <= ?) AND agent_role IN ({}) LIMIT 1",
            role_placeholders
        );

        let now = chrono::Utc::now().to_rfc3339();
        let mut query = sqlx::query(&query_str).bind(&now);
        for role in &roles {
            query = query.bind(role);
        }

        let row = query.fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

        if let Some(row) = row {
            use sqlx::Row;
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let parent_task_id: Option<String> = row.get("parent_task_id");
            let parent_task_id = parent_task_id.unwrap_or_default();
            let agent_role: String = row.get("agent_role");
            let payload: Vec<u8> = row.get("payload");
            let _status: String = row.get("status");
            let attempts: i32 = row.get("attempts");
            let max_attempts: i32 = row.get("max_attempts");
            let run_after_str: Option<String> = row.get("run_after");
            let locked_until_str: Option<String> = row.get("locked_until");
            let created_at_str: Option<String> = row.get("created_at");
            let updated_at_str: Option<String> = row.get("updated_at");
            
            sqlx::query("UPDATE sub_agent_jobs SET status = 'RUNNING', updated_at = ? WHERE id = ?")
                .bind(&now)
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            let run_after = run_after_str.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))).unwrap_or_else(|| chrono::Utc::now());

            let locked_until = locked_until_str.as_deref().and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok().map(|d| d.with_timezone(&chrono::Utc)));

            let created_at = created_at_str.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))).unwrap_or_else(|| chrono::Utc::now());
            let updated_at = updated_at_str.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))).unwrap_or_else(|| chrono::Utc::now());

            Ok(Some(Job {
                id,
                tenant_id,
                parent_task_id,
                agent_role,
                payload: String::from_utf8(payload).unwrap_or_default(),
                status: "RUNNING".to_string(),
                attempts,
                max_attempts,
                run_after,
                locked_until,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn complete(&self, job_id: &str, _tenant_id: &str) -> Result<(), String> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = ? WHERE id = ? AND tenant_id = ?")
            .bind(&now)
            .bind(job_id)
            .bind(_tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn fail(&self, job_id: &str, _tenant_id: &str, _reason: &str) -> Result<(), String> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE sub_agent_jobs SET status = 'FAILED', updated_at = ? WHERE id = ? AND tenant_id = ?")
            .bind(&now)
            .bind(job_id)
            .bind(_tenant_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("UPDATE sub_agent_jobs SET status = 'PENDING', payload = ?, updated_at = ?, attempts = ? WHERE id = ?")
            .bind(new_payload.as_bytes())
            .bind(&now)
            .bind(job.attempts)
            .bind(&job.id)
            .execute(&self.pool)
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
        Ok(RedisTaskQueue {
            client,
            queue_name: queue_name.to_string(),
        })
    }
}

#[async_trait]
impl TaskQueue for RedisTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let payload_json = serde_json::to_string(&job).map_err(|e| e.to_string())?;
        
        // We use an RPUSH to the redis list
        let _: () = redis::cmd("RPUSH")
            .arg(&self.queue_name)
            .arg(payload_json)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        
        // Use BLPOP with 1 second timeout to avoid busy loop
        let result: Option<(String, String)> = redis::cmd("BLPOP")
            .arg(&self.queue_name)
            .arg(1)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
            
        if let Some((_, payload_json)) = result {
            if let Ok(job) = serde_json::from_str::<Job>(&payload_json) {
                if roles.contains(&job.agent_role) {
                    let _: () = redis::cmd("HSET").arg(format!("{}_processing", self.queue_name)).arg(&job.id).arg(&payload_json).query_async(&mut conn).await.map_err(|e| e.to_string())?;
                    return Ok(Some(job));
                } else {
                    // Not intended for this worker role, push it back.
                    let _ = self.enqueue(job).await;
                }
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let processing_key = format!("{}_processing", self.queue_name);
        let result: Option<String> = redis::cmd("HGET").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
        if let Some(payload_json) = result {
            if let Ok(job) = serde_json::from_str::<Job>(&payload_json) {
                if job.tenant_id != tenant_id {
                    return Err("tenant mismatch".to_string());
                }
                let _: () = redis::cmd("HDEL").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
        Err("job not found".to_string())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, _reason: &str) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let processing_key = format!("{}_processing", self.queue_name);
        let result: Option<String> = redis::cmd("HGET").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
        if let Some(payload_json) = result {
            if let Ok(job) = serde_json::from_str::<Job>(&payload_json) {
                if job.tenant_id != tenant_id {
                    return Err("tenant mismatch".to_string());
                }
                let _: () = redis::cmd("HDEL").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
        Err("job not found".to_string())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let payload_json = serde_json::to_string(&job).unwrap_or_default();
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let _: () = redis::cmd("RPUSH")
            .arg(&self.queue_name)
            .arg(&payload_json)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    struct MockHandler;

    #[async_trait]
    impl JobPayloadHandler for MockHandler {
        async fn handle(&self, payload: Vec<u8>) -> Result<(), String> {
            let s = String::from_utf8(payload).unwrap();
            println!("MockHandler received: {}", s);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_in_mem_job_queue_worker_pool() {
        let queue = Arc::new(InMemJobQueue::new());
        let handler = Arc::new(MockHandler);
        let pool = WorkerPool::new(queue.clone(), "test_topic".to_string(), 3, handler);
        
        let (tx, rx) = tokio::sync::broadcast::channel(1);
        // Ensure that we don't drop the rx to keep the channel open
        let _rx = rx;

        pool.start(tx.clone()).await;
        
        queue.push("test_topic", b"hello".to_vec()).await.unwrap();
        queue.push("test_topic", b"world".to_vec()).await.unwrap();
        
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        let _ = tx.send(());
    }

    #[tokio::test]
    async fn test_task_queue_service_push_claim() {
        // Create an actual pool to hit a local database for integration testing.
        // During CI, we assume postgres is available at this URL.
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })

                .connect_lazy(&db_url)
                .unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let service = TaskQueueService::new(pool.clone());

            // Initialize schema for test
            sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id VARCHAR PRIMARY KEY, parent_id VARCHAR, epic_id VARCHAR, title VARCHAR NOT NULL, status VARCHAR NOT NULL, assigned_agent VARCHAR, payload JSONB, tenant_id VARCHAR, dependencies JSONB DEFAULT '[]', created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)")
                .execute(&pool)
                .await
                .unwrap();

            let task_id = uuid::Uuid::new_v4().to_string();
            let task = SharedTaskModel {
                id: task_id.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Test Task".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({"action": "test"}),
                dependencies: serde_json::json!([]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // Push
            let push_res = service.push_task(task).await;
            assert!(push_res.is_ok());

            // Claim
            let claim_res = service.claim_task("agent_1").await.unwrap();
            assert!(claim_res.is_some());
            let claimed = claim_res.unwrap();
            assert_eq!(claimed.id, task_id);
            assert_eq!(claimed.assigned_agent.unwrap(), "agent_1");

            // Complete
            let comp_res = service.complete_task(&task_id).await;
            assert!(comp_res.is_ok());
        }
    }


    #[tokio::test]
    async fn test_queue_manager_tenant_isolation() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .connect_lazy(&db_url)
                .unwrap();

            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }

            let qm = QueueManager::new(pool.clone());
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-a".to_string();

            // Ignore table creation errors if it already exists
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_jobs (id VARCHAR PRIMARY KEY, tenant_id VARCHAR NOT NULL, parent_task_id VARCHAR, payload TEXT, status VARCHAR, worker_id VARCHAR, run_after TIMESTAMP, completed_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP)")
                .execute(&pool)
                .await;

            let job = SubAgentJob {
                id: job_id.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test"}),
                status: "PENDING".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            qm.enqueue(job).await.unwrap();

            // Attempt to complete with the WRONG tenant
            let res = qm.mark_completed(&job_id, "wrong-tenant").await;
            assert!(res.is_ok()); // The query executes successfully but updates 0 rows

            // Verify status is still QUEUED
            let status: (String,) = sqlx::query_as("SELECT status FROM sub_agent_jobs WHERE id = $1")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status.0, "PENDING");

            // Complete with CORRECT tenant
            let res2 = qm.mark_completed(&job_id, &org_id).await;
            assert!(res2.is_ok());

            let status_updated: (String,) = sqlx::query_as("SELECT status FROM sub_agent_jobs WHERE id = $1")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status_updated.0, "COMPLETED");

            // Test mark_failed isolation
            let job_id2 = uuid::Uuid::new_v4().to_string();
            let job2 = SubAgentJob {
                id: job_id2.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test2"}),
                status: "PENDING".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            qm.enqueue(job2).await.unwrap();

            let _ = qm.mark_failed(&job_id2, "error", "wrong-tenant").await;
            let status_failed1: (String,) = sqlx::query_as("SELECT status FROM sub_agent_jobs WHERE id = $1")
                .bind(&job_id2)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status_failed1.0, "PENDING");

            let _ = qm.mark_failed(&job_id2, "error", &org_id).await;
            let status_failed2: (String,) = sqlx::query_as("SELECT status FROM sub_agent_jobs WHERE id = $1")
                .bind(&job_id2)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status_failed2.0, "FAILED");
        }
    }

    #[tokio::test]
    async fn test_task_queue_service_fail_task() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()

                .connect_lazy(&db_url)
                .unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let service = TaskQueueService::new(pool.clone());

            let task_id = uuid::Uuid::new_v4().to_string();
            let task = SharedTaskModel {
                id: task_id.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Test Task to Fail".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({"action": "test_fail"}),
                dependencies: serde_json::json!([]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            service.push_task(task).await.unwrap();

            // Claim it
            let claimed = service.claim_task("agent_1").await.unwrap().unwrap();
            assert_eq!(claimed.id, task_id);

            // Fail it
            service.fail_task(&task_id, "Some failure occurred").await.unwrap();

            // Fetch manually to check
            let row = sqlx::query("SELECT status, payload FROM shared_tasks WHERE id = $1")
                .bind(&task_id)
                .fetch_one(&pool)
                .await
                .unwrap();

            let status: String = sqlx::Row::get(&row, "status");
            let payload: serde_json::Value = sqlx::Row::get(&row, "payload");

            assert_eq!(status, "FAILED");
            assert_eq!(payload["error"], "Some failure occurred");
        }
    }

    #[tokio::test]
    async fn test_task_queue_service_with_dependencies() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })

                .connect_lazy(&db_url)
                .unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let service = TaskQueueService::new(pool.clone());

            let task_id_parent = uuid::Uuid::new_v4().to_string();
            let task_id_child = uuid::Uuid::new_v4().to_string();

            let parent_task = SharedTaskModel {
                id: task_id_parent.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Parent Task".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({}),
                dependencies: serde_json::json!([]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            let child_task = SharedTaskModel {
                id: task_id_child.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Child Task".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({}),
                dependencies: serde_json::json!([task_id_parent]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            service.push_task(parent_task).await.unwrap();
            service.push_task(child_task).await.unwrap();

            // Claiming should ONLY claim the parent since child is blocked by parent
            let claim_1 = service.claim_task("agent_1").await.unwrap().unwrap();
            assert_eq!(claim_1.id, task_id_parent);

            // Second claim should return None because child is blocked
            let claim_2 = service.claim_task("agent_1").await.unwrap();
            assert!(claim_2.is_none());

            // Complete parent
            service.complete_task(&task_id_parent).await.unwrap();

            // Now child should be claimable
            let claim_3 = service.claim_task("agent_2").await.unwrap().unwrap();
            assert_eq!(claim_3.id, task_id_child);
        }
    }
}
