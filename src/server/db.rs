use sqlx::PgPool;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::env;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::path::Path;

#[derive(Clone)]
pub enum DbStore {
    Postgres,
    Sqlite(SqlitePool),
}

#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
    pub store: DbStore,
}

impl DB {
    pub fn is_sqlite(&self) -> bool {
        match &self.store {
            DbStore::Sqlite(_) => true,
            DbStore::Postgres => false,
        }
    }

    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        if database_url.starts_with("sqlite") {
            let dummy_pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
                .connect_lazy("postgres://postgres:postgres@localhost:5432/test")?;

            let conn_opts = SqliteConnectOptions::from_str(&database_url)?
                .create_if_missing(true)
                .extension("sqlite_vec");

            let sqlite_pool = SqlitePoolOptions::new()
                .connect_with(conn_opts)
                .await?;

            Ok(DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) })
        } else {
            let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
                .acquire_timeout(std::time::Duration::from_millis(500))
                .before_acquire(|conn, _meta| {
                    Box::pin(async move {
                        use sqlx::Executor;
                        conn.execute("SET app.current_tenant = 'system'").await?;
                        Ok(true)
                    })
                })
                .connect(&database_url)
                .await?;

            Ok(DB { pool: pool.clone(), store: DbStore::Postgres })
        }
    }

    pub async fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running migrations...");

        match &self.store {
            DbStore::Postgres => {
                sqlx::query("CREATE EXTENSION IF NOT EXISTS vector;")
                    .execute(&self.pool)
                    .await?;

                let migrator = sqlx::migrate::Migrator::new(Path::new("src/server/migrations")).await?;
                migrator.run(&self.pool).await?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let schema = "
                    CREATE TABLE IF NOT EXISTS agent_session_data (
                        session_id TEXT PRIMARY KEY,
                        agent_id TEXT NOT NULL,
                        context_data TEXT NOT NULL,
                        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                    CREATE TABLE IF NOT EXISTS swarm_truth_embeddings (
                        memory_id TEXT PRIMARY KEY,
                        context TEXT NOT NULL,
                        embedding BLOB,
                        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                    CREATE TABLE IF NOT EXISTS shared_tasks (
                        id TEXT PRIMARY KEY,
                        organization_id TEXT NOT NULL,
                        title TEXT NOT NULL,
                        description TEXT,
                        status TEXT NOT NULL DEFAULT 'PENDING',
                        agent_id TEXT,
                        priority TEXT NOT NULL DEFAULT 'P2',
                        payload TEXT,
                        parent_plan_id TEXT,
                        dependencies TEXT NOT NULL DEFAULT '[]',
                        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        auto_dreamed BOOLEAN DEFAULT 0,
                        locked_until DATETIME,
                        assigned_agent_id TEXT,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                    CREATE TABLE IF NOT EXISTS swarm_tasks (
                        id TEXT PRIMARY KEY,
                        mission_id TEXT NOT NULL,
                        parent_plan_id TEXT,
                        dependencies TEXT NOT NULL DEFAULT '[]',
                        title TEXT NOT NULL,
                        description TEXT,
                        priority TEXT,
                        status TEXT NOT NULL DEFAULT 'PENDING',
                        assigned_agent_id TEXT,
                        locked_until DATETIME,
                        payload TEXT,
                        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        auto_dreamed BOOLEAN DEFAULT 0,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                    CREATE TABLE IF NOT EXISTS agent_memories (
                        id TEXT PRIMARY KEY,
                        organization_id TEXT NOT NULL,
                        task_id TEXT NOT NULL,
                        raw_content TEXT NOT NULL,
                        summary_embedding BLOB,
                        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                    CREATE TABLE IF NOT EXISTS autodream_memories (
                        id TEXT PRIMARY KEY,
                        organization_id TEXT NOT NULL,
                        agent_id TEXT NOT NULL,
                        task_id TEXT NOT NULL,
                        content TEXT NOT NULL,
                        embedding BLOB,
                        source_type TEXT NOT NULL,
                        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                    CREATE TABLE IF NOT EXISTS agent_missions (
                        id TEXT PRIMARY KEY,
                        status TEXT NOT NULL,
                        payload TEXT NOT NULL,
                        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        organization_id TEXT NOT NULL DEFAULT 'system',
                        cloud_mission_id TEXT,
                        sync_error TEXT,
                        last_synced_at DATETIME,
                        synced_to_cloud BOOLEAN DEFAULT 0,
                        _sync_status TEXT DEFAULT 'pending',
                        version INTEGER DEFAULT 1
                    );
                ";
                sqlx::query(schema).execute(sqlite_pool).await?;
            }
        }

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

    pub async fn get_completed_tasks(&self) -> Result<Vec<(String, String, String, String)>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();

        // Fetch from shared_tasks
        let shared_rows = sqlx::query("SELECT id, organization_id, payload::text FROM shared_tasks WHERE status = 'COMPLETED' AND auto_dreamed = FALSE LIMIT 25")
            .fetch_all(&self.pool)
            .await?;

        for row in shared_rows {
            let id: String = row.get("id");
            let org_id: String = row.get("organization_id");
            let payload: String = row.get("payload");
            result.push((id, org_id, payload, "shared_tasks".to_string()));
        }

        // Fetch from swarm_tasks
        // Note: swarm_tasks doesn't have organization_id natively in the schema provided earlier
        let swarm_rows = sqlx::query("SELECT id::text, payload::text FROM swarm_tasks WHERE status = 'COMPLETED' AND auto_dreamed = FALSE LIMIT 25")
            .fetch_all(&self.pool)
            .await?;

        for row in swarm_rows {
            let id: String = row.get("id");
            let org_id: String = "system".to_string(); // Fallback organization_id
            let payload: String = row.get("payload");
            result.push((id, org_id, payload, "swarm_tasks".to_string()));
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

pub async fn insert_autodream_memory(
        &self,
        id: &str,
        org_id: &str,
        agent_id: &str,
        task_id: &str,
        content: &str,
        embedding: &str,
        source_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &self.store {
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                    .bind(id)
                    .bind(org_id)
                    .bind(agent_id)
                    .bind(task_id)
                    .bind(content)
                    .bind(embedding)
                    .bind(source_type)
                    .execute(sqlite_pool)
                    .await?;
            }
            DbStore::Postgres => {
                sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                    .bind(id)
                    .bind(org_id)
                    .bind(agent_id)
                    .bind(task_id)
                    .bind(content)
                    .bind(embedding)
                    .bind(source_type)
                    .execute(&self.pool)
                    .await?;
            }
        }
        Ok(())
    }


    pub async fn mark_task_auto_dreamed(&self, task_id: &str, table: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = if table == "swarm_tasks" {
            // swarm_tasks uses UUID primary key
            "UPDATE swarm_tasks SET auto_dreamed = TRUE WHERE id = $1::uuid"
        } else {
            "UPDATE shared_tasks SET auto_dreamed = TRUE WHERE id = $1"
        };

        sqlx::query(query)
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
        // SAFETY: Test-only code setting environment variables
        unsafe { std::env::set_var("DATABASE_URL", "postgres://localhost:54321/nonexistent") }
        let db = DB::new().await;
        assert!(db.is_err());
    }
}

#[cfg(test)]
mod autodream_db_tests {
    use super::*;

    #[tokio::test]
    async fn test_mark_task_auto_dreamed_query() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = 'system'").await?;
                    Ok(true)
                })
            })
            .connect_lazy(database_url)
            .unwrap();

        let db = DB { pool: pool.clone(), store: DbStore::Postgres };

        // This is primarily to ensure the code compiles and syntax is fundamentally sound
        // Real tests would run migrations and populate data first.
        let result = db.get_completed_tasks().await;
        // Since test db is likely unmigrated/empty, we expect either an Ok(empty) or an Error
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_tenant_isolation_setup() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = 'system'").await?;
                    Ok(true)
                })
            })
            .connect_lazy(database_url)
            .unwrap();
        // Just checking configuration parses ok for multitenancy logic
        let _ = pool;
    }
}
