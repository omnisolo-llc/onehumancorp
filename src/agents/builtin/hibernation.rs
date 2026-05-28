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
}

pub struct HibernationManager {
    storage_dir: String,
}

impl HibernationManager {
    pub async fn new(storage_dir: &str) -> Self {
        if !Path::new(storage_dir).exists() {
            let _ = tokio::fs::create_dir_all(storage_dir).await;
        }
        Self {
            storage_dir: storage_dir.to_string(),
        }
    }

    pub async fn hibernate(&self, session_id: &str, state: &HibernationState) -> Result<(), String> {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        let data = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
        tokio::fs::write(path, data).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn wake(&self, session_id: &str) -> Result<HibernationState, String> {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        let data = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
        let state: HibernationState = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        Ok(state)
    }

    pub async fn is_hibernated(&self, session_id: &str) -> bool {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        tokio::fs::try_exists(&path).await.unwrap_or(false)
    }

    pub async fn clear(&self, session_id: &str) -> Result<(), String> {
        let path = format!("{}/{}.json", self.storage_dir, session_id);
        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            tokio::fs::remove_file(path).await.map_err(|e| e.to_string())?;
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
        let manager = HibernationManager::new(&dir).await;
        let session_id = "sess-123";

        let state = HibernationState {
            session_id: session_id.to_string(),
            messages_json: "[]".to_string(),
            current_step: 42,
            active_tools: vec!["Read".to_string()],
        };

        // Hibernate
        assert!(manager.hibernate(session_id, &state).await.is_ok());
        assert!(manager.is_hibernated(session_id).await);

        // Wake
        let woken = manager.wake(session_id).await.unwrap();
        assert_eq!(woken.session_id, session_id);
        assert_eq!(woken.current_step, 42);
        assert_eq!(woken.active_tools.len(), 1);

        // Clear
        assert!(manager.clear(session_id).await.is_ok());
        assert!(!manager.is_hibernated(session_id).await);

        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
