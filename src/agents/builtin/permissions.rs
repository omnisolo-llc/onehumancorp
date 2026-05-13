use crate::types::{ToolCall, PermissionMode, ToolError};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for Permission Manager
#[derive(Debug, Clone, Default)]
pub struct PermissionConfig {
    /// List of tool names that are explicitly auto-approved even in Restrictive mode.
    pub auto_approved_tools: Vec<String>,
    /// List of domains (e.g. "github.com", "api.stripe.com") that are auto-approved.
    pub auto_approved_domains: Vec<String>,
    /// Time-to-live (in seconds) for temporary approvals.
    pub temporary_approval_ttl_seconds: Option<u64>,
}

/// A robust Permission Manager to enforce Permissive vs Restrictive architecture.
pub struct PermissionManager {
    mode: PermissionMode,
    config: PermissionConfig,
    /// Ledger of approved tool calls (using ToolCall ID or a hash)
    approved_ledger: Arc<RwLock<HashSet<String>>>,
    /// Timestamps for temporary approvals mapping ID -> Expiration Time
    temporary_approvals: Arc<RwLock<HashMap<String, tokio::time::Instant>>>,
}

impl PermissionManager {
    pub fn new(mode: PermissionMode, config: PermissionConfig) -> Self {
        Self {
            mode,
            config,
            approved_ledger: Arc::new(RwLock::new(HashSet::new())),
            temporary_approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Checks if a tool call is permitted to execute.
    /// Returns Ok(()) if permitted.
    /// Returns Err(String) with a user-fixable message if approval is required.
    pub async fn check_permission(&self, tool_name: &str, is_read_only: bool, call: &ToolCall) -> Result<(), String> {
        // Mode 1: Permissive (Auto-approve all)
        if self.mode == PermissionMode::Permissive {
            return Ok(());
        }

        // Mode 2: Restrictive
        // Rule: Read-only tools are automatically approved.
        if is_read_only {
            return Ok(());
        }

        // Check if explicitly auto-approved by name
        if self.config.auto_approved_tools.contains(&tool_name.to_string()) {
            return Ok(());
        }

        // Check domain auto-approvals for web-related tools
        let args_str = call.arguments.to_string();
        for domain in &self.config.auto_approved_domains {
            if args_str.contains(domain) {
                return Ok(());
            }
        }

        self.cleanup_expired_approvals().await;

        let ledger = self.approved_ledger.read().await;
        if ledger.contains(&call.id) {
            return Ok(());
        }

        // If we reach here, it's a mutating tool that is NOT approved.
        // Return a user-fixable error that will bubble up as an AgentEvent::UserInterventionRequired
        Err(format!(
            "Tool execution for '{}' requires explicit user approval in Restrictive Mode. Please review the arguments: {}. Type 'approve {}' to allow this execution.",
            tool_name, args_str, call.id
        ))
    }

    /// Manually approve a tool call ID.
    pub async fn approve(&self, call_id: &str) {
        let mut ledger = self.approved_ledger.write().await;
        ledger.insert(call_id.to_string());

        if let Some(ttl) = self.config.temporary_approval_ttl_seconds {
            let expiration = tokio::time::Instant::now() + std::time::Duration::from_secs(ttl);
            let mut temp = self.temporary_approvals.write().await;
            temp.insert(call_id.to_string(), expiration);
        }
    }

    /// Reject a tool call ID explicitly.
    pub async fn reject(&self, call_id: &str) {
        let mut ledger = self.approved_ledger.write().await;
        ledger.remove(call_id);

        let mut temp = self.temporary_approvals.write().await;
        temp.remove(call_id);
    }

    /// Check if an ID is already approved
    pub async fn is_approved(&self, call_id: &str) -> bool {
        self.cleanup_expired_approvals().await;
        let ledger = self.approved_ledger.read().await;
        ledger.contains(call_id)
    }

    /// Clear all approvals
    pub async fn clear(&self) {
        let mut ledger = self.approved_ledger.write().await;
        ledger.clear();
        let mut temp = self.temporary_approvals.write().await;
        temp.clear();
    }

    /// Private internal task: Clean up expired temporary approvals
    async fn cleanup_expired_approvals(&self) {
        if self.config.temporary_approval_ttl_seconds.is_none() {
            return;
        }

        let mut temp = self.temporary_approvals.write().await;
        let mut ledger = self.approved_ledger.write().await;
        let now = tokio::time::Instant::now();

        let expired_keys: Vec<String> = temp
            .iter()
            .filter(|(k, v)| now > **v)
            .map(|(k, _)| k.clone())
            .collect();

        for key in expired_keys {
            temp.remove(&key);
            ledger.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_permissive_mode_allows_all() {
        let manager = PermissionManager::new(PermissionMode::Permissive, PermissionConfig::default());

        let call = ToolCall {
            id: "call_1".to_string(),
            name: "mutating_tool".to_string(),
            arguments: json!({}),
        };

        let result = manager.check_permission("mutating_tool", false, &call).await;
        assert!(result.is_ok(), "Permissive mode should allow mutating tools without approval.");
    }

    #[tokio::test]
    async fn test_restrictive_mode_allows_read_only() {
        let manager = PermissionManager::new(PermissionMode::Restrictive, PermissionConfig::default());

        let call = ToolCall {
            id: "call_2".to_string(),
            name: "read_only_tool".to_string(),
            arguments: json!({}),
        };

        let result = manager.check_permission("read_only_tool", true, &call).await;
        assert!(result.is_ok(), "Restrictive mode should auto-allow read-only tools.");
    }

    #[tokio::test]
    async fn test_restrictive_mode_blocks_mutating() {
        let manager = PermissionManager::new(PermissionMode::Restrictive, PermissionConfig::default());

        let call = ToolCall {
            id: "call_3".to_string(),
            name: "mutating_tool".to_string(),
            arguments: json!({"action": "delete"}),
        };

        let result = manager.check_permission("mutating_tool", false, &call).await;
        assert!(result.is_err(), "Restrictive mode should block unapproved mutating tools.");
        assert!(result.unwrap_err().contains("requires explicit user approval"));
    }

    #[tokio::test]
    async fn test_restrictive_mode_allows_approved() {
        let manager = PermissionManager::new(PermissionMode::Restrictive, PermissionConfig::default());

        let call = ToolCall {
            id: "call_4".to_string(),
            name: "mutating_tool".to_string(),
            arguments: json!({"action": "update"}),
        };

        // First check blocks
        assert!(manager.check_permission("mutating_tool", false, &call).await.is_err());

        // Approve it
        manager.approve("call_4").await;

        // Second check allows
        assert!(manager.check_permission("mutating_tool", false, &call).await.is_ok());
    }

    #[tokio::test]
    async fn test_auto_approved_tools() {
        let config = PermissionConfig {
            auto_approved_tools: vec!["safe_mutating".to_string()],
            ..Default::default()
        };
        let manager = PermissionManager::new(PermissionMode::Restrictive, config);

        let call = ToolCall {
            id: "call_5".to_string(),
            name: "safe_mutating".to_string(),
            arguments: json!({}),
        };

        assert!(manager.check_permission("safe_mutating", false, &call).await.is_ok());
    }

    #[tokio::test]
    async fn test_auto_approved_domains() {
        let config = PermissionConfig {
            auto_approved_domains: vec!["github.com".to_string()],
            ..Default::default()
        };
        let manager = PermissionManager::new(PermissionMode::Restrictive, config);

        let call = ToolCall {
            id: "call_6".to_string(),
            name: "web_fetch".to_string(),
            arguments: json!({"url": "https://api.github.com/repos"}),
        };

        assert!(manager.check_permission("web_fetch", false, &call).await.is_ok());
    }

    #[tokio::test]
    async fn test_temporary_ttl_expiration() {
        let config = PermissionConfig {
            temporary_approval_ttl_seconds: Some(0), // Instantly expires
            ..Default::default()
        };
        let manager = PermissionManager::new(PermissionMode::Restrictive, config);

        let call = ToolCall {
            id: "call_ttl".to_string(),
            name: "fast_tool".to_string(),
            arguments: json!({}),
        };

        manager.approve("call_ttl").await;
        // Should expire immediately because ttl is 0.
        // Sleep briefly just to be sure Instant::now() advances
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert!(manager.check_permission("fast_tool", false, &call).await.is_err());
    }
}
