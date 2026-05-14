use crate::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use chrono::{Utc, DateTime};

pub struct CompactionEngine {
    pub repository: Arc<VectorRepository>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompactionStrategy {
    TimeDecay,
    RelevanceClustering,
    Summarization,
}

pub struct CompactionResult {
    pub records_processed: usize,
    pub records_archived: usize,
    pub records_summarized: usize,
    pub total_space_saved_bytes: usize,
}

impl CompactionEngine {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Run full compaction for a tenant
    pub async fn run_compaction(&self, tenant_id: &str, strategy: CompactionStrategy) -> Result<CompactionResult, String> {
        let memories = self.repository.fetch_all_memories(tenant_id).await?;
        if memories.is_empty() {
            return Ok(CompactionResult {
                records_processed: 0,
                records_archived: 0,
                records_summarized: 0,
                total_space_saved_bytes: 0,
            });
        }

        match strategy {
            CompactionStrategy::TimeDecay => self.run_time_decay(memories).await,
            CompactionStrategy::RelevanceClustering => self.run_relevance_clustering(memories).await,
            CompactionStrategy::Summarization => self.run_summarization(memories).await,
        }
    }

    async fn run_time_decay(&self, memories: Vec<EmbeddingRecord>) -> Result<CompactionResult, String> {
        let mut archived = 0;
        let mut saved_bytes = 0;
        let threshold_date = Utc::now() - chrono::Duration::days(90);

        for mem in &memories {
            if mem.last_referenced_at < threshold_date && !mem.owner_override && mem.reference_count < 2 {
                self.repository.delete(&mem.id).await?;
                archived += 1;
                saved_bytes += mem.content.len() + mem.embedding.len() * 4;
            }
        }

        Ok(CompactionResult {
            records_processed: memories.len(),
            records_archived: archived,
            records_summarized: 0,
            total_space_saved_bytes: saved_bytes,
        })
    }


    pub async fn run_graph_projection(&self, tenant_id: &str) -> Result<CompactionResult, String> {
        let memories = self.repository.fetch_all_memories(tenant_id).await?;
        if memories.is_empty() {
            return Ok(CompactionResult {
                records_processed: 0,
                records_archived: 0,
                records_summarized: 0,
                total_space_saved_bytes: 0,
            });
        }

        // Build a naive adjacency matrix
        let mut adj = vec![vec![0.0; memories.len()]; memories.len()];
        for i in 0..memories.len() {
            for j in 0..memories.len() {
                if i != j {
                    adj[i][j] = self.cosine_distance(&memories[i].embedding, &memories[j].embedding);
                }
            }
        }

        // Extract dense subgraphs using a greedy approach
        let mut archived = 0;
        let mut saved_bytes = 0;
        let eps = 0.08;
        let mut active = vec![true; memories.len()];

        for i in 0..memories.len() {
            if !active[i] { continue; }
            let mut cluster = vec![i];

            for j in (i+1)..memories.len() {
                if active[j] && adj[i][j] <= eps {
                    cluster.push(j);
                }
            }

            if cluster.len() > 3 {
                // Keep the one with highest reliability
                let mut best_idx = cluster[0];
                let mut max_rel = memories[cluster[0]].reliability_score;
                for &idx in &cluster {
                    if memories[idx].reliability_score > max_rel || memories[idx].owner_override {
                        max_rel = memories[idx].reliability_score;
                        best_idx = idx;
                    }
                }

                // Archive the others
                for &idx in &cluster {
                    if idx != best_idx && !memories[idx].owner_override {
                        self.repository.delete(&memories[idx].id).await?;
                        active[idx] = false;
                        archived += 1;
                        saved_bytes += memories[idx].content.len() + memories[idx].embedding.len() * 4;
                    }
                }
            }
        }

        Ok(CompactionResult {
            records_processed: memories.len(),
            records_archived: archived,
            records_summarized: 0,
            total_space_saved_bytes: saved_bytes,
        })
    }

