use crate::agent::AgentEvent;
use std::sync::{Arc, Mutex};

use std::fmt::Debug;

/// DeerFlow Unique Harness Innovations: Built-in observability (LangSmith and Langfuse integration)
pub trait ObservabilityProvider: Send + Sync + std::fmt::Debug {
    fn log_event(&self, event: &AgentEvent);
}

#[derive(Debug)]
pub struct MockLangSmithProvider {
    pub events: Arc<Mutex<Vec<AgentEvent>>>,
}

impl MockLangSmithProvider {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl ObservabilityProvider for MockLangSmithProvider {
    fn log_event(&self, event: &AgentEvent) {
        let mut events = self.events.lock().unwrap();
        events.push(event.clone());
        // Mock sending to LangSmith API: https://api.smith.langchain.com/runs
    }
}

#[derive(Debug)]
pub struct MockLangfuseProvider {
    pub events: Arc<Mutex<Vec<AgentEvent>>>,
}

impl MockLangfuseProvider {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl ObservabilityProvider for MockLangfuseProvider {
    fn log_event(&self, event: &AgentEvent) {
        let mut events = self.events.lock().unwrap();
        events.push(event.clone());
        // Mock sending to Langfuse API: https://cloud.langfuse.com/api/public/v1/traces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentEvent;

    #[test]
    fn test_langsmith_provider_logging() {
        let provider = MockLangSmithProvider::new();
        let event = AgentEvent::RunStarted { iteration: 1 };
        provider.log_event(&event);

        let events = provider.events.lock().unwrap();
        assert_eq!(events.len(), 1);
        if let AgentEvent::RunStarted { iteration } = events[0] {
            assert_eq!(iteration, 1);
        } else {
            panic!("Expected RunStarted event");
        }
    }

    #[test]
    fn test_langfuse_provider_logging() {
        let provider = MockLangfuseProvider::new();
        let event = AgentEvent::TaskComplete { content: "Done".to_string() };
        provider.log_event(&event);

        let events = provider.events.lock().unwrap();
        assert_eq!(events.len(), 1);
        if let AgentEvent::TaskComplete { content } = &events[0] {
            assert_eq!(content, "Done");
        } else {
            panic!("Expected TaskComplete event");
        }
    }
}
