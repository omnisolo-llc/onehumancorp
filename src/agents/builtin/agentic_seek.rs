#![allow(clippy::all)]
use crate::agent::AgentRunConfig;
use crate::llm::LlmClient;
use crate::llm::ollama::OllamaClient;
use crate::provider::{Credentials, Provider, ProviderType, Transport};
use std::collections::HashMap;
use std::sync::Arc;

/// Master Catalog C.19. agenticSeek: Fully local agent, no API costs
pub struct AgenticSeekProvider {
    pub local_endpoint: String,
    pub model_name: String,
}

impl AgenticSeekProvider {
    pub fn new(local_endpoint: &str, model_name: &str) -> Self {
        Self {
            local_endpoint: local_endpoint.to_string(),
            model_name: model_name.to_string(),
        }
    }

    pub fn build_local_config(&self) -> AgentRunConfig {
        let mut config = AgentRunConfig::default();
        config.enable_visual_verification = false;
        config.max_iterations = 25;
        config.enable_llmcompiler_plan_and_execute = false;
        config
    }

    pub fn local_llm_client(&self) -> Arc<dyn LlmClient> {
        Arc::new(OllamaClient::new(&self.local_endpoint))
    }
}

#[async_trait::async_trait]
impl Provider for AgenticSeekProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::AgenticSeek
    }

    fn description(&self) -> String {
        "agenticSeek — Fully local agent, no API costs, operating purely on local compute."
            .to_string()
    }

    fn supported_roles(&self) -> Vec<String> {
        vec!["local_agent".to_string()]
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
    fn test_local_execution_config() {
        let provider = AgenticSeekProvider::new("http://localhost:11434", "llama3");
        let config = provider.build_local_config();

        assert_eq!(provider.local_endpoint, "http://localhost:11434");
        assert_eq!(provider.model_name, "llama3");

        assert!(!(config.enable_visual_verification));
        assert!(!(config.enable_llmcompiler_plan_and_execute));
        assert_eq!(config.max_iterations, 25);
    }

    #[tokio::test]
    async fn test_provider_trait() {
        let provider = AgenticSeekProvider::new("http://localhost:11434", "llama3");
        assert_eq!(provider.provider_type(), ProviderType::AgenticSeek);
        assert!(provider.description().contains("agenticSeek"));
        assert_eq!(provider.supported_roles(), vec!["local_agent".to_string()]);
        assert!(
            provider
                .authenticate(Credentials {
                    api_key: "".to_string(),
                    oauth_token: "".to_string(),
                    extra: HashMap::new()
                })
                .is_ok()
        );
        assert!(provider.is_authenticated());
        assert_eq!(provider.get_credentials().api_key, "");
        assert!(
            provider
                .run_in_isolation("echo hello", "/tmp", None)
                .await
                .is_ok()
        );
    }
}
