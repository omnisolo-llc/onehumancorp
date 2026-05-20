use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub organization_id: String,
    pub parent_task_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_agent_role: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskDependency {
    pub task_id: String,
    pub depends_on_task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CatalogItem {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub name: String,
    pub description: Option<String>,
    pub type_str: String,
    pub price_cents: i64,
    pub currency: String,
    pub duration_minutes: Option<i32>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
