use std::sync::Arc;
use crate::db::{DB, DbStore};
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct AutoDreamMemory {
    pub content: String,
    pub source_type: String,
    pub agent_id: String,
    pub organization_id: String,
}

pub struct AutoDreamWorker {
    pub memory_dir: String,
    pub db: Arc<DB>,
}

impl AutoDreamWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db, memory_dir: ".agent-task/memory".to_string() }
    }

    pub fn with_dir(db: Arc<DB>, dir: &str) -> Self {
        Self { db, memory_dir: dir.to_string() }
    }

    pub async fn process_memories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries = match fs::read_dir(&self.memory_dir).await {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("yml") {
                let content = fs::read_to_string(&path).await?;
                let memory: AutoDreamMemory = match serde_yaml::from_str(&content) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Failed to parse yaml file {}: {}", path.display(), e);
                        continue;
                    }
                };

                let id = uuid::Uuid::new_v4().to_string();

                let result = match &self.db.store {
                    DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query(
                            "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, source_type) VALUES ($1, $2, $3, $4, $5)"
                        )
                        .bind(&id)
                        .bind(&memory.organization_id)
                        .bind(&memory.agent_id)
                        .bind(&memory.content)
                        .bind(&memory.source_type)
                        .execute(sqlite_pool)
                        .await.map(|_| ())
                    }
                    DbStore::Postgres => {
                        sqlx::query(
                            "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, source_type) VALUES ($1, $2, $3, $4, $5)"
                        )
                        .bind(&id)
                        .bind(&memory.organization_id)
                        .bind(&memory.agent_id)
                        .bind(&memory.content)
                        .bind(&memory.source_type)
                        .execute(&self.db.pool)
                        .await.map(|_| ())
                    }
                };

                match result {
                    Ok(_) => {
                        fs::remove_file(path.clone()).await.unwrap_or_else(|e| eprintln!("Failed to delete file {}: {}", path.display(), e));
                    }
                    Err(e) => {
                        eprintln!("Failed to insert memory into database: {}", e);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::{DB, DbStore};

    #[tokio::test]
    async fn test_process_memories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mem_dir = temp_dir.path().join("memory");
        let _ = tokio::fs::create_dir_all(&mem_dir).await;
        let mem_dir_str = mem_dir.to_str().unwrap();

        let dummy_yaml = r#"
content: "test_content"
source_type: "test_type"
agent_id: "agent_1"
organization_id: "org_1"
"#;
        let file1 = format!("{}/test1.yml", mem_dir_str);
        let _ = tokio::fs::write(&file1, dummy_yaml).await;

        let bad_yaml = r#"
content: "test_content"
source_type
"#;
        let file2 = format!("{}/test2.yml", mem_dir_str);
        let _ = tokio::fs::write(&file2, bad_yaml).await;


        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT, agent_id TEXT, content TEXT, source_type TEXT, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
            .execute(&pool)
            .await
            .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap_or_else(|_| panic!("Failed lazy"));
        let db = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(pool.clone()) });

        let worker = AutoDreamWorker::with_dir(db.clone(), mem_dir_str);
        let _ = worker.process_memories().await;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM consolidated_memory").fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 1);

        // Ensure successful file is deleted and bad file is not deleted.
        assert!(!std::path::Path::new(&file1).exists());
        assert!(std::path::Path::new(&file2).exists());

        let _ = tokio::fs::remove_file(&file2).await;
    }
}
