use crate::agent::AgentRunConfig;

/// AgenticSeek Unique Harness Innovations: Fully local agent, no API costs
pub struct AgenticSeekEnforcer;

impl AgenticSeekEnforcer {
    pub fn enforce_local_only(cfg: &mut AgentRunConfig) -> Result<(), String> {
        // Enforce local models
        if !cfg.model.starts_with("ollama/") && !cfg.model.starts_with("local/") {
            return Err(format!("AgenticSeek Mode Error: Model '{}' is not a local model. To ensure zero API costs, use models prefixed with 'ollama/' or 'local/'.", cfg.model));
        }

        // Disable expensive/external tools
        let external_tools = vec!["webfetch", "websearch", "magentic_invoke"];
        if let Some(allowed) = &mut cfg.allowed_tools {
            allowed.retain(|t| !external_tools.contains(&t.as_str()));
        } else {
            // If allowed_tools was None (all tools allowed), we must restrict it
            return Err("AgenticSeek Mode Error: allowed_tools must be explicitly set to restrict external API tools.".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforce_local_only_success() {
        let mut cfg = AgentRunConfig::default();
        cfg.model = "ollama/llama3".to_string();
        cfg.allowed_tools = Some(vec!["bash".to_string(), "read".to_string()]);

        let result = AgenticSeekEnforcer::enforce_local_only(&mut cfg);
        assert!(result.is_ok());
    }

    #[test]
    fn test_enforce_local_only_failure() {
        let mut cfg = AgentRunConfig::default();
        cfg.model = "gpt-4".to_string();
        cfg.allowed_tools = Some(vec!["bash".to_string()]);

        let result = AgenticSeekEnforcer::enforce_local_only(&mut cfg);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a local model"));
    }
}
