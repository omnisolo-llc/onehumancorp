use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use tokio::sync::mpsc;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
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
    async fn complete(&self, job_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String>;
}

pub struct MemoryTaskQueue {
    jobs: RwLock<HashMap<String, Job>>,
}

impl MemoryTaskQueue {
    pub fn new() -> Self {
        MemoryTaskQueue {
            jobs: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut jobs = self.jobs.write().unwrap();
        jobs.insert(job.id.clone(), job);
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let mut jobs = self.jobs.write().unwrap();
        for job in jobs.values_mut() {
            if job.status == "PENDING" && roles.contains(&job.agent_role) {
                job.status = "IN_PROGRESS".to_string();
                job.updated_at = Utc::now();
                return Ok(Some(job.clone()));
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        let mut jobs = self.jobs.write().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.status = "COMPLETED".to_string();
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String> {
        let mut jobs = self.jobs.write().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.status = "FAILED".to_string();
            job.payload = format!("{} (Reason: {})", job.payload, reason);
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
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
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let run_after = job.run_after;
        
        let mut payload_map: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or_else(|_| serde_json::json!({}));
        payload_map["agent_role"] = serde_json::Value::String(job.agent_role.clone());
        payload_map["attempts"] = serde_json::json!(job.attempts);
        payload_map["max_attempts"] = serde_json::json!(job.max_attempts);
        
        let new_payload = serde_json::to_string(&payload_map).unwrap_or_default();
        
        let org_id = payload_map["organization_id"].as_str().unwrap_or("").to_string();
        
        sqlx::query("INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, scheduled_at) VALUES ($1, $2, $3, $4, $5, $6)")
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
        let row = sqlx::query("UPDATE sub_agent_queue SET status = 'RUNNING' WHERE id = (SELECT id FROM sub_agent_queue WHERE status = 'PENDING' AND scheduled_at <= CURRENT_TIMESTAMP AND payload::json->>'agent_role' = ANY($1) ORDER BY scheduled_at ASC FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING id, organization_id, parent_task_id, payload, status, scheduled_at")
            .bind(&roles)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        if let Some(row) = row {
            let id: String = row.get("id");
            let parent_task_id: String = row.get("parent_task_id");
            let payload: String = row.get("payload");
            let status: String = row.get("status");
            let scheduled_at: DateTime<Utc> = row.get("scheduled_at");
            
            let mut j = Job {
                id,
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
            
            let mut payload_map: serde_json::Value = serde_json::from_str(&payload).unwrap_or_else(|_| serde_json::json!({}));
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

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', completed_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', payload = payload || $2 WHERE id = $1")
            .bind(job_id)
            .bind(format!(" (Error: {})", reason))
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
                            match self.handler.handle(job.clone()).await {
                                Ok(_) => {
                                    let _ = self.queue.complete(&job.id).await;
                                }
                                Err(e) => {
                                    let _ = self.queue.fail(&job.id, &e).await;
                                }
                            }
                        }
                        Ok(None) => {
                            // No job available
                        }
                        Err(e) => {
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
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
    topics: RwLock<HashMap<String, (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>)>>,
}

impl InMemJobQueue {
    pub fn new() -> Self {
        InMemJobQueue {
            topics: RwLock::new(HashMap::new()),
        }
    }

    fn get_or_create_topic(&self, topic: &str) -> (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>) {
        let mut topics = self.topics.write().unwrap();
        if let Some(t) = topics.get(topic) {
            return t.clone();
        }
        
        let (tx, rx) = mpsc::channel(10000);
        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        let t = (tx, rx);
        topics.insert(topic.to_string(), t.clone());
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
                loop {
                    tokio::select! {
                        res = queue.pop(&topic) => {
                            match res {
                                Ok(payload) => {
                                    if let Err(e) = handler.handle(payload).await {
                                    }
                                }
                                Err(e) => {
                                }
                            }
                        }
                        _ = rx.recv() => {
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
    pub organization_id: String,
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
        
        sqlx::query("INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(job.id)
            .bind(job.organization_id)
            .bind(job.parent_task_id)
            .bind(payload_str)
            .bind("QUEUED")
            .bind(job.created_at)
            .bind(job.updated_at)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn poll(&self, worker_id: &str) -> Result<Option<SubAgentJob>, sqlx::Error> {
        let row = sqlx::query("UPDATE sub_agent_queue SET status = 'RUNNING', worker_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = (SELECT id FROM sub_agent_queue WHERE status = 'QUEUED' ORDER BY created_at ASC FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at")
            .bind(worker_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            
            Ok(Some(SubAgentJob {
                id: row.get("id"),
                organization_id: row.get("organization_id"),
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

    pub async fn mark_completed(&self, job_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn mark_failed(&self, job_id: &str, _reason: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(job_id)
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
                                match handler(job.clone()).await {
                                    Ok(_) => {
                                        println!("Job handler succeeded: {}", job.id);
                                        let _ = self.mark_completed(&job.id).await;
                                    }
                                    Err(e) => {
                                        println!("Job handler failed: {}, error: {}", job.id, e);
                                        let _ = self.mark_failed(&job.id, &e).await;
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
    pub organization_id: String,
    pub parent_id: Option<String>,
    pub epic_id: Option<String>,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub payload: serde_json::Value,
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
        
        sqlx::query("INSERT INTO shared_tasks (id, parent_id, epic_id, title, status, assigned_agent, payload, organization_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(task.id)
            .bind(task.parent_id)
            .bind(task.epic_id)
            .bind(task.title)
            .bind("PENDING")
            .bind(task.assigned_agent)
            .bind(payload_str)
            .bind(task.organization_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn claim_task(&self, agent_id: &str) -> Result<Option<SharedTaskModel>, sqlx::Error> {
        let row = sqlx::query("UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent = $1 WHERE id = (SELECT id FROM shared_tasks WHERE status = 'PENDING' AND (assigned_agent IS NULL OR assigned_agent = $1) ORDER BY created_at ASC FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING id, organization_id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at")
            .bind(agent_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            
            Ok(Some(SharedTaskModel {
                id: row.get("id"),
                organization_id: row.get("organization_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
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

    pub async fn get_completed_tasks(&self, limit: i64) -> Result<Vec<SharedTaskModel>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, organization_id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at FROM shared_tasks WHERE status = 'COMPLETED' LIMIT $1")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
            
        let mut tasks = Vec::new();
        for row in rows {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            
            tasks.push(SharedTaskModel {
                id: row.get("id"),
                organization_id: row.get("organization_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(tasks)
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
        
        let (tx, _) = tokio::sync::broadcast::channel(1);
        pool.start(tx.clone()).await;
        
        queue.push("test_topic", b"hello".to_vec()).await.unwrap();
        queue.push("test_topic", b"world".to_vec()).await.unwrap();
        
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        let _ = tx.send(());
    }
}
