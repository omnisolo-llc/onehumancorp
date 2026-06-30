
use ohc_builtin_agent_core::types::{
    HumanInLoopSpectrum, PermissionArchitecture, ToolCall, ToolError,
};

/// HumanInLoopManager implements the SOTA Harness Patterns (2025-2026): 5. Human-in-loop as spectrum -> not binary autonomy vs control.
pub struct HumanInLoopManager;

impl HumanInLoopManager {
    /// Evaluates if a given tool call requires human intervention based on the
    /// 5-point Human-in-the-Loop spectrum and current confidence/trust thresholds.
    pub fn evaluate_escalation_tier(
        tc: &ToolCall,
        is_read_only: bool,
        is_high_risk: bool,
        hil_spectrum: &HumanInLoopSpectrum,
        confidence: f32,
        confidence_threshold: f32,
        permission_architecture: &PermissionArchitecture,
        approved_tool_calls: &[String],
        manually_approved_tool_calls: &[String],
    ) -> Result<(), ToolError> {
        let is_approved =
            approved_tool_calls.contains(&tc.id) || manually_approved_tool_calls.contains(&tc.id);

        if is_approved {
            return Ok(());
        }

        // High-risk tools always require explicit confirmation (UserFixable)
        if is_high_risk {
            return Err(ToolError::UserFixable(format!(
                "High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.",
                tc.name
            )));
        }

        match hil_spectrum {
            HumanInLoopSpectrum::Autonomous => {
                // Fully autonomous. No human intervention needed unless high-risk (handled above).
                Ok(())
            }
            HumanInLoopSpectrum::ApprovalOnMutate => {
                // Classic "Restrictive" mode. Requires approval only for mutating tools.
                if !is_read_only {
                    Err(ToolError::UserFixable(format!(
                        "Mutating tool '{}' requires human approval.",
                        tc.name
                    )))
                } else {
                    Ok(())
                }
            }
            HumanInLoopSpectrum::ApprovalOnAll => {
                // Requires explicit human approval before ANY tool executes.
                Err(ToolError::UserFixable(format!(
                    "Tool '{}' requires explicit user confirmation under 'ApprovalOnAll' mode.",
                    tc.name
                )))
            }
            HumanInLoopSpectrum::CollaborativeEdit => {
                // Expects the human to actively review and edit tool arguments.
                Err(ToolError::UserFixable(format!(
                    "Collaborative Edit required for tool '{}'. Please review and optionally edit the tool arguments to proceed.",
                    tc.name
                )))
            }
            HumanInLoopSpectrum::Supervisory => {
                // Triggers human intervention ONLY if confidence is below the threshold.
                if confidence < confidence_threshold {
                    Err(ToolError::UserFixable(format!(
                        "Low confidence ({:.2} < threshold {:.2}) for tool '{}'. Human supervision required.",
                        confidence, confidence_threshold, tc.name
                    )))
                } else {
                    // Fall back to autonomous if confidence is sufficient, but also check the base permission architecture
                    if permission_architecture == &PermissionArchitecture::Restrictive
                        && !is_read_only
                    {
                        Err(ToolError::UserFixable(format!(
                            "Mutating tool '{}' requires human approval due to restrictive base architecture.",
                            tc.name
                        )))
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tool_call(name: &str) -> ToolCall {
        ToolCall {
            id: "test_id".to_string(),
            name: name.to_string(),
            arguments: serde_json::json!({}),
        }
    }

    #[test]
    fn test_high_risk_always_requires_approval() {
        let tc = create_tool_call("delete_database");
        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            false, // is_read_only
            true,  // is_high_risk
            &HumanInLoopSpectrum::Autonomous,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );

        assert!(matches!(result, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = result {
            assert!(msg.contains("High-risk tool"));
        }
    }

    #[test]
    fn test_autonomous_mode() {
        let tc = create_tool_call("read_file");
        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            false,
            false,
            &HumanInLoopSpectrum::Autonomous,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_approval_on_mutate() {
        let tc_mutate = create_tool_call("write_file");
        let tc_read = create_tool_call("read_file");

        let result_mutate = HumanInLoopManager::evaluate_escalation_tier(
            &tc_mutate,
            false, // not read only
            false,
            &HumanInLoopSpectrum::ApprovalOnMutate,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(matches!(result_mutate, Err(ToolError::UserFixable(_))));

        let result_read = HumanInLoopManager::evaluate_escalation_tier(
            &tc_read,
            true, // read only
            false,
            &HumanInLoopSpectrum::ApprovalOnMutate,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(result_read.is_ok());
    }

    #[test]
    fn test_approval_on_all() {
        let tc = create_tool_call("read_file");
        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            true, // read only, shouldn't matter
            false,
            &HumanInLoopSpectrum::ApprovalOnAll,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(matches!(result, Err(ToolError::UserFixable(_))));
    }

    #[test]
    fn test_approval_on_mutate_read_only() {
        let tc = create_tool_call("read_file");
        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            true, // is_read_only
            false,
            &HumanInLoopSpectrum::ApprovalOnMutate,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_manually_approved_tool_calls_bypass_escalation() {
        let tc = create_tool_call("delete_database");
        let mut manually_approved_calls = Vec::new();
        manually_approved_calls.push("test_id".to_string());

        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            false,
            true,
            &HumanInLoopSpectrum::Autonomous,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &manually_approved_calls,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_supervisory_mode_autonomous_fallback() {
        let tc = create_tool_call("read_file");
        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            false, // is_read_only = false
            false, // is_high_risk = false
            &HumanInLoopSpectrum::Supervisory,
            0.9,                                 // high confidence
            0.5,                                 // low threshold
            &PermissionArchitecture::Permissive, // NOT Restrictive -> fallback to Autonomous OK
            &[],
            &[],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_collaborative_edit() {
        let tc = create_tool_call("read_file");
        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            true,
            false,
            &HumanInLoopSpectrum::CollaborativeEdit,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(matches!(result, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = result {
            assert!(msg.contains("Collaborative Edit required"));
        }
    }

    #[test]
    fn test_supervisory_mode() {
        let tc = create_tool_call("read_file");

        // High confidence -> OK
        let result_high_conf = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            true,
            false,
            &HumanInLoopSpectrum::Supervisory,
            0.9,
            0.8, // threshold
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(result_high_conf.is_ok());

        // Low confidence -> UserFixable
        let result_low_conf = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            true,
            false,
            &HumanInLoopSpectrum::Supervisory,
            0.4,
            0.8, // threshold
            &PermissionArchitecture::Permissive,
            &[],
            &[],
        );
        assert!(matches!(result_low_conf, Err(ToolError::UserFixable(_))));
        if let Err(ToolError::UserFixable(msg)) = result_low_conf {
            assert!(msg.contains("Low confidence"));
        }

        // High confidence but restrictive architecture -> UserFixable for mutating tools
        let tc_mutate = create_tool_call("write_file");
        let result_restrictive = HumanInLoopManager::evaluate_escalation_tier(
            &tc_mutate,
            false, // not read only
            false,
            &HumanInLoopSpectrum::Supervisory,
            0.9,
            0.8, // threshold
            &PermissionArchitecture::Restrictive,
            &[],
            &[],
        );
        assert!(matches!(result_restrictive, Err(ToolError::UserFixable(_))));
    }

    #[test]
    fn test_approved_tool_calls_bypass_escalation() {
        let tc = create_tool_call("delete_database"); // High risk
        let mut approved_calls = Vec::new();
        approved_calls.push("test_id".to_string());

        let result = HumanInLoopManager::evaluate_escalation_tier(
            &tc,
            false,
            true, // is_high_risk
            &HumanInLoopSpectrum::Autonomous,
            1.0,
            0.5,
            &PermissionArchitecture::Permissive,
            &approved_calls,
            &[],
        );

        assert!(result.is_ok()); // Should bypass because it's approved
    }
}
