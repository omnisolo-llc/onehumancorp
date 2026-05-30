use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::memory_store::LongTermMemory;

/// Memory: Long-term (OpenAI/LangGraph): Sessions backed by namespace-organized JSON Stores.
/// LangGraph's BaseStore-like interface.
#[async_trait]
pub trait BaseStore: Send + Sync {
    async fn put(&self, namespace: &[&str], key: &str, value: serde_json::Value) -> Result<(), String>;
    async fn get(&self, namespace: &[&str], key: &str) -> Result<Option<serde_json::Value>, String>;
    async fn search(&self, namespace_prefix: &[&str]) -> Result<Vec<JsonStoreItem>, String>;
    async fn delete(&self, namespace: &[&str], key: &str) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonStoreItem {
    pub namespace: Vec<String>,
    pub key: String,
    pub value: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct NamespaceJsonStore {
    base_dir: PathBuf,
}

impl NamespaceJsonStore {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn sanitize_component(c: &str) -> String {
        c.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_")
    }

    async fn get_namespace_dir(&self, namespace: &[&str], create_if_missing: bool) -> Result<Option<PathBuf>, String> {
        let mut path = self.base_dir.clone();
        for comp in namespace {
            let safe_comp = Self::sanitize_component(comp);
            if safe_comp.is_empty() {
                continue;
            }
            path.push(safe_comp);
        }
        if !path.exists() {
            if create_if_missing {
                fs::create_dir_all(&path).await.map_err(|e| format!("Failed to create namespace dir: {}", e))?;
            } else {
                return Ok(None);
            }
        }
        Ok(Some(path))
    }

    async fn get_item_path(&self, namespace: &[&str], key: &str, create_dir: bool) -> Result<Option<PathBuf>, String> {
        if let Some(dir) = self.get_namespace_dir(namespace, create_dir).await? {
            let safe_key = Self::sanitize_component(key);
            Ok(Some(dir.join(format!("{}.json", safe_key))))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl BaseStore for NamespaceJsonStore {
    async fn put(&self, namespace: &[&str], key: &str, value: serde_json::Value) -> Result<(), String> {
        let path = self.get_item_path(namespace, key, true).await?.unwrap();
        let tmp_path = path.with_extension("tmp");

        let now = chrono::Utc::now().timestamp();
        let mut created_at = now;

        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path).await {
                if let Ok(existing) = serde_json::from_str::<JsonStoreItem>(&content) {
                    created_at = existing.created_at;
                }
            }
        }

        let item = JsonStoreItem {
            namespace: namespace.iter().map(|s| s.to_string()).collect(),
            key: key.to_string(),
            value,
            created_at,
            updated_at: now,
        };

        let content = serde_json::to_string_pretty(&item).map_err(|e| format!("Failed to serialize: {}", e))?;

        let mut file = fs::File::create(&tmp_path).await.map_err(|e| format!("Failed to create temp file: {}", e))?;
        file.write_all(content.as_bytes()).await.map_err(|e| format!("Failed to write temp file: {}", e))?;
        file.sync_all().await.map_err(|e| format!("Failed to sync temp file: {}", e))?;

        fs::rename(&tmp_path, &path).await.map_err(|e| format!("Failed to commit file: {}", e))?;
        Ok(())
    }

    async fn get(&self, namespace: &[&str], key: &str) -> Result<Option<serde_json::Value>, String> {
        if let Some(path) = self.get_item_path(namespace, key, false).await? {
            if !path.exists() {
                return Ok(None);
            }
            let content = fs::read_to_string(&path).await.map_err(|e| format!("Failed to read file: {}", e))?;
            let item: JsonStoreItem = serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
            Ok(Some(item.value))
        } else {
            Ok(None)
        }
    }

    async fn search(&self, namespace_prefix: &[&str]) -> Result<Vec<JsonStoreItem>, String> {
        if let Some(dir) = self.get_namespace_dir(namespace_prefix, false).await? {
            let mut results = Vec::new();
            // recursively search all .json files in the dir
            let mut stack = vec![dir];
            while let Some(current_dir) = stack.pop() {
                if let Ok(mut entries) = fs::read_dir(current_dir).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let path = entry.path();
                        if path.is_dir() {
                            stack.push(path);
                        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                            if let Ok(content) = fs::read_to_string(&path).await {
                                if let Ok(item) = serde_json::from_str::<JsonStoreItem>(&content) {
                                    results.push(item);
                                }
                            }
                        }
                    }
                }
            }
            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }

    async fn delete(&self, namespace: &[&str], key: &str) -> Result<(), String> {
        if let Some(path) = self.get_item_path(namespace, key, false).await? {
            if path.exists() {
                fs::remove_file(&path).await.map_err(|e| format!("Failed to delete: {}", e))?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl LongTermMemory for NamespaceJsonStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut all_entries = Vec::new();

        if self.base_dir.exists() {
            // Retrieve all items across all namespaces to match previous naive behavior,
            // or just use search with an empty prefix
            if let Ok(items) = self.search(&[]).await {
                all_entries = items;
            }
        }

        // Basic naive search: sort by recency and filter by substring
        all_entries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        let mut seen = HashSet::new();

        for item in all_entries {
            let value = item.value;
            // assume value is string for LongTermMemory compat, or stringify
            let content = if let Some(s) = value.as_str() {
                s.to_string()
            } else {
                value.to_string()
            };

            if !seen.contains(&content) && (query.is_empty() || content.to_lowercase().contains(&query_lower)) {
                seen.insert(content.clone());
                results.push(content);
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let namespaces = if tags.is_empty() {
            vec!["default".to_string()]
        } else {
            tags
        };

        let value = serde_json::Value::String(content.to_string());

        for ns in namespaces {
            let key = uuid::Uuid::new_v4().to_string();
            let ns_refs: Vec<&str> = vec![ns.as_str()];
            self.put(&ns_refs, &key, value.clone()).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_namespace_json_store_base_operations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = NamespaceJsonStore::new(temp_dir.path());

        let ns1 = vec!["agent", "123"];
        let ns2 = vec!["agent", "456"];

        // Test search when dir doesn't exist
        let missing = store.search(&ns1).await.unwrap();
        assert_eq!(missing.len(), 0);

        // Test put and get
        store.put(&ns1, "preferences", serde_json::json!({"theme": "dark"})).await.unwrap();
        let val = store.get(&ns1, "preferences").await.unwrap();
        assert_eq!(val, Some(serde_json::json!({"theme": "dark"})));

        store.put(&ns2, "settings", serde_json::json!({"lang": "en"})).await.unwrap();

        // Test search
        let results_all = store.search(&["agent"]).await.unwrap();
        assert_eq!(results_all.len(), 2);

        let results_123 = store.search(&ns1).await.unwrap();
        assert_eq!(results_123.len(), 1);
        assert_eq!(results_123[0].key, "preferences");

        // Test delete
        store.delete(&ns1, "preferences").await.unwrap();
        let val_after = store.get(&ns1, "preferences").await.unwrap();
        assert_eq!(val_after, None);
    }

    #[tokio::test]
    async fn test_namespace_json_store_long_term_compat() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = NamespaceJsonStore::new(temp_dir.path());

        // Test storing in default namespace
        store.store("This is a general memory", vec![]).await.unwrap();

        // Test storing in specific namespaces (stored independently)
        store.store("The architecture uses microservices", vec!["architecture".to_string()]).await.unwrap();
        store.store("LangGraph uses state graphs", vec!["architecture".to_string(), "langgraph".to_string()]).await.unwrap();

        // Retrieve all
        let all_res = store.retrieve("", 10).await.unwrap();
        // total unique queries retrieved: "This is a general memory", "The architecture uses microservices", "LangGraph uses state graphs"
        assert_eq!(all_res.len(), 3);

        // Verify that the files were created in hierarchy using base search
        let arch_results = store.search(&["architecture"]).await.unwrap();
        assert_eq!(arch_results.len(), 2);

        let lang_results = store.search(&["langgraph"]).await.unwrap();
        assert_eq!(lang_results.len(), 1);
    }
}
