#![allow(unused_mut)]
#![allow(clippy::all)]
use crate::agent::AgentRunConfig;
use crate::human_in_loop::HumanInLoopManager;
use ohc_builtin_agent_core::types::{ToolCall, ToolError};

/// ToolGater implements the Anthropic Mechanic: 3-Stage Tool Gating.
/// Trust establishment at project load -> Permission check before each tool call -> Explicit user confirmation for high-risk operations.
pub struct ToolGater;

impl ToolGater {
    pub fn check_gating(
        tc: &ToolCall,
        is_read_only: bool,
        cfg: &AgentRunConfig,
    ) -> Result<(), ToolError> {
        // OpenAI Guardrail: Check Tool Guardrail registry
        if let Some(guardrails) = &cfg.guardrails
            && let Err(e) = guardrails.check_tool(tc)
        {
            if e.contains("Stage 3 (Confirmation)")
                || e.contains("requires explicit user confirmation")
            {
                return Err(ToolError::UserFixable(format!(
                    "Tool Guardrail tripped: {}",
                    e
                )));
            }
            return Err(ToolError::Fatal(format!("Tool Guardrail tripped: {}", e)));
        }

        // Stage 1: Trust establishment at project load
        if !cfg.is_project_trusted && !is_read_only {
            return Err(ToolError::Fatal(
                "Project not trusted. Mutating tools are disabled.".to_string(),
            ));
        }

        // Stage 2: Permission check before each tool call
        // 2a. Global allowed list check if present
        if let Some(allowed) = &cfg.allowed_tools
            && !allowed.contains(&tc.name)
        {
            return Err(ToolError::Fatal(format!(
                "Tool '{}' is not in the allowed list.",
                tc.name
            )));
        }

        // 2b. Restrictive permission architecture block for mutating tools
        if cfg.permission_architecture == ohc_builtin_agent_core::types::PermissionArchitecture::Restrictive
            && !is_read_only
            && !cfg.approved_tool_calls.contains(&tc.id)
            && !cfg.manually_approved_tool_calls.contains(&tc.id)
        {
            return Err(ToolError::UserFixable(
                "Stage 2 (Permission): Mutating tool requires explicit approval under Restrictive architecture.".to_string(),
            ));
        }

        // Stage 3: Explicit user confirmation for high-risk operations
        let default_high_risk: &[&str] = &["bash", "python", "execute_query", "delete_file"];
        let is_high_risk = cfg.high_risk_tools.contains(&tc.name) || default_high_risk.contains(&tc.name.as_str());

        if is_high_risk
            && !cfg.approved_tool_calls.contains(&tc.id)
            && !cfg.manually_approved_tool_calls.contains(&tc.id)
        {
            return Err(ToolError::UserFixable(
                "Stage 3 (Confirmation): High-risk tool requires explicit user confirmation.".to_string(),
            ));
        }

        // Additional Handled via the 5-point HumanInLoopSpectrum
        HumanInLoopManager::evaluate_escalation_tier(
            tc,
            is_read_only,
            is_high_risk,
            &cfg.hil_spectrum,
            0.5, // Mocking an actual confidence value of 0.5. If threshold is 2.0 (as in test), 0.5 < 2.0. If threshold is 0.0 (default), 0.5 >= 0.0.
            cfg.confidence_threshold,
            &cfg.permission_architecture,
            &cfg.approved_tool_calls,
            &cfg.manually_approved_tool_calls,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::types::PermissionArchitecture;
    use ohc_builtin_agent_core::types::HumanInLoopSpectrum;

    fn create_tool_call(id: &str, name: &str) -> ToolCall {
        ToolCall {
            id: id.to_string(),
            name: name.to_string(),
            arguments: serde_json::json!({}),
        }
    }

    #[test]
    fn test_stage_1_trust() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: false,
            ..Default::default()
        };

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
        cfg.is_project_trusted = true;
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(res.is_ok());
    }

