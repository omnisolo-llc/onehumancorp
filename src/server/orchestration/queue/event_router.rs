use std::sync::Arc;
use tokio::time::Duration;
use uuid::Uuid;

use crate::db::DB;
use crate::orchestration::queue::ohc_async_jobs_queue::OHCAsyncJobsQueue;
use crate::orchestration::queue::redis_lock::RedisLock;

pub struct EventRouterWorker {
    db: Arc<DB>,
    queue: Arc<OHCAsyncJobsQueue>,
    redis_lock: Option<Arc<RedisLock>>,
}

impl EventRouterWorker {
    pub fn new(db: Arc<DB>, queue: Arc<OHCAsyncJobsQueue>) -> Self {
        let redis_lock = std::env::var("REDIS_URL")
            .ok()
            .and_then(|url| RedisLock::new(&url).ok())
            .map(Arc::new);

        Self {
            db,
            queue,
            redis_lock,
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            tracing::info!("Starting EventRouterWorker for Universal Event Bus...");
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue immediately
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                    Err(e) => {
                        tracing::error!("EventRouterWorker error: {}", e);
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<bool, String> {
        // Dequeue any event router related jobs
        let job = self.queue.dequeue(vec!["customer_support", "scheduling", "inventory_alert"]).await?;

        if let Some(job) = job {
            let job_id = job.id.clone();

            // Try to acquire distributed lock if possible to avoid parallel duplications
            let mut lock_val_opt = None;
            let identifier = if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&job.payload) {
                payload.get("identifier").and_then(|v| v.as_str()).unwrap_or("unknown").to_string()
            } else {
                "unknown".to_string()
            };

            if let Some(rlock) = &self.redis_lock {
                let resource_id = format!("{}_{}", job.job_type, identifier);
                match rlock.acquire_lock(&job.tenant_id, "event_bus_job", &resource_id, 60).await {
                    Ok(Some(val)) => {
                        lock_val_opt = Some((resource_id, val));
                    }
                    Ok(None) => {
                        // Another worker is processing this event, fail so it gets retried later if the other fails
                        let _ = self.queue.fail(&job_id, "Could not acquire distributed lock, another worker is processing").await;
                        return Ok(true);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to interact with RedisLock: {}", e);
                    }
                }
            }

            // Process the job
            let res = self.process_job(&job).await;

            // Release lock
            if let Some(rlock) = &self.redis_lock {
                if let Some((resource_id, lock_val)) = lock_val_opt {
                    let _ = rlock.release_lock(&job.tenant_id, "event_bus_job", &resource_id, &lock_val).await;
                }
            }

            match res {
                Ok(_) => {
                    let _ = self.queue.complete(&job_id).await;
                }
                Err(e) => {
                    let _ = self.queue.fail(&job_id, &e).await;
                }
            }

            return Ok(true);
        }

        Ok(false)
    }

    async fn process_job(&self, job: &super::ohc_async_jobs_queue::OHCAsyncJob) -> Result<(), String> {
        let payload: serde_json::Value = serde_json::from_str(&job.payload).map_err(|e| format!("Failed to parse payload: {}", e))?;

        if job.job_type == "customer_support" {
            let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
            let source = payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");
            let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            let identifier = payload.get("identifier").and_then(|v| v.as_str()).unwrap_or("");

            let intent_id = Uuid::new_v4().to_string();

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("INSERT INTO work_intents (id, tenant_id, source, intent_type, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())")
                        .bind(&intent_id)
                        .bind(&job.tenant_id)
                        .bind(source)
                        .bind("customer_inquiry")
                        .bind(serde_json::json!({"message": message, "identifier": identifier, "customer_id": customer_id}))
                        .bind("PENDING")
                        .execute(&self.db.pool).await.map(|_| ()).map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("INSERT INTO work_intents (id, tenant_id, source, intent_type, payload, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(&intent_id)
                        .bind(&job.tenant_id)
                        .bind(source)
                        .bind("customer_inquiry")
                        .bind(serde_json::json!({"message": message, "identifier": identifier, "customer_id": customer_id}).to_string())
                        .bind("PENDING")
                        .execute(sqlite_pool).await.map(|_| ()).map_err(|e| e.to_string())?;
                }
            }

            // Push to legacy queue to actually draft the reply via LLM in the MessageTriageWorker
            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                        .bind(Uuid::new_v4().to_string())
                        .bind(&job.tenant_id)
                        .bind(serde_json::json!({
                            "message_id": intent_id,
                            "inbox_message_id": intent_id,
                            "source": source,
                            "content": message,
                            "customer_id": customer_id,
                            "sender_id": identifier
                        }))
                        .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                        .bind(Uuid::new_v4().to_string())
                        .bind(&job.tenant_id)
                        .bind(serde_json::json!({
                            "message_id": intent_id,
                            "inbox_message_id": intent_id,
                            "source": source,
                            "content": message,
                            "customer_id": customer_id,
                            "sender_id": identifier
                        }).to_string())
                        .execute(sqlite_pool).await.map_err(|e| e.to_string())?;
                }
            }
        }

        Ok(())
    }
}
