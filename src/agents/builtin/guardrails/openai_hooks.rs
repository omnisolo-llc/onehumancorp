use ohc_builtin_agent_core::types::ToolCall;
use crate::guardrails::{InputGuardrail, OutputGuardrail, ToolGuardrail};

/// OpenAI Mechanic: 3 distinct hooks for guardrails
/// 1. Input Validator
/// 2. Output Auditor
/// 3. Tool Policy Enforcer

pub struct OpenAiInputValidator {
    pub max_length: usize,
    pub require_patterns: Vec<String>,
    pub deny_patterns: Vec<String>,
}

impl OpenAiInputValidator {
    pub fn new(max_length: usize, require_patterns: Vec<String>, deny_patterns: Vec<String>) -> Self {
        Self {
            max_length,
            require_patterns,
            deny_patterns,
        }
    }
}

impl InputGuardrail for OpenAiInputValidator {
    fn check_input(&self, input: &str) -> Result<(), String> {
        if input.len() > self.max_length {
            return Err(format!("OpenAI Input Guardrail tripped: Input exceeds maximum length of {} bytes.", self.max_length));
        }

        for pattern in &self.require_patterns {
            if !input.contains(pattern) {
                return Err(format!("OpenAI Input Guardrail tripped: Input is missing required pattern '{}'.", pattern));
            }
        }

        for pattern in &self.deny_patterns {
            if input.contains(pattern) {
                return Err(format!("OpenAI Input Guardrail tripped: Input contains denied pattern '{}'.", pattern));
            }
        }

        Ok(())
    }
}

pub struct OpenAiOutputAuditor {
    pub min_length: usize,
    pub require_json: bool,
    pub deny_patterns: Vec<String>,
}

impl OpenAiOutputAuditor {
    pub fn new(min_length: usize, require_json: bool, deny_patterns: Vec<String>) -> Self {
        Self {
            min_length,
            require_json,
            deny_patterns,
        }
    }
}

impl OutputGuardrail for OpenAiOutputAuditor {
    fn check_output(&self, output: &str) -> Result<(), String> {
        if output.len() < self.min_length {
            return Err(format!("OpenAI Output Guardrail tripped: Output is shorter than minimum length of {} bytes.", self.min_length));
        }

        if self.require_json {
            // A simple check to see if it starts with { or [
            let trimmed = output.trim();
            if !(trimmed.starts_with('{') && trimmed.ends_with('}')) && !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
                return Err("OpenAI Output Guardrail tripped: Output must be a valid JSON object or array.".to_string());
            }
        }

        for pattern in &self.deny_patterns {
            if output.contains(pattern) {
                return Err(format!("OpenAI Output Guardrail tripped: Output contains denied pattern '{}'.", pattern));
            }
        }

        Ok(())
    }
}

pub struct OpenAiToolPolicyEnforcer {
    pub allowed_tools: Vec<String>,
    pub block_args: Vec<String>,
}

impl OpenAiToolPolicyEnforcer {
    pub fn new(allowed_tools: Vec<String>, block_args: Vec<String>) -> Self {
        Self {
            allowed_tools,
            block_args,
        }
    }
}

impl ToolGuardrail for OpenAiToolPolicyEnforcer {
    fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
        if !self.allowed_tools.is_empty() && !self.allowed_tools.contains(&tc.name) {
            return Err(format!("OpenAI Tool Guardrail tripped: Tool '{}' is not in the allowed policy list.", tc.name));
        }

