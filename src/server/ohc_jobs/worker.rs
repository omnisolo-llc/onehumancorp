use std::sync::Arc;
use tokio::time::{sleep, Duration};
use futures::future::BoxFuture;

use super::queue::{Job, JobQueue};
use super::ledger::Ledger;

pub type JobHandler = Arc<dyn Fn(Job, Arc<Ledger>) -> BoxFuture<'static, Result<(), String>> + Send + Sync>;

pub struct WorkerPool {
    queue: Arc<JobQueue>,
    ledger: Arc<Ledger>,
    handlers: std::collections::HashMap<String, JobHandler>,
}

impl WorkerPool {
    pub fn new(queue: Arc<JobQueue>, ledger: Arc<Ledger>) -> Self {
        Self {
            queue,
            ledger,
            handlers: std::collections::HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, job_type: &str, handler: JobHandler) {
        self.handlers.insert(job_type.to_string(), handler);
    }

    pub async fn run(&self) {
        loop {
            match self.queue.dequeue().await {
                Ok(Some(job)) => {
                    let job_type = job.job_type.clone();

                    if let Some(handler) = self.handlers.get(&job_type) {
                        let result = handler(job.clone(), self.ledger.clone()).await;

                        match result {
                            Ok(_) => {
                                let _ = self.queue.complete(job.id, &job.tenant_id).await;
                            }
                            Err(_) => {
                                // Apply exponential backoff: 2^retry_count * 60 seconds
                                let backoff_secs = 60 * 2_u64.pow(job.retry_count as u32);
                                let _ = self.queue.fail(job.id, &job.tenant_id, Duration::from_secs(backoff_secs)).await;
                            }
                        }
                    } else {
                        // Mark as failed if no handler is registered
                        let _ = self.queue.fail(job.id, &job.tenant_id, Duration::from_secs(60)).await;
                    }
                }
                Ok(None) => {
                    // No jobs available, sleep before polling again
                    sleep(Duration::from_millis(500)).await;
                }
                Err(_) => {
                    // Database error, back off slightly
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
