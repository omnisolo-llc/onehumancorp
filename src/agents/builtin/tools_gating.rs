use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use crate::agent::AgentRunConfig;

/// ToolGater implements the Anthropic Mechanic: 3-Stage Tool Gating.
/// Trust establishment at project load -> Permission check before each tool call -> Explicit user confirmation for high-risk operations.
pub struct ToolGater;

impl ToolGater {
    /// check_gating implements the Anthropic 3-Stage Tool Gating.
    /// Stage 1: Trust establishment at project load (disables mutating tools if untrusted).
    /// Stage 2: Permission check before each tool call (enforces allowed_tools list).
    /// Stage 3: Explicit user confirmation for high-risk operations (requires tool_call_id in approved list).
    pub fn check_gating(tc: &ToolCall, is_read_only: bool, cfg: &AgentRunConfig) -> Result<(), ToolError> {
        // Stage 1: Trust establishment at project load
        // If the project is not trusted, we only allow read-only tools.
        if !cfg.project_trusted && !is_read_only {
            return Err(ToolError::Fatal(format!(
                "Security Violation (Stage 1): Project not trusted. Mutating tool '{}' is blocked. Trust the project at load to enable mutating tools.",
                tc.name
            )));
        }

        // Stage 2: Permission check before each tool call
        // If an explicit allowed list is provided, the tool MUST be in it.
        if let Some(allowed) = &cfg.allowed_tools {
            if !allowed.contains(&tc.name) {
                return Err(ToolError::Fatal(format!(
                    "Security Violation (Stage 2): Tool '{}' is not in the allowed list for this session.",
                    tc.name
                )));
            }
        }

        // Stage 3: Explicit user confirmation for high-risk operations
        // Even if trusted and allowed, some tools are marked as high-risk and need per-invocation approval.
        if cfg.high_risk_tools.contains(&tc.name) {
            if !cfg.approved_tool_calls.contains(&tc.id) {
                return Err(ToolError::UserFixable(format!(
                    "Security Confirmation Required (Stage 3): High-risk tool '{}' requires explicit user confirmation. Please approve tool call ID '{}' to proceed.",
                    tc.name, tc.id
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolCall;
    use crate::agent::AgentRunConfig;
    use serde_json::json;

    #[test]
    fn test_anthropic_3_stage_gating_logic() {
        let mut cfg = AgentRunConfig::default();
        let tc_read = ToolCall { id: "c1".to_string(), name: "read_file".to_string(), arguments: json!({}) };
        let tc_write = ToolCall { id: "c2".to_string(), name: "write_file".to_string(), arguments: json!({}) };
        let tc_high_risk = ToolCall { id: "c3".to_string(), name: "delete_db".to_string(), arguments: json!({}) };

        // Stage 1: Trust
        cfg.project_trusted = false;
        assert!(ToolGater::check_gating(&tc_read, true, &cfg).is_ok(), "Read-only should be allowed in untrusted project");
        let res = ToolGater::check_gating(&tc_write, false, &cfg);
        assert!(res.is_err(), "Mutating should be blocked in untrusted project");
        assert!(res.unwrap_err().to_string().contains("Security Violation (Stage 1)"));

        cfg.project_trusted = true;

        // Stage 2: Permissions
        cfg.allowed_tools = Some(vec!["read_file".to_string(), "write_file".to_string()]);
        assert!(ToolGater::check_gating(&tc_read, true, &cfg).is_ok());
        assert!(ToolGater::check_gating(&tc_write, false, &cfg).is_ok());
        let res2 = ToolGater::check_gating(&tc_high_risk, false, &cfg);
        assert!(res2.is_err(), "Unallowed tool should be blocked");
        assert!(res2.unwrap_err().to_string().contains("Security Violation (Stage 2)"));

        cfg.allowed_tools = None;

        // Stage 3: High-risk confirmation
        cfg.high_risk_tools = vec!["delete_db".to_string()];
        let res3 = ToolGater::check_gating(&tc_high_risk, false, &cfg);
        assert!(res3.is_err(), "High-risk tool should require confirmation");
        assert!(res3.unwrap_err().to_string().contains("Security Confirmation Required (Stage 3)"));

        cfg.approved_tool_calls = vec!["c3".to_string()];
        assert!(ToolGater::check_gating(&tc_high_risk, false, &cfg).is_ok(), "Approved high-risk tool should be allowed");
    }
}
