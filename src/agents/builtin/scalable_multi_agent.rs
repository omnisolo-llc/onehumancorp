use crate::agent::{Agent, AgentRunConfig};
use std::sync::Arc;
use tokio::task::JoinSet;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments.
/// Represents the deployment mode of the multi-agent orchestrator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentMode {
    /// Local execution, limited concurrency for single-user CLI.
    CliSingleUser,
    /// Cloud execution, simulating massive concurrency via workers.
    CloudDistributed,
}

pub struct ScalableMultiAgentOrchestrator {
    pub base_agent: Arc<Agent>,
    pub mode: DeploymentMode,
}

impl ScalableMultiAgentOrchestrator {
    pub fn new(base_agent: Arc<Agent>, mode: DeploymentMode) -> Self {
        Self { base_agent, mode }
    }

    /// Spawns a fleet of identical agents to process a batch of independent tasks.
    /// In CLI mode, it throttles concurrency. In Cloud mode, it fans out immediately.
    pub async fn spawn_fleet(
        &self,
        tasks: Vec<String>,
        config: &AgentRunConfig,
    ) -> Result<Vec<String>, String> {
        let max_concurrency = match self.mode {
            DeploymentMode::CliSingleUser => 4, // Typical local concurrency limit
            DeploymentMode::CloudDistributed => 1000, // Represents unbounded scaling in a cluster
        };

        let mut join_set = JoinSet::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrency));
        let mut results = vec![String::new(); tasks.len()];
        let mut error_msg = None;

        for (i, task) in tasks.into_iter().enumerate() {
            let agent = self.base_agent.clone();
            let config_clone = config.clone();
            let sem_clone = semaphore.clone();

            join_set.spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                let mut on_event = |_| {};
                let res = agent.run(&config_clone, &task, &mut on_event).await;
                (i, res)
            });
        }

        while let Some(res) = join_set.join_next().await {
            match res {
                Ok((idx, agent_res)) => match agent_res {
                    Ok(output) => results[idx] = output,
                    Err(e) => {
                        error_msg = Some(format!("Agent {} failed: {}", idx, e));
                        break;
                    }
                },
                Err(e) => {
                    error_msg = Some(format!("Task execution panicked: {}", e));
                    break;
                }
            }
        }

        if let Some(e) = error_msg {
            Err(e)
        } else {
            Ok(results)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockScalableLlm {
        active_calls: Arc<AtomicUsize>,
        max_observed_calls: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockScalableLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let current = self.active_calls.fetch_add(1, Ordering::SeqCst) + 1;

            // Update max observed
            let mut max = self.max_observed_calls.load(Ordering::SeqCst);
            while current > max {
                match self.max_observed_calls.compare_exchange_weak(max, current, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(_) => break,
                    Err(val) => max = val,
                }
            }

            // Simulate some work so concurrency can be measured
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            self.active_calls.fetch_sub(1, Ordering::SeqCst);

            Ok(ChatResponse {
                message: Message::assistant("Task complete"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_cli_mode() {
        let active = Arc::new(AtomicUsize::new(0));
        let max_obs = Arc::new(AtomicUsize::new(0));

        let client = Arc::new(MockScalableLlm {
            active_calls: active.clone(),
            max_observed_calls: max_obs.clone(),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let orchestrator = ScalableMultiAgentOrchestrator::new(agent, DeploymentMode::CliSingleUser);

        let mut tasks = Vec::new();
        for i in 0..20 {
            tasks.push(format!("Task {}", i));
        }

        let config = AgentRunConfig::default();
        let results = orchestrator.spawn_fleet(tasks, &config).await.unwrap();

        assert_eq!(results.len(), 20);
        for res in results {
            assert_eq!(res, "Task complete");
        }

        // In CLI mode, max concurrency is 4
        assert!(max_obs.load(Ordering::SeqCst) <= 4, "Observed concurrency: {}", max_obs.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_cloud_mode() {
        let active = Arc::new(AtomicUsize::new(0));
        let max_obs = Arc::new(AtomicUsize::new(0));

        let client = Arc::new(MockScalableLlm {
            active_calls: active.clone(),
            max_observed_calls: max_obs.clone(),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let orchestrator = ScalableMultiAgentOrchestrator::new(agent, DeploymentMode::CloudDistributed);

        let mut tasks = Vec::new();
        for i in 0..20 {
            tasks.push(format!("Task {}", i));
        }

        let config = AgentRunConfig::default();
        let results = orchestrator.spawn_fleet(tasks, &config).await.unwrap();

        assert_eq!(results.len(), 20);

        // In Cloud mode, it should fire them all at once (since tasks=20 and max=1000)
        // Note: It might not perfectly hit 20 depending on scheduler, but it should be > 4
        assert!(max_obs.load(Ordering::SeqCst) > 4, "Observed concurrency: {}", max_obs.load(Ordering::SeqCst));
    }
}
