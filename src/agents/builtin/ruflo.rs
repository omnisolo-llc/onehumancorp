use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Ruflo Unique Harness Innovations: SONA neural patterns: Self-learning trajectory patterns.
/// Stores sequences of tools used for tasks. Next time it faces a similar task,
/// it loads the SONA pattern and injects it into the prompt as a recommended execution plan,
/// effectively learning to skip dead ends.
#[derive(Clone, Default)]
pub struct SonaMemory {
    patterns: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl std::fmt::Debug for SonaMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SonaMemory").finish()
    }
}

impl SonaMemory {
    pub fn new() -> Self {
        Self {
            patterns: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn record_trajectory(&self, task: &str, trajectory: Vec<String>) -> Result<(), String> {
        let mut map = self.patterns.write().await;
        // Basic storing, overwrites existing
        map.insert(task.to_string(), trajectory);
        Ok(())
    }

    pub async fn retrieve_pattern(&self, task: &str) -> Result<Option<Vec<String>>, String> {
        let map = self.patterns.read().await;

        // Simple matching algorithm: iterate over keys and check if the given task string
        // contains the key or vice-versa.
        // We prioritize exact matches first.
        if let Some(pattern) = map.get(task) {
            return Ok(Some(pattern.clone()));
        }

        // Fallback to substring matching
        for (k, v) in map.iter() {
            if task.contains(k) || k.contains(task) {
                return Ok(Some(v.clone()));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sona_memory() {
        let memory = SonaMemory::new();

        let trajectory1 = vec!["ToolSearch".to_string(), "ReadFile".to_string(), "Verify".to_string()];
        memory.record_trajectory("Find the bug and fix it", trajectory1.clone()).await.unwrap();

        // Exact match
        let retrieved = memory.retrieve_pattern("Find the bug and fix it").await.unwrap();
        assert_eq!(retrieved, Some(trajectory1.clone()));

        // Substring match (task contains key)
        let retrieved2 = memory.retrieve_pattern("Hey agent, Find the bug and fix it now").await.unwrap();
        assert_eq!(retrieved2, Some(trajectory1.clone()));

        // Substring match (key contains task)
        let retrieved3 = memory.retrieve_pattern("Find the bug").await.unwrap();
        assert_eq!(retrieved3, Some(trajectory1.clone()));

        // No match
        let retrieved4 = memory.retrieve_pattern("Do something completely different").await.unwrap();
        assert_eq!(retrieved4, None);
    }
}