use std::sync::Arc;
use tokio::sync::Mutex;
use crate::types::{ChatRequest, ChatResponse, ToolCall, ToolResult};

/// DeerFlow Unique Harness Innovations: Built-in observability
/// LangSmith and Langfuse integration

#[async_trait::async_trait]
pub trait ObservabilityProvider: Send + Sync {
    async fn log_llm_call(&self, req: &ChatRequest, resp: &ChatResponse);
    async fn log_tool_call(&self, call: &ToolCall, result: &ToolResult);
    async fn log_agent_step(&self, step_name: &str, details: &str);
}

pub struct LangSmithProvider {
    pub traces: Arc<Mutex<Vec<String>>>,
}

impl LangSmithProvider {
    pub fn new() -> Self {
        Self {
            traces: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl ObservabilityProvider for LangSmithProvider {
    async fn log_llm_call(&self, req: &ChatRequest, resp: &ChatResponse) {
        let trace = format!("LangSmith: LLM Call - Model: {}, Tokens: {}", req.model, resp.usage.input_tokens + resp.usage.output_tokens);
        tracing::debug!("{}", trace);
        self.traces.lock().await.push(trace);
    }

    async fn log_tool_call(&self, call: &ToolCall, result: &ToolResult) {
        let trace = format!("LangSmith: Tool Call - Name: {}, Error: {}", call.name, !result.error.is_empty());
        tracing::debug!("{}", trace);
        self.traces.lock().await.push(trace);
    }

    async fn log_agent_step(&self, step_name: &str, details: &str) {
        let trace = format!("LangSmith: Step - {}: {}", step_name, details);
        tracing::debug!("{}", trace);
        self.traces.lock().await.push(trace);
    }
}

pub struct LangfuseProvider {
    pub traces: Arc<Mutex<Vec<String>>>,
}

impl LangfuseProvider {
    pub fn new() -> Self {
        Self {
            traces: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl ObservabilityProvider for LangfuseProvider {
    async fn log_llm_call(&self, req: &ChatRequest, resp: &ChatResponse) {
        let trace = format!("Langfuse: Generation - Model: {}, ID: {:?}", req.model, resp.response_id);
        tracing::debug!("{}", trace);
        self.traces.lock().await.push(trace);
    }

    async fn log_tool_call(&self, call: &ToolCall, result: &ToolResult) {
        let trace = format!("Langfuse: Span - Tool: {}, Result Length: {}", call.name, result.content.len());
        tracing::debug!("{}", trace);
        self.traces.lock().await.push(trace);
    }

    async fn log_agent_step(&self, step_name: &str, details: &str) {
        let trace = format!("Langfuse: Trace Step - {}: {}", step_name, details);
        tracing::debug!("{}", trace);
        self.traces.lock().await.push(trace);
    }
}

pub struct ObservabilityManager {
    providers: Vec<Arc<dyn ObservabilityProvider>>,
}

impl ObservabilityManager {
    pub fn new() -> Self {
        Self { providers: Vec::new() }
    }

    pub fn add_provider(&mut self, provider: Arc<dyn ObservabilityProvider>) {
        self.providers.push(provider);
    }

    pub async fn log_llm_call(&self, req: &ChatRequest, resp: &ChatResponse) {
        for provider in &self.providers {
            provider.log_llm_call(req, resp).await;
        }
    }

    pub async fn log_tool_call(&self, call: &ToolCall, result: &ToolResult) {
        for provider in &self.providers {
            provider.log_tool_call(call, result).await;
        }
    }

    pub async fn log_agent_step(&self, step_name: &str, details: &str) {
        for provider in &self.providers {
            provider.log_agent_step(step_name, details).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Usage;

    #[tokio::test]
    async fn test_observability_manager() {
        let mut manager = ObservabilityManager::new();
        let langsmith = Arc::new(LangSmithProvider::new());
        let langfuse = Arc::new(LangfuseProvider::new());

        manager.add_provider(langsmith.clone());
        manager.add_provider(langfuse.clone());

        let req = ChatRequest {
            model: "test-model".to_string(),
            system: "sys".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.1,
        };

        let resp = ChatResponse {
            message: crate::types::Message::assistant("hi"),
            usage: Usage { input_tokens: 10, output_tokens: 10, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
            stop_reason: "stop".to_string(),
            response_id: Some("id1".to_string()),
        };

        manager.log_llm_call(&req, &resp).await;

        let ls_traces = langsmith.traces.lock().await;
        let lf_traces = langfuse.traces.lock().await;

        assert_eq!(ls_traces.len(), 1);
        assert!(ls_traces[0].contains("LangSmith: LLM Call - Model: test-model"));

        assert_eq!(lf_traces.len(), 1);
        assert!(lf_traces[0].contains("Langfuse: Generation - Model: test-model"));
    }
}
