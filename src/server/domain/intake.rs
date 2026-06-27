use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIntake {
    pub id: String,
    pub tenant_id: String,
    pub source: String,
    pub raw_content: String,
    pub client_info: Option<serde_json::Value>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTask {
    pub id: String,
    pub tenant_id: String,
    pub proposal_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
