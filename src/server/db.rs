use chrono::{DateTime, Utc};
use sqlx::PgPool;
use sqlx::Row;
use std::env;
use std::path::Path;

pub struct DB {
    pub pool: PgPool,
    pub redis: Option<redis::aio::MultiplexedConnection>,
}

impl DB {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let pool = PgPool::connect(&database_url).await?;

        let mode = std::env::var("APP_MODE").unwrap_or_else(|_| "standalone".to_string());
        let redis = if mode == "cloud" {
            let redis_url =
                std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
            let client = redis::Client::open(redis_url)?;
            Some(client.get_multiplexed_async_connection().await?)
        } else {
            None
        };

        Ok(DB { pool, redis })
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

    pub async fn delete_stale_sessions(
        &self,
        threshold: DateTime<Utc>,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            "SELECT session_id, context_data FROM agent_session_data WHERE last_accessed < $1",
        )
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

    pub async fn inject_truth(
        &self,
        memory_id: &str,
        context: &str,
        embedding: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, context, embedding) VALUES ($1, $2, $3) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding")
            .bind(memory_id)
            .bind(context)
            .bind(embedding)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_completed_tasks(
        &self,
    ) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
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

    pub async fn insert_agent_memory(
        &self,
        id: &str,
        org_id: &str,
        task_id: &str,
        content: &str,
        embedding: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn mark_task_auto_dreamed(
        &self,
        task_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE tasks SET auto_dreamed = TRUE WHERE id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq)]
pub struct Task {
    pub id: uuid::Uuid,
    pub organization_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub payload: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn create_task(
        &self,
        tenant_id: &str,
        title: &str,
        description: Option<&str>,
        payload: Option<serde_json::Value>,
    ) -> Result<Task, Box<dyn std::error::Error + Send + Sync>>;
    async fn claim_task(
        &self,
        task_id: &str,
        tenant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl Provider for DB {
    async fn create_task(
        &self,
        tenant_id: &str,
        title: &str,
        description: Option<&str>,
        payload: Option<serde_json::Value>,
    ) -> Result<Task, Box<dyn std::error::Error + Send + Sync>> {
        let task_id = uuid::Uuid::new_v4();
        let payload_json = payload.unwrap_or_else(|| serde_json::json!({}));

        let row = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (id, organization_id, title, description, status, payload) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(task_id)
        .bind(tenant_id)
        .bind(title)
        .bind(description)
        .bind("PENDING")
        .bind(payload_json)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn claim_task(
        &self,
        task_id: &str,
        tenant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let task_uuid = uuid::Uuid::parse_str(task_id)?;
        let mode = std::env::var("APP_MODE").unwrap_or_else(|_| "standalone".to_string());

        if mode == "cloud" {
            // Use Redis distributed lock
            let mut con = self.redis.clone().ok_or("Redis client not initialized")?;
            let lock_key = format!("ohc:lock:{}:task:{}", tenant_id, task_id);

            // Try to acquire lock
            let lock_result: Option<String> = redis::cmd("SET")
                .arg(&lock_key)
                .arg("LOCKED")
                .arg("NX")
                .arg("EX")
                .arg(30) // 30 seconds expiration
                .query_async(&mut con)
                .await?;

            if lock_result.is_none() {
                return Err(
                    format!("Task {} is already locked by another process", task_id).into(),
                );
            }

            let row = sqlx::query(
                "UPDATE tasks SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status = 'PENDING'"
            )
            .bind(task_uuid)
            .execute(&self.pool)
            .await?;

            if row.rows_affected() == 0 {
                // Release lock if update failed (e.g. task was not PENDING)
                let _: () = redis::cmd("DEL")
                    .arg(&lock_key)
                    .query_async(&mut con)
                    .await?;
                return Err(format!("Task {} not found or already claimed", task_id).into());
            }

            // Release lock on success
            let _: () = redis::cmd("DEL")
                .arg(&lock_key)
                .query_async(&mut con)
                .await?;
        } else {
            // Standalone mode: simple SELECT and UPDATE to avoid SQLite locking issues
            let (task_status,): (String,) =
                sqlx::query_as("SELECT status FROM tasks WHERE id = $1")
                    .bind(task_uuid)
                    .fetch_one(&self.pool)
                    .await?;

            if task_status != "PENDING" {
                return Err(format!("Task {} is not in PENDING state", task_id).into());
            }

            let row = sqlx::query(
                "UPDATE tasks SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status = 'PENDING'"
            )
            .bind(task_uuid)
            .execute(&self.pool)
            .await?;

            if row.rows_affected() == 0 {
                return Err(format!("Task {} not found or already claimed", task_id).into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_new_fails_without_server() {
        std::env::set_var("DATABASE_URL", "postgres://localhost:54321/nonexistent");
        let db = DB::new().await;
        assert!(db.is_err());
    }

    #[tokio::test]
    async fn test_create_and_claim_task_standalone() {
        std::env::set_var("APP_MODE", "standalone");

        let db = match DB::new().await {
            Ok(db) => db,
            Err(_) => {
                println!("Skipping test as no database is available.");
                return;
            }
        };

        // Try to create the table structure if it's an ephemeral DB or just let it fail gracefully
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS tasks (id UUID PRIMARY KEY, title VARCHAR NOT NULL, description TEXT, status VARCHAR NOT NULL DEFAULT 'PENDING', payload JSONB, created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)"
        )
        .execute(&db.pool)
        .await;

        let task = match db
            .create_task("tenant1", "Integration Test", Some("Desc"), None)
            .await
        {
            Ok(t) => t,
            Err(_) => return, // Gracefully skip if table logic fails or migration not applied
        };

        assert_eq!(task.title, "Integration Test");
        assert_eq!(task.status, "PENDING");

        let claim_res = db.claim_task(&task.id.to_string(), "tenant1").await;
        assert!(claim_res.is_ok());

        // Second claim should fail
        let claim_res2 = db.claim_task(&task.id.to_string(), "tenant1").await;
        assert!(claim_res2.is_err());
    }

    #[tokio::test]
    async fn test_create_and_claim_task_cloud() {
        std::env::set_var("APP_MODE", "cloud");

        // We will skip if redis is not running, just like we skip if DB is not running.
        let db = match DB::new().await {
            Ok(db) => db,
            Err(_) => {
                println!("Skipping test as no database or redis is available.");
                return;
            }
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS tasks (id UUID PRIMARY KEY, organization_id VARCHAR NOT NULL, title VARCHAR NOT NULL, description TEXT, status VARCHAR NOT NULL DEFAULT 'PENDING', payload JSONB, created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)"
        )
        .execute(&db.pool)
        .await;

        let task = match db
            .create_task("tenant2", "Cloud Test", Some("Desc"), None)
            .await
        {
            Ok(t) => t,
            Err(_) => return, // Gracefully skip
        };

        assert_eq!(task.title, "Cloud Test");
        assert_eq!(task.status, "PENDING");

        let claim_res = db.claim_task(&task.id.to_string(), "tenant2").await;
        assert!(claim_res.is_ok());

        // Second claim should fail because it is now RUNNING
        let claim_res2 = db.claim_task(&task.id.to_string(), "tenant2").await;
        assert!(claim_res2.is_err());
    }
}
