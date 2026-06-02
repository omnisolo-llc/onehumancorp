use super::ohc_job_queue::{OhcJob, OhcTaskQueue};
use super::redlock::Redlock;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait OhcTaskJobHandler: Send + Sync {
    async fn handle(&self, job: OhcJob) -> Result<(), String>;
}

pub struct OhcWorkerPool {
    queue: Arc<dyn OhcTaskQueue>,
    roles: Vec<String>,
    handler: Arc<dyn OhcTaskJobHandler>,
    redlock: Option<Arc<Redlock>>,
}

impl OhcWorkerPool {
    pub fn new(queue: Arc<dyn OhcTaskQueue>, roles: Vec<String>, handler: Arc<dyn OhcTaskJobHandler>, redlock: Option<Arc<Redlock>>) -> Self {
        Self { queue, roles, handler, redlock }
    }

    pub async fn start(&self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    break;
                }
                job_opt = self.queue.dequeue(self.roles.clone()) => {
                    match job_opt {
                        Ok(Some(job)) => {
                            let mut lock_value = None;
                            let mut lock_key = String::new();

                            // Instead of locking the job ID, we attempt to lock an optional "resource_type" and "resource_id"
                            // defined in the job payload. This handles cross-agent coordination over the same domain resources.
                            let payload_json: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);

                            let mut needs_lock = false;
                            if let (Some(res_type), Some(res_id)) = (
                                payload_json.get("resource_type").and_then(|v| v.as_str()),
                                payload_json.get("resource_id").and_then(|v| v.as_str())
                            ) {
                                needs_lock = true;
                                lock_key = Redlock::lock_key(&job.tenant_id, res_type, res_id);
                            }

                            if needs_lock {
                                if let Some(ref rl) = self.redlock {
                                    match rl.acquire(&lock_key, Duration::from_secs(30)).await {
                                        Ok(Some(val)) => {
                                            lock_value = Some(val);
                                        }
                                        Ok(None) => {
                                            // The resource is currently locked by another agent/worker.
                                            // Fail and trigger backoff.
                                            let _ = self.queue.fail(&job.id, "Domain resource locked by another worker").await;
                                            continue;
                                        }
                                        Err(e) => {
                                            let _ = self.queue.fail(&job.id, &format!("Redlock error: {}", e)).await;
                                            continue;
                                        }
                                    }
                                }
                            }

                            match self.handler.handle(job.clone()).await {
                                Ok(_) => {
                                    let _ = self.queue.complete(&job.id).await;
                                }
                                Err(e) => {
                                    let _ = self.queue.fail(&job.id, &e).await;
                                }
                            }

                            if let (Some(ref rl), Some(val)) = (&self.redlock, lock_value) {
                                let _ = rl.release(&lock_key, &val).await;
                            }
                        }
                        Ok(None) => {
                            sleep(Duration::from_millis(100)).await;
                        }
                        Err(e) => {
                            tracing::error!("Failed to dequeue job: {}", e);
                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            }
        }
    }
}
