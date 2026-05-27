use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::future::join_all;
use crate::agent::{Agent, AgentRunConfig};

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
///
/// The `CloudDeploymentManager` handles deploying many instances of an agent concurrently
/// to process a massive batch of tasks, mimicking a 1000+ agent cloud deployment.
pub struct CloudDeploymentManager {
    agent_blueprint: Arc<Agent>,
    config: AgentRunConfig,
    concurrency_limit: usize,
}

#[derive(Debug, Clone)]
pub struct AgentTaskResult {
    pub task_id: String,
    pub output: Result<String, String>,
}

#[derive(Debug, Clone)]
pub struct CloudDeploymentReport {
    pub total_tasks: usize,
    pub successful_tasks: usize,
    pub failed_tasks: usize,
    pub results: Vec<AgentTaskResult>,
}

impl CloudDeploymentManager {
    pub fn new(agent_blueprint: Arc<Agent>, config: AgentRunConfig, concurrency_limit: usize) -> Self {
        Self {
            agent_blueprint,
            config,
            concurrency_limit,
        }
    }

    /// Deploy the agents to process the given tasks concurrently.
    /// This scales up the basic LLM wrapper into a parallel processing engine.
    pub async fn deploy(&self, tasks: Vec<(String, String)>) -> CloudDeploymentReport {
        let semaphore = Arc::new(Semaphore::new(self.concurrency_limit));
        let mut futures = Vec::with_capacity(tasks.len());

        for (task_id, task_prompt) in tasks {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let agent = self.agent_blueprint.clone();
            let config = self.config.clone();

            let fut = tokio::spawn(async move {
                let _permit = permit; // holds the permit until the task is complete
                let mut on_event = |_| {};

                let output = match agent.run(&config, &task_prompt, &mut on_event).await {
                    Ok(res) => Ok(res),
                    Err(e) => Err(format!("Agent failed: {}", e)),
                };

                AgentTaskResult { task_id, output }
            });

            futures.push(fut);
        }

        let completed_tasks = join_all(futures).await;

        let mut results = Vec::new();
        let mut successful_tasks = 0;
        let mut failed_tasks = 0;

        for join_result in completed_tasks {
            match join_result {
                Ok(agent_result) => {
                    if agent_result.output.is_ok() {
                        successful_tasks += 1;
                    } else {
                        failed_tasks += 1;
                    }
                    results.push(agent_result);
                }
                Err(e) => {
                    // Task panicked or was cancelled
                    failed_tasks += 1;
                    results.push(AgentTaskResult {
                        task_id: "unknown".to_string(), // In a real system, we'd wrap the future to catch the ID
                        output: Err(format!("Task execution panicked or was cancelled: {}", e)),
                    });
                }
            }
        }

        CloudDeploymentReport {
            total_tasks: results.len(),
            successful_tasks,
            failed_tasks,
            results,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};

    struct ScalableMockLlm {
        success: bool,
    }

    #[async_trait::async_trait]
    impl LlmClient for ScalableMockLlm {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            if self.success {
                let msg_content = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
                Ok(ChatResponse {
                    message: Message::assistant(format!("Completed: {}", msg_content)),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            } else {
                Err("Simulated cloud agent failure".into())
            }
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_cloud_deployment_success() {
        let llm = Arc::new(ScalableMockLlm { success: true });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let manager = CloudDeploymentManager::new(agent, config, 100);

        let mut tasks = Vec::new();
        // Simulate a 1000+ agent cloud deployment
        for i in 0..1005 {
            tasks.push((format!("task-{}", i), format!("Task payload {}", i)));
        }

        let report = manager.deploy(tasks).await;

        assert_eq!(report.total_tasks, 1005);
        assert_eq!(report.successful_tasks, 1005);
        assert_eq!(report.failed_tasks, 0);

        // Verify some results
        let result_0 = report.results.iter().find(|r| r.task_id == "task-0").unwrap();
        assert_eq!(result_0.output.as_ref().unwrap(), "Completed: Task payload 0");
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_cloud_deployment_partial_failure() {
        // We'll create a mock LLM that fails based on the task content
        struct PartialMockLlm;

        #[async_trait::async_trait]
        impl LlmClient for PartialMockLlm {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let msg_content = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
                if msg_content.contains("fail") {
                    Err("Simulated failure".into())
                } else {
                    Ok(ChatResponse {
                        message: Message::assistant("Success"),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                }
            }
        }

        let llm = Arc::new(PartialMockLlm);
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let manager = CloudDeploymentManager::new(agent, config, 10);

        let tasks = vec![
            ("t1".to_string(), "good task".to_string()),
            ("t2".to_string(), "this will fail".to_string()),
            ("t3".to_string(), "another good task".to_string()),
        ];

        let report = manager.deploy(tasks).await;

        assert_eq!(report.total_tasks, 3);
        assert_eq!(report.successful_tasks, 2);
        assert_eq!(report.failed_tasks, 1);

        let t2_res = report.results.iter().find(|r| r.task_id == "t2").unwrap();
        assert!(t2_res.output.is_err());
        assert!(t2_res.output.as_ref().unwrap_err().contains("Simulated failure"));
    }
}
