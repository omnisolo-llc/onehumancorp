use super::models::{MemoryContext, ConflictResolutionPolicy, PruningPolicy};
use super::metrics::MemoryMetrics;
use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};

#[derive(Clone)]
pub struct MemoryLayer {
    metrics: Arc<MemoryMetrics>,
    repo: Arc<VectorRepository>,
    pruning_policy: Arc<PruningPolicy>,
    conflict_policy: Arc<ConflictResolutionPolicy>,
}

impl MemoryLayer {
    pub fn new(
        repo: Arc<VectorRepository>,
        pruning_policy: PruningPolicy,
        conflict_policy: ConflictResolutionPolicy,
    ) -> Self {
        Self {
            metrics: Arc::new(MemoryMetrics::new()),
            repo,
            pruning_policy: Arc::new(pruning_policy),
            conflict_policy: Arc::new(conflict_policy),
        }
    }

    pub async fn store_context(&self, context: MemoryContext) -> Result<(), String> {
        let record = EmbeddingRecord {
            id: context.id.clone(),
            tenant_id: context.tenant_id.clone(),
            agent_id: context.department_id.clone(),
            content: context.content.clone(),
            embedding: context.semantic_embedding.clone(),
            source_type: context.source_event_type.clone(),
            created_at: context.created_at,
            last_referenced_at: context.last_accessed_at,
            reference_count: context.access_count,
            reliability_score: 50,
            owner_override: context.owner_override,
            metadata: context.metadata_json.clone().map(|v| v.to_string()),
        };

        self.repo.upsert(&record).await.map_err(|e| e.to_string())?;
        self.metrics.record_store();
        Ok(())
    }

    pub async fn retrieve_cross_department(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<MemoryContext>, String> {
        self.metrics.record_cross_dept_query();
        let results = self.repo.cross_department_search(tenant_id, query_embedding, limit.try_into().unwrap())
            .await
            .map_err(|e| e.to_string())?;

        Ok(results.into_iter().map(|r| {
            MemoryContext {
                id: r.id,
                tenant_id: r.tenant_id,
                department_id: r.agent_id,
                content: r.content,
                semantic_embedding: r.embedding,
                source_event_type: r.source_type,
                created_at: r.created_at,
                last_accessed_at: r.last_referenced_at,
                access_count: r.reference_count,
                conflict_resolved: true,
                owner_override: r.owner_override,
                metadata_json: r.metadata.and_then(|m| serde_json::from_str(&m).ok()),
            }
        }).collect())
    }

    pub async fn run_pruning(&self) -> Result<(), String> {
        let now = chrono::Utc::now();
        let default_retention = chrono::Duration::days(self.pruning_policy.retention_days_default);
        let cutoff_date = now - default_retention;

        self.repo.prune_stale(cutoff_date).await.map_err(|e| e.to_string())?;
        self.metrics.record_prune(1);
        Ok(())
    }

    pub async fn run_conflict_resolution(&self) -> Result<(), String> {
        let resolved_count = self.repo.auto_resolve_conflicts().await.map_err(|e| e.to_string())?;

        for _ in 0..resolved_count {
            self.metrics.record_conflict_resolution();
        }

        Ok(())
    }

    pub async fn share_context_cross_department(
        &self,
        tenant_id: &str,
        source_department: &str,
        target_department: &str,
        content: String,
        embedding: Vec<f32>,
    ) -> Result<(), String> {
        let mut context = MemoryContext::new(
            format!("shared_{}_{}", source_department, uuid::Uuid::new_v4()),
            tenant_id.to_string(),
            target_department.to_string(),
            content,
            embedding,
            "CROSS_DEPARTMENT_SHARE".to_string(),
        );

        let metadata = serde_json::json!({
            "source_department": source_department,
            "shared": true,
        });
        context.metadata_json = Some(metadata);

        self.store_context(context).await
    }
}
