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

pub fn check_tool(tool_name: &str, arguments: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if tool_name.contains(kw) || arguments.contains(kw) {
            return Err(format!("Tool {} arguments contain blocked keyword: {}", tool_name, kw));
        }
    }
    Ok(())
}
