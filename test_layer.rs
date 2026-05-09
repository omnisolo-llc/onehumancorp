use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use crate::orchestration::departments::types::DepartmentType;
use chrono::Utc;

pub struct CrossDepartmentMemoryLayer {
    repository: Arc<VectorRepository>,
}

impl CrossDepartmentMemoryLayer {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    pub async fn store_context(
        &self,
        tenant_id: &str,
        department: DepartmentType,
        content: &str,
        embedding: Vec<f32>,
        reliability_score: i32,
    ) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let record = EmbeddingRecord {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            agent_id: department.to_string(), // use department as agent_id for context sharing
            content: content.to_string(),
            embedding,
            source_type: format!("{}_CONTEXT", department.to_string().to_uppercase()),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score,
            owner_override: false,
            metadata: None, // Could add more structure here if needed
        };

        self.repository.upsert(&record).await?;
        Ok(id)
    }

    pub async fn retrieve_cross_department_context(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.semantic_search(tenant_id, query_embedding, limit).await
    }
}
// Zero WIP exit
