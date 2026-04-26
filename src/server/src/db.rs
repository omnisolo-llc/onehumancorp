use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use sqlx::Row;
use chrono::{DateTime, Utc};

pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(DB { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running migrations...");
        
        sqlx::query("CREATE TABLE IF NOT EXISTS schema_migrations (filename TEXT PRIMARY KEY, applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, context_data TEXT, last_accessed TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS swarm_truth_embeddings (memory_id TEXT PRIMARY KEY, context TEXT, embedding TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, organization_id TEXT, status TEXT, payload TEXT, auto_dreamed BOOLEAN DEFAULT FALSE)")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_memories (id TEXT PRIMARY KEY, organization_id TEXT, task_id TEXT, raw_content TEXT, summary_embedding TEXT)")
            .execute(&self.pool)
            .await?;

        sqlx::query("ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system'")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS swarm_memory (key TEXT PRIMARY KEY, value TEXT NOT NULL, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, status TEXT NOT NULL, payload TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system', synced_to_cloud BOOLEAN DEFAULT FALSE)")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_status (agent_id TEXT PRIMARY KEY, role TEXT NOT NULL, status TEXT NOT NULL, last_heartbeat TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS capability_plugins (plugin_id TEXT PRIMARY KEY, name TEXT NOT NULL, version TEXT NOT NULL, manifest_url TEXT NOT NULL, status TEXT NOT NULL, registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT NOT NULL, vector_embedding BYTEA, source_plugin TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE TABLE IF NOT EXISTS telemetry_buffer (id SERIAL PRIMARY KEY, metric_type TEXT NOT NULL, payload TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, organization_id TEXT DEFAULT 'system')")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id TEXT PRIMARY KEY, organization_id TEXT, parent_task_id TEXT, payload TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, worker_id TEXT)")
            .execute(&self.pool)
            .await?;

        sqlx::query("ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS scheduled_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS completed_at TIMESTAMP")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, parent_id TEXT, epic_id TEXT, title TEXT NOT NULL, status TEXT NOT NULL, assigned_agent TEXT, payload TEXT NOT NULL, organization_id TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool)
            .await?;

        self.enable_rls_all().await?;

        Ok(())
    }

    async fn enable_rls_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tables = vec![
            "agent_session_data", "swarm_truth_embeddings", "tasks", "agent_memories",
            "swarm_memory", "agent_missions", "agent_status", "capability_plugins",
            "swarm_memory_embeddings", "telemetry_buffer", "sub_agent_queue", "shared_tasks"
        ];

        for table in tables {
            sqlx::query(&format!("ALTER TABLE {} ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system'", table))
                .execute(&self.pool)
                .await?;

            println!("Enabling RLS on {}", table);
            sqlx::query(&format!("ALTER TABLE {} ENABLE ROW LEVEL SECURITY", table))
                .execute(&self.pool)
                .await?;

            sqlx::query(&format!("DROP POLICY IF EXISTS {}_tenant_isolation ON {}", table, table))
                .execute(&self.pool)
                .await?;

            // Allow access if organization_id matches OR if context is 'system'
            sqlx::query(&format!(
                "CREATE POLICY {}_tenant_isolation ON {} USING (organization_id = current_setting('ohc.current_organization_id', true) OR current_setting('ohc.current_organization_id', true) = 'system')",
                table, table
            ))
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn set_organization_context<'a, E>(&self, executor: E, org_id: &str) -> Result<(), Box<dyn std::error::Error>>
    where E: sqlx::Executor<'a, Database = sqlx::Postgres>
    {
        sqlx::query("SELECT set_config('ohc.current_organization_id', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
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
        // Apply deterministic encryption to content for standalone mode if required
        let final_content = if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            crate::crypto::encrypt_deterministic(content)
        } else {
            content.to_string()
        };

        sqlx::query("INSERT INTO agent_memories (id, organization_id, task_id, raw_content, summary_embedding) VALUES ($1, $2, $3, $4, $5)")
            .bind(id)
            .bind(org_id)
            .bind(task_id)
            .bind(final_content)
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
        std::env::set_var("DATABASE_URL", "postgres://localhost:54321/nonexistent");
        let db = DB::new().await;
        assert!(db.is_err());
    }
}
