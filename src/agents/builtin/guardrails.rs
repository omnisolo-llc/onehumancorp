use ohc_builtin_agent_core::types::ToolCall;

#[derive(Debug, Clone)]
pub struct GuardrailConfig {
    pub blocked_keywords: Vec<String>,
}

pub fn check_input(input: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if input.contains(kw) {
            return Err(format!("Input guardrail tripped: contains blocked keyword: {}", kw));
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
            blocked_keywords: vec!["banned".to_string(), "evil".to_string(), "exfiltrate".to_string()],
        };

        // Input
        assert!(check_input("hello world", &cfg).is_ok());
        assert!(check_input("hello banned word", &cfg).is_err());

        // Output
        assert!(check_output("safe output", &cfg).is_ok());
        assert!(check_output("evil output", &cfg).is_err());
        assert!(check_output("time to exfiltrate data", &cfg).is_err());

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
        assert!(check_tool(&tc_bad_name, &cfg).is_err());

        let tc_bad_arg = ToolCall {
            id: "3".to_string(),
            name: "safe_tool".to_string(),
            arguments: json!({"arg": "banned_value"}),
        };
        assert!(check_tool(&tc_bad_arg, &cfg).is_err());
    }
}

pub fn check_output(output: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if output.contains(kw) {
            return Err(format!("Output guardrail tripped: contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}

pub fn check_tool(tc: &ToolCall, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if tc.name.contains(kw) {
            return Err(format!("Tool guardrail tripped: name contains blocked keyword: {}", kw));
        }
        let args_str = tc.arguments.to_string();
        if args_str.contains(kw) {
            return Err(format!("Tool guardrail tripped: arguments contain blocked keyword: {}", kw));
        }
    }
    Ok(())
}
