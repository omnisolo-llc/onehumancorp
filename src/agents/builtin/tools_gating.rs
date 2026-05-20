use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use crate::agent::AgentRunConfig;

/// ToolGater implements the Anthropic Mechanic: 3-Stage Tool Gating.
/// Trust establishment at project load -> Permission check before each tool call -> Explicit user confirmation for high-risk operations.
pub struct ToolGater;

impl ToolGater {
    pub fn check_gating(tc: &ToolCall, is_read_only: bool, cfg: &AgentRunConfig) -> Result<(), ToolError> {
        // Stage 1: Trust establishment at project load
        if !cfg.project_trusted && !is_read_only {
            return Err(ToolError::Fatal("Project not trusted. Mutating tools are disabled by default. Please verify the workspace trust level.".to_string()));
        }

        // Stage 2: Permission check before each tool call
        if let Some(allowed) = &cfg.allowed_tools {
            if !allowed.contains(&tc.name) {
                return Err(ToolError::Fatal(format!("Tool '{}' is not in the allowed list of tools for this session.", tc.name)));
            }
        }

        // Stage 3: Explicit user confirmation for high-risk operations
        if cfg.high_risk_tools.contains(&tc.name) && !cfg.approved_tool_calls.contains(&tc.id) {
            return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolCall;
    use crate::agent::AgentRunConfig;

    #[test]
    fn test_stage1_trust_establishment() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = false;

        let tc = ToolCall {
            id: "1".to_string(),
            name: "write_file".to_string(),
            arguments: serde_json::Value::Null,
        };

        let is_read_only = false; // Mutating tool

        let result = ToolGater::check_gating(&tc, is_read_only, &cfg);
        assert!(result.is_err());
        if let Err(ToolError::Fatal(msg)) = result {
            assert!(msg.contains("Project not trusted"));
        } else {
            panic!("Expected Stage 1 Fatal Error");
        }

        let is_read_only = true; // Read-only tool should pass
        let result2 = ToolGater::check_gating(&tc, is_read_only, &cfg);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_stage2_permission_check() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = Some(vec!["safe_tool".to_string()]);

        let tc = ToolCall {
            id: "1".to_string(),
            name: "unauthorized_tool".to_string(),
            arguments: serde_json::Value::Null,
        };

        let result = ToolGater::check_gating(&tc, false, &cfg);
        assert!(result.is_err());
        if let Err(ToolError::Fatal(msg)) = result {
            assert!(msg.contains("unauthorized_tool"));
            assert!(msg.contains("not in the allowed list"));
        } else {
            panic!("Expected Stage 2 Fatal Error");
        }
    }

    #[test]
    fn test_stage3_explicit_confirmation() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = Some(vec!["high_risk_tool".to_string()]);
        cfg.high_risk_tools = vec!["high_risk_tool".to_string()];

        let tc = ToolCall {
            id: "call_123".to_string(),
            name: "high_risk_tool".to_string(),
            arguments: serde_json::Value::Null,
        };

        // Call without approval
        let result = ToolGater::check_gating(&tc, false, &cfg);
        assert!(result.is_err());
        if let Err(ToolError::UserFixable(msg)) = result {
            assert!(msg.contains("requires explicit user confirmation"));
        } else {
            panic!("Expected Stage 3 UserFixable Error");
        }

        // Call with approval
        cfg.approved_tool_calls.push("call_123".to_string());
        let result2 = ToolGater::check_gating(&tc, false, &cfg);
        assert!(result2.is_ok());
    }
}
