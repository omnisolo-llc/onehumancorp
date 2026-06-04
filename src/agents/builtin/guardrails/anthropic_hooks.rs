use ohc_builtin_agent_core::types::ToolCall;
use crate::guardrails::{InputGuardrail, ToolGuardrail};
use std::collections::HashSet;
use std::sync::{Mutex};

/// Anthropic Mechanic: 3-stage tool gating:
/// 1. Trust establishment at project load
/// 2. Permission check before each tool call
/// 3. Explicit user confirmation for high-risk operations

pub struct AnthropicTrustEstablishment {
    pub is_trusted_project: bool,
    pub untrusted_warning_issued: Mutex<bool>,
}

impl AnthropicTrustEstablishment {
    pub fn new(is_trusted_project: bool) -> Self {
        Self {
            is_trusted_project,
            untrusted_warning_issued: Mutex::new(false),
        }
    }
}

impl InputGuardrail for AnthropicTrustEstablishment {
    fn check_input(&self, _input: &str) -> Result<(), String> {
        if !self.is_trusted_project {
            let mut issued = self.untrusted_warning_issued.lock().unwrap();
            if !*issued {
                *issued = true;
                return Err("Anthropic Guardrail: Project not trusted. Please establish trust before proceeding.".to_string());
            }
        }
        Ok(())
    }
}

pub struct AnthropicPermissionCheck {
    pub permitted_tools: HashSet<String>,
}

impl AnthropicPermissionCheck {
    pub fn new(permitted_tools: Vec<String>) -> Self {
        let mut set = HashSet::new();
        for t in permitted_tools {
            set.insert(t);
        }
        Self {
            permitted_tools: set,
        }
    }
}

impl ToolGuardrail for AnthropicPermissionCheck {
    fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
        if !self.permitted_tools.contains(&tc.name) {
            return Err(format!("Anthropic Permission Guardrail: Tool '{}' requires explicit permission before each call.", tc.name));
        }
        Ok(())
    }
}

pub struct AnthropicExplicitConfirmation {
    pub high_risk_tools: HashSet<String>,
    pub high_risk_patterns: Vec<String>,
}

impl AnthropicExplicitConfirmation {
    pub fn new(high_risk_tools: Vec<String>, high_risk_patterns: Vec<String>) -> Self {
        let mut set = HashSet::new();
        for t in high_risk_tools {
            set.insert(t);
        }
        Self {
            high_risk_tools: set,
            high_risk_patterns,
        }
    }
}

impl ToolGuardrail for AnthropicExplicitConfirmation {
    fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
        if self.high_risk_tools.contains(&tc.name) {
            return Err(format!("Anthropic Explicit Confirmation: High-risk tool '{}' requires user confirmation.", tc.name));
        }

        let args_str = tc.arguments.to_string();
        for pattern in &self.high_risk_patterns {
            if args_str.contains(pattern) {
                return Err(format!("Anthropic Explicit Confirmation: High-risk pattern '{}' detected in tool '{}'. Requires user confirmation.", pattern, tc.name));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_anthropic_trust_establishment() {
        let trusted = AnthropicTrustEstablishment::new(true);
        assert!(trusted.check_input("any input").is_ok());

        let untrusted = AnthropicTrustEstablishment::new(false);
        let res1 = untrusted.check_input("any input");
        assert!(res1.is_err());
        assert_eq!(res1.unwrap_err(), "Anthropic Guardrail: Project not trusted. Please establish trust before proceeding.");

        assert!(*untrusted.untrusted_warning_issued.lock().unwrap());
        assert!(untrusted.check_input("second input").is_ok());
    }

    #[test]
    fn test_anthropic_permission_check() {
        let permission_check = AnthropicPermissionCheck::new(vec!["safe_tool".to_string(), "read_tool".to_string()]);

        let valid_tc = ToolCall {
            id: "1".to_string(),
            name: "safe_tool".to_string(),
            arguments: json!({}),
        };
        assert!(permission_check.check_tool(&valid_tc).is_ok());

        let invalid_tc = ToolCall {
            id: "2".to_string(),
            name: "dangerous_tool".to_string(),
            arguments: json!({}),
        };
        let res = permission_check.check_tool(&invalid_tc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Anthropic Permission Guardrail: Tool 'dangerous_tool' requires explicit permission before each call.");
    }

    #[test]
    fn test_anthropic_explicit_confirmation() {
        let explicit_conf = AnthropicExplicitConfirmation::new(
            vec!["delete_file".to_string(), "drop_table".to_string()],
            vec!["rm -rf".to_string(), "--force".to_string()],
        );

        let safe_tc = ToolCall {
            id: "1".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "test.txt"}),
        };
        assert!(explicit_conf.check_tool(&safe_tc).is_ok());

        let risky_tool_tc = ToolCall {
            id: "2".to_string(),
            name: "delete_file".to_string(),
            arguments: json!({"path": "important.txt"}),
        };
        let res1 = explicit_conf.check_tool(&risky_tool_tc);
        assert!(res1.is_err());
        assert_eq!(res1.unwrap_err(), "Anthropic Explicit Confirmation: High-risk tool 'delete_file' requires user confirmation.");

        let risky_pattern_tc = ToolCall {
            id: "3".to_string(),
            name: "bash".to_string(),
            arguments: json!({"cmd": "rm -rf /"}),
        };
        let res2 = explicit_conf.check_tool(&risky_pattern_tc);
        assert!(res2.is_err());
        assert_eq!(res2.unwrap_err(), "Anthropic Explicit Confirmation: High-risk pattern 'rm -rf' detected in tool 'bash'. Requires user confirmation.");
    }
}
