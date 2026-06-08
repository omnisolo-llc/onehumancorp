use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;


use super::{SharedTodos, Tool, ToolExecutor};

/// A single todo item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: String, // "pending" | "in_progress" | "completed"
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub priority: String, // "low" | "medium" | "high"
}

// ── TodoWrite ─────────────────────────────────────────────────────────────────

struct TodoWriteExecutor {
    todos: SharedTodos,
}

#[async_trait::async_trait]
impl ToolExecutor for TodoWriteExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let todos_arr = args["todos"]
            .as_array()
            .ok_or_else(|| ToolError::LlmRecoverable("todowrite: todos must be an array".to_string()))?;

        let items: Vec<TodoItem> = todos_arr
            .iter()
            .enumerate()
            .map(|(i, t)| TodoItem {
                id: t["id"]
                    .as_str()
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("todo-{}", i + 1)),
                content: t["content"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                status: t["status"]
                    .as_str()
                    .unwrap_or("pending")
                    .to_string(),
                priority: t["priority"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            })
            .collect();

        let mut todos = self.todos.write().await;
        *todos = items;
        Ok(format!("Todo list updated with {} items.", todos.len()))
    }
}

// ── TodoRead ──────────────────────────────────────────────────────────────────

struct TodoReadExecutor {
    todos: SharedTodos,
}

#[async_trait::async_trait]
impl ToolExecutor for TodoReadExecutor {
    async fn execute(
        &self,
        _args: Value,
    ) -> Result<String, ToolError> {
        let todos = self.todos.read().await;
        if todos.is_empty() {
            return Ok("Todo list is empty.".to_string());
        }
        let s = serde_json::to_string_pretty(&*todos)
            .unwrap_or_else(|_| "[]".to_string());
        Ok(s)
    }
}

pub fn todowrite_tool(todos: SharedTodos) -> Tool {
    Tool {
        name: "TodoWrite".to_string(),
        description: "Write the task todo list. Replaces the entire list with the provided items. \
            Use to track progress on multi-step tasks."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "todos": {
                    "type": "array",
                    "description": "Array of todo items.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {"type": "string"},
                            "content": {"type": "string"},
                            "status": {
                                "type": "string",
                                "enum": ["pending", "in_progress", "completed"]
                            },
                            "priority": {
                                "type": "string",
                                "enum": ["low", "medium", "high", ""]
                            }
                        },
                        "required": ["content", "status"]
                    }
                }
            },
            "required": ["todos"]
        }),
        execute: Arc::new(TodoWriteExecutor { todos }),
    }
}

pub fn todoread_tool(todos: SharedTodos) -> Tool {
    Tool {
        name: "TodoRead".to_string(),
        description: "Read the current todo list.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {}
        }),
        execute: Arc::new(TodoReadExecutor { todos }),
    }
}

#[cfg(test)]
#[path = "todowrite_test.rs"]
mod todowrite_test;
