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

pub fn check_tool_invocation(name: &str, args_json: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if name.contains(kw) || args_json.contains(kw) {
            return Err(format!("Tool invocation contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}

pub fn check_tool_result(result: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if result.contains(kw) {
            return Err(format!("Tool result contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}
