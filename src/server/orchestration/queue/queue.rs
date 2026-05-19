use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub organization_id: String,
    pub parent_task_id: String,
    pub agent_role: String,
    pub payload: String,
    pub status: String,
    pub worker_id: Option<String>,
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
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> { for job in jobs { self.enqueue(job).await?; } Ok(()) }
    async fn dequeue(&self, roles: Vec<String>, estimated_vram: i64, estimated_tokens: i64) -> Result<Option<Job>, String>;
    async fn complete(&self, job_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String>;
}

use std::sync::Arc;

pub struct WorkerPool {
    queue: Arc<dyn TaskQueue>,
    workers: usize,
    roles: Vec<String>,
}

impl WorkerPool {
    pub fn new(queue: Arc<dyn TaskQueue>, workers: usize, roles: Vec<String>) -> Self {
        WorkerPool { queue, workers, roles }
    }

    pub async fn start(&self, shutdown_rx: tokio::sync::broadcast::Sender<()>) {
        for i in 0..self.workers {
            let queue = self.queue.clone();
            let roles = self.roles.clone();
            let mut rx = shutdown_rx.subscribe();

            tokio::spawn(async move {
                // In an actual scenario this would use tracing, but printing is safe for the stub.
                loop {
                    tokio::select! {
                        res = queue.dequeue(roles.clone(), 0, 0) => {
                            match res {
                                Ok(Some(job)) => {
                                    // Parse payload to dynamically route to AgentHarness
                                    let mut success = true;
                                    let mut err_msg = String::new();

                                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&job.payload) {
                                        // Extract an optional custom execution script or fallback
                                        let script = v.get("script").and_then(|s| s.as_str()).unwrap_or("echo 'default agent harness processing'");

                                        // Dynamically spawn the isolated harness
                                        let executor = crate::harness::executor::LocalShellTask::new(None);
                                        match executor.execute(script).await {
                                            Ok(_) => success = true,
                                            Err(e) => {
                                                success = false;
                                                err_msg = e;
                                            }
                                        }
                                    }

                                    if success {
                                        let _ = queue.complete(&job.id).await;
                                    } else {
                                        let _ = queue.fail(&job.id, &err_msg).await;
                                    }
                                }
                                Ok(None) => {
                                    tokio::time::sleep(Duration::from_millis(50)).await;
                                }
                                Err(_) => {
                                    tokio::time::sleep(Duration::from_millis(100)).await;
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
