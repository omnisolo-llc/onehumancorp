
use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

// ── TaskStop ──────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct TaskStopArgs {
    task_id: String,
}

struct TaskStopExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<TaskStopArgs> for TaskStopExecutor {
    async fn execute_typed(&self, args: TaskStopArgs) -> Result<String, ToolError> {
        Ok(format!("Stop requested for task {}.", args.task_id))
    }
}

// ── TaskStatus ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct TaskStatusArgs {
    task_id: String,
}

struct TaskStatusExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<TaskStatusArgs> for TaskStatusExecutor {
    async fn execute_typed(&self, args: TaskStatusArgs) -> Result<String, ToolError> {
        Ok(json!({
            "task_id": args.task_id,
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
        execute: Arc::new(PydanticAdapter::new(TaskStopExecutor)),
    }
}

pub fn agent_status_tool() -> Tool {
    Tool {
        name: "TaskStatus".to_string(),
        description: "Get the status of a running sub-agent task by task ID.".to_string(),
        is_read_only: false,
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
        execute: Arc::new(PydanticAdapter::new(TaskStatusExecutor)),
    }
}
