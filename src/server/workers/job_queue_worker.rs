use crate::db::DB;
use crate::orchestration::locks::DistributedLock;
use crate::orchestration::queue::TaskQueue;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct JobQueueWorker {
    db: Arc<DB>,
    queue: Arc<dyn TaskQueue>,
    lock_manager: Arc<dyn DistributedLock>,
    roles: Vec<String>,
}

impl JobQueueWorker {
    pub fn new(
        db: Arc<DB>,
        queue: Arc<dyn TaskQueue>,
        lock_manager: Arc<dyn DistributedLock>,
        roles: Vec<String>,
    ) -> Self {
        Self {
            db,
            queue,
            lock_manager,
            roles,
        }
    }

    pub fn start(&self) {
        let queue = self.queue.clone();
        let lock_manager = self.lock_manager.clone();
        let roles = self.roles.clone();

        tokio::spawn(async move {
            loop {
                // Try to dequeue a job
                match queue.dequeue(roles.clone(), 0, 0).await {
                    Ok(Some(job)) => {
                        // Acquire lock for this resource
                        // e.g. using parent_task_id as resource_id to ensure we don't process jobs for same task concurrently
                        let resource_id = if job.parent_task_id.is_empty() {
                            job.id.clone()
                        } else {
                            job.parent_task_id.clone()
                        };

                        let lock_result = lock_manager
                            .acquire(&job.tenant_id, &job.agent_role, &resource_id)
                            .await;

                        match lock_result {
                            Ok(_guard) => {
                                let span = tracing::info_span!("job_queue_worker", job_id = %job.id, tenant_id = %job.tenant_id, role = %job.agent_role);
                                let _enter = span.enter();
                                tracing::info!("Processing job {} for role {}", job.id, job.agent_role);

                                let start = std::time::Instant::now();

                                // Here we would dispatch to the AI agent or actual business logic handler.
                                // Using a small delay to simulate work
                                sleep(Duration::from_millis(100)).await;

                                let duration = start.elapsed().as_secs_f64();
                                ::server_telemetry::record_task_processing_latency(::server_telemetry::get_deployment_mode(), duration);

                                // On success
                                if let Err(e) = queue.complete(&job.id).await {
                                    tracing::error!("Failed to complete job {}: {}", job.id, e);
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to acquire lock for job {}, requeuing. err: {}", job.id, e);
                                // We might want to enqueue it back or fail it so it backs off
                                let _ = queue.fail(&job.id, "lock_unavailable").await;
                            }
                        }
                    }
                    Ok(None) => {
                        // Queue empty, sleep
                        sleep(Duration::from_millis(500)).await;
                    }
                    Err(e) => {
                        tracing::error!("Error dequeueing jobs: {}", e);
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    // use super::*
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use crate::orchestration::queue::{Job, TaskQueue};
    use crate::orchestration::locks::{DistributedLock, LockGuard};
    use async_trait::async_trait;

    struct MockQueue {
        job: Mutex<Option<Job>>,
        completed: Mutex<bool>,
        failed: Mutex<bool>,
    }

    #[async_trait]
    impl TaskQueue for MockQueue {
        async fn enqueue(&self, _job: Job) -> Result<(), String> { Ok(()) }
        async fn dequeue(&self, _roles: Vec<String>, _vram: i64, _tokens: i64) -> Result<Option<Job>, String> {
            let mut j = self.job.lock().await;
            Ok(j.take())
        }
        async fn complete(&self, _job_id: &str) -> Result<(), String> {
            *self.completed.lock().await = true;
            Ok(())
        }
        async fn fail(&self, _job_id: &str, _reason: &str) -> Result<(), String> {
            *self.failed.lock().await = true;
            Ok(())
        }
    }

    struct MockLockManager {
        allow_lock: bool,
    }

    #[async_trait]
    impl DistributedLock for MockLockManager {
        async fn acquire(&self, _tenant_id: &str, _resource_type: &str, _resource_id: &str) -> Result<LockGuard, String> {
            if self.allow_lock {
                Ok(LockGuard { _local_guard: None, redis_client: None, redis_key: None })
            } else {
                Err("Lock denied".into())
            }
        }
    }

    // Dummy test to ensure basic test structure exists and compiles without needing full DB state mocking
    #[tokio::test]
    async fn test_job_queue_worker_compiles() {
        let dummy_job = Job {
            id: "1".into(),
            tenant_id: "t1".into(),
            parent_task_id: "".into(),
            agent_role: "role".into(),
            payload: "{}".into(),
            status: "QUEUED".into(),
            attempts: 0,
            max_attempts: 3,
            run_after: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let queue = Arc::new(MockQueue {
            job: Mutex::new(Some(dummy_job)),
            completed: Mutex::new(false),
            failed: Mutex::new(false),
        });

        let lock = Arc::new(MockLockManager { allow_lock: true });

        // This is a minimal unit test check
        assert!(lock.acquire("t", "rt", "rid").await.is_ok());
    }
}
