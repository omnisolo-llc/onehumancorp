#[derive(Debug, Clone)]
pub struct GuardrailConfig {
    pub blocked_keywords: Vec<String>,
}

pub fn check_input(input: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    let lower_input = input.to_lowercase();
    for kw in &cfg.blocked_keywords {
        if lower_input.contains(&kw.to_lowercase()) {
            return Err(format!("Input contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}

pub fn check_output(output: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    let lower_output = output.to_lowercase();
    for kw in &cfg.blocked_keywords {
        if lower_output.contains(&kw.to_lowercase()) {
            return Err(format!("Output contains blocked keyword: {}", kw));
        }
    }
    Ok(())
}

pub fn check_tool(tool_name: &str, tool_args: &str, cfg: &GuardrailConfig) -> Result<(), String> {
    let lower_name = tool_name.to_lowercase();
    let lower_args = tool_args.to_lowercase();

    for kw in &cfg.blocked_keywords {
        let lower_kw = kw.to_lowercase();
        if lower_name.contains(&lower_kw) || lower_args.contains(&lower_kw) {
            return Err(format!("Tool call contains blocked keyword: {}", kw));
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
            blocked_keywords: vec!["secret".to_string(), "password".to_string()],
        };

        assert!(check_input("hello world", &cfg).is_ok());
        assert!(check_input("this is a SecRet message", &cfg).is_err());
        assert!(check_input("password123", &cfg).is_err());
    }

    #[test]
    fn test_check_output() {
        let cfg = GuardrailConfig {
            blocked_keywords: vec!["classified".to_string()],
        };

        assert!(check_output("public info", &cfg).is_ok());
        assert!(check_output("CLASSIFIED material", &cfg).is_err());
    }

    #[test]
    fn test_check_tool() {
        let cfg = GuardrailConfig {
            blocked_keywords: vec!["rm -rf".to_string(), "sudo".to_string()],
        };

        assert!(check_tool("ls", "{}", &cfg).is_ok());
        assert!(check_tool("bash", "{\"cmd\": \"rm -rf /\"}", &cfg).is_err());
        assert!(check_tool("sudo_tool", "{}", &cfg).is_err());
    }
}
