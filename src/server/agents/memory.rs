use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingRecord {
    pub id: String,
    pub organization_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String>;
    async fn semantic_search(&self, organization_id: &str, query_embedding: &[f32], limit: i32) -> Result<Vec<EmbeddingRecord>, String>;
    async fn prune_stale_context(&self, source_types: &[&str], older_than: DateTime<Utc>) -> Result<(), String>;
    async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn resolve_conflicts(&self, organization_id: &str) -> Result<usize, String>;
}

pub struct PgVectorRepository {
    pool: sqlx::PgPool,
}

impl PgVectorRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PgVectorRepository { pool }
    }
}

#[async_trait]
impl VectorStore for PgVectorRepository {
    async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String> {
        let emb_str = serde_json::to_string(&record.embedding).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
             VALUES ($1, $2, $3, $4, $5::vector, $6, $7)
             ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                embedding = EXCLUDED.embedding,
                source_type = EXCLUDED.source_type,
                created_at = EXCLUDED.created_at"
        )
        .bind(&record.id)
        .bind(&record.organization_id)
        .bind(&record.agent_id)
        .bind(&record.content)
        .bind(emb_str)
        .bind(&record.source_type)
        .bind(record.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn semantic_search(&self, organization_id: &str, query_embedding: &[f32], limit: i32) -> Result<Vec<EmbeddingRecord>, String> {
        let emb_str = serde_json::to_string(&query_embedding).map_err(|e| e.to_string())?;

        let rows = sqlx::query(
            "SELECT id, organization_id, agent_id, content, embedding::text as emb, source_type, created_at
             FROM consolidated_memory
             WHERE organization_id = $1
             ORDER BY embedding <-> $2::vector
             LIMIT $3"
        )
        .bind(organization_id)
        .bind(emb_str)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let org_id: String = row.try_get("organization_id").unwrap_or_default();
            let agent_id: String = row.try_get("agent_id").unwrap_or_default();
            let content: String = row.try_get("content").unwrap_or_default();
            let emb_res: String = row.try_get("emb").unwrap_or_default();
            let source_type: String = row.try_get("source_type").unwrap_or_default();
            let created_at: DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());

            let embedding: Vec<f32> = serde_json::from_str(&emb_res).unwrap_or_default();

            results.push(EmbeddingRecord {
                id,
                organization_id: org_id,
                agent_id,
                content,
                embedding,
                source_type,
                created_at,
            });
        }

        Ok(results)
    }

    async fn prune_stale_context(&self, source_types: &[&str], older_than: DateTime<Utc>) -> Result<(), String> {
        if source_types.is_empty() {
            return Ok(());
        }
        for &source_type in source_types {
            sqlx::query("DELETE FROM consolidated_memory WHERE created_at < $1 AND source_type = $2")
                .bind(older_than)
                .bind(source_type)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        self.prune_stale_context(&["TASK_SUMMARY"], older_than).await
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM consolidated_memory WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn resolve_conflicts(&self, organization_id: &str) -> Result<usize, String> {
        let rows = sqlx::query(
            "SELECT id, content, embedding::text as emb, created_at              FROM consolidated_memory              WHERE organization_id = $1"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut memories = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let content: String = row.try_get("content").unwrap_or_default();
            let emb_str_res: String = row.try_get("emb").unwrap_or_default();
            let created_at: DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());

            let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();
            memories.push((id, content, embedding, created_at));
        }

        let mut to_delete: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut pruned_count = 0;

        for i in 0..memories.len() {
            if to_delete.contains(&memories[i].0) || memories[i].2.is_empty() {
                continue;
            }
            for j in (i + 1)..memories.len() {
                if to_delete.contains(&memories[j].0) || memories[j].2.is_empty() {
                    continue;
                }

                let sim = cosine_similarity(&memories[i].2, &memories[j].2);
                if sim > 0.95 {
                    if memories[i].3 < memories[j].3 {
                        to_delete.insert(memories[i].0.clone());
                    } else {
                        to_delete.insert(memories[j].0.clone());
                    }
                }
            }
        }

        for id in to_delete {
            self.delete(&id).await?;
            pruned_count += 1;
        }

        Ok(pruned_count)
    }
}

pub struct SqliteVectorRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteVectorRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        SqliteVectorRepository { pool }
    }
}

