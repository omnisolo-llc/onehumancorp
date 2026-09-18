#![allow(clippy::all)]
use crate::agent::AgentRunConfig;
use crate::llm::LlmClient;
use crate::llm::openai::OpenAIClient;
use crate::provider::{Credentials, Provider, ProviderType, Transport};
use std::collections::HashMap;
use std::sync::Arc;

/// Master Catalog: Kimi: ACP v1 JSON-RPC protocol (omnisolo/harness-worker-kimi:1.49.0).
/// Controls: prompt, stream, cancel, approvals, questions, exact resume.
/// This Rust provider models the Kimi harness structure for ACP v1 JSON-RPC protocol integration.
pub struct KimiProvider {
    pub local_endpoint: String,
    pub model_name: String,
    base: crate::provider::BaseProvider,
}

impl KimiProvider {
    pub fn new(local_endpoint: &str, model_name: &str) -> Self {
        Self {
            local_endpoint: local_endpoint.to_string(),
            model_name: model_name.to_string(),
            base: crate::provider::BaseProvider::new(),
        }
    }

    pub fn build_kimi_config(&self) -> AgentRunConfig {
        let mut config = AgentRunConfig::default();
        config.enable_visual_verification = false;
        config.max_iterations = 20;
        config.enable_llmcompiler_plan_and_execute = false;
        config.enable_time_travel_rewind = true; // "exact resume"
        config.hil_spectrum = crate::types::HumanInLoopSpectrum::ApprovalOnAll; // "questions"
        config.permission_architecture = crate::types::PermissionArchitecture::Restrictive; // "approvals"
        config
    }

    pub fn kimi_llm_client(&self) -> Arc<dyn LlmClient> {
        let creds = self.base.load();
        let api_key = if creds.api_key.is_empty() {
            "dummy_key".to_string()
        } else {
            creds.api_key.clone()
        };
        Arc::new(OpenAIClient::with_base_url(&api_key, &self.local_endpoint))
    }
}

#[async_trait::async_trait]
impl Provider for KimiProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Kimi
    }

    fn description(&self) -> String {
        "Kimi — ACP v1 JSON-RPC protocol harness for robust interactions.".to_string()
    }

    fn supported_roles(&self) -> Vec<String> {
        vec!["GENERAL_ASSISTANT".to_string()]
    }

    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        self.base.store(creds);
        Ok(())
    }

    fn get_credentials(&self) -> Credentials {
        self.base.load()
    }

    fn is_authenticated(&self) -> bool {
        !self.base.load().is_empty()
    }

    async fn run_in_isolation(
        &self,
        command: &str,
        worktree: &str,
        transport: Option<Arc<dyn Transport>>,
    ) -> Result<(), String> {
        crate::provider::execute_in_isolation(
            command,
            &self.provider_type().to_string(),
            worktree,
            transport,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kimi_provider_initialization() {
        let provider = KimiProvider::new("http://localhost:8080", "kimi-latest");
        assert_eq!(provider.local_endpoint, "http://localhost:8080");
        assert_eq!(provider.model_name, "kimi-latest");
    }

    #[test]
    fn test_kimi_build_config() {
        let provider = KimiProvider::new("http://localhost:8080", "kimi-latest");
        let config = provider.build_kimi_config();
        assert!(config.enable_time_travel_rewind); // For "exact resume"
        assert_eq!(config.hil_spectrum, crate::types::HumanInLoopSpectrum::ApprovalOnAll); // For "questions"
        assert_eq!(config.permission_architecture, crate::types::PermissionArchitecture::Restrictive); // For "approvals"
    }

    #[test]
    fn test_kimi_provider_auth() {
        let provider = KimiProvider::new("http://localhost:8080", "kimi-latest");
        assert!(!provider.is_authenticated());

        let creds = Credentials {
            api_key: "test-key".to_string(),
            oauth_token: "".to_string(),
            extra: HashMap::new(),
        };

        assert!(provider.authenticate(creds).is_ok());
        assert!(provider.is_authenticated());
    }

    #[test]
    fn test_kimi_client() {
        let provider = KimiProvider::new("http://localhost:8080", "kimi-latest");
        let _client = provider.kimi_llm_client();
        // Just verifying it instantiates properly
    }
}
