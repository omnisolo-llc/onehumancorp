
use crate::agent::AgentRunConfig;
use crate::llm::LlmClient;
use crate::llm::ollama::OllamaClient;
use crate::provider::{Credentials, Provider, ProviderType, Transport};
use std::collections::HashMap;
use std::sync::Arc;

/// Pi (pi-agent-core): TypeScript monorepo architecture archetype.
/// This Rust provider models the Pi harness structure for monorepo and TypeScript agent integration.
pub struct PiProvider {
    pub local_endpoint: String,
    pub model_name: String,
}

impl PiProvider {
    pub fn new(local_endpoint: &str, model_name: &str) -> Self {
        Self {
            local_endpoint: local_endpoint.to_string(),
            model_name: model_name.to_string(),
        }
    }

    pub fn build_pi_config(&self) -> AgentRunConfig {
        let mut config = AgentRunConfig::default();
        config.enable_visual_verification = true; // Pi often integrates with UI tools
        config.max_iterations = 30; // Pi agents often run longer multi-step processes
        config.enable_llmcompiler_plan_and_execute = false;
        config
    }

    pub fn pi_llm_client(&self) -> Arc<dyn LlmClient> {
        // In a real Pi integration, this might wrap an MCP or specific Pi endpoint.
        // For now, it maps to a standard LLM client.
        Arc::new(OllamaClient::new(&self.local_endpoint))
    }
}

#[async_trait::async_trait]
impl Provider for PiProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Pi
    }

    fn description(&self) -> String {
        "Pi (pi-agent-core) — Agent modeling a TypeScript monorepo harness architecture."
            .to_string()
    }

    fn supported_roles(&self) -> Vec<String> {
        vec!["pi_agent".to_string(), "monorepo_specialist".to_string()]
    }

    fn authenticate(&self, _creds: Credentials) -> Result<(), String> {
        Ok(())
    }

    fn get_credentials(&self) -> Credentials {
        Credentials {
            api_key: "".to_string(),
            oauth_token: "".to_string(),
            extra: HashMap::new(),
        }
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    async fn run_in_isolation(
        &self,
        _command: &str,
        _worktree: &str,
        _transport: Option<Arc<dyn Transport>>,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_execution_config() {
        let provider = PiProvider::new("http://localhost:11434", "pi-model");
        let config = provider.build_pi_config();

        assert_eq!(provider.local_endpoint, "http://localhost:11434");
        assert_eq!(provider.model_name, "pi-model");

        assert!(config.enable_visual_verification);
        assert!(!(config.enable_llmcompiler_plan_and_execute));
        assert_eq!(config.max_iterations, 30);
    }
}
