use async_trait::async_trait;
use std::sync::Arc;
use crate::utils::cache::HybridCache;

#[async_trait]
pub trait EdgeProvider: Send + Sync {
    async fn push_content(&self, cache_key: &str, content: String, tags: Vec<String>);
    async fn invalidate_tag(&self, tag: &str);
    async fn get_content(&self, cache_key: &str) -> Option<String>;
}

pub struct CacheManager {
    provider: Arc<dyn EdgeProvider>,
}

impl CacheManager {
    pub fn new(provider: Arc<dyn EdgeProvider>) -> Self {
        Self { provider }
    }

    pub async fn push_static_content(&self, tenant_id: &str, resource_id: &str, content: String, extra_tags: Vec<String>) {
        let cache_key = format!("storefront:{}:{}", tenant_id, resource_id);
        let mut tags = vec![
            format!("tenant-id:{}", tenant_id),
            format!("resource:{}", resource_id),
        ];
        tags.extend(extra_tags);
        self.provider.push_content(&cache_key, content, tags).await;
    }

    pub async fn invalidate(&self, tenant_id: &str, resource_id: &str) {
        let tag = format!("resource:{}", resource_id);
        self.provider.invalidate_tag(&tag).await;

        let tenant_tag = format!("tenant-id:{}", tenant_id);
        self.provider.invalidate_tag(&tenant_tag).await;
    }
}

#[async_trait]
impl EdgeProvider for HybridCache<String> {
    async fn push_content(&self, cache_key: &str, content: String, tags: Vec<String>) {
        self.set_with_tags(cache_key, content, tags, std::time::Duration::from_secs(3600)).await;
    }

    async fn invalidate_tag(&self, tag: &str) {
        self.invalidate_by_tag(tag).await;
    }

    async fn get_content(&self, cache_key: &str) -> Option<String> {
        self.get(cache_key).await
    }
}

pub struct MockEdgeProvider {
    pub cache: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, String>>>,
    pub tags: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, Vec<String>>>>,
}

impl MockEdgeProvider {
    pub fn new() -> Self {
        Self {
            cache: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            tags: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait]
impl EdgeProvider for MockEdgeProvider {
    async fn push_content(&self, cache_key: &str, content: String, tags: Vec<String>) {
        self.cache.lock().await.insert(cache_key.to_string(), content);
        for tag in tags {
            self.tags.lock().await.entry(tag.clone()).or_insert_with(Vec::new).push(cache_key.to_string());
        }
    }

    async fn invalidate_tag(&self, tag: &str) {
        if let Some(keys) = self.tags.lock().await.remove(tag) {
            for key in keys {
                self.cache.lock().await.remove(&key);
            }
        }
    }

    async fn get_content(&self, cache_key: &str) -> Option<String> {
        self.cache.lock().await.get(cache_key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_manager_flow() {
        let provider = Arc::new(MockEdgeProvider::new());
        let cache_manager = CacheManager::new(provider.clone());

        cache_manager.push_static_content("tenant-1", "prod-1", "<html>prod 1</html>".to_string(), vec![]).await;

        let content = provider.get_content("storefront:tenant-1:prod-1").await;
        assert_eq!(content, Some("<html>prod 1</html>".to_string()));

        cache_manager.invalidate("tenant-1", "prod-1").await;

        let content_after = provider.get_content("storefront:tenant-1:prod-1").await;
        assert_eq!(content_after, None);
    }
}
