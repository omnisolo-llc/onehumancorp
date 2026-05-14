use std::collections::HashMap;
use std::sync::RwLock;
use async_trait::async_trait;

use super::models::{ApprovalRequest, ApprovalStatus};

#[async_trait]
pub trait ApprovalStore: std::fmt::Debug + Send + Sync {
    async fn create_request(&self, request: ApprovalRequest) -> Result<(), String>;
    async fn get_request(&self, id: &str) -> Result<Option<ApprovalRequest>, String>;
    async fn update_status(&self, id: &str, status: ApprovalStatus, reasoning: Option<String>) -> Result<(), String>;
    async fn list_pending(&self, session_id: &str) -> Result<Vec<ApprovalRequest>, String>;
}

#[derive(Debug)]
pub struct InMemoryApprovalStore {
    requests: RwLock<HashMap<String, ApprovalRequest>>,
}

impl InMemoryApprovalStore {
    pub fn new() -> Self {
        Self {
            requests: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl ApprovalStore for InMemoryApprovalStore {
    async fn create_request(&self, request: ApprovalRequest) -> Result<(), String> {
        let mut map = self.requests.write().unwrap();
        if map.contains_key(&request.id) {
            return Err(format!("Request with ID {} already exists", request.id));
        }
        map.insert(request.id.clone(), request);
        Ok(())
    }

    async fn get_request(&self, id: &str) -> Result<Option<ApprovalRequest>, String> {
        let map = self.requests.read().unwrap();
        Ok(map.get(id).cloned())
    }

    async fn update_status(&self, id: &str, status: ApprovalStatus, reasoning: Option<String>) -> Result<(), String> {
        let mut map = self.requests.write().unwrap();
        if let Some(req) = map.get_mut(id) {
            req.status = status;
            req.reasoning = reasoning;
            req.updated_at = chrono::Utc::now().timestamp();
            Ok(())
        } else {
            Err(format!("Request with ID {} not found", id))
        }
    }

    async fn list_pending(&self, session_id: &str) -> Result<Vec<ApprovalRequest>, String> {
        let map = self.requests.read().unwrap();
        let pending = map.values()
            .filter(|r| r.session_id == session_id && r.status == ApprovalStatus::Pending)
            .cloned()
            .collect();
        Ok(pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_store_create_get() {
        let store = InMemoryApprovalStore::new();
        let req = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        assert!(store.create_request(req.clone()).await.is_ok());

        let retrieved = store.get_request("req-1").await.unwrap().unwrap();
        assert_eq!(retrieved.id, "req-1");
        assert_eq!(retrieved.status, ApprovalStatus::Pending);
    }

    #[tokio::test]
    async fn test_in_memory_store_duplicate_create() {
        let store = InMemoryApprovalStore::new();
        let req = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        assert!(store.create_request(req.clone()).await.is_ok());
        let err = store.create_request(req).await.unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[tokio::test]
    async fn test_in_memory_store_update_status() {
        let store = InMemoryApprovalStore::new();
        let req = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        store.create_request(req).await.unwrap();

        assert!(store.update_status("req-1", ApprovalStatus::Approved, Some("Looks good".to_string())).await.is_ok());

        let retrieved = store.get_request("req-1").await.unwrap().unwrap();
        assert_eq!(retrieved.status, ApprovalStatus::Approved);
        assert_eq!(retrieved.reasoning, Some("Looks good".to_string()));
    }

    #[tokio::test]
    async fn test_in_memory_store_list_pending() {
        let store = InMemoryApprovalStore::new();
        let req1 = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };
        let req2 = ApprovalRequest {
            id: "req-2".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool2".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Approved,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        store.create_request(req1).await.unwrap();
        store.create_request(req2).await.unwrap();

        let pending = store.list_pending("sess-1").await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "req-1");
    }
}
