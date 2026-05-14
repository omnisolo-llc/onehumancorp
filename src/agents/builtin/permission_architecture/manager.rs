use std::sync::Arc;
use uuid::Uuid;

use super::models::{ApprovalRequest, ApprovalStatus, AuditLogEntry, PermissionMode};
use super::store::ApprovalStore;
use super::audit::AuditLogger;
use super::rules::RuleEngine;

#[derive(Debug)]
pub enum PermissionResult {
    Approved,
    RequiresApproval(String), // Returns the request ID
    Denied(String),           // Returns the reason
}

#[derive(Debug, Clone)]
pub struct PermissionManager {
    mode: PermissionMode,
    store: Arc<dyn ApprovalStore>,
    audit: Arc<dyn AuditLogger>,
    rule_engine: RuleEngine,
}

impl PermissionManager {
    pub fn new(
        mode: PermissionMode,
        store: Arc<dyn ApprovalStore>,
        audit: Arc<dyn AuditLogger>,
        rule_engine: RuleEngine,
    ) -> Self {
        Self {
            mode,
            store,
            audit,
            rule_engine,
        }
    }

    pub async fn check_permission(
        &self,
        session_id: &str,
        tool_name: &str,
        arguments: &serde_json::Value,
    ) -> Result<PermissionResult, String> {
        // 1. Check auto-approve rules
        if self.rule_engine.check_auto_approve(tool_name, arguments) {
            let _ = self.audit.log(AuditLogEntry {
                id: Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                tool_name: tool_name.to_string(),
                action: "auto_approved".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                mode: self.mode.clone(),
                details: "Matched auto-approve rule".to_string(),
            }).await;
            return Ok(PermissionResult::Approved);
        }

        // 2. If Permissive Mode, auto-approve everything else
        if self.mode == PermissionMode::Permissive {
            let _ = self.audit.log(AuditLogEntry {
                id: Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                tool_name: tool_name.to_string(),
                action: "auto_approved".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                mode: self.mode.clone(),
                details: "Permissive mode auto-approval".to_string(),
            }).await;
            return Ok(PermissionResult::Approved);
        }

        // 3. Restrictive Mode -> Require Approval
        // Create an approval request
        let req_id = Uuid::new_v4().to_string();
        let request = ApprovalRequest {
            id: req_id.clone(),
            session_id: session_id.to_string(),
            tool_name: tool_name.to_string(),
            arguments: arguments.clone(),
            status: ApprovalStatus::Pending,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            reasoning: None,
        };

        self.store.create_request(request).await?;

        let _ = self.audit.log(AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            tool_name: tool_name.to_string(),
            action: "approval_requested".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            mode: self.mode.clone(),
            details: format!("Approval request created with ID {}", req_id),
        }).await;

        Ok(PermissionResult::RequiresApproval(req_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permission_architecture::store::InMemoryApprovalStore;
    use crate::permission_architecture::audit::InMemoryAuditLogger;
    use crate::permission_architecture::rules::Rule;

    #[tokio::test]
    async fn test_manager_permissive_mode() {
        let store = Arc::new(InMemoryApprovalStore::new());
        let audit = Arc::new(InMemoryAuditLogger::new());
        let rules = RuleEngine::new(vec![]);
        let manager = PermissionManager::new(PermissionMode::Permissive, store.clone(), audit.clone(), rules);

        let result = manager.check_permission("sess-1", "test_tool", &serde_json::json!({})).await.unwrap();
        assert!(matches!(result, PermissionResult::Approved));

        let logs = audit.get_logs("sess-1").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].action, "auto_approved");
        assert_eq!(logs[0].details, "Permissive mode auto-approval");
    }

    #[tokio::test]
    async fn test_manager_restrictive_mode_requires_approval() {
        let store = Arc::new(InMemoryApprovalStore::new());
        let audit = Arc::new(InMemoryAuditLogger::new());
        let rules = RuleEngine::new(vec![]);
        let manager = PermissionManager::new(PermissionMode::Restrictive, store.clone(), audit.clone(), rules);

        let result = manager.check_permission("sess-1", "test_tool", &serde_json::json!({})).await.unwrap();

        let req_id = match result {
            PermissionResult::RequiresApproval(id) => id,
            _ => panic!("Expected RequiresApproval"),
        };

        let req = store.get_request(&req_id).await.unwrap().unwrap();
        assert_eq!(req.status, ApprovalStatus::Pending);

        let logs = audit.get_logs("sess-1").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].action, "approval_requested");
    }

    #[tokio::test]
    async fn test_manager_rule_engine_auto_approve_in_restrictive() {
        let store = Arc::new(InMemoryApprovalStore::new());
        let audit = Arc::new(InMemoryAuditLogger::new());
        let rules = RuleEngine::new(vec![Rule {
            tool_name: "safe_tool".to_string(),
            allowed_path_prefix: None,
        }]);
        let manager = PermissionManager::new(PermissionMode::Restrictive, store.clone(), audit.clone(), rules);

        // This one should be auto-approved by the rule engine despite Restrictive mode
        let result1 = manager.check_permission("sess-1", "safe_tool", &serde_json::json!({})).await.unwrap();
        assert!(matches!(result1, PermissionResult::Approved));

        // This one should require approval
        let result2 = manager.check_permission("sess-1", "risky_tool", &serde_json::json!({})).await.unwrap();
        assert!(matches!(result2, PermissionResult::RequiresApproval(_)));
    }
}
