use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use super::task::Task;
use chrono::Utc;

use super::{SharedTaskStore, Tool, ToolExecutor};

pub struct MagenticExecutor {
    store: SharedTaskStore,
}

#[async_trait::async_trait]
impl ToolExecutor for MagenticExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["action"].as_str().unwrap_or("");

        match action {
            "add" => {
                let title = args["title"].as_str().ok_or_else(|| ToolError::LlmRecoverable("magentic: title is required for add".to_string()))?;
                let description = args["description"].as_str().unwrap_or("").to_string();
                let assignee = args["assignee"].as_str().unwrap_or("").to_string();

                let now = Utc::now().timestamp_millis();
                let id = format!("task-{}", uuid::Uuid::new_v4().simple());
                let task = Task {
                    id: id.clone(),
                    title: title.to_string(),
                    description,
                    status: "pending".to_string(),
                    result: None,
                    created_at: now,
                    updated_at: now,
                    assignee,
                };

                self.store.write().await.create(task);
                Ok(format!("Added task: {}", id))
            }
            "update" => {
                let id = args["id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("magentic: id is required for update".to_string()))?;
                let status = args["status"].as_str().map(str::to_string);
                let result = args["result"].as_str().map(str::to_string);

                if self.store.write().await.update(id, status, result) {
                    Ok(format!("Updated task: {}", id))
                } else {
                    Err(ToolError::LlmRecoverable(format!("Task not found: {}", id)))
                }
            }
            "list" => {
                let store = self.store.read().await;
                let tasks: Vec<&Task> = store.list();
                if tasks.is_empty() {
                    return Ok("No tasks in ledger.".to_string());
                }
                Ok(serde_json::to_string_pretty(&tasks).unwrap_or_default())
            }
            _ => Err(ToolError::LlmRecoverable("magentic: valid action is required (add, update, list)".to_string()))
        }
    }
}

pub fn magentic_tool(store: SharedTaskStore) -> Tool {
    Tool {
        name: "MagenticLedger".to_string(),
        description: "Manager agent dynamically updating a task ledger. Supported actions: add, update, list.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["add", "update", "list"],
                    "description": "The action to perform."
                },
                "title": {
                    "type": "string",
                    "description": "Task title (required for add)."
                },
                "description": {
                    "type": "string",
                    "description": "Task description (optional for add)."
                },
                "assignee": {
                    "type": "string",
                    "description": "Task assignee (optional for add)."
                },
                "id": {
                    "type": "string",
                    "description": "Task ID (required for update)."
                },
                "status": {
                    "type": "string",
                    "enum": ["pending", "in_progress", "completed", "failed"],
                    "description": "New status for task (optional for update)."
                },
                "result": {
                    "type": "string",
                    "description": "Result of the task (optional for update)."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(MagenticExecutor { store }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::task::TaskStore;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_magentic_tool_add() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = MagenticExecutor { store: store.clone() };

        let args = json!({
            "action": "add",
            "title": "Test Magentic Add"
        });

        let result = executor.execute(args).await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.starts_with("Added task: task-"));
    }

    #[tokio::test]
    async fn test_magentic_tool_update_and_list() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = MagenticExecutor { store: store.clone() };

        // 1. Add
        let args_add = json!({
            "action": "add",
            "title": "Test Magentic Update"
        });
        let result_add = executor.execute(args_add).await.unwrap();
        let id = result_add.replace("Added task: ", "");

        // 2. Update
        let args_update = json!({
            "action": "update",
            "id": id,
            "status": "in_progress"
        });
        let result_update = executor.execute(args_update).await.unwrap();
        assert_eq!(result_update, format!("Updated task: {}", id));

        // 3. List
        let args_list = json!({
            "action": "list"
        });
        let result_list = executor.execute(args_list).await.unwrap();
        assert!(result_list.contains(&id));
        assert!(result_list.contains("in_progress"));
    }
}
