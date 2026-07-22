use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Escalation {
    pub id: String,
    pub tenant_id: String,
    pub location_id: Option<String>,
    pub related_task_id: Option<String>,
    pub summary: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