#[async_trait]
impl VectorStore for SqliteVectorRepository {
    async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String> {
        let emb_str = serde_json::to_string(&record.embedding).map_err(|e| e.to_string())?;

        let dt_str = record.created_at.to_rfc3339();

        sqlx::query(
            "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                embedding = EXCLUDED.embedding,
                source_type = EXCLUDED.source_type,
                created_at = EXCLUDED.created_at"
        )
        .bind(&record.id)
        .bind(&record.organization_id)
        .bind(&record.agent_id)
        .bind(&record.content)
        .bind(emb_str)
        .bind(&record.source_type)
        .bind(dt_str)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn semantic_search(&self, organization_id: &str, query_embedding: &[f32], limit: i32) -> Result<Vec<EmbeddingRecord>, String> {
        let rows = sqlx::query(
            "SELECT id, organization_id, agent_id, content, embedding, source_type, created_at
             FROM consolidated_memory
             WHERE organization_id = $1"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let org_id: String = row.try_get("organization_id").unwrap_or_default();
            let agent_id: String = row.try_get("agent_id").unwrap_or_default();
            let content: String = row.try_get("content").unwrap_or_default();
            let emb_res: String = row.try_get("embedding").unwrap_or_default();
            let source_type: String = row.try_get("source_type").unwrap_or_default();

            let dt_str: String = row.try_get("created_at").unwrap_or_default();
            let created_at = dt_str.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now());

            let embedding: Vec<f32> = serde_json::from_str(&emb_res).unwrap_or_default();

            if embedding.is_empty() {
                continue;
            }

            let sim = cosine_similarity(query_embedding, &embedding);

            results.push((sim, EmbeddingRecord {
                id,
                organization_id: org_id,
                agent_id,
                content,
                embedding,
                source_type,
                created_at,
            }));
        }

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit as usize);

