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

pub fn check_tool(tool_name: &str, args_json: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if args_json.contains(kw) {
            return Err(format!("Tool {} arguments contain blocked keyword: {}", tool_name, kw));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_input() {
        let cfg = GuardrailConfig {
            blocked_keywords: vec!["bad".to_string(), "evil".to_string()],
        };

        assert!(check_input("This is a good input", &cfg).is_ok());
        assert!(check_input("This is a bad input", &cfg).is_err());
        assert!(check_input("This is an evil input", &cfg).is_err());
    }

    #[test]
    fn test_check_output() {
        let cfg = GuardrailConfig {
            blocked_keywords: vec!["bad".to_string(), "evil".to_string()],
        };

        assert!(check_output("This is a good output", &cfg).is_ok());
        assert!(check_output("This is a bad output", &cfg).is_err());
        assert!(check_output("This is an evil output", &cfg).is_err());
    }

    #[test]
    fn test_check_tool() {
        let cfg = GuardrailConfig {
            blocked_keywords: vec!["bad".to_string(), "evil".to_string()],
        };

        assert!(check_tool("test_tool", r#"{"arg": "good"}"#, &cfg).is_ok());
        assert!(check_tool("test_tool", r#"{"arg": "bad"}"#, &cfg).is_err());
        assert!(check_tool("test_tool", r#"{"arg": "evil"}"#, &cfg).is_err());
    }
}
