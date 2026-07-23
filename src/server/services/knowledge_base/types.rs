use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseDocument {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub content: String,
    pub source_type: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseDocumentChunk {
    pub id: String,
    pub tenant_id: String,
    pub document_id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub chunk_index: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document_id: String,
    pub chunk_id: String,
    pub content: String,
    pub distance: f64,
}
