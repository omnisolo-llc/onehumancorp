use ohc_builtin_agent_core::types::ToolError;
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
    pub priority: i32,  // 0: low, 1: medium, 2: high, 3: critical
    pub dependencies: Vec<String>, // list of task IDs
    pub result: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub assignee: String,
    pub metadata: HashMap<String, String>,
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
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        // Sort by priority (desc) then by created_at (asc)
        tasks.sort_by(|a, b| {
            b.priority.cmp(&a.priority).then(a.created_at.cmp(&b.created_at))
        });
        tasks
    }

    pub fn update(
        &mut self,
        id: &str,
        status: Option<String>,
        priority: Option<i32>,
        result: Option<String>,
        metadata: Option<HashMap<String, String>>,
    ) -> bool {
        if let Some(task) = self.tasks.get_mut(id) {
            if let Some(s) = status {
                task.status = s;
            }
            if let Some(p) = priority {
                task.priority = p;
            }
            if let Some(r) = result {
                task.result = Some(r);
            }
            if let Some(m) = metadata {
                task.metadata.extend(m);
            }
            task.updated_at = Utc::now().timestamp_millis();
            true
        } else {
            false
        }
    }

    pub fn get_ready_tasks(&self) -> Vec<&Task> {
        self.list()
            .into_iter()
            .filter(|t| {
                if t.status != "pending" {
                    return false;
                }
                // Check if all dependencies are completed
                for dep_id in &t.dependencies {
                    if let Some(dep) = self.tasks.get(dep_id) {
                        if dep.status != "completed" {
                            return false;
                        }
                    }
                }
                true
            })
            .collect()
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
    ) -> Result<String, ToolError> {
        let title = args["title"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("task_create: title is required".to_string()))?;
        let description = args["description"].as_str().unwrap_or("").to_string();
        let assignee = args["assignee"].as_str().unwrap_or("").to_string();
        let priority = args["priority"].as_i64().unwrap_or(1) as i32;
        let dependencies = args["dependencies"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let now = Utc::now().timestamp_millis();
        let id = format!("task-{}", uuid::Uuid::new_v4().simple());
        let task = Task {
            id: id.clone(),
            title: title.to_string(),
            description,
            status: "pending".to_string(),
            priority,
            dependencies,
            result: None,
            created_at: now,
            updated_at: now,
            assignee,
            metadata: HashMap::new(),
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
    ) -> Result<String, ToolError> {
        let id = args["id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("task_get: id is required".to_string()))?;
        let store = self.store.read().await;
        if let Some(task) = store.get(id) {
            Ok(serde_json::to_string_pretty(task).unwrap_or_default())
        } else {
            Err(ToolError::LlmRecoverable(format!("Task not found: {}", id)))
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
    ) -> Result<String, ToolError> {
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
    ) -> Result<String, ToolError> {
        let id = args["id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("task_update: id is required".to_string()))?;
        let status = args["status"].as_str().map(str::to_string);
        let priority = args["priority"].as_i64().map(|p| p as i32);
        let result = args["result"].as_str().map(str::to_string);
        let metadata = args["metadata"].as_object().map(|m| {
            m.iter().map(|(k, v)| (k.clone(), v.to_string())).collect()
        });

        if self.store.write().await.update(id, status, priority, result, metadata) {
            Ok(format!("Task updated: {}", id))
        } else {
            Err(ToolError::LlmRecoverable(format!("Task not found: {}", id)))
        }
    }
}

// ── Tool constructors ─────────────────────────────────────────────────────────

pub fn task_create_tool(store: SharedTaskStore) -> Tool {
    Tool {
        name: "TaskCreate".to_string(),
        description: "Create a new task in the task tracker.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "title": {"type": "string", "description": "Task title."},
                "description": {"type": "string", "description": "Task description."},
                "assignee": {"type": "string", "description": "Agent to assign."},
                "priority": {"type": "integer", "description": "0: Low, 1: Medium, 2: High, 3: Critical."},
                "dependencies": {"type": "array", "items": {"type": "string"}, "description": "List of task IDs this task depends on."}
            },
            "required": ["title"]
        }),
        execute: Arc::new(TaskCreateExecutor { store }),
    }
}

pub fn task_get_tool(store: SharedTaskStore) -> Tool {
    Tool {
        name: "TaskGet".to_string(),
        description: "Get details of a specific task by ID.".to_string(),
        is_read_only: true,
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
        name: "TaskList".to_string(),
        description: "List all tasks in the task tracker.".to_string(),
        is_read_only: true,
        parameters: json!({"type": "object", "properties": {}}),
        execute: Arc::new(TaskListExecutor { store }),
    }
}

pub fn task_update_tool(store: SharedTaskStore) -> Tool {
    Tool {
        name: "TaskUpdate".to_string(),
        description: "Update a task's status, priority, or result.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "status": {
                    "type": "string",
                    "enum": ["pending", "in_progress", "completed", "failed"]
                },
                "priority": {"type": "integer"},
                "result": {"type": "string"},
                "metadata": {"type": "object"}
            },
            "required": ["id"]
        }),
        execute: Arc::new(TaskUpdateExecutor { store }),
    }
}
