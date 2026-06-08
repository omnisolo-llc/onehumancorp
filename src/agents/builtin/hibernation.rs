use std::path::Path;
use serde::{Deserialize, Serialize};

/// Hermes Agent Unique Harness Innovations: Serverless persistence
/// Hibernates when idle, wakes on demand (works on $5 VPS to GPU clusters).

#[derive(Debug, Serialize, Deserialize)]
pub struct HibernationState {
    pub session_id: String,
    pub messages_json: String,
    pub current_step: usize,
    pub active_tools: Vec<String>,
    // Enhancing serverless persistence to track memory size
    pub memory_size_bytes: Option<usize>,
}

pub struct HibernationManager {
    storage_dir: String,
    fallback_dirs: Vec<String>,
}

impl HibernationManager {
    pub async fn new(storage_dir: &str, fallback_dirs: Vec<String>) -> Self {
        if !Path::new(storage_dir).exists() {
            let _ = tokio::fs::create_dir_all(storage_dir).await;
        }
        for dir in &fallback_dirs {
            if !Path::new(dir).exists() {
                let _ = tokio::fs::create_dir_all(dir).await;
            }
        }
        Self {
            storage_dir: storage_dir.to_string(),
            fallback_dirs,
        }
    }

    pub async fn hibernate(&self, session_id: &str, state: &HibernationState) -> Result<(), String> {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        let data = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
        tokio::fs::write(&path, &data).await.map_err(|e| e.to_string())?;

        // Multi-region failover / "spin" strategy: write to all fallback directories for redundancy
        for dir in &self.fallback_dirs {
            let fallback_path = format!("{}/{}.json", dir, session_id);
            let _ = tokio::fs::write(&fallback_path, &data).await;
        }

        Ok(())
    }

    pub async fn wake(&self, session_id: &str) -> Result<HibernationState, String> {
        let path = format!("{}/{}.json", self.storage_dir, session_id);

        let mut primary_failed = false;
        let mut last_err = String::new();

        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            if let Ok(data) = tokio::fs::read_to_string(&path).await {
                if let Ok(state) = serde_json::from_str::<HibernationState>(&data) {
                    return Ok(state);
                } else {
                    primary_failed = true;
                    last_err = format!("Failed to parse JSON in primary directory for session {}", session_id);
                }
            } else {
                primary_failed = true;
                last_err = format!("Failed to read file in primary directory for session {}", session_id);
            }
        } else {
            primary_failed = true;
            last_err = format!("Session {} not found in primary directory", session_id);
        }

        if primary_failed {
            for dir in &self.fallback_dirs {
                let fallback_path = format!("{}/{}.json", dir, session_id);
                if tokio::fs::try_exists(&fallback_path).await.unwrap_or(false) {
                    if let Ok(data) = tokio::fs::read_to_string(&fallback_path).await {
                        if let Ok(state) = serde_json::from_str::<HibernationState>(&data) {
                            return Ok(state);
                        }
                    }
                }
            }
        }

        Err(last_err)
    }

    pub async fn is_hibernated(&self, session_id: &str) -> bool {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            return true;
        }
        for dir in &self.fallback_dirs {
            let fallback_path = format!("{}/{}.json", dir, session_id);
            if tokio::fs::try_exists(&fallback_path).await.unwrap_or(false) {
                return true;
            }
        }
        false
    }

    pub async fn clear(&self, session_id: &str) -> Result<(), String> {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            tokio::fs::remove_file(&path).await.map_err(|e| e.to_string())?;
        }
        for dir in &self.fallback_dirs {
            let fallback_path = format!("{}/{}.json", dir, session_id);
            if tokio::fs::try_exists(&fallback_path).await.unwrap_or(false) {
                let _ = tokio::fs::remove_file(&fallback_path).await;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_serverless_persistence_hibernation() {
        let dir = format!("/tmp/hibernation_test_{}", uuid::Uuid::new_v4());
        let manager = HibernationManager::new(&dir, vec![]).await;
        let session_id = "sess-123";

        let state = HibernationState {
            session_id: session_id.to_string(),
            messages_json: "[]".to_string(),
            current_step: 42,
            active_tools: vec!["Read".to_string()],
            memory_size_bytes: Some(2),
        };

        // Hibernate
        assert!(manager.hibernate(session_id, &state).await.is_ok());
        assert!(manager.is_hibernated(session_id).await);

        // Wake
        let woken = manager.wake(session_id).await.unwrap();
        assert_eq!(woken.session_id, session_id);
        assert_eq!(woken.current_step, 42);
        assert_eq!(woken.active_tools.len(), 1);
        assert_eq!(woken.memory_size_bytes, Some(2));

        // Clear
        assert!(manager.clear(session_id).await.is_ok());
        assert!(!manager.is_hibernated(session_id).await);

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_hibernation_wake_not_found() {
        let dir = format!("/tmp/hibernation_test_{}", uuid::Uuid::new_v4());
        let manager = HibernationManager::new(&dir, vec![]).await;
        let session_id = "non-existent-sess";

        let result = manager.wake(session_id).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Session non-existent-sess not found in primary directory");

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_hibernation_clear_not_found() {
        let dir = format!("/tmp/hibernation_test_{}", uuid::Uuid::new_v4());
        let manager = HibernationManager::new(&dir, vec![]).await;
        let session_id = "non-existent-sess";

        // Clearing a non-existent session should return Ok
        let result = manager.clear(session_id).await;
        assert!(result.is_ok());

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_hibernation_corrupt_data() {
        let dir = format!("/tmp/hibernation_test_{}", uuid::Uuid::new_v4());
        let manager = HibernationManager::new(&dir, vec![]).await;
        let session_id = "corrupt-sess";

        let path = format!("{}/{}.json", dir, session_id);
        tokio::fs::write(&path, "{\"invalid\": json}").await.unwrap();

        let result = manager.wake(session_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse JSON in primary directory"));

        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
