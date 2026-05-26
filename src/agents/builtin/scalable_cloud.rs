use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::agent::{Agent, AgentRunConfig};
use crate::tools::runner::CommandRunner;
use tokio::task::JoinHandle;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments.
/// Represents a cloud cluster or local runner capable of dynamically provisioning agents.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentMode {
    LocalCLI,
    CloudDistributed,
}

#[derive(Debug, Clone)]
pub struct AgentDeployment {
    pub agent_id: String,
    pub task: String,
    pub status: String,
    pub result: Option<String>,
}

pub struct CloudDeploymentManager {
    mode: DeploymentMode,
    deployments: Arc<Mutex<HashMap<String, AgentDeployment>>>,
    agent: Arc<Agent>,
    runner: Arc<dyn CommandRunner>,
}

impl CloudDeploymentManager {
    pub fn new(mode: DeploymentMode, agent: Arc<Agent>, runner: Arc<dyn CommandRunner>) -> Self {
        Self {
            mode,
            deployments: Arc::new(Mutex::new(HashMap::new())),
            agent,
            runner,
        }
    }

    /// Submits a batch of agent tasks to the infrastructure.
    /// In LocalCLI mode, it runs them sequentially or via local thread pool.
    /// In CloudDistributed mode, it simulates dispatching to a Kubernetes or Ray cluster.
    pub async fn submit_jobs(&self, cfg: AgentRunConfig, tasks: Vec<String>) -> Result<Vec<String>, String> {
        let mut deployed_ids = Vec::new();
        let mut lock = self.deployments.lock().await;

        for task in tasks {
            let agent_id = format!("agent-{}", uuid::Uuid::new_v4());

            let deployment = AgentDeployment {
                agent_id: agent_id.clone(),
                task: task.clone(),
                status: "Pending".to_string(),
                result: None,
            };

            lock.insert(agent_id.clone(), deployment);
            deployed_ids.push(agent_id);
        }

        drop(lock);
        match self.mode {
            DeploymentMode::LocalCLI => {
                // Simulate local execution setting status to Running quickly
                for id in &deployed_ids {
                    let id_clone = id.clone();
                    let agent_clone = self.agent.clone();
                    let cfg_clone = cfg.clone();
                    let deps_clone = self.deployments.clone();

                    tokio::spawn(async move {
                        // Mark as running
                        {
                            let mut l = deps_clone.lock().await;
                            if let Some(dep) = l.get_mut(&id_clone) {
                                dep.status = "Running Locally".to_string();
                            }
                        }

                        let task_str = {
                            let l = deps_clone.lock().await;
                            l.get(&id_clone).unwrap().task.clone()
                        };

                        let mut on_event = |_| {};
                        let res = agent_clone.run(&cfg_clone, &task_str, &mut on_event).await;

                        let mut l = deps_clone.lock().await;
                        if let Some(dep) = l.get_mut(&id_clone) {
                            match res {
                                Ok(r) => {
                                    dep.status = "Completed".to_string();
                                    dep.result = Some(r);
                                }
                                Err(e) => {
                                    dep.status = format!("Failed: {}", e);
                                }
                            }
                        }
                    });
                }
            }
            DeploymentMode::CloudDistributed => {
                // Simulate cloud dispatch via API
                for id in &deployed_ids {
                    let id_clone = id.clone();
                    let agent_clone = self.agent.clone();
                    let cfg_clone = cfg.clone();
                    let deps_clone = self.deployments.clone();
                    let runner_clone = self.runner.clone();

                    tokio::spawn(async move {
                        {
                            let mut l = deps_clone.lock().await;
                            if let Some(dep) = l.get_mut(&id_clone) {
                                dep.status = "Provisioning Cloud Container".to_string();
                            }
                        }

                        let task_str = {
                            let l = deps_clone.lock().await;
                            l.get(&id_clone).unwrap().task.clone()
                        };

                        // Use kubectl to spawn a Kubernetes Job for this agent
                        let job_name = format!("agent-job-{}", id_clone);

                        let kubectl_args = [
                            "create", "job", job_name.as_str(),
                            "--image=ohc-agent:latest",
                            "--", "ohc-builtin-agent", "--task", task_str.as_str()
                        ];

                        match runner_clone.run("kubectl", &kubectl_args, None, vec![]).await {
                            Ok(out) if out.status.success() => {
                                {
                                    let mut l = deps_clone.lock().await;
                                    if let Some(dep) = l.get_mut(&id_clone) {
                                        dep.status = "Running in Cloud".to_string();
                                    }
                                }

                                // In a true distributed system, we would poll the cluster.
                                // For this demonstration, we just mark it as remotely running.
                                let mut l = deps_clone.lock().await;
                                if let Some(dep) = l.get_mut(&id_clone) {
                                    dep.status = "Remotely Running".to_string();
                                }
                            }
                            Ok(out) => {
                                let err_str = String::from_utf8_lossy(&out.stderr);
                                let mut l = deps_clone.lock().await;
                                if let Some(dep) = l.get_mut(&id_clone) {
                                    dep.status = format!("Cloud Provisioning Failed: {}", err_str);
                                }
                            }
                            Err(e) => {
                                let mut l = deps_clone.lock().await;
                                if let Some(dep) = l.get_mut(&id_clone) {
                                    dep.status = format!("Cloud Provisioning Failed: {}", e);
                                }
                            }
                        }
                    });
                }
            }
        }

        Ok(deployed_ids)
    }

    pub async fn get_status(&self, agent_id: &str) -> Option<String> {
        let lock = self.deployments.lock().await;
        lock.get(agent_id).map(|d| d.status.clone())
    }

    pub async fn get_result(&self, agent_id: &str) -> Option<String> {
        let mut lock = self.deployments.lock().await;
        lock.get(agent_id).and_then(|d| d.result.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::tools::runner::mock::{MockCommandRunner, mock_output};

    struct MockLlmClientCloud;
    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientCloud {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_local_cli_deployment() {
        let agent = Arc::new(Agent::new(Arc::new(MockLlmClientCloud), vec![]));
        let runner = Arc::new(MockCommandRunner::new());
        let manager = CloudDeploymentManager::new(DeploymentMode::LocalCLI, agent, runner);
        let tasks = vec!["Task 1".to_string(), "Task 2".to_string()];

        let cfg = AgentRunConfig::default();
        let ids = manager.submit_jobs(cfg.clone(), tasks).await.unwrap();
        assert_eq!(ids.len(), 2);

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert_eq!(manager.get_status(&ids[0]).await.unwrap(), "Completed");
        assert_eq!(manager.get_result(&ids[0]).await.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_cloud_distributed_deployment() {
        let agent = Arc::new(Agent::new(Arc::new(MockLlmClientCloud), vec![]));
        let runner = Arc::new(MockCommandRunner::new());
        // For 1500 tasks, we queue up 1500 success responses for the mock runner
        for _ in 0..1500 {
            runner.push_response(Ok(mock_output(0, "job created", "")));
        }
        let manager = CloudDeploymentManager::new(DeploymentMode::CloudDistributed, agent, runner.clone());

        // Simulating 1000+ deployments
        let mut tasks = Vec::new();
        for i in 0..1500 {
            tasks.push(format!("Cloud Task {}", i));
        }

        let cfg = AgentRunConfig::default();
        let ids = manager.submit_jobs(cfg.clone(), tasks).await.unwrap();
        assert_eq!(ids.len(), 1500);

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert_eq!(manager.get_status(&ids[1000]).await.unwrap(), "Remotely Running");
    }
}
