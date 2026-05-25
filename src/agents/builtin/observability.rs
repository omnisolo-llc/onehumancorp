use std::fmt::Debug;

/// DeerFlow Unique Harness Innovations: Built-in observability
/// LangSmith and Langfuse integration.

pub trait AgentObserver: Send + Sync + Debug {
    fn on_run_started(&self, run_id: &str, task: &str);
    fn on_llm_call(&self, run_id: &str, prompt: &str);
    fn on_llm_response(&self, run_id: &str, response: &str);
    fn on_tool_call(&self, run_id: &str, tool_name: &str, args: &str);
    fn on_tool_response(&self, run_id: &str, tool_name: &str, result: &str);
    fn on_run_completed(&self, run_id: &str, output: &str);
    fn on_run_error(&self, run_id: &str, error: &str);
}

#[derive(Debug, Clone)]
pub struct LangfuseObserver {
    pub api_key: String,
    pub host: String,
}

impl LangfuseObserver {
    pub fn new(api_key: String, host: String) -> Self {
        Self { api_key, host }
    }
}

impl AgentObserver for LangfuseObserver {
    fn on_run_started(&self, run_id: &str, task: &str) {
        tracing::info!("[Langfuse] Run started: {} - Task: {}", run_id, task);
    }
    fn on_llm_call(&self, run_id: &str, prompt: &str) {
        tracing::info!("[Langfuse] LLM Call for run {}: {}", run_id, prompt);
    }
    fn on_llm_response(&self, run_id: &str, response: &str) {
        tracing::info!("[Langfuse] LLM Response for run {}: {}", run_id, response);
    }
    fn on_tool_call(&self, run_id: &str, tool_name: &str, args: &str) {
        tracing::info!("[Langfuse] Tool Call {}: {} args: {}", run_id, tool_name, args);
    }
    fn on_tool_response(&self, run_id: &str, tool_name: &str, result: &str) {
        tracing::info!("[Langfuse] Tool Response {}: {} result: {}", run_id, tool_name, result);
    }
    fn on_run_completed(&self, run_id: &str, output: &str) {
        tracing::info!("[Langfuse] Run completed: {} - Output: {}", run_id, output);
    }
    fn on_run_error(&self, run_id: &str, error: &str) {
        tracing::error!("[Langfuse] Run error: {} - Error: {}", run_id, error);
    }
}

#[derive(Debug, Clone)]
pub struct LangSmithObserver {
    pub api_key: String,
    pub project_name: String,
}

impl LangSmithObserver {
    pub fn new(api_key: String, project_name: String) -> Self {
        Self { api_key, project_name }
    }
}

impl AgentObserver for LangSmithObserver {
    fn on_run_started(&self, run_id: &str, task: &str) {
        tracing::info!("[LangSmith] Project {} - Run started: {} - Task: {}", self.project_name, run_id, task);
    }
    fn on_llm_call(&self, run_id: &str, prompt: &str) {
        tracing::info!("[LangSmith] Project {} - LLM Call {}: {}", self.project_name, run_id, prompt);
    }
    fn on_llm_response(&self, run_id: &str, response: &str) {
        tracing::info!("[LangSmith] Project {} - LLM Response {}: {}", self.project_name, run_id, response);
    }
    fn on_tool_call(&self, run_id: &str, tool_name: &str, args: &str) {
        tracing::info!("[LangSmith] Project {} - Tool Call {}: {} args: {}", self.project_name, run_id, tool_name, args);
    }
    fn on_tool_response(&self, run_id: &str, tool_name: &str, result: &str) {
        tracing::info!("[LangSmith] Project {} - Tool Response {}: {} result: {}", self.project_name, run_id, tool_name, result);
    }
    fn on_run_completed(&self, run_id: &str, output: &str) {
        tracing::info!("[LangSmith] Project {} - Run completed: {} - Output: {}", self.project_name, run_id, output);
    }
    fn on_run_error(&self, run_id: &str, error: &str) {
        tracing::error!("[LangSmith] Project {} - Run error: {} - Error: {}", self.project_name, run_id, error);
    }
}
