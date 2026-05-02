use async_trait::async_trait;
use std::io;
use std::sync::Arc;
use super::provider::{BlobMetadata, Provider};
use crate::pricing::rate_limit::RedisRateLimiter;

pub struct QuotaEnforcingProvider {
    inner: Arc<dyn Provider>,
    rate_limiter: Arc<RedisRateLimiter>,
}

impl QuotaEnforcingProvider {
    pub fn new(inner: Arc<dyn Provider>, rate_limiter: Arc<RedisRateLimiter>) -> Self {
        Self { inner, rate_limiter }
    }
}

#[async_trait]
impl Provider for QuotaEnforcingProvider {
    fn is_local(&self) -> bool {
        self.inner.is_local()
    }

    async fn list_blobs(&self, prefix: &str) -> io::Result<Vec<BlobMetadata>> {
        self.inner.list_blobs(prefix).await
    }

    async fn read_blob_metadata(&self, key: &str) -> io::Result<BlobMetadata> {
        self.inner.read_blob_metadata(key).await
    }

    async fn get_blob_url(&self, key: &str) -> io::Result<String> {
        self.inner.get_blob_url(key).await
    }

    async fn read_blob(&self, key: &str) -> io::Result<Vec<u8>> {
        self.inner.read_blob(key).await
    }

    async fn write_blob(&self, key: &str, data: &[u8]) -> io::Result<()> {
        // Extract tenant_id from key: tenants/{tenant_id}/...
        let parts: Vec<&str> = key.split('/').collect();
        if parts.len() > 1 && parts[0] == "tenants" {
            let tenant_id = parts[1];

            // Check usage
            let current_usage = self.inner.get_tenant_usage(tenant_id).await?;
            let current_usage_mb = (current_usage / (1024 * 1024)) as u32;

            if let Ok(tier) = self.rate_limiter.get_tenant_tier(tenant_id).await {
                if let Some(limit_mb) = tier.storage_limit_mb() {
                    if current_usage_mb >= limit_mb {
                        return Err(io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            format!("Storage quota exceeded for tier {:?}: {}MB / {}MB", tier, current_usage_mb, limit_mb)
                        ));
                    }
                }
            }
        }

        self.inner.write_blob(key, data).await
    }

    async fn get_tenant_usage(&self, tenant_id: &str) -> io::Result<i64> {
        self.inner.get_tenant_usage(tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::local_provider::LocalProvider;
    use crate::pricing::rate_limit::{PlanTier, RedisRateLimiter};
    use redis::Client;
    use std::fs;

    #[tokio::test]
    async fn test_storage_quota_enforcement() {
        let dir = "/tmp/test_quota_enforcer";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        let local = Arc::new(LocalProvider::new(dir).unwrap());

        // Setup mock redis for rate limiter
        let client = Client::open("redis://127.0.0.1:6379").unwrap();
        let limiter = Arc::new(RedisRateLimiter::new(client));

        let provider = QuotaEnforcingProvider::new(local, limiter.clone());

        let tenant_id = "test_tenant";
        let key = format!("tenants/{}/file.txt", tenant_id);
        let data = vec![0u8; 100];

        let _ = limiter.set_tenant_tier(tenant_id, PlanTier::Free).await;

        let res = provider.write_blob(&key, &data).await;
        assert!(res.is_ok());
    }
}
