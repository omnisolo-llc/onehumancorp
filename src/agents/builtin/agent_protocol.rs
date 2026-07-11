use crate::codex_runner::Runner;
use std::sync::Arc;

/// AutoGPT Unique Harness Innovations: Agent Protocol
/// Standardization via agentprotocol.ai, gaining cross-framework adoption.

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TaskRequestBody {
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Task {
    pub task_id: String,
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Artifact {
    pub artifact_id: String,
    pub agent_created: bool,
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub status: StepStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    Failed, // Added Failed status according to AutoGPT / agentprotocol.ai spec
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
pub struct TaskStepsListResponse {
    pub steps: Vec<Step>,
    pub pagination: Pagination,
}

pub struct AgentProtocolServer {
    pub runner: Arc<Runner>,
    pub artifacts:
        std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, Vec<Artifact>>>>,
}

impl AgentProtocolServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self {
            runner,
            artifacts: std::sync::Arc::new(tokio::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// POST /ap/v1/agent/tasks
    pub async fn create_task(&self, req_json: &str) -> serde_json::Value {
        let req: TaskRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_value(&ErrorResponse {
                    error: "Invalid request".to_string(),
                })
                .unwrap();
            }
        };

        // For simplicity, we just generate a task ID
        let task_id = uuid::Uuid::new_v4().to_string();

        let resp = Task {
            task_id,
            input: req.input,
            additional_input: req.additional_input,
            artifacts: vec![],
        };

        serde_json::to_value(&resp).unwrap()
    }

    /// GET /ap/v1/agent/tasks
    pub async fn list_tasks(&self) -> serde_json::Value {
        let mut tasks = Vec::new();
        if let Some(cp) = &self.runner.core.agent.checkpointer
            && let Ok(threads) = cp.list_threads().await
        {
            for thread_id in threads {
                let status = match cp.list_checkpoints(&thread_id).await {
                    Ok(cps) if !cps.is_empty() => "Running",
                    _ => "Created or Not Found",
                };
                tasks.push(Task {
                    task_id: thread_id.clone(),
                    input: Some(format!("State from checkpoint: {}", status)),
                    additional_input: None,
                    artifacts: vec![],
                });
            }
        }

        let total_items = tasks.len();
        let resp = TaskListResponse {
            tasks,
            pagination: Pagination {
                total_items,
                total_pages: 1,
                current_page: 1,
                page_size: std::cmp::max(total_items, 10),
            },
        };
        serde_json::to_value(&resp).unwrap()
    }

    /// GET /ap/v1/agent/tasks/{task_id}
    pub async fn get_task(&self, task_id: &str) -> serde_json::Value {
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
            input: Some(format!("State from checkpoint: {}", status)),
            additional_input: None,
            artifacts: vec![],
        };
        serde_json::to_value(&resp).unwrap()
    }

    /// GET /ap/v1/agent/tasks/{task_id}/steps
    pub async fn list_steps(&self, task_id: &str) -> serde_json::Value {
        let mut steps = Vec::new();
        if let Some(cp) = &self.runner.core.agent.checkpointer
            && let Ok(checkpoints) = cp.list_checkpoints(task_id).await
        {
            for (i, checkpoint) in checkpoints.into_iter().enumerate() {
                steps.push(Step {
                    task_id: task_id.to_string(),
                    step_id: checkpoint.checkpoint_id.clone(),
                    name: Some(format!("Step {}", i + 1)),
                    status: StepStatus::Completed,
                    output: Some("Completed step from checkpoint".to_string()),
                    additional_output: Some(checkpoint.data),
                    artifacts: vec![],
                    is_last: i == 0, // Since checkpoints are usually sorted DESC
                });
            }
        }

        let total_items = steps.len();
        let resp = TaskStepsListResponse {
            steps,
            pagination: Pagination {
                total_items,
                total_pages: 1,
                current_page: 1,
                page_size: std::cmp::max(total_items, 10),
            },
        };
        serde_json::to_value(&resp).unwrap()
    }

    /// GET /ap/v1/agent/tasks/{task_id}/steps/{step_id}
    pub async fn get_step(&self, task_id: &str, step_id: &str) -> serde_json::Value {
        if let Some(cp) = &self.runner.core.agent.checkpointer
            && let Ok(Some(checkpoint)) = cp.get_checkpoint(task_id, step_id).await
        {
            let step = Step {
                task_id: task_id.to_string(),
                step_id: checkpoint.checkpoint_id.clone(),
                name: None,
                status: StepStatus::Completed,
                output: Some("Completed step from checkpoint".to_string()),
                additional_output: Some(checkpoint.data),
                artifacts: vec![],
                is_last: true, // simplified
            };
            return serde_json::to_value(&step).unwrap();
        }

        serde_json::to_value(&ErrorResponse {
            error: "Step not found".to_string(),
        })
        .unwrap()
    }

    /// POST /ap/v1/agent/tasks/{task_id}/artifacts
    pub async fn upload_artifact(
        &self,
        task_id: &str,
        file_name: &str,
        content: &[u8],
    ) -> serde_json::Value {
        let artifact_id = uuid::Uuid::new_v4().to_string();
        let artifact_dir = std::path::Path::new("/tmp/agent_protocol_artifacts").join(task_id);
        let _ = tokio::fs::create_dir_all(&artifact_dir).await;
        let file_path = artifact_dir.join(file_name);

        if let Err(e) = tokio::fs::write(&file_path, content).await {
            return serde_json::to_value(&ErrorResponse {
                error: format!("Failed to write artifact to disk: {}", e),
            })
            .unwrap();
        }

        let artifact = Artifact {
            artifact_id,
            agent_created: false,
            file_name: file_name.to_string(),
            relative_path: Some(file_path.to_string_lossy().to_string()),
        };

        let mut map = self.artifacts.lock().await;
        map.entry(task_id.to_string())
            .or_insert_with(Vec::new)
            .push(artifact.clone());

        serde_json::to_value(&artifact).unwrap()
    }

    /// GET /ap/v1/agent/tasks/{task_id}/artifacts
    pub async fn list_artifacts(&self, task_id: &str) -> serde_json::Value {
        let map = self.artifacts.lock().await;
        let list = map.get(task_id).cloned().unwrap_or_default();
        let resp = serde_json::json!({
            "artifacts": list,
            "pagination": {
                "total_items": list.len(),
                "total_pages": 1,
                "current_page": 1,
                "page_size": 100
            }
        });
        resp
    }

    /// GET /ap/v1/agent/tasks/{task_id}/artifacts/{artifact_id}
    pub async fn get_artifact(&self, task_id: &str, artifact_id: &str) -> serde_json::Value {
        let map = self.artifacts.lock().await;
        if let Some(list) = map.get(task_id)
            && let Some(artifact) = list.iter().find(|a| a.artifact_id == artifact_id)
        {
            return serde_json::to_value(artifact).unwrap();
        }
        serde_json::to_value(&ErrorResponse {
            error: "Artifact not found".to_string(),
        })
        .unwrap()
    }

    /// GET /ap/v1/agent/tasks/{task_id}/artifacts/{artifact_id}/content
    pub async fn list_checkpoints(&self, task_id: &str) -> serde_json::Value {
        if let Some(cp) = &self.runner.core.agent.checkpointer {
            match cp.list_checkpoints(task_id).await {
                Ok(checkpoints) => {
                    let mut cp_values = Vec::new();
                    for c in checkpoints {
                        cp_values.push(serde_json::json!({
                            "checkpoint_id": c.checkpoint_id,
                            "parent_id": c.parent_id,
                            "created_at": c.created_at.to_rfc3339()
                        }));
                    }
                    serde_json::json!({ "checkpoints": cp_values })
                }
                Err(e) => serde_json::json!({ "error": format!("Failed to list checkpoints: {}", e) }),
            }
        } else {
            serde_json::json!({ "error": "Checkpointer not configured" })
        }
    }

    pub async fn restore_checkpoint(&self, _task_id: &str, req_json: &str) -> serde_json::Value {
        let req: serde_json::Value = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => return serde_json::json!({ "error": "Invalid request" }),
        };

        let checkpoint_id = match req.get("checkpoint_id").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return serde_json::json!({ "error": "checkpoint_id is required" }),
        };

        if let Some(cp) = &self.runner.core.agent.checkpointer {
            match cp.restore_checkpoint(checkpoint_id).await {
                Ok(_) => serde_json::json!({ "success": true, "message": format!("Restored to checkpoint {}", checkpoint_id) }),
                Err(e) => serde_json::json!({ "error": format!("Failed to restore checkpoint: {}", e) }),
            }
        } else {
            serde_json::json!({ "error": "Checkpointer not configured" })
        }
    }

    pub async fn download_artifact(
        &self,
        task_id: &str,
        artifact_id: &str,
    ) -> Result<Vec<u8>, String> {
        let map = self.artifacts.lock().await;
        let artifact = map
            .get(task_id)
            .and_then(|list| list.iter().find(|a| a.artifact_id == artifact_id))
            .cloned();

        drop(map); // drop the lock before file IO

        if let Some(a) = artifact
            && let Some(path) = &a.relative_path
        {
            return tokio::fs::read(path)
                .await
                .map_err(|e| format!("Failed to read file: {}", e));
        }

        Err("Artifact not found".to_string())
    }

    /// POST /ap/v1/agent/tasks/{task_id}/steps
    pub async fn execute_step(&self, task_id: &str, req_json: &str) -> serde_json::Value {
        let req: StepRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_value(&ErrorResponse {
                    error: "Invalid request".to_string(),
                })
                .unwrap();
            }
        };

        let initial_message = req.input.unwrap_or_else(|| "Continue".to_string());
        let _cfg = crate::agent::AgentRunConfig::default();

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
                serde_json::to_value(&resp).unwrap()
            }
            Err(e) => {
                let resp = Step {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: None,
                    status: StepStatus::Failed, // Using standard Failed status for errors
                    output: Some(format!("Error: {}", e)),
                    additional_output: None,
                    is_last: true,
                    artifacts: vec![],
                };
                serde_json::to_value(&resp).unwrap()
            }
        }
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
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        // Test create task
        let req_json = r#"{"input": "do this task"}"#;
        let resp_json = server.create_task(req_json).await;
        let resp: Task = serde_json::from_value(resp_json).unwrap();
        assert_eq!(resp.input, Some("do this task".to_string()));
        let task_id = resp.task_id;

        // Test execute step
        let step_req = r#"{"input": "step 1"}"#;
        let step_resp_json = server.execute_step(&task_id, step_req).await;
        let step_resp: Step = serde_json::from_value(step_resp_json).unwrap();

        assert_eq!(step_resp.task_id, task_id);
        assert_eq!(step_resp.output.unwrap(), "agent protocol success");
        assert_eq!(step_resp.status, StepStatus::Completed);
        assert!(step_resp.is_last);
    }

    #[tokio::test]
    async fn test_agent_protocol_list_tasks() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let resp_json = server.list_tasks().await;
        let resp: TaskListResponse = serde_json::from_value(resp_json).unwrap();
        assert_eq!(resp.tasks.len(), 0);
        assert_eq!(resp.pagination.total_pages, 1);
    }

    #[tokio::test]
    async fn test_agent_protocol_list_steps() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let resp_json = server.list_steps("task-123").await;
        let resp: TaskStepsListResponse = serde_json::from_value(resp_json).unwrap();
        assert_eq!(resp.steps.len(), 0);
        assert_eq!(resp.pagination.total_pages, 1);
    }

    #[tokio::test]
    async fn test_agent_protocol_get_task() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let resp_json = server.get_task("task-123").await;
        let resp: Task = serde_json::from_value(resp_json).unwrap();
        assert_eq!(resp.task_id, "task-123");
        assert!(resp.input.unwrap().contains("State from checkpoint: "));
    }

    #[tokio::test]
    async fn test_agent_protocol_get_step() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let resp_json = server.get_step("task-123", "step-123").await;
        let err_resp: ErrorResponse = serde_json::from_value(resp_json).unwrap();
        assert_eq!(err_resp.error, "Step not found");
    }

    #[tokio::test]
    async fn test_agent_protocol_create_task_invalid_json() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let req_json = r#"{"input": "do this task", "#; // Invalid JSON
        let resp_json = server.create_task(req_json).await;

        let err_resp: ErrorResponse = serde_json::from_value(resp_json).unwrap();
        assert_eq!(err_resp.error, "Invalid request");
    }

    #[tokio::test]
    async fn test_agent_protocol_upload_artifact() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let task_id = "task-artifact-123";
        let resp_json = server
            .upload_artifact(task_id, "test.txt", b"hello world")
            .await;

        let artifact: Artifact = serde_json::from_value(resp_json).unwrap();
        assert_eq!(artifact.file_name, "test.txt");
        assert_eq!(
            artifact.relative_path,
            Some("/tmp/agent_protocol_artifacts/task-artifact-123/test.txt".to_string())
        );
        assert!(!artifact.agent_created);
    }

    #[tokio::test]
    async fn test_agent_protocol_list_artifacts() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let task_id = "task-artifact-123";
        server.upload_artifact(task_id, "test1.txt", b"A").await;
        server.upload_artifact(task_id, "test2.txt", b"B").await;

        let resp_json = server.list_artifacts(task_id).await;

        let artifacts: Vec<Artifact> =
            serde_json::from_value(resp_json["artifacts"].clone()).unwrap();
        assert_eq!(artifacts.len(), 2);
        assert!(artifacts.iter().any(|a| a.file_name == "test1.txt"));
        assert!(artifacts.iter().any(|a| a.file_name == "test2.txt"));
    }

    #[tokio::test]
    async fn test_agent_protocol_get_artifact() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let task_id = "task-artifact-123";
        let upload_resp = server
            .upload_artifact(task_id, "test.txt", b"hello world")
            .await;
        let created_artifact: Artifact = serde_json::from_value(upload_resp).unwrap();

        let get_resp = server
            .get_artifact(task_id, &created_artifact.artifact_id)
            .await;
        let fetched_artifact: Artifact = serde_json::from_value(get_resp).unwrap();

        assert_eq!(fetched_artifact.artifact_id, created_artifact.artifact_id);
        assert_eq!(fetched_artifact.file_name, "test.txt");

        // Test non-existent artifact
        let not_found_resp = server.get_artifact(task_id, "does-not-exist").await;
        let err_resp: ErrorResponse = serde_json::from_value(not_found_resp).unwrap();
        assert_eq!(err_resp.error, "Artifact not found");
    }

    #[tokio::test]
    async fn test_agent_protocol_download_artifact() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let task_id = "task-artifact-1234";
        let upload_resp = server
            .upload_artifact(task_id, "download.txt", b"hello download")
            .await;
        let created_artifact: Artifact = serde_json::from_value(upload_resp).unwrap();

        let content = server
            .download_artifact(task_id, &created_artifact.artifact_id)
            .await
            .unwrap();
        assert_eq!(content, b"hello download");

        let err = server.download_artifact(task_id, "fake_id").await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_agent_protocol_execute_step_invalid_json() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let req_json = r#"{"input": "step 1", "#; // Invalid JSON
        let resp_json = server.execute_step("task-123", req_json).await;

        let err_resp: ErrorResponse = serde_json::from_value(resp_json).unwrap();
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
        let client = Arc::new(FailingMockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AgentProtocolServer::new(runner);

        let req_json = r#"{"input": "step 1"}"#;
        let resp_json = server.execute_step("task-123", req_json).await;

        let err_resp: Step = serde_json::from_value(resp_json).unwrap();
        assert_eq!(err_resp.status, StepStatus::Failed);
        assert!(err_resp.output.unwrap().contains("LLM execution failed"));
    }
}
