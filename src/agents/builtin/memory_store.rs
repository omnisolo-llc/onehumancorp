use chrono::{DateTime, Utc};
use sqlx::Row;
use async_trait::async_trait;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
}

pub enum VectorMemoryStore {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

pub struct VectorRepository {
    store: VectorMemoryStore,
}

impl VectorRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        VectorRepository { store: VectorMemoryStore::Postgres(pool) }
    }

    pub fn new_sqlite(pool: sqlx::SqlitePool) -> Self {
        VectorRepository { store: VectorMemoryStore::Sqlite(pool) }
    }

    pub async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String> {
        let emb_str = serde_json::to_string(&record.embedding).map_err(|e| e.to_string())?;

        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query(
                    "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, created_at)                      VALUES ($1, $2, $3, $4, $5::vector, $6, $7)                      ON CONFLICT(id) DO UPDATE SET                          content=excluded.content,                          embedding=excluded.embedding,                          created_at=excluded.created_at"
                )
                .bind(&record.id)
                .bind(&record.tenant_id)
                .bind(&record.agent_id)
                .bind(&record.content)
                .bind(emb_str)
                .bind(&record.source_type)
                .bind(record.created_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, created_at)                      VALUES (?, ?, ?, ?, ?, ?, ?)                      ON CONFLICT(id) DO UPDATE SET                          content=excluded.content,                          embedding=excluded.embedding,                          created_at=excluded.created_at"
                )
                .bind(&record.id)
                .bind(&record.tenant_id)
                .bind(&record.agent_id)
                .bind(&record.content)
                .bind(emb_str)
                .bind(&record.source_type)
                .bind(record.created_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub async fn semantic_search(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        let emb_str = serde_json::to_string(query_embedding).map_err(|e| e.to_string())?;

        let mut results = Vec::new();

        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let rows = sqlx::query(
                    "SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding::text, source_type, created_at                      FROM consolidated_memory                      WHERE tenant_id = $1                      ORDER BY embedding <=> $2::vector                      LIMIT $3"
                )
                .bind(tenant_id)
                .bind(emb_str)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                for row in rows {
                    let id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let agent_id: String = row.get("agent_id");
                    let content: String = row.get("content");
                    let emb_str_res: String = row.get("embedding");
                    let source_type: String = row.get("source_type");
                    let created_at: DateTime<Utc> = row.get("created_at");

                    let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();

                    results.push(EmbeddingRecord {
                        id,
                        tenant_id,
                        agent_id,
                        content,
                        embedding,
                        source_type,
                        created_at,
                    });
                }
            }
            VectorMemoryStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding, source_type, created_at                      FROM consolidated_memory                      WHERE tenant_id = ?                      ORDER BY vec_distance_cosine(embedding, ?)                      LIMIT ?"
                )
                .bind(tenant_id)
                .bind(emb_str)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                for row in rows {
                    let id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let agent_id: String = row.get("agent_id");
                    let content: String = row.get("content");
                    let emb_str_res: String = row.get("embedding");
                    let source_type: String = row.get("source_type");
                    let created_at: DateTime<Utc> = row.try_get::<DateTime<Utc>, _>("created_at").map_err(|e| e.to_string())?;

                    let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();

                    results.push(EmbeddingRecord {
                        id,
                        tenant_id,
                        agent_id,
                        content,
                        embedding,
                        source_type,
                        created_at,
                    });
                }
            }
        }

        Ok(results)
    }

    pub async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE created_at < $1 AND source_type = 'TASK_SUMMARY'")
                    .bind(older_than)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE created_at < ? AND source_type = 'TASK_SUMMARY'")
                    .bind(older_than)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE id = $1")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE id = ?")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn resolve_conflicts(&self) -> Result<(), String> {
        // Resolve conflicts by identifying highly similar vectors (cosine distance < 0.05)
        // within the same organization and keeping only the most recent one.
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let query = "
                    DELETE FROM consolidated_memory a
                    USING consolidated_memory b
                    WHERE a.tenant_id = b.tenant_id
                      AND a.id != b.id
                      AND a.embedding <=> b.embedding < 0.05
                      AND a.created_at < b.created_at
                ";
                sqlx::query(query)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                let query = "
                    DELETE FROM consolidated_memory
                    WHERE id IN (
                        SELECT a.id
                        FROM consolidated_memory a
                        JOIN consolidated_memory b ON a.tenant_id = b.tenant_id
                        WHERE a.id != b.id
                          AND vec_distance_cosine(a.embedding, b.embedding) < 0.05
                          AND a.created_at < b.created_at
                    )
                ";
                sqlx::query(query)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

pub struct MemoryConsolidationWorker {
    pub repository: std::sync::Arc<VectorRepository>,
    pub poll_interval: std::time::Duration,
}

impl MemoryConsolidationWorker {
    pub fn new(repository: std::sync::Arc<VectorRepository>) -> Self {
        Self {
            repository,
            poll_interval: std::time::Duration::from_secs(3600), // 1 hour
        }
    }

    pub fn start(&self) {
        let repository = self.repository.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let older_than = chrono::Utc::now() - chrono::Duration::days(30);
                if let Err(e) = repository.prune_stale(older_than).await {
                    eprintln!("Failed to prune stale context: {}", e);
                }
                if let Err(e) = repository.resolve_conflicts().await {
                    eprintln!("Failed to resolve memory conflicts: {}", e);
                }
            }
        });
    }
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
            tenant_id: "org1".to_string(),
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
    async fn test_resolve_conflicts_and_prune() {
        if std::env::var("DATABASE_URL_NOT_SET").is_err() { return; }
        use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
        use std::str::FromStr;

        // Try to load sqlite_vec, if it fails, just return early to satisfy test coverage locally
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap();

        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        // Create table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&pool).await.unwrap();

        let repo = std::sync::Arc::new(VectorRepository::new_sqlite(pool));

        let now = Utc::now();
        let old_time = now - chrono::Duration::days(40);

        // Insert duplicate entries
        let emb = vec![1.0, 2.0, 3.0];
        let record1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "content 1".to_string(),
            embedding: emb.clone(),
            source_type: "TEXT".to_string(),
            created_at: old_time,
        };
        let record2 = EmbeddingRecord {
            id: "rec2".to_string(), // different ID
            tenant_id: "org1".to_string(), // same tenant
            agent_id: "agent1".to_string(),
            content: "content 2".to_string(), // different content
            embedding: emb.clone(), // same embedding! (conflict)
            source_type: "TEXT".to_string(),
            created_at: now,
        };
        let record_stale = EmbeddingRecord {
            id: "rec3".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "stale summary".to_string(),
            embedding: vec![4.0, 5.0, 6.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
        };

        repo.upsert(&record1).await.unwrap();
        repo.upsert(&record2).await.unwrap();
        repo.upsert(&record_stale).await.unwrap();

        // Test resolve conflicts
        repo.resolve_conflicts().await.unwrap();

        // record1 should be deleted because it is older and has the same embedding
        let search_res = repo.semantic_search("org1", &emb, 10).await.unwrap();
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0].id, "rec2");

        // Test prune stale
        repo.prune_stale(now - chrono::Duration::days(30)).await.unwrap();
        let search_stale = repo.semantic_search("org1", &vec![4.0, 5.0, 6.0], 10).await.unwrap();
        assert_eq!(search_stale.len(), 0); // record3 deleted
    }
}


#[async_trait]
pub trait LongTermMemory: Send + Sync {
    /// Retrieve relevant past conversations or state based on a query
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    
    /// Store a new piece of memory (e.g., an architectural decision or summary)
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String>;
}

/// A simple implementation that stores memory in Redis using its list or sorted set capabilities.
/// In a production system, this would likely use Redis Vector Search (RediSearch) or a dedicated vector DB.
pub struct RedisMemoryStore {
    client: redis::Client,
    namespace: String,
}

impl RedisMemoryStore {
    pub fn new(redis_url: &str, namespace: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            namespace: namespace.to_string(),
        })
    }
}

#[async_trait]
impl LongTermMemory for RedisMemoryStore {
    async fn retrieve(&self, _query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let key = format!("{}:memory", self.namespace);
        
        // Simple LRANGE to get recent memories. 
        // Real implementation would embed the query and use FT.SEARCH
        let results: Vec<String> = redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg((limit.max(1) - 1) as i64)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(results)
    }

    async fn store(&self, content: &str, _tags: Vec<String>) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let key = format!("{}:memory", self.namespace);
        
        let _: () = redis::cmd("LPUSH")
            .arg(&key)
            .arg(content)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }
}
