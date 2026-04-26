use ohc_builtin_agent_core::types::ToolCall;

#[derive(Debug, Clone, Default)]
pub struct GuardrailConfig {
    pub blocked_keywords: Vec<String>,
}

pub fn check_input(input: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if input.contains(kw) {
            return Err(format!("Input contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}

pub fn check_output(output: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if output.contains(kw) {
            return Err(format!("Output contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}

pub fn check_tool_call(tc: &ToolCall, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if tc.name.contains(kw) || tc.arguments.to_string().contains(kw) {
            return Err(format!("Tool call contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}
