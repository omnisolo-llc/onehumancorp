use crate::json_store::JsonMemoryEntry;
use crate::memory_store::LongTermMemory;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock; // Re-use the struct for consistency

/// Memory: Short-term in-memory namespace store
#[derive(Debug, Default)]
pub struct InMemoryNamespaceStore {
    // Maps namespace to a list of entries
    namespaces: RwLock<HashMap<String, Vec<JsonMemoryEntry>>>,
}

impl InMemoryNamespaceStore {
    pub fn new() -> Self {
        Self {
            namespaces: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl LongTermMemory for InMemoryNamespaceStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut all_entries = Vec::new();

        if let Ok(namespaces) = self.namespaces.read() {
            for entries in namespaces.values() {
                all_entries.extend(entries.clone());
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

        let namespaces_to_store = if tags.is_empty() {
            vec!["default".to_string()]
        } else {
            tags
        };

        if let Ok(mut namespaces) = self.namespaces.write() {
            for ns in namespaces_to_store {
                namespaces.entry(ns).or_default().push(entry.clone());
            }
        } else {
            return Err("Failed to acquire write lock".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_namespace_store() {
        let store = InMemoryNamespaceStore::new();

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

        // Retrieve all
        let all_res = store.retrieve("", 10).await.unwrap();
        // 3 entries total. Deduplication in `retrieve` should ensure we only get 3 unique entries.
        assert_eq!(all_res.len(), 3);

        // Test query filtering
        let filter_res = store.retrieve("microservices", 10).await.unwrap();
        assert_eq!(filter_res.len(), 1);
        assert!(filter_res[0].contains("microservices"));
    }
}
