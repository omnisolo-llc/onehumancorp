use crate::codex_runner::Runner;
use std::sync::Arc;

/// AutoGPT Unique Harness Innovations: Agent Protocol
/// Standardization via agentprotocol.ai, gaining cross-framework adoption.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Artifact {
    pub artifact_id: String,
    pub agent_created: bool,
    pub file_name: String,
    pub relative_path: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskRequestBody {
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub task_id: String,
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StepRequestBody {
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Step {
    pub task_id: String,
    pub step_id: String,
    pub name: Option<String>,
    pub status: String,
    pub output: Option<String>,
    pub additional_output: Option<serde_json::Value>,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    pub is_last: bool,
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
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
            Err(_) => return r#"{"error": "Invalid request"}"#.to_string(),
        };

        // For simplicity, we just generate a task ID
        let task_id = uuid::Uuid::new_v4().to_string();

        let resp = Task {
            task_id,
            input: req.input,
            additional_input: req.additional_input,
            artifacts: vec![],
        };

        serde_json::to_string(&resp).unwrap()
    }

    /// POST /ap/v1/agent/tasks/{task_id}/steps
    pub async fn execute_step(&self, task_id: &str, req_json: &str) -> String {
        let req: StepRequestBody = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => return r#"{"error": "Invalid request"}"#.to_string(),
        };

        let initial_message = req.input.clone().unwrap_or_else(|| "Continue".to_string());
        let cfg = crate::agent::AgentRunConfig::default();

        match self.runner.run_async(&cfg, &initial_message).await {
            Ok(result) => {
                let resp = Step {
                    task_id: task_id.to_string(),
                    step_id: uuid::Uuid::new_v4().to_string(),
                    name: Some("execute_step".to_string()),
                    status: "completed".to_string(),
                    output: Some(result),
                    additional_output: None,
                    artifacts: vec![],
                    is_last: true,
                    input: req.input,
                    additional_input: req.additional_input,
                };
                serde_json::to_string(&resp).unwrap()
            }
            Err(e) => {
                format!(r#"{{"error": "{}"}}"#, e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{Agent, AgentRunConfig};
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
        assert_eq!(resp.input.unwrap(), "do this task");
        let task_id = resp.task_id;

        // Test execute step
        let step_req = r#"{"input": "step 1"}"#;
        let step_resp_json = server.execute_step(&task_id, step_req).await;
        let step_resp: Step = serde_json::from_str(&step_resp_json).unwrap();

        assert_eq!(step_resp.task_id, task_id);
        assert_eq!(step_resp.output.unwrap(), "agent protocol success");
        assert_eq!(step_resp.status, "completed");
        assert!(step_resp.is_last);
    }
}
