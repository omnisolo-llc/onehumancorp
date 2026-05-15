#[derive(Debug, Clone)]
pub struct SharedTaskModel {
    pub id: String,
    pub tenant_id: String,
    pub parent_id: Option<String>,
    pub epic_id: Option<String>,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub payload: serde_json::Value,
    pub dependencies: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
