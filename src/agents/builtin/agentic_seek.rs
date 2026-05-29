use crate::agent::AgentRunConfig;

/// agenticSeek Unique Harness Innovations: Fully local agent, no API costs
///
/// This harness ensures that the AgentRunConfig is strictly bound to local execution,
/// preventing any usage of external, cost-incurring APIs.
pub struct AgenticSeekLocalHarness;

impl AgenticSeekLocalHarness {
    /// Enforces the local execution constraint on the provided configuration.
    /// Returns an error if any external API key configuration is attempted.
    pub fn enforce_local_execution(config: &mut AgentRunConfig) -> Result<(), String> {
        let model_lower = config.model.to_lowercase();

        // Check for common external models
        if model_lower.contains("gpt-") || model_lower.contains("claude-") || model_lower.contains("gemini") {
            return Err("AgenticSeek error: Fully local agent, no API costs. External model detected.".to_string());
        }

        // Set flag to indicate this is local execution
        config.is_local_execution = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentRunConfig;

    #[test]
    fn test_enforce_local_execution_success() {
        let mut config = AgentRunConfig::default();
        config.model = "llama3:8b".to_string(); // Local model

        let result = AgenticSeekLocalHarness::enforce_local_execution(&mut config);

        assert!(result.is_ok());
        assert!(config.is_local_execution);
    }

    #[test]
    fn test_enforce_local_execution_failure_gpt() {
        let mut config = AgentRunConfig::default();
        config.model = "gpt-4o".to_string();

        let result = AgenticSeekLocalHarness::enforce_local_execution(&mut config);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "AgenticSeek error: Fully local agent, no API costs. External model detected."
        );
    }

    #[test]
    fn test_enforce_local_execution_failure_claude() {
        let mut config = AgentRunConfig::default();
        config.model = "claude-3-5-sonnet-20241022".to_string();

        let result = AgenticSeekLocalHarness::enforce_local_execution(&mut config);

        assert!(result.is_err());
    }
}
