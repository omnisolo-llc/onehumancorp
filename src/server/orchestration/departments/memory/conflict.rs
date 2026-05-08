use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;

pub async fn auto_resolve_conflicts(repository: Arc<VectorRepository>) -> Result<usize, String> {
    repository.auto_resolve_conflicts().await
}