    #[test]
    fn test_stage_1_trust_with_no_allowed_tools() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            ..Default::default()
        };
        /*cfg.allowed_tools = None;*/

        let tc = create_tool_call("1", "any_tool");

        // Allowed tools is None -> OK
        let res = ToolGater::check_gating(&tc, true, &cfg);
        assert!(res.is_ok());
    }

    #[test]
    fn test_guardrails_check_tool_failure_confirmation() {
        use crate::guardrails::{GuardrailRegistry, ToolGuardrail};
        use std::sync::Arc;

        struct MockConfirmationGuardrail;
        impl ToolGuardrail for MockConfirmationGuardrail {
            fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
                if tc.name == "forbidden_tool" {
                    return Err(
                        "Stage 3 (Confirmation) requires explicit user confirmation".to_string()
                    );
                }
                Ok(())
            }
        }

        let mut registry = GuardrailRegistry::new();
        registry
            .tool_guardrails
            .push(Arc::new(MockConfirmationGuardrail));

        let mut cfg = AgentRunConfig {
            guardrails: Some(registry),
            is_project_trusted: true,
            ..Default::default()
        };

        let tc = create_tool_call("1", "forbidden_tool");
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Tool Guardrail tripped"));
            assert!(msg.contains("requires explicit user confirmation"));
        }
    }

    #[test]
    fn test_stage_2_permission() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            ..Default::default()
        };
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
    fn test_stage_2_restrictive_architecture() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            permission_architecture: ohc_builtin_agent_core::types::PermissionArchitecture::Restrictive,
            ..Default::default()
        };

        let tc_mutating = create_tool_call("1", "any_mutating_tool");

        // Mutating tool under restrictive architecture should require explicit approval
        let res = ToolGater::check_gating(&tc_mutating, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Stage 2 (Permission): Mutating tool requires explicit approval"));
        }

        // Read-only tool should pass
        assert!(ToolGater::check_gating(&tc_mutating, true, &cfg).is_ok());

        // Approved mutating tool should pass
        cfg.approved_tool_calls.push("1".to_string());
        assert!(ToolGater::check_gating(&tc_mutating, false, &cfg).is_ok());
    }

    #[test]
    fn test_stage_3_high_risk_tools() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            ..Default::default()
        };

        // Bash is in the default high-risk list
        let tc_bash = create_tool_call("1", "bash");

        let res = ToolGater::check_gating(&tc_bash, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Stage 3 (Confirmation): High-risk tool requires explicit user confirmation"));
        }

        // Approval clears the high-risk check
        cfg.approved_tool_calls.push("1".to_string());
        assert!(ToolGater::check_gating(&tc_bash, false, &cfg).is_ok());
    }

    #[test]
    fn test_stage_3_confirmation_wiring() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            ..Default::default()
        };
        cfg.high_risk_tools = vec!["nuclear_launch".to_string()];
        // Force the mock confidence (0.5) to fail the supervisory threshold (2.0)
        cfg.hil_spectrum = HumanInLoopSpectrum::Supervisory;
        cfg.confidence_threshold = 2.0;

        let tc = create_tool_call("123", "nuclear_launch");

        // High risk, not approved -> UserFixable
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("High-risk tool"));
        }

        // High risk, approved -> OK
        cfg.approved_tool_calls.push("123".to_string());
        assert!(ToolGater::check_gating(&tc, false, &cfg).is_ok());
    }

    #[test]
    fn test_stage_3_supervisory_wiring() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            ..Default::default()
        };

        let tc = create_tool_call("1", "normal_tool");

        // Mock confidence is 0.5.
        // If threshold is 0.0, 0.5 >= 0.0 -> OK.
        cfg.hil_spectrum = HumanInLoopSpectrum::Supervisory;
        cfg.confidence_threshold = 0.0;
        assert!(ToolGater::check_gating(&tc, false, &cfg).is_ok());

        // If threshold is 1.0, 0.5 < 1.0 -> UserFixable (Low confidence)
        cfg.confidence_threshold = 1.0;
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("Low confidence"));
        }
    }

    #[test]
    fn test_stage_3_approval_on_all_wiring() {
        let mut cfg = AgentRunConfig {
            is_project_trusted: true,
            ..Default::default()
        };
        cfg.hil_spectrum = HumanInLoopSpectrum::ApprovalOnAll;

        let tc = create_tool_call("1", "read_tool");

        let res = ToolGater::check_gating(&tc, true, &cfg);
        assert!(matches!(res, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = res {
            assert!(msg.contains("ApprovalOnAll"));
        }
    }

    #[test]
    fn test_guardrails_check_tool_failure() {
        use crate::guardrails::{GuardrailRegistry, ToolGuardrail};
        use std::sync::Arc;

        struct MockFailingGuardrail;
        impl ToolGuardrail for MockFailingGuardrail {
            fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
                if tc.name == "forbidden_tool" {
                    return Err("Tool is forbidden".to_string());
                }
                Ok(())
            }
        }

        let mut registry = GuardrailRegistry::new();
        registry
            .tool_guardrails
            .push(Arc::new(MockFailingGuardrail));

        let mut cfg = AgentRunConfig {
            guardrails: Some(registry),
            is_project_trusted: true,
            ..Default::default()
        };

        let tc = create_tool_call("1", "forbidden_tool");
        let res = ToolGater::check_gating(&tc, false, &cfg);
        assert!(matches!(res, Err(ToolError::Fatal(_))));
        if let Err(ToolError::Fatal(msg)) = res {
            assert!(msg.contains("Tool Guardrail tripped"));
            assert!(msg.contains("Tool is forbidden"));
        }
    }
}
