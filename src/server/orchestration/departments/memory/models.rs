use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryContext {
    pub id: String,
    pub tenant_id: String,
    pub department_id: String,
    pub content: String,
    pub semantic_embedding: Vec<f32>,
    pub source_event_type: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub access_count: i32,
    pub conflict_resolved: bool,
    pub owner_override: bool,
    pub metadata_json: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionPolicy {
    pub prefer_recency: bool,
    pub require_owner_approval: bool,
    pub department_weights: std::collections::HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruningPolicy {
    pub retention_days_default: i64,
    pub retention_days_override: Option<i64>,
    pub keep_high_access_count: i32,
}

impl MemoryContext {
    pub fn new(
        id: String,
        tenant_id: String,
        department_id: String,
        content: String,
        semantic_embedding: Vec<f32>,
        source_event_type: String,
    ) -> Self {
        Self {
            id,
            tenant_id,
            department_id,
            content,
            semantic_embedding,
            source_event_type,
            created_at: Utc::now(),
            last_accessed_at: Utc::now(),
            access_count: 0,
            conflict_resolved: false,
            owner_override: false,
            metadata_json: None,
        }
    }

    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed_at = Utc::now();
    }
}
