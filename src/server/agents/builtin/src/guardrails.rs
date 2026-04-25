#[derive(Debug, Clone)]
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

pub fn check_tool(name: &str, arguments: &serde_json::Value, cfg: &GuardrailConfig) -> Result<(), String> {
    for kw in &cfg.blocked_keywords {
        if name.contains(kw) {
            return Err(format!("Tool name contains blocked keyword: {}", kw));
        }
        let args_str = arguments.to_string();
        if args_str.contains(kw) {
            return Err(format!("Tool arguments contain blocked keyword: {}", kw));
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
            blocked_keywords: vec!["password".to_string(), "secret".to_string()],
        };

        assert!(check_input("hello world", &cfg).is_ok());
        assert!(check_input("my password is 123", &cfg).is_err());

        assert!(check_output("safe response", &cfg).is_ok());
        assert!(check_output("the secret key is abc", &cfg).is_err());

        assert!(check_tool("my_tool", &json!({"arg": "value"}), &cfg).is_ok());
        assert!(check_tool("get_password", &json!({"arg": "value"}), &cfg).is_err());
        assert!(check_tool("my_tool", &json!({"arg": "secret_key"}), &cfg).is_err());
    }
}
