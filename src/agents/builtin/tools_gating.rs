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
        // Handled via the 5-point HumanInLoopSpectrum
        let is_high_risk = cfg.high_risk_tools.contains(&tc.name);

        use ohc_builtin_agent_core::types::HumanInLoopSpectrum;
        let requires_approval = is_high_risk
            || cfg.hil_spectrum == HumanInLoopSpectrum::ApprovalOnAll
            || (!is_read_only && cfg.hil_spectrum == HumanInLoopSpectrum::ApprovalOnMutate)
            || cfg.hil_spectrum == HumanInLoopSpectrum::CollaborativeEdit
            || (cfg.hil_spectrum == HumanInLoopSpectrum::Supervisory && cfg.confidence_threshold < 0.5) // Fallback: requires approval if low confidence
            || (cfg.permission_architecture == crate::types::PermissionArchitecture::Restrictive && !is_read_only); // C.5 Permission Architecture

        if requires_approval {
            let is_approved = cfg.approved_tool_calls.contains(&tc.id) || cfg.manually_approved_tool_calls.contains(&tc.id);
            if !is_approved {
                if cfg.hil_spectrum == HumanInLoopSpectrum::CollaborativeEdit {
                    return Err(ToolError::UserFixable(format!("Collaborative Edit required for tool '{}'. Please review and edit the tool payload to proceed.", tc.name)));
                } else if is_high_risk {
                    return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
                } else {
                    return Err(ToolError::UserFixable(format!("Tool '{}' requires explicit user confirmation under current Human-in-the-Loop spectrum level. Approve this tool call to proceed.", tc.name)));
                }
            }
        }


        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_permission_architecture() {
        use crate::types::PermissionArchitecture;
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;

        let tc_mutating = create_tool_call("1", "mutating_tool");
        let tc_readonly = create_tool_call("2", "readonly_tool");

        // Permissive (auto-approve)
        cfg.permission_architecture = PermissionArchitecture::Permissive;
        assert!(ToolGater::check_gating(&tc_mutating, false, &cfg).is_ok());
        assert!(ToolGater::check_gating(&tc_readonly, true, &cfg).is_ok());

        // Restrictive (require approval for mutating tools)
        cfg.permission_architecture = PermissionArchitecture::Restrictive;
        assert!(ToolGater::check_gating(&tc_readonly, true, &cfg).is_ok());
        let res_mutate = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res_mutate, Err(ToolError::UserFixable(_))));

        // Restrictive + Approved
        cfg.approved_tool_calls.push("1".to_string());
        assert!(ToolGater::check_gating(&tc_mutating, false, &cfg).is_ok());
    }

    use super::*;

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
    fn test_hil_spectrum() {
        use ohc_builtin_agent_core::types::HumanInLoopSpectrum;
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;

        let tc_mutating = create_tool_call("1", "mutating_tool");
        let tc_readonly = create_tool_call("2", "readonly_tool");

        // 1. Autonomous -> Both OK
        cfg.hil_spectrum = HumanInLoopSpectrum::Autonomous;
        assert!(ToolGater::check_gating(&tc_mutating, false, &cfg).is_ok());
        assert!(ToolGater::check_gating(&tc_readonly, true, &cfg).is_ok());

        // 2. ApprovalOnMutate -> Read-only OK, Mutating UserFixable
        cfg.hil_spectrum = HumanInLoopSpectrum::ApprovalOnMutate;
        assert!(ToolGater::check_gating(&tc_readonly, true, &cfg).is_ok());
        let res_mutate = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res_mutate, Err(ToolError::UserFixable(_))));

        // 3. ApprovalOnAll -> Both UserFixable
        cfg.hil_spectrum = HumanInLoopSpectrum::ApprovalOnAll;
        let res_read_all = ToolGater::check_gating(&tc_readonly, true, &cfg);
        assert!(matches!(res_read_all, Err(ToolError::UserFixable(_))));
        let res_mutate_all = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res_mutate_all, Err(ToolError::UserFixable(_))));

        // 4. CollaborativeEdit
        cfg.hil_spectrum = HumanInLoopSpectrum::CollaborativeEdit;
        let res_collab = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res_collab, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res_collab {
            assert!(msg.contains("Collaborative Edit required"));
        }

        // 5. Supervisory -> OK if confidence >= 0.5, UserFixable if < 0.5
        cfg.hil_spectrum = HumanInLoopSpectrum::Supervisory;
        cfg.confidence_threshold = 0.8;
        assert!(ToolGater::check_gating(&tc_mutating, false, &cfg).is_ok());

        cfg.confidence_threshold = 0.2;
        let res_super = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res_super, Err(ToolError::UserFixable(_))));
    }
}
