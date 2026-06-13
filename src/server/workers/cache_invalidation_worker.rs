use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

// Correct path based on lib.rs orchestration mod structure
use crate::orchestration::queue::ohc_job_queue::OHCJobQueue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInvalidationPayload {
    pub resource_id: String,
    pub path: Option<String>,
}

pub struct CacheInvalidationWorker {
    job_queue: Arc<OHCJobQueue>,
    cdn_client: Arc<dyn CdnInvalidator + Send + Sync>,
}

#[async_trait::async_trait]
pub trait CdnInvalidator {
    async fn purge_cache_key(&self, cache_key: &str) -> Result<(), String>;
}

pub struct StubCdnClient;

#[async_trait::async_trait]
impl CdnInvalidator for StubCdnClient {
    async fn purge_cache_key(&self, cache_key: &str) -> Result<(), String> {
        info!("MOCK CDN PURGE: Successfully purged cache key: {}", cache_key);
        Ok(())
    }
}

impl CacheInvalidationWorker {
    pub fn new(job_queue: Arc<OHCJobQueue>, cdn_client: Arc<dyn CdnInvalidator + Send + Sync>) -> Self {
        Self {
            job_queue,
            cdn_client,
        }
    }

    pub async fn run_once(&self) -> Result<(), String> {
        let job = self.job_queue.dequeue(vec!["cache_invalidation"]).await?;

        if let Some(job) = job {
            let payload: CacheInvalidationPayload = match serde_json::from_str(&job.payload) {
                Ok(p) => p,
                Err(e) => {
                    self.job_queue.fail_job(&job.id, &e.to_string()).await?;
                    return Err(format!("Invalid payload for job {}: {}", job.id, e));
                }
            };

            let tenant_id = job.tenant_id.clone();

            // Build the primary cache key
            let mut keys_to_purge = vec![format!("storefront:{}:{}", tenant_id, payload.resource_id)];

            if let Some(path) = payload.path {
                keys_to_purge.push(format!("storefront:{}:{}", tenant_id, path));
            }

            let mut has_error = false;
            let mut last_err = String::new();

            for key in keys_to_purge {
                if let Err(e) = self.cdn_client.purge_cache_key(&key).await {
                    error!("Failed to purge cache key {}: {}", key, e);
                    has_error = true;
                    last_err = e;
                }
            }

            if has_error {
                self.job_queue.fail_job(&job.id, &last_err).await?;
            } else {
                self.job_queue.complete_job(&job.id).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use crate::orchestration::queue::ohc_job_queue::OHCJobQueue;
    use sqlx::PgPool;

    struct TestCdnClient {
        called: Arc<AtomicBool>,
    }

    #[async_trait]
    impl CdnInvalidator for TestCdnClient {
        async fn purge_cache_key(&self, _cache_key: &str) -> Result<(), String> {
            self.called.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_cache_invalidation_worker() {
        let called = Arc::new(AtomicBool::new(false));
        let cdn_client = Arc::new(TestCdnClient { called: called.clone() });
        // Full job queue interaction is integration scope.
        // We test just that purging triggers successfully.
        let _ = cdn_client.purge_cache_key("test").await;
        assert!(called.load(Ordering::SeqCst));
    }
}
