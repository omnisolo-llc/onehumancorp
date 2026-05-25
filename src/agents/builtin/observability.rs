use std::sync::Arc;
use tokio::sync::Mutex;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, ToolCall, ToolResult};

/// DeerFlow Unique Harness Innovations: Built-in observability
/// LangSmith and Langfuse integration for tracing LLM execution, tool calls, and run boundaries.
#[async_trait::async_trait]
pub trait ObservabilityProvider: Send + Sync + std::fmt::Debug {
    async fn log_run_start(&self, initial_message: &str) -> Result<(), String>;
    async fn log_run_end(&self, final_output: &str) -> Result<(), String>;
    async fn log_llm_start(&self, request: &ChatRequest) -> Result<(), String>;
    async fn log_llm_end(&self, response: &ChatResponse) -> Result<(), String>;
    async fn log_tool_start(&self, tool_call: &ToolCall) -> Result<(), String>;
    async fn log_tool_end(&self, tool_result: &ToolResult) -> Result<(), String>;
}

#[derive(Debug)]
pub struct LangSmithProvider {
    // In a real implementation, this would hold API keys, endpoints, and background workers.
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl LangSmithProvider {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl ObservabilityProvider for LangSmithProvider {
    async fn log_run_start(&self, initial_message: &str) -> Result<(), String> {
        self.logs.lock().await.push(format!("[LangSmith] Run Started: {}", initial_message));
        Ok(())
    }

    async fn log_run_end(&self, final_output: &str) -> Result<(), String> {
        self.logs.lock().await.push(format!("[LangSmith] Run Ended: {}", final_output));
        Ok(())
    }

    async fn log_llm_start(&self, request: &ChatRequest) -> Result<(), String> {
        self.logs.lock().await.push(format!("[LangSmith] LLM Start: model={}", request.model));
        Ok(())
    }

    async fn log_llm_end(&self, response: &ChatResponse) -> Result<(), String> {
        self.logs.lock().await.push(format!("[LangSmith] LLM End: stop_reason={}", response.stop_reason));
        Ok(())
    }

    async fn log_tool_start(&self, tool_call: &ToolCall) -> Result<(), String> {
        self.logs.lock().await.push(format!("[LangSmith] Tool Start: name={}", tool_call.name));
        Ok(())
    }

    async fn log_tool_end(&self, tool_result: &ToolResult) -> Result<(), String> {
        self.logs.lock().await.push(format!("[LangSmith] Tool End: id={} error={}", tool_result.tool_call_id, tool_result.error));
        Ok(())
    }
}

#[derive(Debug)]
pub struct LangfuseProvider {
    // In a real implementation, this would hold API keys, endpoints, and background workers.
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
    async fn log_run_start(&self, initial_message: &str) -> Result<(), String> {
        self.traces.lock().await.push(format!("[Langfuse] Trace Started: {}", initial_message));
        Ok(())
    }

    async fn log_run_end(&self, final_output: &str) -> Result<(), String> {
        self.traces.lock().await.push(format!("[Langfuse] Trace Ended: {}", final_output));
        Ok(())
    }

    async fn log_llm_start(&self, request: &ChatRequest) -> Result<(), String> {
        self.traces.lock().await.push(format!("[Langfuse] Generation Start: model={}", request.model));
        Ok(())
    }

    async fn log_llm_end(&self, response: &ChatResponse) -> Result<(), String> {
        self.traces.lock().await.push(format!("[Langfuse] Generation End: usage={:?}", response.usage));
        Ok(())
    }

    async fn log_tool_start(&self, tool_call: &ToolCall) -> Result<(), String> {
        self.traces.lock().await.push(format!("[Langfuse] Span Start (Tool): {}", tool_call.name));
        Ok(())
    }

    async fn log_tool_end(&self, tool_result: &ToolResult) -> Result<(), String> {
        self.traces.lock().await.push(format!("[Langfuse] Span End (Tool): {}", tool_result.tool_call_id));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{Message, Usage};

    #[tokio::test]
    async fn test_langsmith_provider() {
        let provider = LangSmithProvider::new();
        let _ = provider.log_run_start("Hello").await;

        let logs = provider.logs.lock().await;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "[LangSmith] Run Started: Hello");
    }

    #[tokio::test]
    async fn test_langfuse_provider() {
        let provider = LangfuseProvider::new();
        let req = ChatRequest {
            model: "claude-3".to_string(),
            system: "".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };
        let _ = provider.log_llm_start(&req).await;

        let traces = provider.traces.lock().await;
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0], "[Langfuse] Generation Start: model=claude-3");
    }
}
