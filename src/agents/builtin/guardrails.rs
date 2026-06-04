use ohc_builtin_agent_core::types::ToolCall;
use std::sync::Arc;
use std::fmt::Debug;

pub trait InputGuardrail: Send + Sync {
    fn check_input(&self, input: &str) -> Result<(), String>;
}

pub trait OutputGuardrail: Send + Sync {
    fn check_output(&self, output: &str) -> Result<(), String>;
}

pub trait ToolGuardrail: Send + Sync {
    fn check_tool(&self, tc: &ToolCall) -> Result<(), String>;
}

#[derive(Clone)]
pub struct GuardrailRegistry {
    pub input_guardrails: Vec<Arc<dyn InputGuardrail>>,
    pub output_guardrails: Vec<Arc<dyn OutputGuardrail>>,
    pub tool_guardrails: Vec<Arc<dyn ToolGuardrail>>,
}

impl Debug for GuardrailRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuardrailRegistry").finish()
    }
}

impl Default for GuardrailRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GuardrailRegistry {
    pub fn new() -> Self {
        Self {
            input_guardrails: Vec::new(),
            output_guardrails: Vec::new(),
            tool_guardrails: Vec::new(),
        }
    }

    pub fn check_input(&self, input: &str) -> Result<(), String> {
        for hook in &self.input_guardrails {
            hook.check_input(input)?;
        }
        Ok(())
    }

    pub fn check_output(&self, output: &str) -> Result<(), String> {
        for hook in &self.output_guardrails {
            hook.check_output(output)?;
        }
        Ok(())
    }

    pub fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
        for hook in &self.tool_guardrails {
            hook.check_tool(tc)?;
        }
        Ok(())
    }
}

pub struct KeywordGuardrail {
    blocked_keywords: Vec<String>,
}

impl KeywordGuardrail {
    pub fn new(blocked_keywords: Vec<String>) -> Self {
        Self { blocked_keywords }
    }
}

impl InputGuardrail for KeywordGuardrail {
    fn check_input(&self, input: &str) -> Result<(), String> {
        for kw in &self.blocked_keywords {
            if input.contains(kw) {
                return Err(format!("Input guardrail tripped: contains blocked keyword: {}", kw));
            }
        }
        Ok(())
    }
}

impl OutputGuardrail for KeywordGuardrail {
    fn check_output(&self, output: &str) -> Result<(), String> {
        for kw in &self.blocked_keywords {
            if output.contains(kw) {
                return Err(format!("Output guardrail tripped: contains blocked keyword: {}", kw));
            }
        }
        Ok(())
    }
}

impl ToolGuardrail for KeywordGuardrail {
    fn check_tool(&self, tc: &ToolCall) -> Result<(), String> {
        for kw in &self.blocked_keywords {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_guardrails() {
        let kw_hook = Arc::new(KeywordGuardrail::new(vec!["banned".to_string(), "evil".to_string()]));

        let mut registry = GuardrailRegistry::new();
        registry.input_guardrails.push(kw_hook.clone());
        registry.output_guardrails.push(kw_hook.clone());
        registry.tool_guardrails.push(kw_hook.clone());

        // Input
        assert!(registry.check_input("hello world").is_ok());
        assert!(registry.check_input("hello banned word").is_err());

        // Output
        assert!(registry.check_output("safe output").is_ok());
        assert!(registry.check_output("evil output").is_err());

        // Tool
        let tc_safe = ToolCall {
            id: "1".to_string(),
            name: "safe_tool".to_string(),
            arguments: json!({"arg": "value"}),
        };
        assert!(registry.check_tool(&tc_safe).is_ok());

        let tc_bad_name = ToolCall {
            id: "2".to_string(),
            name: "evil_tool".to_string(),
            arguments: json!({"arg": "value"}),
        };
        assert!(registry.check_tool(&tc_bad_name).is_err());

        let tc_bad_arg = ToolCall {
            id: "3".to_string(),
            name: "safe_tool".to_string(),
            arguments: json!({"arg": "banned_value"}),
        };
        assert!(registry.check_tool(&tc_bad_arg).is_err());
    }
}
