use crate::codex_runner::Runner;
use std::sync::Arc;

/// AutoGPT Unique Harness Innovations: Agent Protocol
/// Standardization via agentprotocol.ai, gaining cross-framework adoption.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskRequest {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StepRequest {
    pub input: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StepResponse {
    pub task_id: String,
    pub step_id: String,
    pub output: String,
    pub is_last: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String,
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
        let req: TaskRequest = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_string(&ErrorResponse {
                    error: "Invalid request".to_string(),
                }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        };

        // For simplicity, we just generate a task ID
        let task_id = uuid::Uuid::new_v4().to_string();

        let resp = TaskResponse {
            task_id,
            input: req.input,
            additional_input: req.additional_input,
        };

        serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
    }

    /// POST /ap/v1/agent/tasks/{task_id}/steps
    pub async fn execute_step(&self, task_id: &str, req_json: &str) -> String {
        let req: StepRequest = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                return serde_json::to_string(&ErrorResponse {
                    error: "Invalid request".to_string(),
                }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string());
            }
        };

        let initial_message = req.input.unwrap_or_else(|| "Continue".to_string());
        let _cfg = crate::agent::AgentRunConfig::default();

        match self.runner.run_async(&initial_message).await {
            Ok(result) => {
                let resp = StepResponse {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    output: result,
                    is_last: true,
                    artifacts: None,
                };
                serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
            }
            Err(e) => {
                serde_json::to_string(&ErrorResponse {
                    error: e.to_string(),
                }).unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_string())
            }
        }
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
        let resp: TaskResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(resp.input, "do this task");
        let task_id = resp.task_id;

        // Test execute step
        let step_req = r#"{"input": "step 1"}"#;
        let step_resp_json = server.execute_step(&task_id, step_req).await;
        let step_resp: StepResponse = serde_json::from_str(&step_resp_json).unwrap();

        assert_eq!(step_resp.task_id, task_id);
        assert_eq!(step_resp.output, "agent protocol success");
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

        let err_resp: ErrorResponse = serde_json::from_str(&resp_json).unwrap();
        assert!(err_resp.error.contains("LLM execution failed"));
    }
}
