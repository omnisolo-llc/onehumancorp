use std::sync::Arc;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use crate::orchestration::queue::OHCJobQueue;
use crate::orchestration::queue::redis_lock::RedisLock;
use serde_json::Value;

pub struct AgentActionWorker {
    pool: PgPool,
    redis_url: String,
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
                    let parsed: Result<Value, _> = serde_json::from_str(&job.payload);
                    if let Ok(payload) = parsed {
                        let tenant_id = &job.tenant_id;
                        let action_id = payload.get("action_id").and_then(|v| v.as_str()).unwrap_or(&job.id);

                        // Acquire lock
                        match redis_lock.acquire_lock(tenant_id, "agent_feed", action_id, 300).await {
                            Ok(Some(lock_val)) => {
                                // Process action
                                let is_incident = payload.get("is_incident").and_then(|v| v.as_bool()).unwrap_or(false);
                                let dispatch_payload = payload.get("payload");
                                let feature_type = payload.get("feature_type").and_then(|v| v.as_str());

                                let mut success = true;

                                if is_incident {
                                    if let Some(payload_val) = dispatch_payload {
                                        if let Err(e) = crate::domain::incidents::handle_incident_resolution(tenant_id, &sqlx::types::Json(payload_val.clone()), &self.pool).await {
                                            tracing::error!("Incident resolution failed: {}", e);
                                            success = false;
                                        }
                                    }
                                } else if let Some(ft) = feature_type {
                                    if let Some(payload_val) = dispatch_payload {
                                        if let Err(e) = crate::domain::action_router::dispatch_action(ft, tenant_id, &sqlx::types::Json(payload_val.clone()), &self.pool).await {
                                            tracing::error!("Action dispatch failed: {}", e);
                                            success = false;
                                        }
                                    }
                                }

                                if success {
                                    let _ = queue.complete(&job.id).await;
                                } else {
                                    let _ = queue.fail(&job.id, 3, "Action execution failed").await;
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