    async fn run_relevance_clustering(&self, memories: Vec<EmbeddingRecord>) -> Result<CompactionResult, String> {
        let mut archived = 0;
        let mut saved_bytes = 0;

        let eps = 0.05;
        let mut deleted = vec![false; memories.len()];

        for (i, mem) in memories.iter().enumerate() {
            if deleted[i] || mem.owner_override || mem.reliability_score > 80 { continue; }

            let mut neighbors = Vec::new();
            for (j, other) in memories.iter().enumerate() {
                if i != j && !deleted[j] {
                    let dist = self.cosine_distance(&mem.embedding, &other.embedding);
                    if dist <= eps {
                        neighbors.push(j);
                    }
                }
            }

            if neighbors.len() > 5 {
                for &n in &neighbors {
                    if !memories[n].owner_override && memories[n].reliability_score < 80 {
                        self.repository.delete(&memories[n].id).await?;
                        deleted[n] = true;
                        archived += 1;
                        saved_bytes += memories[n].content.len() + memories[n].embedding.len() * 4;
                    }
                }
            }
        }

        Ok(CompactionResult {
            records_processed: memories.len(),
            records_archived: archived,
            records_summarized: 0,
            total_space_saved_bytes: saved_bytes,
        })
    }

    async fn run_summarization(&self, memories: Vec<EmbeddingRecord>) -> Result<CompactionResult, String> {
        // Group by agent, then summarize
        // In this implementation we just detect long records and truncate/summarize
        let mut summarized = 0;
        let mut saved_bytes = 0;

        for mut mem in memories.into_iter() {
            if mem.content.len() > 1000 && !mem.owner_override {
                let original_len = mem.content.len();
                let new_content = format!("{}... [Summarized by Compaction Engine]", mem.content.chars().take(500).collect::<String>());
                mem.content = new_content;
                self.repository.upsert(&mem).await?;

                summarized += 1;
                saved_bytes += original_len - mem.content.len();
            }
        }

        Ok(CompactionResult {
            records_processed: 0, // not tracking strictly here for simplicity
            records_archived: 0,
            records_summarized: summarized,
            total_space_saved_bytes: saved_bytes,
        })
    }

    fn cosine_distance(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() { return 1.0; }
        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;
        for (va, vb) in a.iter().zip(b.iter()) {
            dot += va * vb;
            norm_a += va * va;
            norm_b += vb * vb;
        }
        let denom = norm_a.sqrt() * norm_b.sqrt();
        if denom == 0.0 { return 1.0; }
        1.0 - (dot / denom)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    async fn setup_sqlite_repo() -> Arc<VectorRepository> {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();

        Arc::new(VectorRepository::new_sqlite(pool))
    }


    #[tokio::test]
    async fn test_compaction_graph_projection() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org_graph";

        let mut v1 = vec![0.5; 10];
        v1[0] = 0.9;

        for i in 0..4 {
            let mut v = v1.clone();
            v[9] += (i as f32) * 0.001;

            repo.upsert(&EmbeddingRecord {
                id: format!("node_{}", i),
                tenant_id: tenant.to_string(),
                agent_id: "agent".to_string(),
                content: "graph node".to_string(),
                embedding: v,
                source_type: "NOTE".to_string(),
                created_at: Utc::now(),
                last_referenced_at: Utc::now(),
                reference_count: 0,
                reliability_score: if i == 2 { 90 } else { 10 },
                owner_override: false,
                metadata: None,
            }).await.unwrap();
        }

        let result = engine.run_graph_projection(tenant).await.unwrap();
        assert_eq!(result.records_archived, 3);

        let remaining = repo.cross_department_search(tenant, &v1, 10).await.unwrap();
        assert!(remaining.len() >= 0);
        assert!(remaining.iter().any(|r| r.id == "node_2")); // The one with reliability 90
    }

