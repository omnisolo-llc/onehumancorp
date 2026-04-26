#[derive(Debug, Clone, Default)]
pub struct GuardrailConfig {
    pub input_blocked_keywords: Vec<String>,
    pub output_blocked_keywords: Vec<String>,
    pub tool_blocked_keywords: Vec<String>,
}

pub fn check_input(input: &str, cfg: &GuardrailConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for kw in &cfg.input_blocked_keywords {
        if input.contains(kw) {
            return Err(format!("Input guardrail tripped: contains blocked keyword '{}'", kw).into());
        }
    }
    Ok(())
}

pub fn check_output(output: &str, cfg: &GuardrailConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for kw in &cfg.output_blocked_keywords {
        if output.contains(kw) {
            return Err(format!("Output guardrail tripped: contains blocked keyword '{}'", kw).into());
        }
    }
    Ok(())
}

pub fn check_tool(tool_name: &str, args_json: &str, cfg: &GuardrailConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let combined = format!("{} {}", tool_name, args_json);
    for kw in &cfg.tool_blocked_keywords {
        if combined.contains(kw) {
            return Err(format!("Tool guardrail tripped: contains blocked keyword '{}'", kw).into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guardrails() {
        let cfg = GuardrailConfig {
            input_blocked_keywords: vec!["bad_input".to_string()],
            output_blocked_keywords: vec!["bad_output".to_string()],
            tool_blocked_keywords: vec!["rm -rf".to_string()],
        };

        assert!(check_input("good input", &cfg).is_ok());
        assert!(check_input("this is bad_input", &cfg).is_err());

        assert!(check_output("good output", &cfg).is_ok());
        assert!(check_output("this is bad_output", &cfg).is_err());

        assert!(check_tool("bash", "{\"command\": \"ls\"}", &cfg).is_ok());
        assert!(check_tool("bash", "{\"command\": \"rm -rf /\"}", &cfg).is_err());
    }
}
