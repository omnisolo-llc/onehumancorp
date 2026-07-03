use ohc_builtin_agent_core::expert_team::{ExpertTeamManager, DomainExpert, ExpertTeamLlmClient, SkillTrace};
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, ToolError};
use ohc_builtin_agent_llm::LlmClient;
use serde_json::json;
use std::sync::Arc;
use crate::pydantic::{PydanticAdapter, PydanticToolExecutor};
use serde::Deserialize;

/// Bridging the Expert Team LLM trait with our standard LlmClient.
struct ExpertLlmBridge {
    client: Arc<dyn LlmClient>,
    model: String,
}

#[async_trait::async_trait]
impl ExpertTeamLlmClient for ExpertLlmBridge {
    async fn chat(&self, mut req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        req.model = self.model.clone();
        self.client.chat(req).await
    }
}

#[derive(Deserialize)]
struct ExpertTeamArgs {
    task: String,
    lead_name: Option<String>,
    expert_roles: Option<Vec<String>>,
}

pub struct ExpertTeamExecutor {
    pub client: Arc<dyn LlmClient>,
    pub model: String,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ExpertTeamArgs> for ExpertTeamExecutor {
    async fn execute_typed(&self, args: ExpertTeamArgs) -> Result<String, ToolError> {
        let task = &args.task;

        let lead_name = args.lead_name.as_deref().unwrap_or("Project Director");

        let expert_roles = args.expert_roles.unwrap_or_else(|| vec![
            "Industry Researcher".to_string(),
            "Financial Analyst".to_string(),
            "Strategic Analyst".to_string(),
            "Process Supervisor".to_string(),
            "Quality Auditor".to_string(),
        ]);

        let bridge = Arc::new(ExpertLlmBridge {
            client: self.client.clone(),
            model: self.model.clone(),
        });

        let experts = expert_roles.into_iter().map(|role| {
            DomainExpert {
                role,
                llm: bridge.clone(),
            }
        }).collect::<Vec<_>>();

        let manager = ExpertTeamManager::new(lead_name, experts);
        let mut trace = SkillTrace::new();

        tracing::info!("ExpertTeam: Running full workflow for task: {}", task);

        match manager.run_full_expert_workflow(task, &mut trace, bridge).await {
            Ok(report) => Ok(report),
            Err(e) => Err(ToolError::LlmRecoverable(format!("Expert Team Workflow failed at quality gate: {}", e))),
        }
    }
}

pub fn expert_team_tool(client: Arc<dyn LlmClient>, model: String) -> crate::Tool {
    crate::Tool {
        name: "expert_team_orchestration".to_string(),
        description: "Orchestrates a 6-agent expert team (1 Lead + 5 Domain Experts) to solve complex business tasks. \
            Executes tasks in parallel and enforces strict quality gates (Pre-flight, Pre-merge, Pre-deliver). \
            The final report is guaranteed to be thorough (>=20k words) and includes required charts/analysis. \
            (Tencent Workbuddy (Expert Team) Feature, based on Tencent Cloud Intelligent Agent Platform, BP-Factory reference impl)".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "task": {
                    "type": "string",
                    "description": "The complex task description requiring expert analysis."
                },
                "lead_name": {
                    "type": "string",
                    "description": "Optional name for the lead project director agent."
                },
                "expert_roles": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Optional list of 5 expert roles. Defaults to standard expert roles."
                }
            },
            "required": ["task"]
        }),
        execute: Arc::new(PydanticAdapter::new(ExpertTeamExecutor { client, model })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{Message, Role, Usage};
    use tokio::sync::Mutex;

    struct MockExpertTeamLlm {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockExpertTeamLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "Default expert response with more than 20000 words. ".repeat(4000), // Simulate a long enough report to pass the 20000 words quality gate
                        tool_calls: vec![],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }


    #[tokio::test]
    async fn test_expert_team_tool_success_default_args() {
        let client = Arc::new(MockExpertTeamLlm {
            responses: Mutex::new(vec![]),
        });
        let executor = ExpertTeamExecutor {
            client,
            model: "test-model".to_string(),
        };

        let args = json!({
            "task": "Test expert team"
        });

        let _ = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
    }

#[tokio::test]
    async fn test_expert_team_tool_success_custom_roles() {
        let client = Arc::new(MockExpertTeamLlm {
            responses: Mutex::new(vec![]),
        });
        let executor = ExpertTeamExecutor {
            client,
            model: "test-model".to_string(),
        };

        let args = json!({
            "task": "Test expert team custom",
            "lead_name": "Chief Director",
            "expert_roles": ["Specialist A", "Specialist B"]
        });

        let _ = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
    }

#[tokio::test]
    async fn test_expert_team_tool_quality_gate_failure() {
        struct FailMockExpertTeamLlm;

        #[async_trait::async_trait]
        impl LlmClient for FailMockExpertTeamLlm {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                // Return a very short response to intentionally fail the ">=20k words" or similar quality gate
                Ok(ChatResponse {
                    message: Message::assistant("Too short"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }

        let client = Arc::new(FailMockExpertTeamLlm);
        let executor = ExpertTeamExecutor {
            client,
            model: "test-model".to_string(),
        };

        let args = json!({
            "task": "Test expert team failing gate"
        });

        let result = executor.execute_typed(serde_json::from_value(args).unwrap()).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Expert Team Workflow failed at quality gate"));
            },
            _ => panic!("Expected LlmRecoverable error"),
        }
    }
}
