//! Explicit text inference for tenant HTTP tasks. This entry point has no tool
//! catalog, memory store, workspace, agent loop, subprocess or fallback model.
use crate::tools::tenant::TenantContext;
use omnisolo_builtin_agent_core::types::{ChatRequest, Message};
use omnisolo_builtin_agent_llm::{
    LlmClient,
    anthropic::AnthropicClient,
    ollama::OllamaClient,
    openai::{OpenAIClient, OpenAIClientConfig},
};
use std::sync::Arc;

pub struct ConfiguredTextAnalysis {
    provider: String,
    model: String,
    max_output_tokens: i32,
    llm: Arc<dyn LlmClient>,
}

pub struct TenantTextAnalysis {
    configured: Arc<ConfiguredTextAnalysis>,
    tenant: TenantContext,
    actor_id: String,
}

impl ConfiguredTextAnalysis {
    pub fn from_environment() -> Result<Self, &'static str> {
        Self::from_values(|name| std::env::var(name).ok())
    }
    fn from_values(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, &'static str> {
        let configured_value = |name: &str| {
            lookup(name)
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
        };
        let provider = configured_value("OMNISOLO_LLM_PROVIDER")
            .ok_or("Text analysis provider is not configured")?;
        let model = configured_value("OMNISOLO_LLM_MODEL")
            .ok_or("Text analysis model is not configured")?;
        let max_output_tokens = configured_value("OMNISOLO_MAX_TOKENS")
            .map(|value| value.parse::<i32>())
            .transpose()
            .map_err(|_| "Invalid text analysis token limit")?
            .unwrap_or(2048);
        if model.len() > 200 || !(1..=4096).contains(&max_output_tokens) {
            return Err("Invalid text analysis limits");
        }
        let endpoint = configured_value("OMNISOLO_LLM_ENDPOINT")
            .or_else(|| configured_value("OMNISOLO_LOCAL_LLM_ENDPOINT"));
        let llm: Arc<dyn LlmClient> = match provider.as_str() {
            "openai" | "openai-compatible" | "minimax" => {
                let key_name = if provider == "minimax" {
                    "MINIMAX_API_KEY"
                } else {
                    "OPENAI_API_KEY"
                };
                let key = configured_value("OMNISOLO_LLM_API_KEY")
                    .or_else(|| configured_value(key_name))
                    .ok_or("Text analysis credentials are not configured")?;
                let mut config = if provider == "openai" && endpoint.is_none() {
                    OpenAIClientConfig::openai(key)
                } else {
                    let endpoint = endpoint.ok_or("Text analysis endpoint is not configured")?;
                    OpenAIClientConfig::openai_compatible(key, endpoint, Some(model.clone()))
                };
                config.default_model = Some(model.clone());
                Arc::new(OpenAIClient::from_config(config))
            }
            "anthropic" => {
                if endpoint.is_some() {
                    return Err("Custom Anthropic endpoints are not supported by text analysis");
                }
                let key = configured_value("ANTHROPIC_API_KEY")
                    .ok_or("Text analysis credentials are not configured")?;
                Arc::new(AnthropicClient::new(key))
            }
            "ollama" => Arc::new(OllamaClient::new(
                endpoint.ok_or("Text analysis endpoint is not configured")?,
            )),
            _ => return Err("Text analysis provider is unsupported"),
        };
        Ok(Self {
            provider,
            model,
            max_output_tokens,
            llm,
        })
    }
    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn model(&self) -> &str {
        &self.model
    }
    pub fn max_output_tokens(&self) -> i32 {
        self.max_output_tokens
    }
    pub fn bind(
        self: &Arc<Self>,
        tenant_id: &str,
        actor_id: &str,
    ) -> Result<TenantTextAnalysis, &'static str> {
        if tenant_id.trim().eq_ignore_ascii_case("system") || actor_id.trim().is_empty() {
            return Err("A verified tenant and actor are required");
        }
        let tenant = TenantContext::new(tenant_id)?;
        Ok(TenantTextAnalysis {
            configured: self.clone(),
            tenant,
            actor_id: actor_id.to_owned(),
        })
    }
}

