#[derive(Debug, Clone)]
pub struct GuardrailConfig {
    pub input_blocked_keywords: Vec<String>,
    pub output_blocked_keywords: Vec<String>,
    pub tool_blocked_keywords: Vec<String>,
}

pub fn check_input(input: &str, cfg: &Option<GuardrailConfig>) -> Result<(), String> {
    if let Some(config) = cfg {
        for kw in &config.input_blocked_keywords {
            if input.contains(kw) {
                return Err(format!("Input contains blocked keyword: {}", kw));
            }
        }
    }
    Ok(())
}

pub fn check_output(output: &str, cfg: &Option<GuardrailConfig>) -> Result<(), String> {
    if let Some(config) = cfg {
        for kw in &config.output_blocked_keywords {
            if output.contains(kw) {
                return Err(format!("Output contains blocked keyword: {}", kw));
            }
        }
    }
    Ok(())
}

pub fn check_tool(tool_name: &str, args_json: &str, cfg: &Option<GuardrailConfig>) -> Result<(), String> {
    if let Some(config) = cfg {
        for kw in &config.tool_blocked_keywords {
            if tool_name.contains(kw) || args_json.contains(kw) {
                return Err(format!("Tool call contains blocked keyword: {}", kw));
            }
        }
    }
    Ok(())
}
