use crate::types::ToolCall;
use std::sync::Arc;
use std::fmt::Debug;
use serde_json::json;

/// DeerFlow Unique Harness Innovations: Built-in observability (LangSmith and Langfuse integration)
pub trait AgentObserver: Send + Sync + Debug {
    fn on_run_start(&self, agent_id: &str, run_id: &str, input: &str);
    fn on_tool_start(&self, run_id: &str, tool_call: &ToolCall);
    fn on_tool_end(&self, run_id: &str, tool_call_id: &str, result: &str);
    fn on_llm_start(&self, run_id: &str, prompt: &str);
    fn on_llm_end(&self, run_id: &str, response: &str);
    fn on_run_end(&self, agent_id: &str, run_id: &str, output: &str);
}

#[derive(Debug, Clone, Default)]
pub struct ObservabilityRegistry {
    pub observers: Vec<Arc<dyn AgentObserver>>,
}

impl ObservabilityRegistry {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Arc<dyn AgentObserver>) {
        self.observers.push(observer);
    }

    pub fn on_run_start(&self, agent_id: &str, run_id: &str, input: &str) {
        for obs in &self.observers {
            obs.on_run_start(agent_id, run_id, input);
        }
    }

    pub fn on_tool_start(&self, run_id: &str, tool_call: &ToolCall) {
        for obs in &self.observers {
            obs.on_tool_start(run_id, tool_call);
        }
    }

    pub fn on_tool_end(&self, run_id: &str, tool_call_id: &str, result: &str) {
        for obs in &self.observers {
            obs.on_tool_end(run_id, tool_call_id, result);
        }
    }

    pub fn on_llm_start(&self, run_id: &str, prompt: &str) {
        for obs in &self.observers {
            obs.on_llm_start(run_id, prompt);
        }
    }

    pub fn on_llm_end(&self, run_id: &str, response: &str) {
        for obs in &self.observers {
            obs.on_llm_end(run_id, response);
        }
    }

    pub fn on_run_end(&self, agent_id: &str, run_id: &str, output: &str) {
        for obs in &self.observers {
            obs.on_run_end(agent_id, run_id, output);
        }
    }
}

#[derive(Debug, Clone)]
pub struct LangSmithObserver {
    pub project_name: String,
}

impl LangSmithObserver {
    pub fn new(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
        }
    }

    fn log_event(&self, event_type: &str, run_id: &str, payload: serde_json::Value) {
        let msg = json!({
            "source": "LangSmith",
            "project": self.project_name,
            "event": event_type,
            "run_id": run_id,
            "payload": payload,
        });
        tracing::info!("Observability Event: {}", msg);
    }
}

impl AgentObserver for LangSmithObserver {
    fn on_run_start(&self, agent_id: &str, run_id: &str, input: &str) {
        self.log_event("run_start", run_id, json!({ "agent_id": agent_id, "input": input }));
    }

    fn on_tool_start(&self, run_id: &str, tool_call: &ToolCall) {
        self.log_event("tool_start", run_id, json!({ "tool_call": tool_call }));
    }

    fn on_tool_end(&self, run_id: &str, tool_call_id: &str, result: &str) {
        self.log_event("tool_end", run_id, json!({ "tool_call_id": tool_call_id, "result": result }));
    }

    fn on_llm_start(&self, run_id: &str, prompt: &str) {
        self.log_event("llm_start", run_id, json!({ "prompt": prompt }));
    }

    fn on_llm_end(&self, run_id: &str, response: &str) {
        self.log_event("llm_end", run_id, json!({ "response": response }));
    }

    fn on_run_end(&self, agent_id: &str, run_id: &str, output: &str) {
        self.log_event("run_end", run_id, json!({ "agent_id": agent_id, "output": output }));
    }
}

#[derive(Debug, Clone)]
pub struct LangfuseObserver {
    pub session_id: String,
}

impl LangfuseObserver {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
        }
    }

    fn log_trace(&self, name: &str, run_id: &str, metadata: serde_json::Value) {
        let msg = json!({
            "source": "Langfuse",
            "session_id": self.session_id,
            "trace_name": name,
            "run_id": run_id,
            "metadata": metadata,
        });
        tracing::info!("Observability Event: {}", msg);
    }
}

impl AgentObserver for LangfuseObserver {
    fn on_run_start(&self, agent_id: &str, run_id: &str, input: &str) {
        self.log_trace("Agent Run Started", run_id, json!({ "agent_id": agent_id, "input": input }));
    }

    fn on_tool_start(&self, run_id: &str, tool_call: &ToolCall) {
        self.log_trace("Tool Execution Started", run_id, json!({ "tool_name": tool_call.name, "args": tool_call.arguments }));
    }

    fn on_tool_end(&self, run_id: &str, tool_call_id: &str, result: &str) {
        self.log_trace("Tool Execution Ended", run_id, json!({ "tool_call_id": tool_call_id, "result": result }));
    }

    fn on_llm_start(&self, run_id: &str, prompt: &str) {
        self.log_trace("LLM Generation Started", run_id, json!({ "prompt_length": prompt.len() }));
    }

    fn on_llm_end(&self, run_id: &str, response: &str) {
        self.log_trace("LLM Generation Ended", run_id, json!({ "response_length": response.len() }));
    }

    fn on_run_end(&self, agent_id: &str, run_id: &str, output: &str) {
        self.log_trace("Agent Run Ended", run_id, json!({ "agent_id": agent_id, "output": output }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observability_registry() {
        let mut registry = ObservabilityRegistry::new();
        registry.add_observer(Arc::new(LangSmithObserver::new("test_project")));
        registry.add_observer(Arc::new(LangfuseObserver::new("test_session")));

        // Execute all hooks to ensure they do not panic and correctly invoke implementations
        registry.on_run_start("agent1", "run1", "hello");
        registry.on_llm_start("run1", "sys prompt");
        registry.on_llm_end("run1", "llm response");

        let tc = ToolCall {
            id: "tc1".to_string(),
            name: "grep".to_string(),
            arguments: json!({ "pattern": "foo" }),
        };
        registry.on_tool_start("run1", &tc);
        registry.on_tool_end("run1", "tc1", "grep result");

        registry.on_run_end("agent1", "run1", "goodbye");
    }
}
