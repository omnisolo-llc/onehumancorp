use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};

pub struct MemoryLayer {
    repository: Arc<VectorRepository>,
}

impl MemoryLayer {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Task 4: Cross-Department Context Sharing
    /// We simply delegate to semantic search, which searches by tenant_id across all agents implicitly,
    /// avoiding siloed memory where each department only knows what it itself has seen.
    pub async fn share_cross_department_context(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.semantic_search(tenant_id, query_embedding, limit).await
    }
}
