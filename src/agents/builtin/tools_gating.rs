use ohc_builtin_agent_core::types::{ToolCall, ToolError, PermissionArchitecture};
use crate::agent::AgentRunConfig;

/// ToolGater implements the Anthropic Mechanic: 3-Stage Tool Gating.
/// Trust establishment at project load -> Permission check before each tool call -> Explicit user confirmation for high-risk operations.
pub struct ToolGater;

impl ToolGater {
    pub fn check_gating(tc: &ToolCall, is_read_only: bool, cfg: &AgentRunConfig) -> Result<(), ToolError> {
        // Stage 1: Trust establishment at project load
        if !cfg.project_trusted && !is_read_only {
            return Err(ToolError::Fatal("Project not trusted. Mutating tools are disabled.".to_string()));
        }

        // Stage 2: Permission check before each tool call
        if let Some(allowed) = &cfg.allowed_tools {
            if !allowed.contains(&tc.name) {
                return Err(ToolError::Fatal(format!("Tool '{}' is not in the allowed list.", tc.name)));
            }
        }

        // Stage 3: Explicit user confirmation for high-risk operations or when Restrictive
        let is_high_risk = cfg.high_risk_tools.contains(&tc.name);
        let requires_approval = is_high_risk || (!is_read_only && cfg.permission_architecture == PermissionArchitecture::Restrictive);

        if requires_approval {
            // Check both approved lists: approved_tool_calls (session-scoped) and manually_approved_tool_calls (persisted)
            let is_approved = cfg.approved_tool_calls.contains(&tc.id) || cfg.manually_approved_tool_calls.contains(&tc.id);
            if !is_approved {
                if is_high_risk {
                    return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
                } else {
                    return Err(ToolError::UserFixable(format!("Mutating tool '{}' requires explicit user confirmation in Restrictive mode. Approve this tool call to proceed.", tc.name)));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_1_project_trusted() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = false;

        let tc = ToolCall {
            id: "call_1".to_string(),
            name: "mutating_tool".to_string(),
            arguments: serde_json::json!({}),
        };

        // If not trusted and not read-only, it should fail Stage 1
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::Fatal(msg)) if msg.contains("Project not trusted")));

        // If not trusted but read-only, it should pass Stage 1
        cfg.allowed_tools = Some(vec!["mutating_tool".to_string()]);
        let res_read = ToolGater::check_gating(&tc, true, &cfg);
        assert!(res_read.is_ok());
    }

    #[test]
    fn test_stage_2_allowed_tools() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = Some(vec!["safe_tool".to_string()]);

        let tc = ToolCall {
            id: "call_2".to_string(),
            name: "unsafe_tool".to_string(),
            arguments: serde_json::json!({}),
        };

        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::Fatal(msg)) if msg.contains("is not in the allowed list")));
    }

    #[test]
    fn test_stage_3_high_risk_tools() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = None;
        cfg.high_risk_tools = vec!["delete_db".to_string()];

        let tc = ToolCall {
            id: "call_3".to_string(),
            name: "delete_db".to_string(),
            arguments: serde_json::json!({}),
        };

        // Not in approved list, should require explicit user confirmation
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(msg)) if msg.contains("requires explicit user confirmation")));

        // Add to approved list, should pass
        cfg.approved_tool_calls.push("call_3".to_string());
        let res_approved = ToolGater::check_gating(&tc, false, &cfg);
        assert!(res_approved.is_ok());
    }

    #[test]
    fn test_stage_3_restrictive_mode() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = None;
        cfg.permission_architecture = PermissionArchitecture::Restrictive;

        let tc = ToolCall {
            id: "call_4".to_string(),
            name: "write_file".to_string(),
            arguments: serde_json::json!({}),
        };

        // Mutating tool in Restrictive mode requires approval
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(msg)) if msg.contains("Requires explicit user confirmation in Restrictive mode") || msg.contains("requires explicit user confirmation in Restrictive mode")));

        // Read-only tool in Restrictive mode does NOT require approval
        let res_ro = ToolGater::check_gating(&tc, true, &cfg);
        assert!(res_ro.is_ok());

        // Approve it, then it should pass
        cfg.manually_approved_tool_calls.push("call_4".to_string());
        let res_approved = ToolGater::check_gating(&tc, false, &cfg);
        assert!(res_approved.is_ok());
    }
}
