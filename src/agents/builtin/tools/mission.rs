use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::{ToolError, MissionManager};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct MissionHandoverExecutor {
    pub manager: Arc<dyn MissionManager>,
}

#[async_trait::async_trait]
impl ToolExecutor for MissionHandoverExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let mission_id = args.get("mission_id").and_then(|v| v.as_str()).ok_or_else(|| ToolError::LlmRecoverable("mission_id is required".to_string()))?;
        let blockers = args.get("blockers").and_then(|v| v.as_str()).ok_or_else(|| ToolError::LlmRecoverable("blockers description is required".to_string()))?;
        let tenant_id = args.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("system");

        match self.manager.handoff_mission(mission_id, blockers, tenant_id).await {
            Ok(_) => {
                tracing::info!("Mission {} successfully handed off/blocked: {}", mission_id, blockers);
                Ok(format!("Mission {} marked as blocked with reason: {}", mission_id, blockers))
            }
            Err(e) => {
                tracing::error!("Failed to hand off mission {}: {}", mission_id, e);
                Err(ToolError::LlmRecoverable(format!("Failed to update mission status in database: {}", e)))
            }
        }
    }
}

pub fn mission_handover_tool(manager: Arc<dyn MissionManager>) -> Tool {
    Tool {
        name: "handoff_mission".to_string(),
        description: "Mark a mission as 'blocked' and append blockers to the mission log when you cannot complete it autonomously. This triggers the Mission Handover Protocol.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "mission_id": {
                    "type": "string",
                    "description": "The unique identifier of the mission."
                },
                "blockers": {
                    "type": "string",
                    "description": "A detailed description of why the mission is blocked and what actions are needed to proceed."
                },
                "tenant_id": {
                    "type": "string",
                    "description": "The tenant/organization ID. Defaults to 'system'."
                }
            },
            "required": ["mission_id", "blockers"]
        }),
        execute: Arc::new(MissionHandoverExecutor { manager }),
    }
}
