use crate::memory_store::LongTermMemory;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Memory: Long-term (OpenAI/LangGraph): Sessions backed by namespace-organized JSON Stores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonMemoryEntry {
    pub content: String,
    pub timestamp: i64,
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

    async fn get_namespace_path(&self, namespace: &str) -> Result<PathBuf, String> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir)
                .await
                .map_err(|e| format!("Failed to create base dir: {}", e))?;
        }
        // Sanitize namespace to avoid path traversal
        let safe_namespace = namespace.replace(|c: char| !c.is_alphanumeric(), "_");
        let path = self.base_dir.join(format!("{}.json", safe_namespace));
        Ok(path)
    }

    async fn read_namespace(&self, namespace: &str) -> Result<Vec<JsonMemoryEntry>, String> {
        let path = self.get_namespace_path(namespace).await?;
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        let entries: Vec<JsonMemoryEntry> =
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
        Ok(entries)
    }

    async fn write_namespace(
        &self,
        namespace: &str,
        entries: &[JsonMemoryEntry],
    ) -> Result<(), String> {
        let path = self.get_namespace_path(namespace).await?;
        let tmp_path = path.with_extension("tmp");

        let content = serde_json::to_string_pretty(entries)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        let mut file = match fs::File::create(&tmp_path).await {
            Ok(f) => f,
            Err(e) => return Err(format!("Failed to create temp file: {}", e)),
        };

        if let Err(e) = file.write_all(content.as_bytes()).await {
            drop(file);
            let _ = fs::remove_file(&tmp_path).await;
            return Err(format!("Failed to write temp file: {}", e));
        }

        if let Err(e) = file.sync_all().await {
            drop(file);
            let _ = fs::remove_file(&tmp_path).await;
            return Err(format!("Failed to sync temp file: {}", e));
        }

        if let Err(e) = fs::rename(&tmp_path, &path).await {
            let _ = fs::remove_file(&tmp_path).await;
            return Err(format!("Failed to commit file: {}", e));
        }
        Ok(())
    }
}

#[async_trait]
impl LongTermMemory for NamespaceJsonStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut all_entries = Vec::new();

        if self.base_dir.exists()
            && let Ok(mut entries) = fs::read_dir(&self.base_dir).await
        {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file()
                    && path.extension().and_then(|s| s.to_str()) == Some("json")
                    && let Some(filename) = path.file_stem().and_then(|s| s.to_str())
                    && let Ok(ns_entries) = self.read_namespace(filename).await
                {
                    all_entries.extend(ns_entries);
                }
            }
        }

        // Basic naive search: sort by recency and filter by substring
        all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        let mut seen = HashSet::new();

        for entry in all_entries {
            if !seen.contains(&entry.content)
                && (query.is_empty() || entry.content.to_lowercase().contains(&query_lower))
            {
                seen.insert(entry.content.clone());
                results.push(entry.content.clone());
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let timestamp = chrono::Utc::now().timestamp();
        let entry = JsonMemoryEntry {
            content: content.to_string(),
            timestamp,
        };

        let namespaces = if tags.is_empty() {
            vec!["default".to_string()]
        } else {
            tags
        };

        for ns in namespaces {
            let mut entries = self.read_namespace(&ns).await.unwrap_or_default();
            entries.push(entry.clone());
            self.write_namespace(&ns, &entries).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_namespace_json_store() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = NamespaceJsonStore::new(temp_dir.path());

        // Test storing in default namespace
        store
            .store("This is a general memory", vec![])
            .await
            .unwrap();

        // Test storing in specific namespaces
        store
            .store(
                "The architecture uses microservices",
                vec!["architecture".to_string()],
            )
            .await
            .unwrap();
        store
            .store(
                "LangGraph uses state graphs",
                vec!["architecture".to_string(), "langgraph".to_string()],
            )
            .await
            .unwrap();

        // Retrieve all (we changed it to retrieve from all namespaces)
        let all_res = store.retrieve("", 10).await.unwrap();
        // 3 entries total across files. Even though "LangGraph uses state graphs" is in both
        // architecture and langgraph namespaces, the deduplication in `retrieve` should ensure we only get 3 unique entries.
        assert_eq!(all_res.len(), 3);

        // Verify that the files were created
        assert!(temp_dir.path().join("default.json").exists());
        assert!(temp_dir.path().join("architecture.json").exists());
        assert!(temp_dir.path().join("langgraph.json").exists());

        // Check content of architecture namespace
        let arch_entries = store.read_namespace("architecture").await.unwrap();
        assert_eq!(arch_entries.len(), 2);
        assert!(arch_entries[0].content.contains("microservices"));
        assert!(arch_entries[1].content.contains("state graphs"));
    }
}
