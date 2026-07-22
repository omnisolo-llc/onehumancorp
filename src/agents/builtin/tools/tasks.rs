use serde::{Deserialize, Serialize};
use crate::tools::tenant::TenantContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub tenant_id: String,
    pub location_id: Option<String>,
    pub shift_id: Option<String>,
    pub assigned_to: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub status: String,
    pub due_date: Option<String>,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub tenant_id: String,
    pub location_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub source: Option<String>,
}

pub async fn create_task(ctx: &TenantContext, req: CreateTaskRequest) -> Result<Task, String> {
    // Basic task creation stub
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: ctx.tenant_id.clone(),
        location_id: req.location_id,
        shift_id: None,
        assigned_to: None,
        title: req.title,
        description: req.description,
        priority: req.priority.unwrap_or_else(|| "medium".to_string()),
        status: "pending".to_string(),
        due_date: None,
        source: req.source.unwrap_or_else(|| "agent_operations".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    Ok(task)
}
