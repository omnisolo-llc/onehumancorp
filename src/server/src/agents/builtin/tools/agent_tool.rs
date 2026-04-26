use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

// ── Agent (spawn sub-agent) ───────────────────────────────────────────────────

struct AgentExecutor;

#[async_trait::async_trait]
impl ToolExecutor for AgentExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = args["prompt"]
            .as_str()
            .ok_or("agent: prompt is required")?;
        let description = args["description"]
            .as_str()
            .unwrap_or(prompt);

        // In the agent loop, the sub-agent tool use is handled by the agent
        // dispatcher. Here we return a placeholder. The actual dispatch
        // happens in service.rs / agent.rs.
        Ok(format!(
            "{{\"task_id\":\"spawned\",\"description\":\"{}\"}}",
            description
        ))
    }
}

// ── TaskStop ──────────────────────────────────────────────────────────────────

struct TaskStopExecutor;

#[async_trait::async_trait]
impl ToolExecutor for TaskStopExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let task_id = args["task_id"]
            .as_str()
            .ok_or("taskstop: task_id is required")?;
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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let task_id = args["task_id"]
            .as_str()
            .ok_or("taskstatus: task_id is required")?;
        Ok(json!({
            "task_id": task_id,
            "status": "running",
            "message": "Status check not available for this agent mode."
        })
        .to_string())
    }
}

// ── Tool constructors ─────────────────────────────────────────────────────────

pub fn agent_tool() -> Tool {
    Tool {
        name: "Agent".to_string(),
        description: "Spawn a sub-agent to execute a task autonomously. \
            The sub-agent runs its own ReAct loop with access to all tools. \
            Use for parallelizable or delegatable work."
            .to_string(),
        is_mutating: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "prompt": {
                    "type": "string",
                    "description": "The task prompt for the sub-agent."
                },
                "description": {
                    "type": "string",
                    "description": "Short description of the sub-task."
                },
                "model": {
                    "type": "string",
                    "description": "LLM model override for the sub-agent."
                },
                "allowed_tools": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Optional list of tool names to allow."
                }
            },
            "required": ["prompt"]
        }),
        execute: Arc::new(AgentExecutor),
    }
}

pub fn agent_stop_tool() -> Tool {
    Tool {
        name: "TaskStop".to_string(),
        description: "Stop a running sub-agent task by task ID.".to_string(),
        is_mutating: true,
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
        is_mutating: false,
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
