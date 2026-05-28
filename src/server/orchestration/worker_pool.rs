use crate::orchestration::mesh::TeammateMesh;
use crate::orchestration::TeammateMeshEvent;
use async_trait::async_trait;
use std::sync::Arc;

use crate::orchestration::queue::{Job, TaskQueue};

#[async_trait]
pub trait AgentHarness: Send + Sync {
    async fn execute(&self, job: Job) -> Result<(), String>;
}

pub struct WorkerPool {
    queue: Arc<dyn TaskQueue>,
    mesh: Arc<dyn TeammateMesh>,
    roles: Vec<String>,
    harness: Arc<dyn AgentHarness>,
    workers: usize,
    estimated_vram: i64,
    estimated_tokens: i64,
}

impl WorkerPool {
    pub fn new(
        queue: Arc<dyn TaskQueue>,
        mesh: Arc<dyn TeammateMesh>,
        roles: Vec<String>,
        harness: Arc<dyn AgentHarness>,
        workers: usize,
    ) -> Self {
        WorkerPool {
            queue,
            mesh,
            roles,
            harness,
            workers,
            estimated_vram: 0,
            estimated_tokens: 0,
        }
    }

    pub async fn start(&self, shutdown_rx: tokio::sync::broadcast::Sender<()>) {
        for i in 0..self.workers {
            let queue = self.queue.clone();
            let mesh = self.mesh.clone();
            let roles = self.roles.clone();
            let harness = self.harness.clone();
            let mut rx = shutdown_rx.subscribe();

            let vram = self.estimated_vram;
            let tokens = self.estimated_tokens;

            tokio::spawn(async move {
                tracing::info!("WorkerPool worker {} starting", i);
                loop {
                    tokio::select! {
                                            res = queue.dequeue(roles.clone(), vram, tokens) => {
                                                match res {
                                                    Ok(Some(job)) => {
                                                        tracing::debug!("Worker {} processing job {}", i, job.id);

                                                        // Broadcast START
                                                        let event = TeammateMeshEvent {
                                                            agent_id: "worker_pool".to_string(),
                                                            action: "START".to_string(),
                                                            status: "ok".to_string(),
                                                            payload: job.payload.as_bytes().to_vec(),
                                                            msg_id: uuid::Uuid::new_v4().to_string(),
                                                        };
                    let mut buf = vec![];
                    if let Err(e) = prost::Message::encode(&event, &mut buf) { tracing::error!("Failed to encode mesh event: {}", e); }
                    if let Err(e) = mesh.publish(&format!("task:{}", job.id), buf).await { tracing::error!("Failed to publish mesh event: {}", e); }

                                                        let job_id = job.id.clone();

                                                        match harness.execute(job).await {
                                                            Ok(_) => {
                                                                tracing::debug!("Worker {} completed job {}", i, job_id);
                                                                let _ = queue.complete(&job_id).await;

                                                                // Broadcast SUCCESS
                                                                let event = TeammateMeshEvent {
                                                                    agent_id: "worker_pool".to_string(),
                                                                    action: "SUCCESS".to_string(),
                                                                    status: "ok".to_string(),
                                                                    payload: vec![],
                                                                    msg_id: uuid::Uuid::new_v4().to_string(),
                                                                };
                    let mut buf = vec![];
                    if let Err(e) = prost::Message::encode(&event, &mut buf) { tracing::error!("Failed to encode mesh event: {}", e); }
                    if let Err(e) = mesh.publish(&format!("task:{}", job.id), buf).await { tracing::error!("Failed to publish mesh event: {}", e); }
                                                            }
                                                            Err(e) => {
                                                                tracing::error!("Worker {} failed job {}: {}", i, job_id, e);
                                                                let _ = queue.fail(&job_id, &e).await;

                                                                // Broadcast FAIL
                                                                let event = TeammateMeshEvent {
                                                                    agent_id: "worker_pool".to_string(),
                                                                    action: "FAIL".to_string(),
                                                                    status: "error".to_string(),
                                                                    payload: e.as_bytes().to_vec(),
                                                                    msg_id: uuid::Uuid::new_v4().to_string(),
                                                                };
                    let mut buf = vec![];
                    if let Err(e) = prost::Message::encode(&event, &mut buf) { tracing::error!("Failed to encode mesh event: {}", e); }
                    if let Err(e) = mesh.publish(&format!("task:{}", job.id), buf).await { tracing::error!("Failed to publish mesh event: {}", e); }
                                                            }
                                                        }
                                                    }
                                                    Ok(None) => {
                                                        // No jobs, sleep a bit to prevent tight loop
                                                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                                    }
                                                    Err(e) => {
                                                        tracing::error!("Worker {} failed to dequeue: {}", i, e);
                                                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                                                    }
                                                }
                                            }
                                            _ = rx.recv() => {
                                                tracing::info!("WorkerPool worker {} shutting down", i);
                                                break;
                                            }
                                        }
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Message;
    use prost::Message;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockTaskQueue {
        jobs: Mutex<Vec<Job>>,
        completed: Mutex<Vec<String>>,
        failed: Mutex<Vec<(String, String)>>,
    }

    impl MockTaskQueue {
        fn new() -> Self {
            Self {
                jobs: Mutex::new(Vec::new()),
                completed: Mutex::new(Vec::new()),
                failed: Mutex::new(Vec::new()),
            }
        }

        async fn add_job(&self, job: Job) {
            self.jobs.lock().await.push(job);
        }
    }

    #[async_trait]
    impl TaskQueue for MockTaskQueue {
        async fn enqueue(&self, _job: Job) -> Result<(), String> {
            Ok(())
        }
        async fn dequeue(
            &self,
            _roles: Vec<String>,
            _estimated_vram: i64,
            _estimated_tokens: i64,
        ) -> Result<Option<Job>, String> {
            Ok(self.jobs.lock().await.pop())
        }
        async fn complete(&self, job_id: &str) -> Result<(), String> {
            self.completed.lock().await.push(job_id.to_string());
            Ok(())
        }
        async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String> {
            self.failed
                .lock()
                .await
                .push((job_id.to_string(), reason.to_string()));
            Ok(())
        }
    }

    struct MockTeammateMesh {
        published: Mutex<Vec<(String, TeammateMeshEvent)>>,
    }

    impl MockTeammateMesh {
        fn new() -> Self {
            Self {
                published: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl TeammateMesh for MockTeammateMesh {
        async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
            if let Ok(event) = TeammateMeshEvent::decode(&payload[..]) {
                self.published.lock().await.push((topic.to_string(), event));
            }
            Ok(())
        }
        async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
            Ok(())
        }
        async fn subscribe(
            &self,
            _topic: &str,
            _handler: Box<dyn Fn(Message) + Send + Sync>,
        ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }
        async fn acquire_lock(
            &self,
            _resource: &str,
            _owner: &str,
            _ttl_seconds: u64,
        ) -> Result<bool, String> {
            Ok(true)
        }
        async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
            Ok(())
        }
        async fn register_presence(
            &self,
            _agent_id: &str,
            _status: &str,
            _ttl_seconds: u64,
        ) -> Result<(), String> {
            Ok(())
        }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
            Ok(vec![])
        }
    }

    struct MockAgentHarness {
        should_fail: bool,
    }

    #[async_trait]
    impl AgentHarness for MockAgentHarness {
        async fn execute(&self, _job: Job) -> Result<(), String> {
            if self.should_fail {
                Err("simulated failure".to_string())
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_worker_pool_success() {
        let queue = Arc::new(MockTaskQueue::new());
        let mesh = Arc::new(MockTeammateMesh::new());
        let harness = Arc::new(MockAgentHarness { should_fail: false });

        let job = Job {
            id: "job-123".to_string(),
            tenant_id: "tenant-1".to_string(),
            parent_task_id: "parent-1".to_string(),
            agent_role: "test-role".to_string(),
            payload: "{}".to_string(),
            status: "QUEUED".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        queue.add_job(job.clone()).await;

        let pool = WorkerPool::new(
            queue.clone(),
            mesh.clone(),
            vec!["test-role".to_string()],
            harness.clone(),
            1,
        );

        let (tx, _rx) = tokio::sync::broadcast::channel(1);
        pool.start(tx.clone()).await;

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let completed = queue.completed.lock().await;
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0], "job-123");

        let published = mesh.published.lock().await;
        assert_eq!(published.len(), 2);

        assert_eq!(published[0].0, "task:job-123");
        assert_eq!(published[0].1.action, "START");

        assert_eq!(published[1].0, "task:job-123");
        assert_eq!(published[1].1.action, "SUCCESS");

        let _ = tx.send(()); // shutdown
    }

    #[tokio::test]
    async fn test_worker_pool_failure() {
        let queue = Arc::new(MockTaskQueue::new());
        let mesh = Arc::new(MockTeammateMesh::new());
        let harness = Arc::new(MockAgentHarness { should_fail: true });

        let job = Job {
            id: "job-failed".to_string(),
            tenant_id: "tenant-1".to_string(),
            parent_task_id: "parent-1".to_string(),
            agent_role: "test-role".to_string(),
            payload: "{}".to_string(),
            status: "QUEUED".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        queue.add_job(job.clone()).await;

        let pool = WorkerPool::new(
            queue.clone(),
            mesh.clone(),
            vec!["test-role".to_string()],
            harness.clone(),
            1,
        );

        let (tx, _rx) = tokio::sync::broadcast::channel(1);
        pool.start(tx.clone()).await;

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let failed = queue.failed.lock().await;
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].0, "job-failed");
        assert_eq!(failed[0].1, "simulated failure");

        let published = mesh.published.lock().await;
        assert_eq!(published.len(), 2);

        assert_eq!(published[0].0, "task:job-failed");
        assert_eq!(published[0].1.action, "START");

        assert_eq!(published[1].0, "task:job-failed");
        assert_eq!(published[1].1.action, "FAIL");

        let _ = tx.send(()); // shutdown
    }
}