impl TenantTextAnalysis {
    pub fn tenant_id(&self) -> &str {
        self.tenant.as_str()
    }
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }
    pub async fn analyze(&self, task: &str) -> Result<String, ()> {
        let result = self.configured.llm.chat(ChatRequest {
            model: self.configured.model.clone(),
            system: "Analyze only the text submitted by this tenant. No workspace, memory, tools, external actions or command execution are available. State missing information and never claim an action was performed.".into(),
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: self.configured.max_output_tokens,
            temperature: 0.0,
        }).await.map_err(|_| ())?;
        // These requests contain no custom stop sequence or tool capability.
        // Truncation, refusal, paused/tool turns and unknown reasons do not
        // acknowledge completed text, even if a partial body was returned.
        let complete = match self.configured.provider.as_str() {
            "anthropic" => result.stop_reason == "end_turn",
            "openai" | "openai-compatible" | "minimax" | "ollama" => result.stop_reason == "stop",
            _ => false,
        };
        if !complete
            || !result.message.tool_calls.is_empty()
            || !result.message.tool_results.is_empty()
            || result.message.content.trim().is_empty()
        {
            return Err(());
        }
        Ok(result.message.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use omnisolo_builtin_agent_core::types::{ChatResponse, ToolCall, Usage};
    use std::sync::Mutex;

    struct ObservedClient {
        seen: Mutex<Vec<ChatRequest>>,
        tool: bool,
        fail: bool,
        stop_reason: &'static str,
    }
    #[async_trait]
    impl LlmClient for ObservedClient {
        async fn chat(
            &self,
            request: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            self.seen.lock().unwrap().push(request);
            if self.fail {
                return Err("public fixture transport failure".into());
            }
            let mut message = Message::assistant("Actual supplied text analysis");
            if self.tool {
                message.tool_calls.push(ToolCall {
                    id: "unsupported-call".into(),
                    name: "bash".into(),
                    arguments: serde_json::json!({"command":"must never execute"}),
                });
            }
            Ok(ChatResponse {
                message,
                usage: Usage::default(),
                stop_reason: self.stop_reason.into(),
                response_id: None,
            })
        }
    }
    fn configured(client: Arc<ObservedClient>) -> Arc<ConfiguredTextAnalysis> {
        Arc::new(ConfiguredTextAnalysis {
            provider: "ollama".into(),
            model: "explicit-fixture-model".into(),
            max_output_tokens: 128,
            llm: client,
        })
    }
    #[tokio::test]
    async fn two_tenants_keep_their_context_and_send_only_supplied_text_with_no_tools() {
        let client = Arc::new(ObservedClient {
            seen: Mutex::new(vec![]),
            tool: false,
            fail: false,
            stop_reason: "stop",
        });
        let service = configured(client.clone());
        for (tenant, actor, text) in [
            ("tenant-a", "actor-a", "Private text A"),
            ("tenant-b", "actor-b", "Different text B"),
        ] {
            let bound = service.bind(tenant, actor).unwrap();
            assert_eq!(bound.tenant_id(), tenant);
            assert_eq!(bound.actor_id(), actor);
            assert_eq!(
                bound.analyze(text).await.unwrap(),
                "Actual supplied text analysis"
            );
        }
        let seen = client.seen.lock().unwrap();
        assert_eq!(seen.len(), 2);
        for (request, text) in seen.iter().zip(["Private text A", "Different text B"]) {
            assert_eq!(request.model, "explicit-fixture-model");
            assert_eq!(request.max_tokens, 128);
            assert!(request.tools.is_empty());
            assert_eq!(request.messages.len(), 1);
            assert_eq!(request.messages[0].content, text);
            assert!(request.messages[0].tool_calls.is_empty());
            assert!(request.system.contains("No workspace, memory, tools"));
            assert!(!request.system.contains("tenant-a"));
            assert!(!request.system.contains("actor-a"));
        }
    }
    #[tokio::test]
    async fn tool_requests_and_transport_errors_never_become_success_or_a_second_attempt() {
        for (tool, fail) in [(true, false), (false, true)] {
            let client = Arc::new(ObservedClient {
                seen: Mutex::new(vec![]),
                tool,
                fail,
                stop_reason: "stop",
            });
            let bound = configured(client.clone())
                .bind("tenant-a", "actor-a")
                .unwrap();
            assert!(bound.analyze("Supplied text").await.is_err());
            assert_eq!(client.seen.lock().unwrap().len(), 1);
        }
    }
    #[test]
    fn system_or_missing_authority_cannot_bind_a_tenant_service() {
        let client = Arc::new(ObservedClient {
            seen: Mutex::new(vec![]),
            tool: false,
            fail: false,
            stop_reason: "stop",
        });
        let service = configured(client.clone());
        for (tenant, actor) in [
            ("system", "actor"),
            ("SYSTEM", "actor"),
            ("", "actor"),
            ("tenant", " "),
        ] {
            assert!(service.bind(tenant, actor).is_err());
        }
        assert!(client.seen.lock().unwrap().is_empty());
    }
    #[test]
    fn explicit_configuration_is_required_without_automatic_provider_or_model_fallback() {
        let from = |pairs: &[(&str, &str)]| {
            ConfiguredTextAnalysis::from_values(|name| {
                pairs
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| value.to_string())
            })
        };
        assert!(from(&[]).is_err());
        assert!(from(&[("OMNISOLO_LLM_PROVIDER", "ollama")]).is_err());
        assert!(
            from(&[
                ("OMNISOLO_LLM_PROVIDER", "ollama"),
                ("OMNISOLO_LLM_MODEL", "explicit")
            ])
            .is_err()
        );
        assert!(
            from(&[
                ("OMNISOLO_LLM_PROVIDER", "unknown"),
                ("OMNISOLO_LLM_MODEL", "explicit")
            ])
            .is_err()
        );
        assert!(
            from(&[
                ("OMNISOLO_LLM_PROVIDER", "openai"),
                ("OMNISOLO_LLM_MODEL", "explicit")
            ])
            .is_err()
        );
        assert!(
            from(&[
                ("OMNISOLO_LLM_PROVIDER", "ollama"),
                ("OMNISOLO_LLM_MODEL", "explicit"),
                ("OMNISOLO_LLM_ENDPOINT", "http://127.0.0.1:1/api/chat"),
                ("OMNISOLO_MAX_TOKENS", "99999")
            ])
            .is_err()
        );
        let configured = from(&[
            ("OMNISOLO_LLM_PROVIDER", "ollama"),
            ("OMNISOLO_LLM_MODEL", "explicit"),
            ("OMNISOLO_LLM_ENDPOINT", "http://127.0.0.1:1/api/chat"),
            ("OMNISOLO_MAX_TOKENS", "256"),
        ])
        .unwrap();
        assert_eq!(configured.provider(), "ollama");
        assert_eq!(configured.model(), "explicit");
        assert_eq!(configured.max_output_tokens(), 256);
        // Constructing configuration sends no request and reads no credential file.
    }
    #[tokio::test]
    async fn only_explicit_provider_completion_reasons_certify_complete_text() {
        for (provider, reason, accepted) in [
            ("openai", "stop", true),
            ("openai-compatible", "stop", true),
            ("minimax", "stop", true),
            ("ollama", "stop", true),
            ("anthropic", "end_turn", true),
            ("openai", "length", false),
            ("anthropic", "max_tokens", false),
            ("openai", "tool_calls", false),
            ("anthropic", "tool_use", false),
            ("anthropic", "pause_turn", false),
            ("anthropic", "stop_sequence", false),
            ("openai", "content_filter", false),
            ("ollama", "length", false),
            ("ollama", "", false),
            ("anthropic", "stop", false),
            ("openai-compatible", "unknown", false),
        ] {
            let client = Arc::new(ObservedClient {
                seen: Mutex::new(vec![]),
                tool: false,
                fail: false,
                stop_reason: reason,
            });
            let service = Arc::new(ConfiguredTextAnalysis {
                provider: provider.into(),
                model: "explicit-fixture-model".into(),
                max_output_tokens: 128,
                llm: client.clone(),
            });
            let bound = service.bind("tenant-a", "actor-a").unwrap();
            assert_eq!(
                bound.analyze("Supplied text").await.is_ok(),
                accepted,
                "{provider}: {reason}"
            );
            assert_eq!(
                client.seen.lock().unwrap().len(),
                1,
                "receipt handling must not retry"
            );
        }
    }
}
