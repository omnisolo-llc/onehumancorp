use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub organization_id: String,
    pub parent_task_id: Option<String>,
    pub epic_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_agent_role: Option<String>,
    pub payload: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskDependency {
    pub task_id: String,
    pub depends_on_task_id: String,
}
