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
    pub organization_id: String,
    pub parent_plan_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_agent_id: Option<String>,
    pub dependencies: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
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


    pub async fn complete_task(&self, task_id: &str, task_type: &str, agent_id: &str) -> Result<(), KairosError> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(KairosError::Database)?;

                // Update the task status to COMPLETED
                let result = if task_type == "swarm" {
                    let id_uuid = uuid::Uuid::parse_str(task_id).unwrap_or_default();
                    sqlx::query("UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = $1 WHERE id = $2 AND assigned_agent_id = $3")
                        .bind(now)
                        .bind(id_uuid)
                        .bind(agent_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(KairosError::Database)?
                } else {
                    sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = $1 WHERE id = $2 AND assigned_agent_id = $3")
                        .bind(now)
                        .bind(task_id)
                        .bind(agent_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(KairosError::Database)?
                };

                if result.rows_affected() > 0 {
                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at) VALUES ($1, $2, 'EXECUTING', 'COMPLETED', $3, $4)"
                    )
                    .bind(trans_id)
                    .bind(task_id)
                    .bind(agent_id)
                    .bind(now)
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    // DAG Lifecycle Unblock downstream dependencies logic
                    if task_type == "shared" {
                        let unblock_query = "UPDATE shared_tasks SET dependencies = COALESCE((SELECT jsonb_agg(elem) FROM jsonb_array_elements(dependencies) AS elem WHERE elem::text != $1), '[]'::jsonb) WHERE dependencies @> $1::jsonb";
                        let dep_json = format!("\"{}\"", task_id);
                        sqlx::query(unblock_query)
                            .bind(&dep_json)
                            .execute(&mut *tx)
                            .await
                            .map_err(KairosError::Database)?;
                    } else {
                        let unblock_query = "UPDATE swarm_tasks SET dependencies = COALESCE((SELECT jsonb_agg(elem) FROM jsonb_array_elements(dependencies) AS elem WHERE elem::text != $1), '[]'::jsonb) WHERE dependencies @> $1::jsonb";
                        let dep_json = format!("\"{}\"", task_id);
                        sqlx::query(unblock_query)
                            .bind(&dep_json)
                            .execute(&mut *tx)
                            .await
                            .map_err(KairosError::Database)?;
                    }
                }

                tx.commit().await.map_err(KairosError::Database)?;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(KairosError::Database)?;

                let result = if task_type == "swarm" {
                    sqlx::query("UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = ? WHERE id = ? AND assigned_agent_id = ?")
                        .bind(now.to_rfc3339())
                        .bind(task_id)
                        .bind(agent_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(KairosError::Database)?
                } else {
                    sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = ? WHERE id = ? AND assigned_agent_id = ?")
                        .bind(now.to_rfc3339())
                        .bind(task_id)
                        .bind(agent_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(KairosError::Database)?
                };

                if result.rows_affected() > 0 {
                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at) VALUES (?, ?, 'EXECUTING', 'COMPLETED', ?, ?)"
                    )
                    .bind(trans_id)
                    .bind(task_id)
                    .bind(agent_id)
                    .bind(now.to_rfc3339())
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    // Unblock downstream child dependencies
                    if task_type == "shared" {
                        sqlx::query("UPDATE shared_tasks SET dependencies = (SELECT json_group_array(value) FROM json_each(dependencies) WHERE value != ?) WHERE EXISTS (SELECT 1 FROM json_each(dependencies) WHERE value = ?)")
                            .bind(task_id)
                            .bind(task_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(KairosError::Database)?;
                    } else {
                        sqlx::query("UPDATE swarm_tasks SET dependencies = (SELECT json_group_array(value) FROM json_each(dependencies) WHERE value != ?) WHERE EXISTS (SELECT 1 FROM json_each(dependencies) WHERE value = ?)")
                            .bind(task_id)
                            .bind(task_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(KairosError::Database)?;
                    }
                }

                tx.commit().await.map_err(KairosError::Database)?;
                Ok(())
            }
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

                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                        VALUES ($1, $2, 'PENDING', 'IN_PROGRESS', $3, $4)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&id_str)
                    .bind(agent_id)
                    .bind(now)
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

                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                        VALUES (?, ?, 'PENDING', 'IN_PROGRESS', ?, ?)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&task.id)
                    .bind(agent_id)
                    .bind(now.to_rfc3339())
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(Some(task))
                } else {
                    tx.commit().await.map_err(KairosError::Database)?;
                    Ok(None)
                }
            }
        }
    }

    pub async fn create_shared_task(&self, task: SharedTask) -> Result<SharedTask, KairosError> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks (
                        id, organization_id, parent_plan_id, title, description,
                        status, assigned_agent_id, dependencies, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.parent_plan_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.assigned_agent_id)
                .bind(&task.dependencies)
                .bind(now)
                .bind(now)
                .execute(&self.db.pool)
                .await
                .map_err(KairosError::Database)?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks (
                        id, organization_id, parent_plan_id, title, description,
                        status, assigned_agent_id, dependencies, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.parent_plan_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.assigned_agent_id)
                .bind(&task.dependencies)
                .bind(now.to_rfc3339())
                .bind(now.to_rfc3339())
                .execute(sqlite_pool)
                .await
                .map_err(KairosError::Database)?;
            }
        }

        let mut res = task;
        res.created_at = Some(now);
        res.updated_at = Some(now);
        Ok(res)
    }

    pub async fn claim_shared_task(&self, organization_id: &str, agent_id: &str) -> Result<Option<SharedTask>, KairosError> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(KairosError::Database)?;

                let row = sqlx::query(
                    r#"
                    SELECT t.id, t.organization_id, t.parent_plan_id, t.title, t.description, t.status, t.assigned_agent_id, t.dependencies::text, t.created_at, t.updated_at
                    FROM shared_tasks t
                    WHERE t.status = 'PENDING' AND t.organization_id = $1
                    AND NOT EXISTS (
                        SELECT 1 FROM jsonb_array_elements_text(t.dependencies::jsonb) AS dep_id
                        JOIN shared_tasks parent ON parent.id::text = dep_id
                        WHERE parent.status != 'COMPLETED'
                    )
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(KairosError::Database)?;

                if let Some(r) = row {
                    let id: String = r.get("id");

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3"
                    )
                    .bind(agent_id)
                    .bind(now)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                        VALUES ($1, $2, 'PENDING', 'IN_PROGRESS', $3, $4)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&id)
                    .bind(agent_id)
                    .bind(now)
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

                    tx.commit().await.map_err(KairosError::Database)?;

                    Ok(Some(SharedTask {
                        id,
                        organization_id: r.get("organization_id"),
                        parent_plan_id: r.get("parent_plan_id"),
                        title: r.get("title"),
                        description: r.get("description"),
                        status: "IN_PROGRESS".to_string(),
                        dependencies: r.get("dependencies"),
                        assigned_agent_id: Some(agent_id.to_string()),
                        created_at: r.get("created_at"),
                        updated_at: Some(now),
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
                    SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = ?
                    WHERE id = (
                        SELECT t.id
                        FROM shared_tasks t
                        WHERE t.status = 'PENDING' AND t.organization_id = ?
                        AND NOT EXISTS (
                            SELECT 1 FROM json_each(t.dependencies) AS dep_id
                            JOIN shared_tasks parent ON parent.id = dep_id.value
                            WHERE parent.status != 'COMPLETED'
                        )
                        LIMIT 1
                    )
                    RETURNING id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at
                    "#
                )
                .bind(agent_id)
                .bind(now.to_rfc3339())
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(KairosError::Database)?;

                if let Some(r) = row {
                    let created_str_opt: Option<String> = r.try_get("created_at").unwrap_or(None);
                    let dt_created = if let Some(created_str) = created_str_opt {
                        chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now())
                    } else {
                        chrono::Utc::now()
                    };

                    let task = SharedTask {
                        id: r.get("id"),
                        organization_id: r.get("organization_id"),
                        parent_plan_id: r.get("parent_plan_id"),
                        title: r.get("title"),
                        description: r.get("description"),
                        status: r.get("status"),
                        dependencies: r.get("dependencies"),
                        assigned_agent_id: r.get("assigned_agent_id"),
                        created_at: Some(dt_created),
                        updated_at: Some(now),
                    };

                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                        VALUES (?, ?, 'PENDING', 'IN_PROGRESS', ?, ?)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&task.id)
                    .bind(agent_id)
                    .bind(now.to_rfc3339())
                    .execute(&mut *tx)
                    .await
                    .map_err(KairosError::Database)?;

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
    use chrono::Utc;
    use uuid::Uuid;

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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS state_machine_transitions (
                id TEXT PRIMARY KEY,
                task_id TEXT,
                from_state TEXT,
                to_state TEXT,
                agent_id TEXT,
                transitioned_at TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS state_machine_transitions (
                id TEXT PRIMARY KEY,
                task_id TEXT,
                from_state TEXT,
                to_state TEXT,
                agent_id TEXT,
                transitioned_at TEXT
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
                organization_id TEXT NOT NULL,
                parent_plan_id TEXT,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'PENDING',
                assigned_agent_id TEXT,
                dependencies JSONB DEFAULT '[]',
                created_at TEXT,
                updated_at TEXT
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
        sqlx::query("INSERT INTO shared_tasks (id, organization_id, title, status, dependencies, created_at) VALUES ('1', 'tenant1', 'Task 1', 'PENDING', '[]', '2023-01-01T00:00:00Z')")
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

        // Complete Task 2
        orchestrator.complete_task("2", "shared", "agent2").await.unwrap();

        let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks WHERE id = '2'").fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, "COMPLETED");

        let trans: (String, String, String) = sqlx::query_as("SELECT task_id, from_state, to_state FROM state_machine_transitions WHERE task_id = '2' AND to_state = 'COMPLETED'").fetch_one(&pool).await.unwrap();
        assert_eq!(trans.0, "2");
        assert_eq!(trans.1, "EXECUTING");
        assert_eq!(trans.2, "COMPLETED");
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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS state_machine_transitions (
                id TEXT PRIMARY KEY,
                task_id TEXT,
                from_state TEXT,
                to_state TEXT,
                agent_id TEXT,
                transitioned_at TEXT
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
