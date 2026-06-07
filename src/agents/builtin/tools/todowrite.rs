use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;


use super::{SharedTodos, Tool, ToolExecutor};

/// A single todo item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub status: String, // "pending" | "in_progress" | "completed"
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub priority: String, // "low" | "medium" | "high"
}

// ── TodoWrite ─────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct TodoWriteArgs {
    pub todos: Vec<TodoItem>,
}

struct TodoWriteExecutor {
    todos: SharedTodos,
}

#[async_trait::async_trait]
impl super::pydantic::PydanticToolExecutor<TodoWriteArgs> for TodoWriteExecutor {
    async fn execute_typed(&self, args: TodoWriteArgs) -> Result<String, ToolError> {
        let mut items = args.todos;
        for (i, item) in items.iter_mut().enumerate() {
            if item.id.is_empty() {
                item.id = format!("todo-{}", i + 1);
            }
            if item.status.is_empty() {
                item.status = "pending".to_string();
            }
        }
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
        execute: Arc::new(super::pydantic::PydanticAdapter::new(TodoWriteExecutor { todos })),
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
