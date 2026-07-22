use serde::{Deserialize, Serialize};

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
pub struct CreateTaskDto {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
}
