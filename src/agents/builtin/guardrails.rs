use ohc_builtin_agent_core::types::ToolCall;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GuardrailConfig {
    pub blocked_keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailType {
    Input,
    Output,
    Tool,
}

#[derive(Debug, Clone)]
pub struct GuardrailError {
    pub guardrail_type: GuardrailType,
    pub reason: String,
}

impl fmt::Display for GuardrailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self.guardrail_type {
            GuardrailType::Input => "Input guardrail tripped",
            GuardrailType::Output => "Output guardrail tripped",
            GuardrailType::Tool => "Tool guardrail tripped",
        };
        write!(f, "{}: {}", type_str, self.reason)
    }
}

impl std::error::Error for GuardrailError {}

pub fn check_input(input: &str, cfg: &GuardrailConfig) -> Result<(), GuardrailError> {
    for kw in &cfg.blocked_keywords {
        if input.contains(kw) {
            return Err(GuardrailError {
                guardrail_type: GuardrailType::Input,
                reason: format!("contains blocked keyword: {}", kw),
            });
        }
    }
    Ok(())
}

pub fn check_output(output: &str, cfg: &GuardrailConfig) -> Result<(), GuardrailError> {
    for kw in &cfg.blocked_keywords {
        if output.contains(kw) {
            return Err(GuardrailError {
                guardrail_type: GuardrailType::Output,
                reason: format!("contains blocked keyword: {}", kw),
            });
        }
    }
    Ok(())
}

pub fn check_tool(tc: &ToolCall, cfg: &GuardrailConfig) -> Result<(), GuardrailError> {
    for kw in &cfg.blocked_keywords {
        if tc.name.contains(kw) {
            return Err(GuardrailError {
                guardrail_type: GuardrailType::Tool,
                reason: format!("name contains blocked keyword: {}", kw),
            });
        }
        let args_str = tc.arguments.to_string();
        if args_str.contains(kw) {
            return Err(GuardrailError {
                guardrail_type: GuardrailType::Tool,
                reason: format!("arguments contain blocked keyword: {}", kw),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_guardrails() {
        let cfg = GuardrailConfig {
            blocked_keywords: vec!["banned".to_string(), "evil".to_string()],
        };

        // Input
        assert!(check_input("hello world", &cfg).is_ok());
        let err = check_input("hello banned word", &cfg).unwrap_err();
        assert_eq!(err.guardrail_type, GuardrailType::Input);
        assert!(err.reason.contains("banned"));

        // Output
        assert!(check_output("safe output", &cfg).is_ok());
        let err = check_output("evil output", &cfg).unwrap_err();
        assert_eq!(err.guardrail_type, GuardrailType::Output);
        assert!(err.reason.contains("evil"));

        // Tool
        let tc_safe = ToolCall {
            id: "1".to_string(),
            name: "safe_tool".to_string(),
            arguments: json!({"arg": "value"}),
        };
        assert!(check_tool(&tc_safe, &cfg).is_ok());

        let tc_bad_name = ToolCall {
            id: "2".to_string(),
            name: "evil_tool".to_string(),
            arguments: json!({"arg": "value"}),
        };
        let err = check_tool(&tc_bad_name, &cfg).unwrap_err();
        assert_eq!(err.guardrail_type, GuardrailType::Tool);

        let tc_bad_arg = ToolCall {
            id: "3".to_string(),
            name: "safe_tool".to_string(),
            arguments: json!({"arg": "banned_value"}),
        };
        let err = check_tool(&tc_bad_arg, &cfg).unwrap_err();
        assert_eq!(err.guardrail_type, GuardrailType::Tool);
    }
}