        let args_str = tc.arguments.to_string();
        for blocked in &self.block_args {
            if args_str.contains(blocked) {
                return Err(format!("OpenAI Tool Guardrail tripped: Tool '{}' arguments contain blocked pattern '{}'.", tc.name, blocked));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use crate::guardrails::GuardrailRegistry;
    use serde_json::json;

    #[test]
    fn test_openai_input_validator() {
        let validator = OpenAiInputValidator::new(
            100,
            vec!["AGENT_INSTRUCTION".to_string()],
            vec!["DROP TABLE".to_string()],
        );

        // Success
        assert!(validator.check_input("AGENT_INSTRUCTION: list files").is_ok());

        // Max length fail
        let long_input = format!("AGENT_INSTRUCTION: {}", "A".repeat(150));
        let res = validator.check_input(&long_input);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("exceeds maximum length"));

        // Missing required pattern
        let res2 = validator.check_input("list files please");
        assert!(res2.is_err());
        assert!(res2.unwrap_err().contains("missing required pattern"));

        // Denied pattern
        let res3 = validator.check_input("AGENT_INSTRUCTION: DROP TABLE users;");
        assert!(res3.is_err());
        assert!(res3.unwrap_err().contains("contains denied pattern"));
    }

    #[test]
    fn test_openai_output_auditor() {
        let auditor = OpenAiOutputAuditor::new(
            10,
            true,
            vec!["SECRET_KEY".to_string()],
        );

        // Success
        assert!(auditor.check_output(r#"{"status": "success", "data": 123}"#).is_ok());

        // Min length fail
        let res1 = auditor.check_output(r#"{}"#);
        assert!(res1.is_err());
        assert!(res1.unwrap_err().contains("shorter than minimum length"));

        // JSON check fail
        let res2 = auditor.check_output("This is a long enough text but not JSON.");
        assert!(res2.is_err());
        assert!(res2.unwrap_err().contains("must be a valid JSON object"));

        // Denied pattern
        let res3 = auditor.check_output(r#"{"key": "SECRET_KEY123"}"#);
        assert!(res3.is_err());
        assert!(res3.unwrap_err().contains("contains denied pattern"));
    }

    #[test]
    fn test_openai_tool_policy_enforcer() {
        let enforcer = OpenAiToolPolicyEnforcer::new(
            vec!["read_file".to_string(), "list_files".to_string()],
            vec!["/etc/passwd".to_string()],
        );

        let tc_valid = ToolCall {
            id: "1".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "/home/user/test.txt"}),
        };

        // Success
        assert!(enforcer.check_tool(&tc_valid).is_ok());

        // Not allowed tool
        let tc_invalid_name = ToolCall {
            id: "2".to_string(),
            name: "delete_file".to_string(),
            arguments: json!({"path": "/home/user/test.txt"}),
        };
        let res1 = enforcer.check_tool(&tc_invalid_name);
        assert!(res1.is_err());
        assert!(res1.unwrap_err().contains("not in the allowed policy list"));

        // Blocked argument pattern
        let tc_blocked_arg = ToolCall {
            id: "3".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "/etc/passwd"}),
        };
        let res2 = enforcer.check_tool(&tc_blocked_arg);
        assert!(res2.is_err());
        assert!(res2.unwrap_err().contains("arguments contain blocked pattern"));
    }

    #[test]
    fn test_openai_hooks_registry_integration() {
        let mut registry = GuardrailRegistry::new();

        let input_hook = Arc::new(OpenAiInputValidator::new(500, vec![], vec!["rm -rf".to_string()]));
        let output_hook = Arc::new(OpenAiOutputAuditor::new(5, false, vec!["error".to_string()]));
        let tool_hook = Arc::new(OpenAiToolPolicyEnforcer::new(vec!["safe_tool".to_string()], vec![]));

        registry.input_guardrails.push(input_hook);
        registry.output_guardrails.push(output_hook);
        registry.tool_guardrails.push(tool_hook);

        assert!(registry.check_input("run this task").is_ok());
        assert!(registry.check_input("run this task and rm -rf /").is_err());

        assert!(registry.check_output("valid output").is_ok());
        assert!(registry.check_output("system error").is_err());

        let tc1 = ToolCall { id: "1".to_string(), name: "safe_tool".to_string(), arguments: json!({}) };
        assert!(registry.check_tool(&tc1).is_ok());

        let tc2 = ToolCall { id: "2".to_string(), name: "unsafe_tool".to_string(), arguments: json!({}) };
        assert!(registry.check_tool(&tc2).is_err());
    }
}
