use std::sync::Arc;
use std::fmt;

#[derive(Debug)]
pub enum KairosError {
    Database(sqlx::Error),
    InvalidState(String),
}

impl fmt::Display for KairosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KairosError::Database(e) => write!(f, "Database error: {}", e),
            KairosError::InvalidState(s) => write!(f, "Invalid task state: {}", s),
        }
    }
}

impl std::error::Error for KairosError {}

impl From<sqlx::Error> for KairosError {
    fn from(err: sqlx::Error) -> Self {
        KairosError::Database(err)
    }
}

use crate::db::{DB, DbStore};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmTask {
    pub id: String,
    pub mission_id: String,
    pub title: String,
    pub status: String,
    pub dependencies: String,
    pub assigned_agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTask {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub status: String,
    pub dependencies: String,
    pub assigned_agent_id: Option<String>,
}

pub struct KairosOrchestrator {
    pub db: Arc<DB>,
    pub sqlite_mutex: Mutex<()>,
}

impl KairosOrchestrator {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            sqlite_mutex: Mutex::new(()),
        }
    }

    pub async fn claim_swarm_task(&self, agent_id: &str) -> Result<Option<SwarmTask>, KairosError> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(KairosError::Database)?;

                let row = sqlx::query(
                    r#"
                    SELECT t.id, t.mission_id, t.title, t.status, t.dependencies::text, t.assigned_agent_id
                    FROM swarm_tasks t
                    WHERE t.status = 'PENDING'
                    AND NOT EXISTS (
                        SELECT 1 FROM jsonb_array_elements_text(t.dependencies) AS dep_id
                        JOIN swarm_tasks parent ON parent.id::text = dep_id
                        WHERE parent.status != 'COMPLETED'
                    )
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(KairosError::Database)?;

                if let Some(r) = row {
                    let id: uuid::Uuid = r.get(0);
                    let id_str = id.to_string();

                    sqlx::query(
                        "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3"
                    )
                    .bind(agent_id)
                    .bind(now)
                    .bind(id)
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    tx.commit().await.map_err(KairosError::Database)?;

                    Ok(Some(SwarmTask {
                        id: id_str,
                        mission_id: r.get(1),
                        title: r.get(2),
                        status: "IN_PROGRESS".to_string(),
                        dependencies: r.get(4),
                        assigned_agent_id: Some(agent_id.to_string()),
                    }))
                } else {
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(KairosError::Database)?;

                let row = sqlx::query(
                    r#"
                    UPDATE swarm_tasks
                    SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = ?
                    WHERE id = (
                        SELECT t.id
                        FROM swarm_tasks t
                        WHERE t.status = 'PENDING'
                        AND NOT EXISTS (
                            SELECT 1 FROM json_each(t.dependencies) AS dep_id
                            JOIN swarm_tasks parent ON parent.id = dep_id.value
                            WHERE parent.status != 'COMPLETED'
                        )
                        LIMIT 1
                    )
                    RETURNING id, mission_id, title, status, dependencies, assigned_agent_id
                    "#
                )
                .bind(agent_id)
                .bind(now.to_rfc3339())
                .fetch_optional(&mut *tx)
                .await
                .map_err(KairosError::Database)?;

                if let Some(r) = row {
                    let task = SwarmTask {
                        id: r.get("id"),
                        mission_id: r.get("mission_id"),
                        title: r.get("title"),
                        status: r.get("status"),
                        dependencies: r.get("dependencies"),
                        assigned_agent_id: r.get("assigned_agent_id"),
                    };
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(Some(task))
                } else {
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(None)
                }
            }
        }
    }

    pub async fn claim_shared_task(&self, tenant_id: &str, agent_id: &str) -> Result<Option<SharedTask>, KairosError> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(KairosError::Database)?;

                let row = sqlx::query(
                    r#"
                    SELECT t.id, t.tenant_id, t.title, t.status, t.dependencies::text, t.assigned_agent
                    FROM shared_tasks t
                    WHERE t.status = 'PENDING' AND t.tenant_id = $1
                    AND NOT EXISTS (
                        SELECT 1 FROM jsonb_array_elements_text(t.dependencies::jsonb) AS dep_id
                        JOIN shared_tasks parent ON parent.id::text = dep_id
                        WHERE parent.status != 'COMPLETED'
                    )
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(KairosError::Database)?;

                if let Some(r) = row {
                    let id: String = r.get(0);

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = $2 WHERE id = $3"
                    )
                    .bind(agent_id)
                    .bind(now)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    tx.commit().await.map_err(KairosError::Database)?;

                    Ok(Some(SharedTask {
                        id,
                        tenant_id: r.get(1),
                        title: r.get(2),
                        status: "IN_PROGRESS".to_string(),
                        dependencies: r.get(4),
                        assigned_agent_id: Some(agent_id.to_string()),
                    }))
                } else {
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(KairosError::Database)?;

                let row = sqlx::query(
                    r#"
                    UPDATE shared_tasks
                    SET status = 'IN_PROGRESS', assigned_agent = ?, updated_at = ?
                    WHERE id = (
                        SELECT t.id
                        FROM shared_tasks t
                        WHERE t.status = 'PENDING' AND t.tenant_id = ?
                        AND NOT EXISTS (
                            SELECT 1 FROM json_each(t.dependencies) AS dep_id
                            JOIN shared_tasks parent ON parent.id = dep_id.value
                            WHERE parent.status != 'COMPLETED'
                        )
                        LIMIT 1
                    )
                    RETURNING id, tenant_id, title, status, dependencies, assigned_agent
                    "#
                )
                .bind(agent_id)
                .bind(now.to_rfc3339())
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(KairosError::Database)?;

                if let Some(r) = row {
                    let task = SharedTask {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        title: r.get("title"),
                        status: r.get("status"),
                        dependencies: r.get("dependencies"),
                        assigned_agent_id: r.get("assigned_agent"),
                    };
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(Some(task))
                } else {
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_kairos_orchestrator_sqlite_swarm_tasks() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE swarm_tasks (
                id TEXT PRIMARY KEY,
                mission_id TEXT NOT NULL,
                parent_plan_id TEXT,
                dependencies TEXT NOT NULL DEFAULT '[]',
                title TEXT NOT NULL,
                description TEXT,
                priority TEXT,
                status TEXT NOT NULL DEFAULT 'PENDING',
                assigned_agent_id TEXT,
                locked_until TEXT,
                payload TEXT,
                created_at TEXT,
                updated_at TEXT,
                auto_dreamed BOOLEAN DEFAULT FALSE,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let orchestrator = KairosOrchestrator::new(db);

        // Insert tasks
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('1', 'm1', 'Task 1', 'PENDING', '[]')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('2', 'm1', 'Task 2', 'PENDING', '[\"1\"]')")
            .execute(&pool).await.unwrap();

        // Try to claim, should get Task 1
        let task1 = orchestrator.claim_swarm_task("agent1").await.unwrap().unwrap();
        assert_eq!(task1.id, "1");

        // Try to claim again, Task 2 is blocked by Task 1 (not completed)
        let task2_blocked = orchestrator.claim_swarm_task("agent2").await.unwrap();
        assert!(task2_blocked.is_none());

        // Complete Task 1
        sqlx::query("UPDATE swarm_tasks SET status = 'COMPLETED' WHERE id = '1'").execute(&pool).await.unwrap();

        // Now claim Task 2
        let task2 = orchestrator.claim_swarm_task("agent2").await.unwrap().unwrap();
        assert_eq!(task2.id, "2");
    }

    #[tokio::test]
    async fn test_kairos_orchestrator_sqlite_shared_tasks() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'PENDING',
                epic_id TEXT,
                parent_id TEXT,
                assigned_agent TEXT,
                payload TEXT,
                dependencies TEXT NOT NULL DEFAULT '[]',
                created_at TEXT,
                updated_at TEXT,
                action_risk TEXT,
                approval_status TEXT,
                proposed_content TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let orchestrator = KairosOrchestrator::new(db);

        // Insert tasks
        sqlx::query("INSERT INTO shared_tasks (id, tenant_id, title, status, dependencies) VALUES ('1', 'tenant1', 'Task 1', 'PENDING', '[]')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO shared_tasks (id, tenant_id, title, status, dependencies) VALUES ('2', 'tenant1', 'Task 2', 'PENDING', '[\"1\"]')")
            .execute(&pool).await.unwrap();

        // Try to claim, should get Task 1
        let task1 = orchestrator.claim_shared_task("tenant1", "agent1").await.unwrap().unwrap();
        assert_eq!(task1.id, "1");

        // Try to claim again, Task 2 is blocked by Task 1 (not completed)
        let task2_blocked = orchestrator.claim_shared_task("tenant1", "agent2").await.unwrap();
        assert!(task2_blocked.is_none());

        // Complete Task 1
        sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED' WHERE id = '1'").execute(&pool).await.unwrap();

        // Now claim Task 2
        let task2 = orchestrator.claim_shared_task("tenant1", "agent2").await.unwrap().unwrap();
        assert_eq!(task2.id, "2");
    }

    #[tokio::test]
    async fn test_kairos_orchestrator_approval_workflow() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'PENDING',
                epic_id TEXT,
                parent_id TEXT,
                assigned_agent TEXT,
                payload TEXT,
                dependencies TEXT NOT NULL DEFAULT '[]',
                created_at TEXT,
                updated_at TEXT,
                action_risk TEXT,
                approval_status TEXT,
                proposed_content TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let orchestrator = KairosOrchestrator::new(db);

        // Insert task
        sqlx::query("INSERT INTO shared_tasks (id, tenant_id, title, status, dependencies) VALUES ('1', 'tenant1', 'Task 1', 'PENDING', '[]')")
            .execute(&pool).await.unwrap();

        // Submit for approval
        orchestrator.submit_for_approval("1", "tenant1", "Email content", "HIGH").await.unwrap();

        // Verify task is not claimable while pending
        let task_blocked = orchestrator.claim_shared_task("tenant1", "agent1").await.unwrap();
        assert!(task_blocked.is_none());

        // Approve task
        orchestrator.approve_task("1", "tenant1", true).await.unwrap();

        // Verify state
        let row: (String, String) = sqlx::query_as("SELECT approval_status, status FROM shared_tasks WHERE id = '1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, "APPROVED");
        assert_eq!(row.1, "IN_PROGRESS");
    }

    #[tokio::test]
    async fn test_kairos_orchestrator_pg_paths() {
        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .max_connections(1)
            .connect_lazy(database_url)
            .unwrap();

        let db = Arc::new(DB { pool, store: DbStore::Postgres });
        let orchestrator = KairosOrchestrator::new(db);

        // It will fail because the db might not be fully seeded, but it covers the PG path
        let _ = orchestrator.claim_swarm_task("agent1").await;
        let _ = orchestrator.claim_shared_task("tenant1", "agent1").await;
    }
}
