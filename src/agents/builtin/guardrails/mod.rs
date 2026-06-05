pub mod openai_hooks;
pub mod anthropic_hooks;

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