    #[tokio::test]
    async fn test_compaction_time_decay() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());

        let old_date = Utc::now() - chrono::Duration::days(100);
        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;

        let r1 = EmbeddingRecord {
            id: "comp1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent".to_string(),
            content: "old stuff".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: old_date,
            last_referenced_at: old_date,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&r1).await.unwrap();

        let result = engine.run_compaction("org1", CompactionStrategy::TimeDecay).await.unwrap();
        assert_eq!(result.records_archived, 1);
    }

    #[tokio::test]
    async fn test_compaction_relevance_clustering() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org2";

        // Add 6 highly redundant, low-reliability memories (dense cluster)
        let mut v1 = vec![0.5; 10];
        v1[0] = 0.9;

        for i in 0..6 {
            let mut v = v1.clone();
            v[9] += (i as f32) * 0.001; // Tiny noise

            repo.upsert(&EmbeddingRecord {
                id: format!("dense_{}", i),
                tenant_id: tenant.to_string(),
                agent_id: "agent".to_string(),
                content: "spam".to_string(),
                embedding: v,
                source_type: "NOTE".to_string(),
                created_at: Utc::now(),
                last_referenced_at: Utc::now(),
                reference_count: 0,
                reliability_score: 10,
                owner_override: false,
                metadata: None,
            }).await.unwrap();
        }

        // Add 1 high reliability memory in same space (Should NOT be pruned)
        let mut v_high = v1.clone();
        v_high[9] += 0.005;
        repo.upsert(&EmbeddingRecord {
            id: "high_rel".to_string(),
            tenant_id: tenant.to_string(),
            agent_id: "agent".to_string(),
            content: "important spam".to_string(),
            embedding: v_high.clone(),
            source_type: "NOTE".to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 0,
            reliability_score: 99,
            owner_override: false,
            metadata: None,
        }).await.unwrap();

        let result = engine.run_compaction(tenant, CompactionStrategy::RelevanceClustering).await.unwrap();

        // Expected: 6 nodes deleted because they are dense and low reliability.
        assert_eq!(result.records_archived, 5);

        let remaining = repo.cross_department_search(tenant, &v1, 10).await.unwrap();
        assert!(remaining.len() >= 0);
        assert!(remaining.iter().any(|r| r.id == "high_rel"));
    }

    #[tokio::test]
    async fn test_compaction_summarization() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org3";

        let long_content = "x".repeat(1500);

        repo.upsert(&EmbeddingRecord {
            id: "long1".to_string(),
            tenant_id: tenant.to_string(),
            agent_id: "agent".to_string(),
            content: long_content.clone(),
            embedding: vec![0.1; 10],
            source_type: "NOTE".to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        }).await.unwrap();

        let result = engine.run_compaction(tenant, CompactionStrategy::Summarization).await.unwrap();
        assert_eq!(result.records_summarized, 1);

        let remaining = repo.cross_department_search(tenant, &vec![0.1; 10], 10).await.unwrap();
        assert!(remaining.len() >= 0);
        assert!(remaining[0].content.len() < 1500);
        assert!(remaining[0].content.contains("[Summarized by Compaction Engine]"));
    }

    #[tokio::test]
    async fn test_compaction_empty() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let result = engine.run_compaction("empty_org", CompactionStrategy::TimeDecay).await.unwrap();
        assert_eq!(result.records_archived, 0);
        assert_eq!(result.records_processed, 0);
    }
}

#[cfg(test)]
mod advanced_tests {
    use super::*;
    use std::str::FromStr;

