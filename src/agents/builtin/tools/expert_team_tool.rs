use ohc_builtin_agent_core::expert_team::{ExpertTeamManager, DomainExpert, ExpertTeamLlmClient, SkillTrace};
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, ToolError};
use ohc_builtin_agent_llm::LlmClient;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::ToolExecutor;

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

pub struct ExpertTeamExecutor {
    pub client: Arc<dyn LlmClient>,
    pub model: String,
}

#[async_trait::async_trait]
impl ToolExecutor for ExpertTeamExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let task = args.get("task").and_then(|v| v.as_str()).ok_or_else(|| {
            ToolError::LlmRecoverable("ExpertTeam: 'task' is required".to_string())
        })?;

        let lead_name = args.get("lead_name").and_then(|v| v.as_str()).unwrap_or("Project Director");

        let expert_roles = match args.get("expert_roles").and_then(|v| v.as_array()) {
            Some(arr) => arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<_>>(),
            None => vec![
                "Industry Researcher".to_string(),
                "Financial Analyst".to_string(),
                "Strategic Analyst".to_string(),
                "Process Supervisor".to_string(),
                "Quality Auditor".to_string(),
            ],
        };

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
            The final report is guaranteed to be thorough (>=20k words) and includes required charts/analysis.".to_string(),
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
                    "description": "Optional list of 5 expert roles. Defaults to standard Tencent Workbuddy roles."
                }
            },
            "required": ["task"]
        }),
        execute: Arc::new(ExpertTeamExecutor { client, model }),
    }
}
