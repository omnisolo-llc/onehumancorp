use std::env;

/// agenticSeek Unique Harness Innovations: Fully local agent, no API costs
/// This mechanic strictly enforces that the agent runs entirely locally
/// by verifying no external API keys are configured, guaranteeing zero API costs.
pub struct AgenticSeekEnforcer;

impl AgenticSeekEnforcer {
    /// Enforces that no external API keys are configured.
    /// If any are found, it returns an error to halt execution.
    pub fn enforce_no_api_costs(mock_env: Option<&[(&str, &str)]>) -> Result<(), String> {
        let forbidden_keys = ["OPENAI_API_KEY", "ANTHROPIC_API_KEY", "MINIMAX_API_KEY"];

        for key in forbidden_keys {
            let is_set = if let Some(env_map) = mock_env {
                env_map.iter().any(|(k, _)| k == &key)
            } else {
                env::var(key).is_ok()
            };

            if is_set {
                return Err(format!("agenticSeek Error: External API key {} found. agenticSeek requires a fully local agent with no API costs.", key));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforce_no_api_costs_success() {
        // Use a clean mocked environment
        let mock_env: &[(&str, &str)] = &[];
        let result = AgenticSeekEnforcer::enforce_no_api_costs(Some(mock_env));
        assert!(result.is_ok(), "Expected success when no API keys are set");
    }

    #[test]
    fn test_enforce_no_api_costs_failure() {
        // Use a mocked environment with a forbidden key
        let mock_env = [("OPENAI_API_KEY", "dummy_key")];

        let result = AgenticSeekEnforcer::enforce_no_api_costs(Some(&mock_env));
        assert!(result.is_err(), "Expected failure when OPENAI_API_KEY is set");
        assert!(result.unwrap_err().contains("agenticSeek Error"));
    }
}
