use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorefrontCacheConfig {
    pub tenant_id: String,
    pub default_ttl_seconds: u64,
    pub stale_while_revalidate_seconds: u64,
    pub cache_rules: Vec<CacheRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheRule {
    pub path_pattern: String,
    pub ttl_seconds: u64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeAsset {
    pub tenant_id: String,
    pub site_id: String,
    pub path: String,
    pub content_type: String,
    pub tags: Vec<String>,
    pub etag: Option<String>,
}
