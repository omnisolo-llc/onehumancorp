use std::collections::HashMap;
use sqlx::{Pool, Any};
use async_trait::async_trait;

#[async_trait]
pub trait FeatureStore: Send + Sync {
    async fn evaluate_flag(&self, flag_key: &str, context: &HashMap<String, String>) -> Result<bool, String>;
    async fn list_flags(&self) -> Result<Vec<String>, String>;
}

pub struct FeatureFlagsManager {
    store: Box<dyn FeatureStore>,
}

impl FeatureFlagsManager {
    pub fn new(store: Box<dyn FeatureStore>) -> Self {
        Self {
            store,
        }
    }

    pub fn from_env(store: Box<dyn FeatureStore>) -> Self {
        Self::new(store)
    }

    pub async fn evaluate_flag(&self, flag_key: &str, context: &HashMap<String, String>) -> Result<bool, String> {
        self.store.evaluate_flag(flag_key, context).await
    }

    pub async fn list_flags(&self) -> Result<Vec<String>, String> {
        self.store.list_flags().await
    }
}

// Memory-based implementation
pub struct MemoryFeatureStore {
    flags: std::sync::RwLock<HashMap<String, bool>>,
}

impl MemoryFeatureStore {
    pub fn new() -> Self {
        Self {
            flags: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn set_flag(&self, flag_key: &str, value: bool) {
        let mut map = self.flags.write().unwrap();
        map.insert(flag_key.to_string(), value);
    }
}

#[async_trait]
impl FeatureStore for MemoryFeatureStore {
    async fn evaluate_flag(&self, flag_key: &str, _context: &HashMap<String, String>) -> Result<bool, String> {
        let map = self.flags.read().unwrap();
        if let Some(&val) = map.get(flag_key) {
            return Ok(val);
        }
        Ok(false) // Default to false if not found
    }

    async fn list_flags(&self) -> Result<Vec<String>, String> {
        let map = self.flags.read().unwrap();
        Ok(map.keys().cloned().collect())
    }
}

pub struct NamespacedFeatureStore {
    prefix: String,
    store: Box<dyn FeatureStore>,
}

impl NamespacedFeatureStore {
    pub fn new(tenant_id: &str, store: Box<dyn FeatureStore>) -> Self {
        let prefix = if tenant_id.is_empty() || tenant_id == "local" {
            "".to_string()
        } else {
            format!("{}:", tenant_id)
        };
        Self { prefix, store }
    }

    fn format_key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }
}

#[async_trait]
impl FeatureStore for NamespacedFeatureStore {
    async fn evaluate_flag(&self, flag_key: &str, context: &HashMap<String, String>) -> Result<bool, String> {
        self.store.evaluate_flag(&self.format_key(flag_key), context).await
    }

    async fn list_flags(&self) -> Result<Vec<String>, String> {
        let all_flags = self.store.list_flags().await?;
        if self.prefix.is_empty() {
            return Ok(all_flags);
        }
        Ok(all_flags
            .into_iter()
            .filter_map(|key| {
                if key.starts_with(&self.prefix) {
                    Some(key.strip_prefix(&self.prefix).unwrap().to_string())
                } else {
                    None
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feature_flags_manager_cloud() {
        let store = MemoryFeatureStore::new();
        store.set_flag("tenant_123:new_ui", true);

        let namespaced = NamespacedFeatureStore::new("tenant_123", Box::new(store));
        let manager = FeatureFlagsManager::new(Box::new(namespaced));

        let context = HashMap::new();
        let result = manager.evaluate_flag("new_ui", &context).await.unwrap();
        assert!(result);

        let result = manager.evaluate_flag("nonexistent", &context).await.unwrap();
        assert!(!result);

        let mut flags = manager.list_flags().await.unwrap();
        flags.sort();
        assert_eq!(flags, vec!["new_ui"]);
    }

    #[tokio::test]
    async fn test_feature_flags_manager_standalone() {
        let store = MemoryFeatureStore::new();
        store.set_flag("new_ui", true);

        let namespaced = NamespacedFeatureStore::new("local", Box::new(store));
        let manager = FeatureFlagsManager::new(Box::new(namespaced));

        let context = HashMap::new();
        let result = manager.evaluate_flag("new_ui", &context).await.unwrap();
        assert!(result);

        let mut flags = manager.list_flags().await.unwrap();
        flags.sort();
        assert_eq!(flags, vec!["new_ui"]);
    }
}
