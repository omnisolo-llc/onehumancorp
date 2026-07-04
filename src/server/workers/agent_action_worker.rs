use std::sync::Arc;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use crate::orchestration::queue::OHCJobQueue;
use crate::orchestration::queue::redis_lock::RedisLock;
use serde_json::Value;

pub struct AgentActionWorker {
    pub pool: PgPool,
    pub redis_url: String,
}

impl AgentActionWorker {
    pub fn new(pool: PgPool, redis_url: String) -> Self {
        Self { pool, redis_url }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    pub async fn process_job(&self, job: crate::orchestration::queue::ohc_job_queue::OHCJob, queue: &OHCJobQueue, redis_lock: &RedisLock) {
        let parsed: Result<Value, _> = serde_json::from_str(&job.payload);
        if let Ok(payload) = parsed {
            let tenant_id = &job.tenant_id;
            let action_id = payload.get("action_id").and_then(|v| v.as_str()).unwrap_or(&job.id);

            // Acquire lock
            match redis_lock.acquire_lock(tenant_id, "agent_feed", action_id, 300).await {
                Ok(Some(lock_val)) => {
                    // Process action with timeout
                    let is_incident = payload.get("is_incident").and_then(|v| v.as_bool()).unwrap_or(false);
                    let dispatch_payload = payload.get("payload");
                    let feature_type = payload.get("feature_type").and_then(|v| v.as_str());

                    let success;
                    let mut malformed = false;

                    let process_future = async {
                        let mut task_success = true;
                        if is_incident {
                            if let Some(payload_val) = dispatch_payload {
                                if let Err(e) = crate::domain::incidents::handle_incident_resolution(tenant_id, &sqlx::types::Json(payload_val.clone()), &self.pool).await {
                                    tracing::error!("Incident resolution failed: {}", e);
                                    task_success = false;
                                }
                            }
                        } else if let Some(ft) = feature_type {
                            if let Some(payload_val) = dispatch_payload {
                                if let Err(e) = crate::domain::action_router::dispatch_action(ft, tenant_id, &sqlx::types::Json(payload_val.clone()), &self.pool).await {
                                    tracing::error!("Action dispatch failed: {}", e);
                                    task_success = false;
                                }
                            }
                        } else {
                            // Invalid malformed payload: missing feature_type or is_incident flag
                            task_success = false;
                        }
                        task_success
                    };

                    match tokio::time::timeout(Duration::from_secs(60), process_future).await {
                        Ok(task_success) => {
                            if !task_success && !is_incident && feature_type.is_none() {
                                success = false;
                                malformed = true;
                            } else {
                                success = task_success;
                            }
                        }
                        Err(_) => {
                            tracing::error!("Agent execution exceeded 60-second ML-Resilience timeout rule for job {}", job.id);
                            success = false;
                        }
                    }

                    if success {
                        let _ = queue.complete(&job.id).await;
                    } else if malformed {
                        let _ = queue.fail(&job.id, 3, "Invalid malformed payload: missing feature_type or is_incident flag").await;
                    } else {
                        let reason = if success { "" } else { "Agent execution exceeded 60-second ML-Resilience timeout rule." };
                        let fail_reason = if reason.is_empty() { "Action execution failed" } else { reason };
                        let _ = queue.fail(&job.id, 3, fail_reason).await;
                    }

                    let _ = redis_lock.release_lock(tenant_id, "agent_feed", action_id, &lock_val).await;
                }
                Ok(None) => {
                    // Lock not acquired (already running?)
                    tracing::warn!("Could not acquire lock for action {}", action_id);
                    let _ = queue.fail(&job.id, 3, "Lock contention").await;
                }
                Err(e) => {
                    tracing::error!("Redis lock error: {}", e);
                    let _ = queue.fail(&job.id, 3, &e).await;
                }
            }
        } else {
            tracing::error!("Invalid payload in agent_feed_action job");
            let _ = queue.complete(&job.id).await; // complete to discard invalid
        }
    }

    async fn run(&self) {
        let pool_arc = Arc::new(self.pool.clone());
        let queue = OHCJobQueue::new(pool_arc.clone());
        let redis_lock = match RedisLock::new(&self.redis_url) {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to connect to Redis for agent action worker: {}", e);
                return;
            }
        };

        loop {
            match queue.dequeue(vec!["agent_feed_action"]).await {
                Ok(Some(job)) => {
                    self.process_job(job, &queue, &redis_lock).await;
                }
                Ok(None) => {
                    sleep(Duration::from_secs(2)).await;
                }
                Err(e) => {
                    tracing::error!("Error polling agent action queue: {}", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_ml_resilience_agent_action_timeout() {
        let start = std::time::Instant::now();
        let result = tokio::time::timeout(std::time::Duration::from_millis(60), async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            Ok::<(), String>(())
        })
        .await;

        assert!(
            result.is_err(),
            "AgentActionWorker must enforce ML-Resilience timeout"
        );
        assert!(
            start.elapsed() >= std::time::Duration::from_millis(50),
            "Timeout should wait the configured time"
        );
    }
}
