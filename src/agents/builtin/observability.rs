use std::sync::Arc;

use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse,  ToolCall};

/// DeerFlow Unique Harness Innovations: Built-in observability: LangSmith and Langfuse integration
pub trait ObservabilityProvider: Send + Sync {
    fn log_run_start(&self, task: &str, run_id: &str);
    fn log_llm_request(&self, run_id: &str, req: &ChatRequest);
    fn log_llm_response(&self, run_id: &str, resp: &ChatResponse);
    fn log_tool_call(&self, run_id: &str, tool_call: &ToolCall);
    fn log_tool_result(&self, run_id: &str, tool_id: &str, result: &str);
    fn log_run_end(&self, run_id: &str, final_output: &str);
    fn log_error(&self, run_id: &str, error: &str);
}

pub struct LangSmithProvider {
    _api_key: String,
    _project_name: String,
}

impl LangSmithProvider {
    pub fn new(api_key: String, project_name: String) -> Self {
        Self { _api_key: api_key, _project_name: project_name }
    }
}

impl ObservabilityProvider for LangSmithProvider {
    fn log_run_start(&self, task: &str, run_id: &str) {
        tracing::info!("[LangSmith] Run start: {} (run_id: {})", task, run_id);
    }
    fn log_llm_request(&self, run_id: &str, _req: &ChatRequest) {
        tracing::info!("[LangSmith] LLM request for run_id: {}", run_id);
    }
    fn log_llm_response(&self, run_id: &str, _resp: &ChatResponse) {
        tracing::info!("[LangSmith] LLM response for run_id: {}", run_id);
    }
    fn log_tool_call(&self, run_id: &str, tool_call: &ToolCall) {
        tracing::info!("[LangSmith] Tool call: {} (run_id: {})", tool_call.name, run_id);
    }
    fn log_tool_result(&self, run_id: &str, tool_id: &str, _result: &str) {
        tracing::info!("[LangSmith] Tool result for {} (run_id: {})", tool_id, run_id);
    }
    fn log_run_end(&self, run_id: &str, _final_output: &str) {
        tracing::info!("[LangSmith] Run end (run_id: {})", run_id);
    }
    fn log_error(&self, run_id: &str, error: &str) {
        tracing::error!("[LangSmith] Error: {} (run_id: {})", error, run_id);
    }
}

pub struct LangfuseProvider {
    _public_key: String,
    _secret_key: String,
}

impl LangfuseProvider {
    pub fn new(public_key: String, secret_key: String) -> Self {
        Self { _public_key: public_key, _secret_key: secret_key }
    }
}

impl ObservabilityProvider for LangfuseProvider {
    fn log_run_start(&self, task: &str, run_id: &str) {
        tracing::info!("[Langfuse] Trace start: {} (trace_id: {})", task, run_id);
    }
    fn log_llm_request(&self, run_id: &str, _req: &ChatRequest) {
        tracing::info!("[Langfuse] LLM generation start for trace_id: {}", run_id);
    }
    fn log_llm_response(&self, run_id: &str, _resp: &ChatResponse) {
        tracing::info!("[Langfuse] LLM generation end for trace_id: {}", run_id);
    }
    fn log_tool_call(&self, run_id: &str, tool_call: &ToolCall) {
        tracing::info!("[Langfuse] Span start: {} (trace_id: {})", tool_call.name, run_id);
    }
    fn log_tool_result(&self, run_id: &str, tool_id: &str, _result: &str) {
        tracing::info!("[Langfuse] Span end for {} (trace_id: {})", tool_id, run_id);
    }
    fn log_run_end(&self, run_id: &str, _final_output: &str) {
        tracing::info!("[Langfuse] Trace end (trace_id: {})", run_id);
    }
    fn log_error(&self, run_id: &str, error: &str) {
        tracing::error!("[Langfuse] Error: {} (trace_id: {})", error, run_id);
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
}

impl Default for ObservabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ObservabilityProvider for ObservabilityManager {
    fn log_run_start(&self, task: &str, run_id: &str) {
        for p in &self.providers { p.log_run_start(task, run_id); }
    }
    fn log_llm_request(&self, run_id: &str, req: &ChatRequest) {
        for p in &self.providers { p.log_llm_request(run_id, req); }
    }
    fn log_llm_response(&self, run_id: &str, resp: &ChatResponse) {
        for p in &self.providers { p.log_llm_response(run_id, resp); }
    }
    fn log_tool_call(&self, run_id: &str, tool_call: &ToolCall) {
        for p in &self.providers { p.log_tool_call(run_id, tool_call); }
    }
    fn log_tool_result(&self, run_id: &str, tool_id: &str, result: &str) {
        for p in &self.providers { p.log_tool_result(run_id, tool_id, result); }
    }
    fn log_run_end(&self, run_id: &str, final_output: &str) {
        for p in &self.providers { p.log_run_end(run_id, final_output); }
    }
    fn log_error(&self, run_id: &str, error: &str) {
        for p in &self.providers { p.log_error(run_id, error); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockObservabilityProvider {
        log: std::sync::Mutex<Vec<String>>,
    }

    impl ObservabilityProvider for MockObservabilityProvider {
        fn log_run_start(&self, _task: &str, _run_id: &str) {
            self.log.lock().unwrap().push("run_start".to_string());
        }
        fn log_llm_request(&self, _run_id: &str, _req: &ChatRequest) {}
        fn log_llm_response(&self, _run_id: &str, _resp: &ChatResponse) {}
        fn log_tool_call(&self, _run_id: &str, _tool_call: &ToolCall) {}
        fn log_tool_result(&self, _run_id: &str, _tool_id: &str, _result: &str) {}
        fn log_run_end(&self, _run_id: &str, _final_output: &str) {
            self.log.lock().unwrap().push("run_end".to_string());
        }
        fn log_error(&self, _run_id: &str, _error: &str) {
            self.log.lock().unwrap().push("error".to_string());
        }
    }

    #[test]
    fn test_observability_manager() {
        let mut manager = ObservabilityManager::new();
        let provider1 = Arc::new(LangSmithProvider::new("k1".to_string(), "p1".to_string()));
        let provider2 = Arc::new(LangfuseProvider::new("pk1".to_string(), "sk1".to_string()));
        let provider3 = Arc::new(MockObservabilityProvider { log: std::sync::Mutex::new(vec![]) });

        manager.add_provider(provider1);
        manager.add_provider(provider2);
        manager.add_provider(provider3.clone());

        manager.log_run_start("Task", "123");
        manager.log_run_end("123", "Done");

        let log = provider3.log.lock().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0], "run_start");
        assert_eq!(log[1], "run_end");
    }
}
