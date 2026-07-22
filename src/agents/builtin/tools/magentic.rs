use chrono::Utc;
use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::pydantic::{PydanticAdapter, PydanticToolExecutor};
use super::task::Task;
use super::{SharedTaskStore, Tool};

#[derive(Deserialize)]
struct MagenticArgs {
    action: String,
    title: Option<String>,
    description: Option<String>,
    assignee: Option<String>,
    id: Option<String>,
    status: Option<String>,
    result: Option<String>,
}

pub struct MagenticExecutor {
    store: SharedTaskStore,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<MagenticArgs> for MagenticExecutor {
    async fn execute_typed(&self, args: MagenticArgs) -> Result<String, ToolError> {
        let action = args.action.as_str();

        match action {
            "add" => {
                let title = args
                    .title
                    .as_deref()
                    .ok_or_else(|| ToolError::LlmRecoverable("magentic: title is required for add".to_string()))?;
                let description = args.description.unwrap_or_default();
                let assignee = args.assignee.unwrap_or_default();

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
                let id = args
                    .id
                    .as_deref()
                    .ok_or_else(|| ToolError::LlmRecoverable("magentic: id is required for update".to_string()))?;

                if self.store.write().await.update(id, args.status, args.result) {
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
        execute: Arc::new(PydanticAdapter::new(MagenticExecutor { store })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::task::TaskStore;
    use tokio::sync::RwLock;
    use crate::ToolExecutor;

    #[tokio::test]
    async fn test_magentic_missing_title_on_add() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = PydanticAdapter::new(MagenticExecutor { store });
        let args = json!({ "action": "add" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("magentic: title is required for add")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_magentic_missing_id_on_update() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = PydanticAdapter::new(MagenticExecutor { store });
        let args = json!({ "action": "update" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("magentic: id is required for update")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_magentic_update_non_existent_task() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = PydanticAdapter::new(MagenticExecutor { store });
        let args = json!({ "action": "update", "id": "does-not-exist" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("Task not found")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_magentic_list_empty() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = PydanticAdapter::new(MagenticExecutor { store });
        let args = json!({ "action": "list" });
        let result = executor.execute(args).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No tasks in ledger.");
    }

    #[tokio::test]
    async fn test_magentic_invalid_action() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = PydanticAdapter::new(MagenticExecutor { store });
        let args = json!({ "action": "invalid" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("valid action is required")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_magentic_tool_add() {
        let store = Arc::new(RwLock::new(TaskStore::default()));
        let executor = PydanticAdapter::new(MagenticExecutor {
            store: store.clone(),
        });

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
        let executor = PydanticAdapter::new(MagenticExecutor {
            store: store.clone(),
        });

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
