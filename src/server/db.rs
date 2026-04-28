use sqlx::PgPool;
use std::env;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::path::Path;

pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let pool = PgPool::connect(&database_url).await?;

        Ok(DB { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running migrations...");
        
        sqlx::query("CREATE EXTENSION IF NOT EXISTS vector;")
            .execute(&self.pool)
            .await?;

        let migrator = sqlx::migrate::Migrator::new(Path::new("src/server/migrations")).await?;
        migrator.run(&self.pool).await?;

        Ok(())
    }

    pub async fn delete_stale_sessions(&self, threshold: DateTime<Utc>) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let rows = sqlx::query("SELECT session_id, context_data FROM agent_session_data WHERE last_accessed < $1")
            .bind(threshold)
            .fetch_all(&self.pool)
            .await?;
            
        let mut result = Vec::new();
        for row in rows {
            let id: String = row.get("session_id");
            let data: String = row.get("context_data");
            result.push((id, data));
        }
        
        sqlx::query("DELETE FROM agent_session_data WHERE last_accessed < $1")
            .bind(threshold)
            .execute(&self.pool)
            .await?;
            
        Ok(result)
    }

    pub async fn inject_truth(&self, memory_id: &str, context: &str, embedding: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, context, embedding) VALUES ($1, $2, $3) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding")
            .bind(memory_id)
            .bind(context)
            .bind(embedding)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn get_completed_tasks(&self) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
        let rows = sqlx::query("SELECT id, organization_id, payload FROM tasks WHERE status = 'COMPLETED' AND auto_dreamed = FALSE LIMIT 50")
            .fetch_all(&self.pool)
            .await?;
            
        let mut result = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let org_id: String = row.get("organization_id");
            let payload: String = row.get("payload");
            result.push((id, org_id, payload));
        }
        
        Ok(result)
    }

    pub async fn insert_agent_memory(&self, id: &str, org_id: &str, task_id: &str, content: &str, embedding: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("INSERT INTO agent_memories (id, organization_id, task_id, raw_content, summary_embedding) VALUES ($1, $2, $3, $4, $5)")
            .bind(id)
            .bind(org_id)
            .bind(task_id)
            .bind(content)
            .bind(embedding)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }

    pub async fn mark_task_auto_dreamed(&self, task_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE tasks SET auto_dreamed = TRUE WHERE id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_new_fails_without_server() {
        unsafe { std::env::set_var("DATABASE_URL", "postgres://localhost:54321/nonexistent") };
        let db = DB::new().await;
        assert!(db.is_err());
    }
}
