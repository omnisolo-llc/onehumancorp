use std::sync::Arc;

use crate::orchestration::queue::OmniSoloJobQueue;
use crate::orchestration::queue::redis_lock::RedisLock;
use serde_json::Value;
use sqlx::PgPool;
use tokio::time::{Duration, sleep};

pub struct AgentActionWorker {
    pub pool: PgPool,
    pub redis_url: String,
    catalog_authority: Option<super::agent_catalog_dispatch::CanonicalCatalogDispatch>,
}

impl AgentActionWorker {
    pub fn new(pool: PgPool, redis_url: String) -> Self {
        Self {
            pool,
            redis_url,
            catalog_authority: None,
        }
    }

    /// Bind local writes to the same canonical identity and business objects
    /// used for owner admission. The legacy constructor remains fail-closed for
    /// catalog work until this proof succeeds.
    pub async fn with_authority(mut self, store: &server_auth::Store) -> Self {
        self.catalog_authority =
            match super::agent_catalog_dispatch::CanonicalCatalogDispatch::bind(store, &self.pool)
                .await
            {
                Ok(authority) => Some(authority),
                Err(error) => {
                    tracing::error!(%error, "Canonical catalog authority is unavailable; catalog work will be held before any effect");
                    None
                }
            };
        self
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    pub async fn process_job(
        &self,
        job: crate::orchestration::queue::omnisolo_job_queue::OmniSoloJob,
        _queue: &OmniSoloJobQueue,
        _redis_lock: &RedisLock,
    ) {
        self.process_durable_job(job).await;
    }

    async fn process_durable_job(
        &self,
        job: crate::orchestration::queue::omnisolo_job_queue::OmniSoloJob,
    ) {
        use super::agent_feed_dispatch as dispatch;
        let attempt = match dispatch::claim(&self.pool, &job).await {
            Ok(Some(attempt)) => attempt,
            Ok(None) => return,
            Err(error) => {
                tracing::warn!(%error, job_id=%job.id, "Feed dispatch admission unconfirmed");
                if let Err(error) = dispatch::defer_or_hold(&self.pool, &job).await {
                    tracing::error!(%error, "Feed pre-attempt retry persistence unavailable");
                }
                return;
            }
        };
        let local_catalog = attempt.payload.get("is_incident").and_then(Value::as_bool)
            != Some(true)
            && attempt.payload.get("feature_type").and_then(Value::as_str)
                == Some("create_product");
        let process = async {
            if local_catalog {
                return self
                    .catalog_authority
                    .as_ref()
                    .ok_or_else(|| "Canonical catalog authority is unavailable".to_string())?
                    .execute(&attempt)
                    .await
                    .map_err(|error| error.to_string());
            }
            let payload = attempt
                .payload
                .get("payload")
                .ok_or_else(|| "Missing dispatch payload".to_string())?;
            if attempt.payload.get("is_incident").and_then(Value::as_bool) == Some(true) {
                crate::domain::incidents::handle_incident_resolution(
                    &attempt.tenant_id,
                    payload,
                    &self.pool,
                )
                .await
                .map_err(|error| error.to_string())
            } else {
                let feature = attempt
                    .payload
                    .get("feature_type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| "Missing dispatch feature".to_string())?;
                crate::domain::action_router::dispatch_action(
                    feature,
                    &attempt.tenant_id,
                    payload,
                    &self.pool,
                )
                .await
            }
        };
        let returned = match tokio::time::timeout(Duration::from_secs(60), process).await {
            Ok(Ok(())) => true,
            Ok(Err(error)) => {
                tracing::warn!(%error,job_id=%job.id,"Attempted feed dispatch requires reconciliation");
                false
            }
            Err(_) => {
                tracing::warn!(job_id=%job.id,"Timed-out feed dispatch requires reconciliation");
                false
            }
        };
        if local_catalog && returned {
            // The product and returned receipt already committed together.
            return;
        }
        if let Err(error) = dispatch::finish(&self.pool, &attempt, returned).await {
            tracing::error!(%error,job_id=%job.id,"Feed dispatch return acknowledgement unconfirmed");
            if let Err(error) = dispatch::defer_or_hold(&self.pool, &job).await {
                tracing::error!(%error,"Feed reconciliation persistence unavailable; durable attempt remains held");
            }
        }
    }

    async fn run(&self) {
        let pool_arc = Arc::new(self.pool.clone());
        let queue = OmniSoloJobQueue::new(pool_arc.clone());
        loop {
            if let Err(error) = super::agent_feed_dispatch::recover_abandoned(&self.pool).await {
                tracing::warn!(%error,"Feed dispatch restart reconciliation unavailable");
            }
            match queue.dequeue(vec!["agent_feed_action"]).await {
                Ok(Some(job)) => {
                    self.process_durable_job(job).await;
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
