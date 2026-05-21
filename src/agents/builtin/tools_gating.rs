use ohc_builtin_agent_core::types::{ToolCall, ToolError};
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
        // Stage 3: Explicit user confirmation for high-risk operations
        // If a tool is in high_risk_tools, or if PermissionArchitecture is Restrictive and the tool is mutating.
        let is_high_risk = cfg.high_risk_tools.contains(&tc.name);
        let requires_approval = is_high_risk || (!is_read_only && cfg.permission_architecture == ohc_builtin_agent_core::types::PermissionArchitecture::Restrictive);

        if requires_approval {
            let is_approved = cfg.approved_tool_calls.contains(&tc.id) || cfg.manually_approved_tool_calls.contains(&tc.id);
            if !is_approved {
                if is_high_risk {
                    return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
                } else {
                    return Err(ToolError::UserFixable(format!("Mutating tool '{}' requires explicit user confirmation under Restrictive permission architecture. Approve this tool call to proceed.", tc.name)));
                }
            }
        }


        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::PermissionArchitecture;

    fn create_tool_call(id: &str, name: &str) -> ToolCall {
        ToolCall {
            id: id.to_string(),
            name: name.to_string(),
            arguments: serde_json::json!({}),
        }
    }

    #[test]
    fn test_stage_1_trust() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = false;

        let tc = create_tool_call("1", "mutating_tool");

        // Untrusted + Mutating -> Fatal Error
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::Fatal(_))));
        if let Err(ToolError::Fatal(msg)) = res {
            assert!(msg.contains("Project not trusted"));
        }

        // Untrusted + Read-only -> OK
        let res = ToolGater::check_gating(&tc, true, &cfg);
        assert!(res.is_ok());

        // Trusted + Mutating -> OK
        cfg.project_trusted = true;
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(res.is_ok());
    }

    #[test]
    fn test_stage_2_permission() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = Some(vec!["allowed_tool".to_string()]);

        let tc_allowed = create_tool_call("1", "allowed_tool");
        let tc_denied = create_tool_call("2", "denied_tool");

        assert!(ToolGater::check_gating(&tc_allowed, true, &cfg).is_ok());

        let res = ToolGater::check_gating(&tc_denied, true, &cfg);
        assert!(matches!(res, Err(ToolError::Fatal(_))));
        if let Err(ToolError::Fatal(msg)) = res {
            assert!(msg.contains("not in the allowed list"));
        }
    }

    #[test]
    fn test_stage_3_confirmation() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.high_risk_tools = vec!["nuclear_launch".to_string()];

        let tc = create_tool_call("123", "nuclear_launch");

        // High risk, not approved -> UserFixable
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));

        // High risk, approved -> OK
        cfg.approved_tool_calls.push("123".to_string());
        assert!(ToolGater::check_gating(&tc, false, &cfg).is_ok());

        // Test manually approved
        let tc2 = create_tool_call("456", "nuclear_launch");
        let res2 = ToolGater::check_gating(&tc2, false, &cfg);
        assert!(matches!(res2, Err(ToolError::UserFixable(_))));

        cfg.manually_approved_tool_calls.push("456".to_string());
        assert!(ToolGater::check_gating(&tc2, false, &cfg).is_ok());
    }

    #[test]
    fn test_permission_architecture() {
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.permission_architecture = PermissionArchitecture::Restrictive;

        let tc_mutating = create_tool_call("1", "mutating_tool");
        let tc_readonly = create_tool_call("2", "readonly_tool");

        // Restrictive + Read-only -> OK
        assert!(ToolGater::check_gating(&tc_readonly, true, &cfg).is_ok());

        // Restrictive + Mutating -> UserFixable
        let res = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Restrictive permission architecture"));
        }

        // Restrictive + Mutating (Approved) -> OK
        cfg.approved_tool_calls.push("1".to_string());
        assert!(ToolGater::check_gating(&tc_mutating, false, &cfg).is_ok());

        // Switch to Permissive -> Mutating tool OK without approval
        let mut cfg_permissive = AgentRunConfig::default();
        cfg_permissive.project_trusted = true;
        cfg_permissive.permission_architecture = PermissionArchitecture::Permissive;

        let tc_mutating_new = create_tool_call("3", "another_mutating_tool");
        assert!(ToolGater::check_gating(&tc_mutating_new, false, &cfg_permissive).is_ok());
    }
}
