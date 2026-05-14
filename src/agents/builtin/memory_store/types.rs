use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
    pub last_referenced_at: DateTime<Utc>,
    pub reference_count: i32,
    pub reliability_score: i32,
    pub owner_override: bool,
    pub metadata: Option<String>,
}

pub enum VectorMemoryStore {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

use async_trait::async_trait;

#[async_trait]
pub trait LongTermMemory: Send + Sync + std::fmt::Debug {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    async fn store(&self, content: &str, metadata: Vec<String>) -> Result<(), String>;
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String>;
    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    async fn get_lightweight_index(&self) -> Result<String, String> { Ok(String::new()) }



    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> { None }

}
