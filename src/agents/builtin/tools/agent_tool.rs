
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

// ── TaskStop ──────────────────────────────────────────────────────────────────

struct TaskStopExecutor;

#[async_trait::async_trait]
impl ToolExecutor for TaskStopExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let task_id = args["task_id"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("taskstop: task_id is required".to_string()))?;
        Ok(format!("Stop requested for task {}.", task_id))
    }
}

// ── TaskStatus ────────────────────────────────────────────────────────────────

struct TaskStatusExecutor;

#[async_trait::async_trait]
impl ToolExecutor for TaskStatusExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let task_id = args["task_id"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("taskstatus: task_id is required".to_string()))?;
        Ok(json!({
            "task_id": task_id,
            "status": "running",
            "message": "Status check not available for this agent mode."
        })
        .to_string())
    }
}

// ── Tool constructors ─────────────────────────────────────────────────────────

pub fn agent_stop_tool() -> Tool {
    Tool {
        name: "TaskStop".to_string(),
        description: "Stop a running sub-agent task by task ID.".to_string(),
        is_read_only: false,
        is_subagent: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "task_id": {
                    "type": "string",
                    "description": "Task ID to stop."
                }
            },
            "required": ["task_id"]
        }),
        execute: Arc::new(TaskStopExecutor),
    }
}

pub fn agent_status_tool() -> Tool {
    Tool {
        name: "TaskStatus".to_string(),
        description: "Get the status of a running sub-agent task by task ID.".to_string(),
        is_read_only: false,
        is_subagent: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "task_id": {
                    "type": "string",
                    "description": "Task ID to check."
                }
            },
            "required": ["task_id"]
        }),
        execute: Arc::new(TaskStatusExecutor),
    }
}
