use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait SubagentOrchestrator: Send + Sync {
    /// Dispatches a task to a subagent and returns a condensed summary.
    /// Enforces the rule that subagents return 1k-2k token condensed summaries,
    /// never their full context loop.
    async fn dispatch_task(&self, task: &str) -> Result<String, String>;
}

/// A mock client trait to simulate the subagent LLM or service.
#[async_trait]
pub trait SubagentClient: Send + Sync {
    async fn execute(&self, task: &str) -> Result<String, String>;
}

pub struct DelegatingOrchestrator {
    client: Arc<dyn SubagentClient>,
}

impl DelegatingOrchestrator {
    pub fn new(client: Arc<dyn SubagentClient>) -> Self {
        Self { client }
    }

    /// Helper to enforce the 1k-2k token summary rule.
    /// In a real scenario, this might call another LLM prompt to summarize if it's too long.
    /// Here we just truncate mechanically to simulate condensation if it exceeds limits.
    fn condense_summary(raw_output: &str) -> String {
        let max_chars = 8000; // rough approximation for ~2k tokens
        if raw_output.chars().count() > max_chars {
            let truncated: String = raw_output.chars().take(max_chars).collect();
            format!("{}\n\n[Output truncated. Subagent failed to condense summary directly, forced truncation applied.]", truncated)
        } else {
            raw_output.to_string()
        }
    }
}

#[async_trait]
impl SubagentOrchestrator for DelegatingOrchestrator {
    async fn dispatch_task(&self, task: &str) -> Result<String, String> {
        let augmented_task = format!(
            "{}\n\nCRITICAL INSTRUCTION: You are a subagent. When you finish your work, you MUST return a 1k-2k token condensed summary of your findings and actions. NEVER return your full context loop or raw unsummarized output.",
            task
        );

        let raw_result = self.client.execute(&augmented_task).await?;

        // Condense summary to enforce the output constraint
        Ok(Self::condense_summary(&raw_result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSubagentClient {
        response: String,
    }

    #[async_trait]
    impl SubagentClient for MockSubagentClient {
        async fn execute(&self, _task: &str) -> Result<String, String> {
            Ok(self.response.clone())
        }
    }

    #[tokio::test]
    async fn test_delegating_orchestrator_short_summary() {
        let client = Arc::new(MockSubagentClient {
            response: "Short summary of work done.".to_string(),
        });
        let orchestrator = DelegatingOrchestrator::new(client);

        let result = orchestrator.dispatch_task("Do something simple").await.unwrap();
        assert_eq!(result, "Short summary of work done.");
    }

    #[tokio::test]
    async fn test_delegating_orchestrator_long_summary_truncation() {
        let long_response = "a".repeat(9000);
        let client = Arc::new(MockSubagentClient {
            response: long_response,
        });
        let orchestrator = DelegatingOrchestrator::new(client);

        let result = orchestrator.dispatch_task("Do something complex").await.unwrap();
        assert!(result.contains("[Output truncated"));
        assert!(result.len() < 9000);
    }
}
