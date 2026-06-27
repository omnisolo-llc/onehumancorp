use async_trait::async_trait;
use std::sync::Arc;
use super::cache::HybridCache;

#[async_trait]
pub trait EdgeCacheProvider: Send + Sync {
    async fn invalidate_tags(&self, tags: Vec<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct DefaultEdgeCacheProvider {
    cache: Arc<HybridCache<String>>,
}

impl DefaultEdgeCacheProvider {
    pub fn new(cache: Arc<HybridCache<String>>) -> Self {
        Self { cache }
    }
}

#[async_trait]
impl EdgeCacheProvider for DefaultEdgeCacheProvider {
    async fn invalidate_tags(&self, tags: Vec<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for tag in tags {
            self.cache.invalidate_by_tag(&tag).await;
        }
        Ok(())
    }
}
