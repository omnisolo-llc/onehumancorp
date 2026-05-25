use std::sync::Arc;
use tokio::sync::Mutex;
use ohc_builtin_agent_core::types::Usage;

/// DeerFlow Unique Harness Innovations: Built-in observability: LangSmith and Langfuse integration.
/// This module implements the observability layer for tracking traces, spans, and LLM calls.

#[derive(Debug, Clone)]
pub struct TraceData {
    pub trace_id: String,
    pub task: String,
}

#[derive(Debug, Clone)]
pub struct SpanData {
    pub span_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct LlmCallData {
    pub span_id: String,
    pub model: String,
    pub usage: Usage,
    pub input: String,
    pub output: String,
}

#[async_trait::async_trait]
pub trait ObservabilityProvider: Send + Sync {
    async fn start_trace(&self, task: &str) -> String; // returns trace_id
    async fn end_trace(&self, trace_id: &str);
    async fn start_span(&self, trace_id: &str, name: &str) -> String; // returns span_id
    async fn end_span(&self, span_id: &str);
    async fn log_llm_call(&self, data: LlmCallData);
}

// Mock provider for testing and default behavior when not connected
pub struct MockObservabilityProvider {
    pub traces: Arc<Mutex<Vec<TraceData>>>,
    pub spans: Arc<Mutex<Vec<SpanData>>>,
    pub calls: Arc<Mutex<Vec<LlmCallData>>>,
}

impl MockObservabilityProvider {
    pub fn new() -> Self {
        Self {
            traces: Arc::new(Mutex::new(Vec::new())),
            spans: Arc::new(Mutex::new(Vec::new())),
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl ObservabilityProvider for MockObservabilityProvider {
    async fn start_trace(&self, task: &str) -> String {
        let trace_id = format!("trace-{}", uuid::Uuid::new_v4());
        let mut traces = self.traces.lock().await;
        traces.push(TraceData {
            trace_id: trace_id.clone(),
            task: task.to_string(),
        });
        trace_id
    }

    async fn end_trace(&self, _trace_id: &str) {
        // Implementation for ending trace
    }

    async fn start_span(&self, _trace_id: &str, name: &str) -> String {
        let span_id = format!("span-{}", uuid::Uuid::new_v4());
        let mut spans = self.spans.lock().await;
        spans.push(SpanData {
            span_id: span_id.clone(),
            name: name.to_string(),
        });
        span_id
    }

    async fn end_span(&self, _span_id: &str) {
        // Implementation for ending span
    }

    async fn log_llm_call(&self, data: LlmCallData) {
        let mut calls = self.calls.lock().await;
        calls.push(data);
    }
}
