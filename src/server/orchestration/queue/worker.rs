use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::time::sleep;

use super::{TaskQueue, Job};

#[async_trait]
pub trait JobHandler: Send + Sync {
    async fn handle_job(&self, job: Job) -> Result<Vec<u8>, String>;
}

pub struct WorkerPool {
    queue: Arc<dyn TaskQueue>,
    handler: Arc<dyn JobHandler>,
    roles: Vec<String>,
    estimated_vram: i64,
    estimated_tokens: i64,
    concurrency: usize,
}

impl WorkerPool {
    pub fn new(
        queue: Arc<dyn TaskQueue>,
        handler: Arc<dyn JobHandler>,
        roles: Vec<String>,
        estimated_vram: i64,
        estimated_tokens: i64,
        concurrency: usize,
    ) -> Self {
        Self {
            queue,
            handler,
            roles,
            estimated_vram,
            estimated_tokens,
            concurrency,
        }
    }

    pub fn start(&self) {
        for _ in 0..self.concurrency {
            let queue = self.queue.clone();
            let handler = self.handler.clone();
            let roles = self.roles.clone();
            let estimated_vram = self.estimated_vram;
            let estimated_tokens = self.estimated_tokens;

            tokio::spawn(async move {
                loop {
                    match queue.dequeue(roles.clone(), estimated_vram, estimated_tokens).await {
                        Ok(Some(job)) => {
                            let job_id = job.id.clone();
                            match handler.handle_job(job).await {
                                Ok(_result) => {
                                    if let Err(e) = queue.complete(&job_id).await {
                                        tracing::error!("Failed to complete job {}: {}", job_id, e);
                                    }
                                }
                                Err(e) => {
                                    if let Err(e) = queue.fail(&job_id, &e).await {
                                        tracing::error!("Failed to fail job {}: {}", job_id, e);
                                    }
                                }
                            }
                        }
                        Ok(None) => {
                            sleep(Duration::from_millis(500)).await;
                        }
                        Err(e) => {
                            tracing::error!("Failed to dequeue job: {}", e);
                            sleep(Duration::from_millis(1000)).await;
                        }
                    }
                }
            });
        }
    }
}
