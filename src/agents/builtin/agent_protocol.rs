use crate::codex_runner::Runner;
use std::sync::Arc;

/// AutoGPT Unique Harness Innovations: Agent Protocol
/// Standardization via agentprotocol.ai, gaining cross-framework adoption.

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TaskRequestBody {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Task {
    pub task_id: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Artifact {
    pub artifact_id: String,
    pub file_name: String,
    pub relative_path: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StepRequestBody {
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Step {
    pub task_id: String,
    pub step_id: String,
    pub name: Option<String>,
    pub status: StepStatus,
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_output: Option<serde_json::Value>,
    pub artifacts: Vec<Artifact>,
    pub is_last: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Created,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct AgentProtocolServer {
    pub runner: Arc<Runner>,
    pub tasks: Arc<RwLock<HashMap<String, Task>>>,
    pub steps: Arc<RwLock<HashMap<String, Vec<Step>>>>,
}

impl AgentProtocolServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self {
            runner,
            tasks: Arc::new(RwLock::new(HashMap::new())),
            steps: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// GET /ap/v1/agent/tasks
    pub async fn list_tasks(&self) -> String {
        let tasks_read = self.tasks.read().await;
        let tasks: Vec<Task> = tasks_read.values().cloned().collect();
        serde_json::to_string(&tasks).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// GET /ap/v1/agent/tasks/{task_id}
    pub async fn get_task(&self, task_id: &str) -> String {
        let tasks_read = self.tasks.read().await;
        if let Some(task) = tasks_read.get(task_id) {
            serde_json::to_string(task).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
        } else {
            serde_json::to_string(&ErrorResponse {
                error: format!("Task {} not found", task_id),
            }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
        }
    }

    /// POST /ap/v1/agent/tasks
    pub async fn create_task(&self, req_json: &str) -> String {
        let req: TaskRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_string(&ErrorResponse {
                    error: "Invalid request".to_string(),
                }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        };

        // For simplicity, we just generate a task ID
        let task_id = uuid::Uuid::new_v4().to_string();

        let resp = Task {
            task_id: task_id.clone(),
            input: req.input,
            additional_input: req.additional_input,
            artifacts: vec![],
        };

        let mut tasks_write = self.tasks.write().await;
        tasks_write.insert(task_id, resp.clone());

        serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// GET /ap/v1/agent/tasks/{task_id}/steps
    pub async fn list_steps(&self, task_id: &str) -> String {
        let steps_read = self.steps.read().await;
        if let Some(steps) = steps_read.get(task_id) {
            serde_json::to_string(steps).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
        } else {
            serde_json::to_string(&Vec::<Step>::new()).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
        }
    }

    /// GET /ap/v1/agent/tasks/{task_id}/steps/{step_id}
    pub async fn get_step(&self, task_id: &str, step_id: &str) -> String {
        let steps_read = self.steps.read().await;
        if let Some(steps) = steps_read.get(task_id) {
            if let Some(step) = steps.iter().find(|s| s.step_id == step_id) {
                return serde_json::to_string(step).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        }
        serde_json::to_string(&ErrorResponse {
            error: format!("Step {} not found for task {}", step_id, task_id),
        }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// POST /ap/v1/agent/tasks/{task_id}/steps
    pub async fn execute_step(&self, task_id: &str, req_json: &str) -> String {
        let req: StepRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_string(&ErrorResponse {
                    error: "Invalid request".to_string(),
                }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        };

        let initial_message = req.input.unwrap_or_else(|| "Continue".to_string());
        let _cfg = crate::agent::AgentRunConfig::default();

        let resp = match self.runner.run_async(&initial_message).await {
            Ok(result) => {
                Step {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: None,
                    status: StepStatus::Completed,
                    output: Some(result),
                    additional_output: None,
                    is_last: true,
                    artifacts: vec![],
                }
            }
            Err(e) => {
                Step {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: None,
                    status: StepStatus::Failed,
                    output: Some(e.to_string()),
                    additional_output: None,
                    is_last: true,
                    artifacts: vec![],
                }
            }
        };

        let mut steps_write = self.steps.write().await;
        steps_write.entry(task_id.to_string()).or_insert_with(Vec::new).push(resp.clone());

        serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{Agent};
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("agent protocol success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_agent_protocol_server() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        // Test create task
        let req_json = r#"{"input": "do this task"}"#;
        let resp_json = server.create_task(req_json).await;
        let resp: Task = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(resp.input, "do this task");
        let task_id = resp.task_id;

        // Test execute step
        let step_req = r#"{"input": "step 1"}"#;
        let step_resp_json = server.execute_step(&task_id, step_req).await;
        let step_resp: Step = serde_json::from_str(&step_resp_json).unwrap();

        assert_eq!(step_resp.task_id, task_id);
        assert_eq!(step_resp.output.unwrap(), "agent protocol success");
        assert_eq!(step_resp.status, StepStatus::Completed);
        assert!(step_resp.is_last);
    }

    #[tokio::test]
    async fn test_agent_protocol_create_task_invalid_json() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let req_json = r#"{"input": "do this task", "#; // Invalid JSON
        let resp_json = server.create_task(req_json).await;

        let err_resp: ErrorResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(err_resp.error, "Invalid request");
    }

    #[tokio::test]
    async fn test_agent_protocol_get_task_and_steps() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        // Test create task
        let req_json = r#"{"input": "get task test"}"#;
        let resp_json = server.create_task(req_json).await;
        let created_task: Task = serde_json::from_str(&resp_json).unwrap();

        // Test list_tasks
        let tasks_json = server.list_tasks().await;
        let tasks: Vec<Task> = serde_json::from_str(&tasks_json).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].task_id, created_task.task_id);

        // Test get_task
        let task_json = server.get_task(&created_task.task_id).await;
        let fetched_task: Task = serde_json::from_str(&task_json).unwrap();
        assert_eq!(fetched_task.task_id, created_task.task_id);
        assert_eq!(fetched_task.input, "get task test");

        // Test execute step
        let step_req = r#"{"input": "step 1"}"#;
        let step_resp_json = server.execute_step(&created_task.task_id, step_req).await;
        let executed_step: Step = serde_json::from_str(&step_resp_json).unwrap();

        // Test list_steps
        let steps_json = server.list_steps(&created_task.task_id).await;
        let steps: Vec<Step> = serde_json::from_str(&steps_json).unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].step_id, executed_step.step_id);

        // Test get_step
        let step_json = server.get_step(&created_task.task_id, &executed_step.step_id).await;
        let fetched_step: Step = serde_json::from_str(&step_json).unwrap();
        assert_eq!(fetched_step.step_id, executed_step.step_id);
        assert_eq!(fetched_step.output.unwrap(), "agent protocol success");

        // Test not found
        let err_json = server.get_task("invalid").await;
        let err_resp: ErrorResponse = serde_json::from_str(&err_json).unwrap();
        assert_eq!(err_resp.error, "Task invalid not found");

        let err_step_json = server.get_step(&created_task.task_id, "invalid").await;
        let err_step_resp: ErrorResponse = serde_json::from_str(&err_step_json).unwrap();
        assert_eq!(err_step_resp.error, format!("Step invalid not found for task {}", created_task.task_id));
    }

    #[tokio::test]
    async fn test_agent_protocol_execute_step_invalid_json() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let req_json = r#"{"input": "step 1", "#; // Invalid JSON
        let resp_json = server.execute_step("task-123", req_json).await;

        let err_resp: ErrorResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(err_resp.error, "Invalid request");
    }

    struct FailingMockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for FailingMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("LLM execution failed".into())
        }
    }

    #[tokio::test]
    async fn test_agent_protocol_execute_step_runner_failure() {
        let client = Arc::new(FailingMockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let req_json = r#"{"input": "step 1"}"#;
        let resp_json = server.execute_step("task-123", req_json).await;

        let err_resp: Step = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(err_resp.status, StepStatus::Failed);
        assert!(err_resp.output.unwrap().contains("LLM execution failed"));
    }
}
