use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

use super::{SharedTaskStore, Tool, ToolExecutor};

/// A task in the task store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String, // "pending" | "in_progress" | "completed" | "failed"
    pub result: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub assignee: String,
}

/// In-memory task store.
#[derive(Default)]
pub struct TaskStore {
    tasks: HashMap<String, Task>,
}

impl TaskStore {
    pub fn create(&mut self, task: Task) -> String {
        let id = task.id.clone();
        self.tasks.insert(id.clone(), task);
        id
    }

    pub fn get(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn list(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn update(
        &mut self,
        id: &str,
        status: Option<String>,
        result: Option<String>,
    ) -> bool {
        if let Some(task) = self.tasks.get_mut(id) {
            if let Some(s) = status {
                task.status = s;
            }
            if let Some(r) = result {
                task.result = Some(r);
            }
            task.updated_at = Utc::now().timestamp_millis();
            true
        } else {
            false
        }
    }
}

// ── TaskCreate ────────────────────────────────────────────────────────────────

struct TaskCreateExecutor {
    store: SharedTaskStore,
}

#[async_trait::async_trait]
impl ToolExecutor for TaskCreateExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let title = args["title"]
            .as_str()
            .ok_or("task_create: title is required")?;
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
        Ok(format!("Task created: {}", id))
    }
}

// ── TaskGet ───────────────────────────────────────────────────────────────────

struct TaskGetExecutor {
    store: SharedTaskStore,
}

#[async_trait::async_trait]
impl ToolExecutor for TaskGetExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let id = args["id"].as_str().ok_or("task_get: id is required")?;
        let store = self.store.read().await;
        if let Some(task) = store.get(id) {
            Ok(serde_json::to_string_pretty(task).unwrap_or_default())
        } else {
            Err(format!("Task not found: {}", id).into())
        }
    }
}

// ── TaskList ──────────────────────────────────────────────────────────────────

struct TaskListExecutor {
    store: SharedTaskStore,
}

#[async_trait::async_trait]
impl ToolExecutor for TaskListExecutor {
    async fn execute(
        &self,
        _args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let store = self.store.read().await;
        let tasks: Vec<&Task> = store.list();
        if tasks.is_empty() {
            return Ok("No tasks found.".to_string());
        }
        Ok(serde_json::to_string_pretty(&tasks).unwrap_or_default())
    }
}

// ── TaskUpdate ────────────────────────────────────────────────────────────────

struct TaskUpdateExecutor {
    store: SharedTaskStore,
}

#[async_trait::async_trait]
impl ToolExecutor for TaskUpdateExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let id = args["id"].as_str().ok_or("task_update: id is required")?;
        let status = args["status"].as_str().map(str::to_string);
        let result = args["result"].as_str().map(str::to_string);

        if self.store.write().await.update(id, status, result) {
            Ok(format!("Task updated: {}", id))
        } else {
            Err(format!("Task not found: {}", id).into())
        }
    }
}

// ── Tool constructors ─────────────────────────────────────────────────────────

pub fn task_create_tool(store: SharedTaskStore) -> Tool {
    Tool {
        name: "TaskCreate".to_string(),
        description: "Create a new task in the task tracker.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "title": {"type": "string", "description": "Task title."},
                "description": {"type": "string", "description": "Task description."},
                "assignee": {"type": "string", "description": "Agent to assign."}
            },
            "required": ["title"]
        }),
        is_mutating: true,
        execute: Arc::new(TaskCreateExecutor { store }),
    }
}

pub fn task_get_tool(store: SharedTaskStore) -> Tool {
    Tool {
        is_mutating: false,
        name: "TaskGet".to_string(),
        description: "Get details of a specific task by ID.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "id": {"type": "string", "description": "Task ID."}
            },
            "required": ["id"]
        }),
        execute: Arc::new(TaskGetExecutor { store }),
    }
}

pub fn task_list_tool(store: SharedTaskStore) -> Tool {
    Tool {
        is_mutating: false,
        name: "TaskList".to_string(),
        description: "List all tasks in the task tracker.".to_string(),
        parameters: json!({"type": "object", "properties": {}}),
        execute: Arc::new(TaskListExecutor { store }),
    }
}

pub fn task_update_tool(store: SharedTaskStore) -> Tool {
    Tool {
        name: "TaskUpdate".to_string(),
        description: "Update a task's status or result.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "status": {
                    "type": "string",
                    "enum": ["pending", "in_progress", "completed", "failed"]
                },
                "result": {"type": "string"}
            },
            "required": ["id"]
        }),
        is_mutating: true,
        execute: Arc::new(TaskUpdateExecutor { store }),
    }
}
