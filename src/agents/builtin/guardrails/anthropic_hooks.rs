use ohc_builtin_agent_core::types::ToolCall;
use crate::guardrails::ToolGuardrail;
use std::collections::HashSet;

/// Anthropic Mechanic: 3-stage tool gating:
/// 1. Trust establishment at project load
/// 2. Permission check before each tool call
/// 3. Explicit user confirmation for high-risk operations

pub struct AnthropicToolGater {
    pub project_is_trusted: bool,
    pub safe_tools_for_untrusted: HashSet<String>,
    pub session_allowed_tools: HashSet<String>,
    pub high_risk_tools: HashSet<String>,
}

impl AnthropicToolGater {
    pub fn new(
        project_is_trusted: bool,
        safe_tools_for_untrusted: Vec<String>,
        session_allowed_tools: Vec<String>,
        high_risk_tools: Vec<String>,
    ) -> Self {
        Self {
            project_is_trusted,
            safe_tools_for_untrusted: safe_tools_for_untrusted.into_iter().collect(),
            session_allowed_tools: session_allowed_tools.into_iter().collect(),
            high_risk_tools: high_risk_tools.into_iter().collect(),
        }
    }
}

impl ToolGuardrail for AnthropicToolGater {
    fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
        // Stage 1: Trust establishment check
        if !self.project_is_trusted && !self.safe_tools_for_untrusted.contains(&tc.name) {
            return Err(format!(
                "Anthropic Guardrail Stage 1 (Trust) tripped: Project is not trusted. Tool '{}' is not in the safe-for-untrusted list.",
                tc.name
            ));
        }

        // Stage 2: Session permission check
        if !self.session_allowed_tools.is_empty() && !self.session_allowed_tools.contains(&tc.name) {
            return Err(format!(
                "Anthropic Guardrail Stage 2 (Permission) tripped: Tool '{}' is not allowed in this session.",
                tc.name
            ));
        }

        // Stage 3: High-risk explicit confirmation requirement
        // In a real loop, returning this specific error string might trigger a User-in-loop confirmation interrupt.
        // For the guardrail layer, we reject it if we don't have a mechanism to auto-confirm,
        // ensuring the upper harness intercepts this specific message.
        if self.high_risk_tools.contains(&tc.name) {
            return Err(format!(
                "Anthropic Guardrail Stage 3 (Confirmation) tripped: Tool '{}' is marked as high-risk and requires explicit user confirmation.",
                tc.name
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_tool_call(name: &str) -> ToolCall {
        ToolCall {
            id: "test-id".to_string(),
            name: name.to_string(),
            arguments: json!({}),
        }
    }

    #[test]
    fn test_anthropic_stage1_trust() {
        let untrusted_gater = AnthropicToolGater::new(
            false,
            vec!["read_file".to_string(), "list_files".to_string()],
            vec![],
            vec![],
        );

        let safe_tc = make_tool_call("read_file");
        assert!(untrusted_gater.check_tool(&safe_tc).is_ok());

        let unsafe_tc = make_tool_call("write_file");
        let res = untrusted_gater.check_tool(&unsafe_tc);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Stage 1 (Trust) tripped"));
    }

    #[test]
    fn test_anthropic_stage2_permission() {
        let restricted_session_gater = AnthropicToolGater::new(
            true, // trusted
            vec![],
            vec!["search_web".to_string(), "read_file".to_string()],
            vec![],
        );

        let allowed_tc = make_tool_call("search_web");
        assert!(restricted_session_gater.check_tool(&allowed_tc).is_ok());

        let unallowed_tc = make_tool_call("execute_bash");
        let res = restricted_session_gater.check_tool(&unallowed_tc);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Stage 2 (Permission) tripped"));
    }

    #[test]
    fn test_anthropic_stage3_confirmation() {
        let high_risk_gater = AnthropicToolGater::new(
            true,
            vec![],
            vec![],
            vec!["delete_database".to_string(), "execute_bash".to_string()],
        );

        let low_risk_tc = make_tool_call("read_file");
        assert!(high_risk_gater.check_tool(&low_risk_tc).is_ok());

        let high_risk_tc = make_tool_call("execute_bash");
        let res = high_risk_gater.check_tool(&high_risk_tc);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Stage 3 (Confirmation) tripped"));
    }

    #[test]
    fn test_anthropic_all_stages() {
        let gater = AnthropicToolGater::new(
            false, // untrusted
            vec!["read_file".to_string(), "list_files".to_string(), "execute_bash".to_string()],
            vec!["read_file".to_string(), "execute_bash".to_string()],
            vec!["execute_bash".to_string()],
        );

        // Fail Stage 1
        let list_tc = make_tool_call("write_file");
        assert!(gater.check_tool(&list_tc).unwrap_err().contains("Stage 1"));

        // Fail Stage 2
        let list_files_tc = make_tool_call("list_files");
        assert!(gater.check_tool(&list_files_tc).unwrap_err().contains("Stage 2"));

        // Fail Stage 3
        let bash_tc = make_tool_call("execute_bash");
        assert!(gater.check_tool(&bash_tc).unwrap_err().contains("Stage 3"));

        // Pass all
        let read_tc = make_tool_call("read_file");
        assert!(gater.check_tool(&read_tc).is_ok());
    }
}
