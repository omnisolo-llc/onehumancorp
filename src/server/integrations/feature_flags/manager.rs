use std::collections::HashMap;
use sqlx::{Pool, Any};
use async_trait::async_trait;

#[async_trait]
pub trait FeatureStore: Send + Sync {
    async fn evaluate_flag(&self, tenant_id: &str, flag_key: &str, context: &HashMap<String, String>) -> Result<bool, String>;
    async fn list_flags(&self, tenant_id: &str) -> Result<Vec<String>, String>;
}

pub struct FeatureFlagsManager {
    store: Box<dyn FeatureStore>,
    pub is_cloud: bool,
}

impl FeatureFlagsManager {
    pub fn new(store: Box<dyn FeatureStore>, is_cloud: bool) -> Self {
        Self {
            store,
            is_cloud,
        }
    }

    pub fn from_env(store: Box<dyn FeatureStore>) -> Self {
        let is_cloud = std::env::var("OHC_MULTITENANT").unwrap_or_default() == "true";
        Self::new(store, is_cloud)
    }

    pub async fn evaluate_flag(&self, tenant_id: &str, flag_key: &str, context: &HashMap<String, String>) -> Result<bool, String> {
        let actual_tenant = if self.is_cloud { tenant_id } else { "local" };
        self.store.evaluate_flag(actual_tenant, flag_key, context).await
    }

    pub async fn list_flags(&self, tenant_id: &str) -> Result<Vec<String>, String> {
        let actual_tenant = if self.is_cloud { tenant_id } else { "local" };
        self.store.list_flags(actual_tenant).await
    }
}

// In-Memory store for tests
pub struct MemoryFeatureStore {
    flags: std::sync::RwLock<HashMap<String, HashMap<String, bool>>>,
}

impl MemoryFeatureStore {
    pub fn new() -> Self {
        Self {
            flags: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn set_flag(&self, tenant_id: &str, flag_key: &str, value: bool) {
        let mut map = self.flags.write().unwrap();
        map.entry(tenant_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(flag_key.to_string(), value);
    }
}

#[async_trait]
impl FeatureStore for MemoryFeatureStore {
    async fn evaluate_flag(&self, tenant_id: &str, flag_key: &str, _context: &HashMap<String, String>) -> Result<bool, String> {
        let map = self.flags.read().unwrap();
        if let Some(tenant_flags) = map.get(tenant_id) {
            if let Some(&val) = tenant_flags.get(flag_key) {
                return Ok(val);
            }
        }
        Ok(false) // Default to false if not found
    }

    async fn list_flags(&self, tenant_id: &str) -> Result<Vec<String>, String> {
        let map = self.flags.read().unwrap();
        if let Some(tenant_flags) = map.get(tenant_id) {
            Ok(tenant_flags.keys().cloned().collect())
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feature_flags_manager_cloud() {
        let store = MemoryFeatureStore::new();
        store.set_flag("tenant_123", "new_ui", true);

        let manager = FeatureFlagsManager::new(Box::new(store), true);

        let context = HashMap::new();
        let result = manager.evaluate_flag("tenant_123", "new_ui", &context).await.unwrap();
        assert!(result);

        let result = manager.evaluate_flag("tenant_123", "nonexistent", &context).await.unwrap();
        assert!(!result);

        let mut flags = manager.list_flags("tenant_123").await.unwrap();
        flags.sort();
        assert_eq!(flags, vec!["new_ui"]);

        // Different tenant
        let result = manager.evaluate_flag("tenant_other", "new_ui", &context).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_feature_flags_manager_standalone() {
        let store = MemoryFeatureStore::new();
        // In standalone, tenant is ignored and always uses "local"
        store.set_flag("local", "new_ui", true);

        let manager = FeatureFlagsManager::new(Box::new(store), false);

        let context = HashMap::new();
        let result = manager.evaluate_flag("tenant_any", "new_ui", &context).await.unwrap();
        assert!(result);

        let mut flags = manager.list_flags("tenant_any").await.unwrap();
        flags.sort();
        assert_eq!(flags, vec!["new_ui"]);
    }

    #[tokio::test]
    async fn test_from_env() {
        temp_env::with_var("OHC_MULTITENANT", Some("true"), || {
            let store = Box::new(MemoryFeatureStore::new());
            let manager = FeatureFlagsManager::from_env(store);
            assert!(manager.is_cloud);
        });

        temp_env::with_var("OHC_MULTITENANT", None::<&str>, || {
            let store = Box::new(MemoryFeatureStore::new());
            let manager = FeatureFlagsManager::from_env(store);
            assert!(!manager.is_cloud);
        });
    }
}
