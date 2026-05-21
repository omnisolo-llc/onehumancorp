use crate::orchestration::queue::{Job, TaskQueue};
use crate::orchestration::mesh::TeammateMesh;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use serde_json::json;

#[async_trait]
pub trait AgentHarness: Send + Sync {
    async fn execute(&self, job: &Job) -> Result<(), String>;
}

pub struct WorkerPoolConfig {
    pub roles: Vec<String>,
    pub num_workers: usize,
    pub worker_id: String,
    pub estimated_vram: i64,
    pub estimated_tokens: i64,
}

pub struct WorkerPool {
    queue: Arc<dyn TaskQueue>,
    mesh: Arc<dyn TeammateMesh>,
    harness: Arc<dyn AgentHarness>,
    config: WorkerPoolConfig,
    shutdown_tx: broadcast::Sender<()>,
    worker_handles: std::sync::Mutex<Vec<JoinHandle<()>>>,
}

impl WorkerPool {
    pub fn new(
        queue: Arc<dyn TaskQueue>,
        mesh: Arc<dyn TeammateMesh>,
        harness: Arc<dyn AgentHarness>,
        config: WorkerPoolConfig,
    ) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            queue,
            mesh,
            harness,
            config,
            shutdown_tx,
            worker_handles: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn start(&self) {
        let mut handles = self.worker_handles.lock().unwrap();
        for _ in 0..self.config.num_workers {
            let queue = Arc::clone(&self.queue);
            let mesh = Arc::clone(&self.mesh);
            let harness = Arc::clone(&self.harness);
            let roles = self.config.roles.clone();
            let worker_id = self.config.worker_id.clone();
            let estimated_vram = self.config.estimated_vram;
            let estimated_tokens = self.config.estimated_tokens;
            let mut shutdown_rx = self.shutdown_tx.subscribe();

            let handle = tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            break;
                        }
                        job_opt = queue.dequeue(roles.clone(), estimated_vram, estimated_tokens) => {
                            match job_opt {
                                Ok(Some(job)) => {
                                    let lock_key = format!("job_lock_{}", job.id);
                                    let lock_acquired = match mesh.acquire_lock(&lock_key, &worker_id, 300).await {
                                        Ok(acquired) => acquired,
                                        Err(_) => false,
                                    };

                                    if !lock_acquired {
                                        continue;
                                    }

                                    let start_msg = json!({
                                        "job_id": job.id,
                                        "status": "START",
                                        "worker_id": worker_id
                                    }).to_string().into_bytes();
                                    let _ = mesh.publish("task_status", start_msg).await;

                                    let execute_res = harness.execute(&job).await;

                                    match execute_res {
                                        Ok(_) => {
                                            let _ = queue.complete(&job.id).await;
                                            let success_msg = json!({
                                                "job_id": job.id,
                                                "status": "SUCCESS",
                                                "worker_id": worker_id
                                            }).to_string().into_bytes();
                                            let _ = mesh.publish("task_status", success_msg).await;
                                        }
                                        Err(e) => {
                                            let _ = queue.fail(&job.id, &e).await;
                                            let fail_msg = json!({
                                                "job_id": job.id,
                                                "status": "FAIL",
                                                "worker_id": worker_id,
                                                "error": e
                                            }).to_string().into_bytes();
                                            let _ = mesh.publish("task_status", fail_msg).await;
                                        }
                                    }

                                    let _ = mesh.release_lock(&lock_key, &worker_id).await;
                                }
                                Ok(None) => {
                                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                }
                                Err(_) => {
                                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                }
                            }
                        }
                    }
                }
            });
            handles.push(handle);
        }
    }

    pub async fn stop(&self) {
        let _ = self.shutdown_tx.send(());
        let handles_to_await: Vec<_> = self.worker_handles.lock().unwrap().drain(..).collect();
        for handle in handles_to_await {
            let _ = handle.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use chrono::Utc;
    use ohc_builtin_agent::mesh::transport::Message;

    struct MockQueue {
        job_count: AtomicUsize,
        completed: AtomicUsize,
        failed: AtomicUsize,
        should_fail_dequeue: bool,
    }

    #[async_trait]
    impl TaskQueue for MockQueue {
        async fn enqueue(&self, _job: Job) -> Result<(), String> { Ok(()) }
        async fn enqueue_batch(&self, _jobs: Vec<Job>) -> Result<(), String> { Ok(()) }
        async fn dequeue(&self, _roles: Vec<String>, _est_vram: i64, _est_tokens: i64) -> Result<Option<Job>, String> {
            if self.should_fail_dequeue {
                return Err("DB error".into());
            }
            let remaining = self.job_count.load(Ordering::SeqCst);
            if remaining > 0 {
                self.job_count.fetch_sub(1, Ordering::SeqCst);
                Ok(Some(Job {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: "t1".into(),
                    parent_task_id: "p1".into(),
                    agent_role: "agent".into(),
                    payload: "{}".into(),
                    status: "QUEUED".into(),
                    attempts: 0,
                    max_attempts: 3,
                    run_after: Utc::now(),
                    locked_until: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }))
            } else {
                Ok(None)
            }
        }
        async fn complete(&self, _job_id: &str) -> Result<(), String> {
            self.completed.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        async fn fail(&self, _job_id: &str, _reason: &str) -> Result<(), String> {
            self.failed.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct MockMesh {
        lock_fails: bool,
    }
    #[async_trait]
    impl TeammateMesh for MockMesh {
        async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(||{})) }
        async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl: u64) -> Result<bool, String> {
            if self.lock_fails { Ok(false) } else { Ok(true) }
        }
        async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
        async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl: u64) -> Result<(), String> { Ok(()) }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
        async fn ping(&self) -> Result<(), String> { Ok(()) }
        async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(||{})) }
        async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(||{})) }
    }

    struct MockHarness {
        should_fail: bool,
    }
    #[async_trait]
    impl AgentHarness for MockHarness {
        async fn execute(&self, _job: &Job) -> Result<(), String> {
            if self.should_fail { Err("Failed".into()) } else { Ok(()) }
        }
    }

    #[tokio::test]
    async fn test_worker_pool_success() {
        let queue = Arc::new(MockQueue { job_count: AtomicUsize::new(2), completed: AtomicUsize::new(0), failed: AtomicUsize::new(0), should_fail_dequeue: false });
        let mesh = Arc::new(MockMesh { lock_fails: false });
        let harness = Arc::new(MockHarness { should_fail: false });

        let pool = WorkerPool::new(queue.clone(), mesh, harness, WorkerPoolConfig {
            roles: vec!["agent".into()],
            num_workers: 2,
            worker_id: "w1".into(),
            estimated_vram: 0,
            estimated_tokens: 0,
        });

        pool.start();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        pool.stop().await;

        assert_eq!(queue.completed.load(Ordering::SeqCst), 2);
        assert_eq!(queue.failed.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn test_worker_pool_harness_failure() {
        let queue = Arc::new(MockQueue { job_count: AtomicUsize::new(2), completed: AtomicUsize::new(0), failed: AtomicUsize::new(0), should_fail_dequeue: false });
        let mesh = Arc::new(MockMesh { lock_fails: false });
        let harness = Arc::new(MockHarness { should_fail: true });

        let pool = WorkerPool::new(queue.clone(), mesh, harness, WorkerPoolConfig {
            roles: vec!["agent".into()],
            num_workers: 2,
            worker_id: "w1".into(),
            estimated_vram: 0,
            estimated_tokens: 0,
        });

        pool.start();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        pool.stop().await;

        assert_eq!(queue.completed.load(Ordering::SeqCst), 0);
        assert_eq!(queue.failed.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_worker_pool_lock_failure() {
        let queue = Arc::new(MockQueue { job_count: AtomicUsize::new(2), completed: AtomicUsize::new(0), failed: AtomicUsize::new(0), should_fail_dequeue: false });
        let mesh = Arc::new(MockMesh { lock_fails: true });
        let harness = Arc::new(MockHarness { should_fail: false });

        let pool = WorkerPool::new(queue.clone(), mesh, harness, WorkerPoolConfig {
            roles: vec!["agent".into()],
            num_workers: 2,
            worker_id: "w1".into(),
            estimated_vram: 0,
            estimated_tokens: 0,
        });

        pool.start();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        pool.stop().await;

        // None completed or failed because lock couldn't be acquired
        assert_eq!(queue.completed.load(Ordering::SeqCst), 0);
        assert_eq!(queue.failed.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn test_worker_pool_dequeue_error() {
        let queue = Arc::new(MockQueue { job_count: AtomicUsize::new(2), completed: AtomicUsize::new(0), failed: AtomicUsize::new(0), should_fail_dequeue: true });
        let mesh = Arc::new(MockMesh { lock_fails: false });
        let harness = Arc::new(MockHarness { should_fail: false });

        let pool = WorkerPool::new(queue.clone(), mesh, harness, WorkerPoolConfig {
            roles: vec!["agent".into()],
            num_workers: 2,
            worker_id: "w1".into(),
            estimated_vram: 0,
            estimated_tokens: 0,
        });

        pool.start();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        pool.stop().await;

        // None completed because dequeue fails
        assert_eq!(queue.completed.load(Ordering::SeqCst), 0);
        assert_eq!(queue.failed.load(Ordering::SeqCst), 0);
    }
}