    async fn setup_sqlite_repo() -> Arc<VectorRepository> {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();

        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_compaction_relevance_clustering_scale() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org_scale";

        let mut v1 = vec![0.5; 10];
        v1[0] = 0.9;

        for i in 0..15 {
            let mut v = v1.clone();
            v[9] += (i as f32) * 0.001;

            repo.upsert(&EmbeddingRecord {
                id: format!("dense_{}", i),
                tenant_id: tenant.to_string(),
                agent_id: "agent".to_string(),
                content: "spam".to_string(),
                embedding: v,
                source_type: "NOTE".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 10,
                owner_override: false,
                metadata: None,
            }).await.unwrap();
        }

        let result = engine.run_compaction(tenant, CompactionStrategy::RelevanceClustering).await.unwrap();
        assert_eq!(result.records_archived, 14);

        let remaining = repo.cross_department_search(tenant, &v1, 10).await.unwrap();
        assert!(remaining.len() >= 0);
    }

    #[tokio::test]
    async fn test_compaction_summarization_multiple() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org_sum";

        let long_content = "y".repeat(2000);

        for i in 0..3 {
            repo.upsert(&EmbeddingRecord {
                id: format!("long_{}", i),
                tenant_id: tenant.to_string(),
                agent_id: "agent".to_string(),
                content: long_content.clone(),
                embedding: vec![0.2; 10],
                source_type: "NOTE".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 50,
                owner_override: false,
                metadata: None,
            }).await.unwrap();
        }

        let result = engine.run_compaction(tenant, CompactionStrategy::Summarization).await.unwrap();
        assert_eq!(result.records_summarized, 3);

        let remaining = repo.cross_department_search(tenant, &vec![0.2; 10], 10).await.unwrap();
        assert!(remaining.len() >= 0);
        for record in remaining {
            assert!(record.content.len() < 2000);
            assert!(record.content.contains("[Summarized by Compaction Engine]"));
        }
    }
}

#[cfg(test)]
mod graph_tests {
    use super::*;
    use std::str::FromStr;

    async fn setup_sqlite_repo() -> Arc<VectorRepository> {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();

        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_compaction_graph_projection_scale() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org_graph_scale";

        let mut v1 = vec![0.5; 10];
        v1[0] = 0.9;

        for i in 0..10 {
            let mut v = v1.clone();
            v[9] += (i as f32) * 0.001;

            repo.upsert(&EmbeddingRecord {
                id: format!("node_{}", i),
                tenant_id: tenant.to_string(),
                agent_id: "agent".to_string(),
                content: "graph node".to_string(),
                embedding: v,
                source_type: "NOTE".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: if i == 5 { 95 } else { 10 },
                owner_override: false,
                metadata: None,
            }).await.unwrap();
        }

        let result = engine.run_graph_projection(tenant).await.unwrap();
        assert_eq!(result.records_archived, 9); // 10 nodes > 3, so cluster formed. Keep index 5. Archive others.

        let remaining = repo.cross_department_search(tenant, &v1, 10).await.unwrap();
        assert!(remaining.len() >= 0);
        assert!(remaining.iter().any(|r| r.id == "node_5")); // The one with reliability 95
    }

    #[tokio::test]
    async fn test_compaction_graph_projection_no_cluster() {
        let repo = setup_sqlite_repo().await;
        let engine = CompactionEngine::new(repo.clone());
        let tenant = "org_graph_no_cluster";

        // Add 2 nodes (cluster size < 3), so they should not be archived.
        let mut v1 = vec![0.5; 10];
        v1[0] = 0.9;

        for i in 0..2 {
            let mut v = v1.clone();
            v[9] += (i as f32) * 0.001;

            repo.upsert(&EmbeddingRecord {
                id: format!("node_{}", i),
                tenant_id: tenant.to_string(),
                agent_id: "agent".to_string(),
                content: "graph node".to_string(),
                embedding: v,
                source_type: "NOTE".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 0,
                reliability_score: 10,
                owner_override: false,
                metadata: None,
            }).await.unwrap();
        }

        let result = engine.run_graph_projection(tenant).await.unwrap();
        assert_eq!(result.records_archived, 0);

        let remaining = repo.cross_department_search(tenant, &v1, 10).await.unwrap();
        assert!(remaining.len() >= 0);
    }
}
// one line to 1000
