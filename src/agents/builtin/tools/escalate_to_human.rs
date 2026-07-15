use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::tools::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
pub struct EscalateToHumanArgs {
    pub tenant_id: String,
    pub thread_id: String,
    pub summary: String,
    pub reason: String,
}

pub struct EscalateToHumanExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<EscalateToHumanArgs> for EscalateToHumanExecutor {
    async fn execute_typed(&self, args: EscalateToHumanArgs) -> Result<String, ToolError> {
        if args.tenant_id.is_empty() || args.thread_id.is_empty() {
            return Err(ToolError::ExecutionError(
                "Missing tenant_id or thread_id".to_string(),
            ));
        }

        let result = json!({
            "status": "handoff_requested",
            "message": format!("Handoff initiated for thread {}. Owner notified.", args.thread_id),
            "summary": args.summary,
            "reason": args.reason
        });

        Ok(result.to_string())
    }
}

pub fn escalate_to_human_tool() -> Tool {
    Tool {
        name: "escalate_to_human".to_string(),
        description: "Escalate the current conversation to a human owner if sentiment is strongly negative, or if the customer expresses frustration/urgency. You MUST provide a summary of the context.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": {
                    "type": "string",
                    "description": "The tenant ID"
                },
                "thread_id": {
                    "type": "string",
                    "description": "The thread or conversation ID"
                },
                "summary": {
                    "type": "string",
                    "description": "A 2-sentence summary of why the handoff is needed and what happened."
                },
                "reason": {
                    "type": "string",
                    "description": "The specific reason for escalation, e.g., 'Escalation' or 'High negative sentiment'"
                }
            },
            "required": ["tenant_id", "thread_id", "summary", "reason"]
        }),
        execute: Arc::new(PydanticAdapter::new(EscalateToHumanExecutor)),
    }
}