        Ok(results.into_iter().map(|(_, r)| r).collect())
    }

    async fn prune_stale_context(&self, source_types: &[&str], older_than: DateTime<Utc>) -> Result<(), String> {
        if source_types.is_empty() {
            return Ok(());
        }
        let dt_str = older_than.to_rfc3339();
        for &source_type in source_types {
            sqlx::query("DELETE FROM consolidated_memory WHERE created_at < $1 AND source_type = $2")
                .bind(&dt_str)
                .bind(source_type)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        self.prune_stale_context(&["TASK_SUMMARY"], older_than).await
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM consolidated_memory WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn resolve_conflicts(&self, organization_id: &str) -> Result<usize, String> {
        let rows = sqlx::query(
            "SELECT id, content, embedding, created_at              FROM consolidated_memory              WHERE organization_id = $1"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut memories = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let content: String = row.try_get("content").unwrap_or_default();
            let emb_str_res: String = row.try_get("embedding").unwrap_or_default();

            let dt_str: String = row.try_get("created_at").unwrap_or_default();
            let created_at: DateTime<Utc> = dt_str.parse().unwrap_or_else(|_| Utc::now());

            let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();
            memories.push((id, content, embedding, created_at));
        }

        let mut to_delete: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut pruned_count = 0;

        for i in 0..memories.len() {
            if to_delete.contains(&memories[i].0) || memories[i].2.is_empty() {
                continue;
            }
            for j in (i + 1)..memories.len() {
                if to_delete.contains(&memories[j].0) || memories[j].2.is_empty() {
                    continue;
                }

                let sim = cosine_similarity(&memories[i].2, &memories[j].2);
                if sim > 0.95 {
                    if memories[i].3 < memories[j].3 {
                        to_delete.insert(memories[i].0.clone());
                    } else {
                        to_delete.insert(memories[j].0.clone());
                    }
                }
            }
        }

        for id in to_delete {
            self.delete(&id).await?;
            pruned_count += 1;
        }

        Ok(pruned_count)
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (va, vb) in a.iter().zip(b.iter()) {
        dot_product += va * vb;
        norm_a += va * va;
        norm_b += vb * vb;
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

#[async_trait]
pub trait OHCMemory: Send + Sync {
    async fn write(&self, namespace: &str, key: &str, data: &[u8]) -> Result<(), String>;
    async fn read(&self, namespace: &str, key: &str) -> Result<Vec<u8>, String>;
}

pub struct FileBasedMemory {
    base_dir: std::path::PathBuf,
}

impl FileBasedMemory {
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Self {
        FileBasedMemory {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn secure_join(&self, elem: &[&str]) -> Result<std::path::PathBuf, String> {
        let mut path = self.base_dir.clone();
        for e in elem {
            if e.contains("..") {
                return Err("path traversal detected (..)".to_string());
            }
            path.push(e);
        }
        if !path.starts_with(&self.base_dir) {
            return Err("invalid path: attempts to traverse outside base directory".to_string());
        }
        Ok(path)
    }
}

#[async_trait]
impl OHCMemory for FileBasedMemory {
    async fn write(&self, namespace: &str, key: &str, data: &[u8]) -> Result<(), String> {
        let dir = self.secure_join(&[namespace])?;
        tokio::fs::create_dir_all(&dir).await.map_err(|e| e.to_string())?;
        
        let path = self.secure_join(&[namespace, key])?;
        tokio::fs::write(path, data).await.map_err(|e| e.to_string())?;
        
        Ok(())
    }

    async fn read(&self, namespace: &str, key: &str) -> Result<Vec<u8>, String> {
        let path = self.secure_join(&[namespace, key])?;
        tokio::fs::read(path).await.map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_embedding_record_serialization() {
        let now = Utc.with_ymd_and_hms(2026, 4, 26, 0, 0, 0).unwrap();
        let record = EmbeddingRecord {
            id: "rec1".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "Hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TEXT".to_string(),
            created_at: now,
        };

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: EmbeddingRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.embedding, deserialized.embedding);
        assert_eq!(record.created_at, deserialized.created_at);
    }

    #[tokio::test]
    async fn test_file_based_memory() {
        let dir = "/tmp/test_memory";
        let mem = FileBasedMemory::new(dir);
        let namespace = "test_ns";
        let key = "test_key";
        let data = b"hello memory";

        mem.write(namespace, key, data).await.unwrap();

        let read_data = mem.read(namespace, key).await.unwrap();
        assert_eq!(read_data, data);

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_sqlite_semantic_search_and_conflict_resolution() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE consolidated_memory (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = SqliteVectorRepository::new(pool);
        let now = Utc::now();

        // Insert older memory
        let older = EmbeddingRecord {
            id: "older".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "Price is 50".to_string(),
            embedding: vec![1.0, 0.0, 0.0],
            source_type: "OBSERVATION".to_string(),
            created_at: now - chrono::Duration::hours(1),
        };

        // Insert newer memory that conflicts (> 0.95 similarity, e.g., 0.999)
        let newer = EmbeddingRecord {
            id: "newer".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent2".to_string(),
            content: "Price is 55".to_string(),
            embedding: vec![0.999, 0.0, 0.0],
            source_type: "OBSERVATION".to_string(),
            created_at: now,
        };

        // Insert completely different memory
        let different = EmbeddingRecord {
            id: "different".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "Color is blue".to_string(),
            embedding: vec![0.0, 1.0, 0.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: now - chrono::Duration::hours(2),
        };

        repo.upsert(&older).await.unwrap();
        repo.upsert(&newer).await.unwrap();
        repo.upsert(&different).await.unwrap();

        // 1. Cross-department verification (agent1 and agent2 records retrieved)
        let search = repo.semantic_search("org1", &[1.0, 0.0, 0.0], 10).await.unwrap();
        assert_eq!(search.len(), 3);
        assert_eq!(search[0].id, "older");
        assert_eq!(search[1].id, "newer");

        // 2. Conflict resolution
        let pruned = repo.resolve_conflicts("org1").await.unwrap();
        assert_eq!(pruned, 1);

        // older should be pruned, newer kept
        let after_conflict = repo.semantic_search("org1", &[1.0, 0.0, 0.0], 10).await.unwrap();
        assert_eq!(after_conflict.len(), 2);
        assert!(after_conflict.iter().any(|r| r.id == "newer"));
        assert!(after_conflict.iter().any(|r| r.id == "different"));

        // 3. Stale pruning test
        repo.prune_stale_context(&["TASK_SUMMARY"], now - chrono::Duration::hours(1)).await.unwrap();
        let after_stale = repo.semantic_search("org1", &[0.0, 1.0, 0.0], 10).await.unwrap();
        assert_eq!(after_stale.len(), 1); // "different" is removed because it's TASK_SUMMARY and older
        assert_eq!(after_stale[0].id, "newer"); // newer remains
    }
}


pub struct PruneWorker {
    store: std::sync::Arc<dyn VectorStore>,
}

impl PruneWorker {
    pub fn new(store: std::sync::Arc<dyn VectorStore>) -> Self {
        PruneWorker { store }
    }

    pub async fn start(&self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600)); // Run every hour
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let older_than = Utc::now() - chrono::Duration::days(30);
                    if let Err(e) = self.store.prune_stale(older_than).await {
                        eprintln!("Failed to prune stale memory: {}", e);
                    }
                }
                _ = shutdown_rx.recv() => {
                    break;
                }
            }
        }
    }
}
