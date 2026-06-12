use crate::codex_runner::Runner;
use dashmap::DashMap;
use std::sync::{Arc, OnceLock};

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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Pagination {
    pub total_items: usize,
    pub total_pages: usize,
    pub current_page: usize,
    pub page_size: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
    pub pagination: Pagination,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StepListResponse {
    pub steps: Vec<Step>,
    pub pagination: Pagination,
}

// Global in-memory storage for the Agent Protocol (as required by the harness standards)
pub fn tasks_store() -> &'static DashMap<String, Task> {
    static TASKS: OnceLock<DashMap<String, Task>> = OnceLock::new();
    TASKS.get_or_init(DashMap::new)
}

pub fn steps_store() -> &'static DashMap<String, Vec<Step>> {
    static STEPS: OnceLock<DashMap<String, Vec<Step>>> = OnceLock::new();
    STEPS.get_or_init(DashMap::new)
}


pub struct AgentProtocolServer {
    pub runner: Arc<Runner>,
}

impl AgentProtocolServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self { runner }
    }

    /// POST /ap/v1/agent/tasks
    pub async fn create_task(&self, req_json: &str) -> String {
        let req: TaskRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_string(&ErrorResponse {
                    error: "Invalid request".to_string(),
                })
                .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        };

        let task_id = uuid::Uuid::new_v4().to_string();

        let resp = Task {
            task_id: task_id.clone(),
            input: req.input,
            additional_input: req.additional_input,
            artifacts: vec![],
        };

        tasks_store().insert(task_id.clone(), resp.clone());
        steps_store().insert(task_id.clone(), vec![]); // Initialize steps array

        serde_json::to_string(&resp)
            .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// GET /ap/v1/agent/tasks
    pub async fn list_tasks(&self) -> String {
        let tasks: Vec<Task> = tasks_store().iter().map(|kv| kv.value().clone()).collect();
        let total = tasks.len();

        let resp = TaskListResponse {
            tasks,
            pagination: Pagination {
                total_items: total,
                total_pages: 1,
                current_page: 1,
                page_size: std::cmp::max(10, total),
            },
        };
        serde_json::to_string(&resp)
            .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// GET /ap/v1/agent/tasks/{task_id}
    pub async fn get_task(&self, task_id: &str) -> String {
        if let Some(task) = tasks_store().get(task_id) {
            serde_json::to_string(&*task)
                .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
        } else {
            // Query the Checkpointer to see if the task exists and its state.
            let status = if let Some(cp) = &self.runner.core.agent.checkpointer {
                match cp.list_checkpoints(task_id).await {
                    Ok(cps) if !cps.is_empty() => "Running",
                    _ => "Created or Not Found",
                }
            } else {
                "Created or Not Found"
            };

            let resp = Task {
                task_id: task_id.to_string(),
                input: format!("Task state: {}", status),
                additional_input: None,
                artifacts: vec![],
            };
            serde_json::to_string(&resp)
                .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
        }
    }

    /// GET /ap/v1/agent/tasks/{task_id}/steps
    pub async fn list_steps(&self, task_id: &str) -> String {
        let steps = if let Some(steps_ref) = steps_store().get(task_id) {
            steps_ref.clone()
        } else {
            vec![]
        };

        let total = steps.len();
        let resp = StepListResponse {
            steps,
            pagination: Pagination {
                total_items: total,
                total_pages: 1,
                current_page: 1,
                page_size: std::cmp::max(10, total),
            },
        };
        serde_json::to_string(&resp)
            .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// POST /ap/v1/agent/tasks/{task_id}/steps
    pub async fn execute_step(&self, task_id: &str, req_json: &str) -> String {
        let req: StepRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_string(&ErrorResponse {
                    error: "Invalid request".to_string(),
                })
                .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        };

        // Determine input based on req.input or task.input
        let initial_message = if let Some(i) = req.input {
            i
        } else if let Some(task) = tasks_store().get(task_id) {
            task.input.clone()
        } else {
            "Continue".to_string()
        };

        match self.runner.run_async(&initial_message).await {
            Ok(result) => {
                let resp = Step {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: None,
                    status: StepStatus::Completed,
                    output: Some(result),
                    additional_output: None,
                    is_last: true,
                    artifacts: vec![],
                };

                if let Some(mut steps) = steps_store().get_mut(task_id) {
                    steps.push(resp.clone());
                } else {
                    steps_store().insert(task_id.to_string(), vec![resp.clone()]);
                }

                serde_json::to_string(&resp)
                    .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
            }
            Err(e) => {
                let resp = Step {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: None,
                    status: StepStatus::Failed,
                    output: Some(e.to_string()),
                    additional_output: None,
                    is_last: true,
                    artifacts: vec![],
                };

                if let Some(mut steps) = steps_store().get_mut(task_id) {
                    steps.push(resp.clone());
                } else {
                    steps_store().insert(task_id.to_string(), vec![resp.clone()]);
                }

                serde_json::to_string(&resp)
                    .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
            }
        }
    }

    // Test helper to clear state between tests
    #[cfg(test)]
    pub fn reset_state() {
        tasks_store().clear();
        steps_store().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Agent;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
        AgentProtocolServer::reset_state();
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

        // Verify task stored
        let get_task_json = server.get_task(&task_id).await;
        let get_task: Task = serde_json::from_str(&get_task_json).unwrap();
        assert_eq!(get_task.input, "do this task");

        // Test execute step
        let step_req = r#"{"input": "step 1"}"#;
        let step_resp_json = server.execute_step(&task_id, step_req).await;
        let step_resp: Step = serde_json::from_str(&step_resp_json).unwrap();

        assert_eq!(step_resp.task_id, task_id);
        assert_eq!(step_resp.output.unwrap(), "agent protocol success");
        assert_eq!(step_resp.status, StepStatus::Completed);
        assert!(step_resp.is_last);

        // Verify step stored
        let list_steps_json = server.list_steps(&task_id).await;
        let list_steps: StepListResponse = serde_json::from_str(&list_steps_json).unwrap();
        assert_eq!(list_steps.steps.len(), 1);
        assert_eq!(list_steps.steps[0].step_id, step_resp.step_id);
    }

    #[tokio::test]
    async fn test_agent_protocol_list_tasks() {
        AgentProtocolServer::reset_state();
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        server.create_task(r#"{"input": "task 1"}"#).await;
        server.create_task(r#"{"input": "task 2"}"#).await;

        let resp_json = server.list_tasks().await;
        let resp: TaskListResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(resp.tasks.len(), 2);
        assert_eq!(resp.pagination.total_items, 2);
    }

    #[tokio::test]
    async fn test_agent_protocol_list_steps() {
        AgentProtocolServer::reset_state();
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let task_json = server.create_task(r#"{"input": "task 1"}"#).await;
        let task: Task = serde_json::from_str(&task_json).unwrap();

        server.execute_step(&task.task_id, r#"{"input": "step 1"}"#).await;

        let resp_json = server.list_steps(&task.task_id).await;
        let resp: StepListResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(resp.steps.len(), 1);
        assert_eq!(resp.pagination.total_items, 1);
    }

    #[tokio::test]
    async fn test_agent_protocol_get_task() {
        AgentProtocolServer::reset_state();
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let resp_json = server.get_task("task-123").await;
        let resp: Task = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(resp.task_id, "task-123");
        assert!(resp.input.contains("Task state: Created or Not Found"));
    }

    #[tokio::test]
    async fn test_agent_protocol_create_task_invalid_json() {
        AgentProtocolServer::reset_state();
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
    async fn test_agent_protocol_execute_step_invalid_json() {
        AgentProtocolServer::reset_state();
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
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("LLM execution failed".into())
        }
    }

    #[tokio::test]
    async fn test_agent_protocol_execute_step_runner_failure() {
        AgentProtocolServer::reset_state();
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
