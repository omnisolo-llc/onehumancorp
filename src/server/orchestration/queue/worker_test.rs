use super::{TaskQueue, Job};
use super::worker::{JobHandler, WorkerPool};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use async_trait::async_trait;

struct MockQueue {
    jobs: Mutex<Vec<Job>>,
    completed: Mutex<Vec<String>>,
    failed: Mutex<Vec<(String, String)>>,
}

impl MockQueue {
    fn new() -> Self {
        Self {
            jobs: Mutex::new(Vec::new()),
            completed: Mutex::new(Vec::new()),
            failed: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl TaskQueue for MockQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        self.jobs.lock().await.push(job);
        Ok(())
    }

    async fn dequeue(&self, _roles: Vec<String>, _estimated_vram: i64, _estimated_tokens: i64) -> Result<Option<Job>, String> {
        let mut jobs = self.jobs.lock().await;
        if jobs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(jobs.remove(0)))
        }
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        self.completed.lock().await.push(job_id.to_string());
        Ok(())
    }

    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String> {
        self.failed.lock().await.push((job_id.to_string(), reason.to_string()));
        Ok(())
    }
}

struct MockHandler {
    success: bool,
}

#[async_trait]
impl JobHandler for MockHandler {
    async fn handle_job(&self, _job: Job) -> Result<Vec<u8>, String> {
        if self.success {
            Ok(b"success".to_vec())
        } else {
            Err("failed".to_string())
        }
    }
}

#[tokio::test]
async fn test_worker_success() {
    let queue = Arc::new(MockQueue::new());
    let handler = Arc::new(MockHandler { success: true });

    let job = Job {
        id: "job-1".to_string(),
        tenant_id: "system".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now(),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    queue.enqueue(job).await.unwrap();

    let worker = WorkerPool::new(queue.clone(), handler, vec!["test-role".to_string()], 100, 100, 1);
    worker.start();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let completed = queue.completed.lock().await;
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0], "job-1");
}

#[tokio::test]
async fn test_worker_fail() {
    let queue = Arc::new(MockQueue::new());
    let handler = Arc::new(MockHandler { success: false });

    let job = Job {
        id: "job-2".to_string(),
        tenant_id: "system".to_string(),
        parent_task_id: "parent-1".to_string(),
        agent_role: "test-role".to_string(),
        payload: "{}".to_string(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now(),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    queue.enqueue(job).await.unwrap();

    let worker = WorkerPool::new(queue.clone(), handler, vec!["test-role".to_string()], 100, 100, 1);
    worker.start();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let failed = queue.failed.lock().await;
    assert_eq!(failed.len(), 1);
    assert_eq!(failed[0].0, "job-2");
    assert_eq!(failed[0].1, "failed");
}
