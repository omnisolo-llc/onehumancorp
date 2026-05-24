use crate::agent::AgentEvent;
use std::sync::{Arc, Mutex};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tracing::{info, debug};

/// DeerFlow Unique Harness Innovations: Built-in observability
/// LangSmith and Langfuse integration
pub trait Observer: Send + Sync {
    fn observe(&self, event: &AgentEvent);
}

/// A mock LangSmith observer for the Built-in Observability harness feature.
pub struct LangSmithObserver {
    pub api_key: String,
    pub project_name: String,
    // In a real implementation this might be an HTTP client sending traces.
    // For our harness, we capture traces in memory to verify they are recorded.
    pub traces: Arc<Mutex<Vec<Value>>>,
}

impl LangSmithObserver {
    pub fn new(api_key: String, project_name: String) -> Self {
        Self {
            api_key,
            project_name,
            traces: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Observer for LangSmithObserver {
    fn observe(&self, event: &AgentEvent) {
        let trace_payload = match event {
            AgentEvent::RunStarted { iteration } => json!({ "type": "run_started", "iteration": iteration }),
            AgentEvent::ToolCall { name, args_json, result, iteration } => json!({
                "type": "tool_call",
                "name": name,
                "args": args_json,
                "result": result,
                "iteration": iteration
            }),
            AgentEvent::TaskComplete { content } => json!({ "type": "task_complete", "content": content }),
            AgentEvent::TaskError { error } => json!({ "type": "task_error", "error": error }),
            _ => json!({ "type": "other" }),
        };

        if trace_payload["type"] != "other" {
            debug!("LangSmithObserver: Recording trace -> {}", trace_payload);
            self.traces.lock().unwrap().push(trace_payload);
        }
    }
}

/// A mock Langfuse observer for the Built-in Observability harness feature.
pub struct LangfuseObserver {
    pub public_key: String,
    pub secret_key: String,
    pub traces: Arc<Mutex<Vec<Value>>>,
}

impl LangfuseObserver {
    pub fn new(public_key: String, secret_key: String) -> Self {
        Self {
            public_key,
            secret_key,
            traces: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Observer for LangfuseObserver {
    fn observe(&self, event: &AgentEvent) {
        let trace_payload = match event {
            AgentEvent::IterationStarted { iteration, message_count } => json!({
                "type": "iteration_started",
                "iteration": iteration,
                "message_count": message_count
            }),
            AgentEvent::ToolCall { name, args_json, result, iteration } => json!({
                "type": "tool_call",
                "name": name,
                "args": args_json,
                "result": result,
                "iteration": iteration
            }),
            AgentEvent::TaskComplete { content } => json!({ "type": "task_complete", "content": content }),
            AgentEvent::TaskError { error } => json!({ "type": "task_error", "error": error }),
            _ => json!({ "type": "other" }),
        };

        if trace_payload["type"] != "other" {
            debug!("LangfuseObserver: Recording trace -> {}", trace_payload);
            self.traces.lock().unwrap().push(trace_payload);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentEvent;

    #[test]
    fn test_langsmith_observer() {
        let observer = LangSmithObserver::new("test_key".to_string(), "test_project".to_string());

        observer.observe(&AgentEvent::RunStarted { iteration: 1 });
        observer.observe(&AgentEvent::ToolCall {
            name: "test_tool".to_string(),
            args_json: "{}".to_string(),
            result: "success".to_string(),
            iteration: 1,
        });

        let traces = observer.traces.lock().unwrap();
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0]["type"], "run_started");
        assert_eq!(traces[1]["type"], "tool_call");
        assert_eq!(traces[1]["name"], "test_tool");
    }

    #[test]
    fn test_langfuse_observer() {
        let observer = LangfuseObserver::new("pub_key".to_string(), "sec_key".to_string());

        observer.observe(&AgentEvent::IterationStarted { iteration: 1, message_count: 5 });
        observer.observe(&AgentEvent::TaskComplete { content: "done".to_string() });

        let traces = observer.traces.lock().unwrap();
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0]["type"], "iteration_started");
        assert_eq!(traces[1]["type"], "task_complete");
        assert_eq!(traces[1]["content"], "done");
    }
}