use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeliberationState {
    Pending,
    Deliberating,
    ResolvingDependencies,
    Claimed,
    Completed,
    Failed,
}

impl AsRef<str> for DeliberationState {
    fn as_ref(&self) -> &str {
        match self {
            DeliberationState::Pending => "PENDING",
            DeliberationState::Deliberating => "DELIBERATING",
            DeliberationState::ResolvingDependencies => "RESOLVING_DEPENDENCIES",
            DeliberationState::Claimed => "CLAIMED",
            DeliberationState::Completed => "COMPLETED",
            DeliberationState::Failed => "FAILED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskDeliberation {
    pub id: String,
    pub organization_id: String,
    pub status: String,
    pub dependencies: serde_json::Value,
    pub agent_id: Option<String>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum DbStore {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

pub struct DeliberationStateMachine {
    pub db: DbStore,
    sqlite_mutex: Arc<Mutex<()>>,
}

impl DeliberationStateMachine {
    pub fn new(db: DbStore) -> Self {
        Self {
            db,
            sqlite_mutex: Arc::new(Mutex::new(())),
        }
    }

    pub async fn claim_for_deliberation(
        &self,
        organization_id: &str,
        agent_id: &str,
    ) -> Result<Option<TaskDeliberation>, String> {
        match &self.db {
            DbStore::Postgres(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    SELECT * FROM shared_tasks_decomposition
                    WHERE status = 'PENDING' AND organization_id = $1
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = row {
                    let task_id: String = row.get("id");

                    sqlx::query(
                        r#"
                        UPDATE shared_tasks_decomposition
                        SET status = 'DELIBERATING', agent_id = $1, updated_at = $2
                        WHERE id = $3 AND organization_id = $4
                        "#
                    )
                    .bind(agent_id)
                    .bind(Utc::now())
                    .bind(&task_id)
                    .bind(organization_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Ok(Some(TaskDeliberation {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        status: "DELIBERATING".to_string(),
                        dependencies: row.get("dependencies"),
                        agent_id: Some(agent_id.to_string()),
                        locked_until: row.try_get("locked_until").unwrap_or(None),
                        created_at: row.get("created_at"),
                        updated_at: Utc::now(),
                    }))
                } else {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    SELECT * FROM shared_tasks_decomposition
                    WHERE status = 'PENDING' AND organization_id = ?
                    LIMIT 1
                    "#
                )
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = row {
                    let task_id: String = row.get("id");

                    sqlx::query(
                        r#"
                        UPDATE shared_tasks_decomposition
                        SET status = 'DELIBERATING', agent_id = ?, updated_at = ?
                        WHERE id = ? AND organization_id = ?
                        "#
                    )
                    .bind(agent_id)
                    .bind(Utc::now().to_rfc3339())
                    .bind(&task_id)
                    .bind(organization_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    let created_str: String = row.get("created_at");
                    let dt_created = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S")
                        .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                        .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(|_| chrono::Utc::now());

                    let locked_str: Option<String> = row.try_get("locked_until").unwrap_or(None);
                    let locked_until = locked_str.map(|s| chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()));


                    Ok(Some(TaskDeliberation {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        status: "DELIBERATING".to_string(),
                        dependencies: row.get("dependencies"),
                        agent_id: Some(agent_id.to_string()),
                        locked_until,
                        created_at: dt_created,
                        updated_at: Utc::now(),
                    }))
                } else {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
        }
    }

    pub async fn resolve_dependencies(
        &self,
        organization_id: &str,
        task_id: &str,
        dependencies: serde_json::Value,
    ) -> Result<(), String> {
        match &self.db {
            DbStore::Postgres(pool) => {
                let res = sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'RESOLVING_DEPENDENCIES', dependencies = $1, updated_at = $2
                    WHERE id = $3 AND organization_id = $4 AND status = 'DELIBERATING'
                    RETURNING id
                    "#
                )
                .bind(dependencies)
                .bind(Utc::now())
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;

                if res.is_none() {
                    return Err("Invalid state transition or task not found for organization".to_string());
                }
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let res = sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'RESOLVING_DEPENDENCIES', dependencies = ?, updated_at = ?
                    WHERE id = ? AND organization_id = ? AND status = 'DELIBERATING'
                    RETURNING id
                    "#
                )
                .bind(dependencies)
                .bind(Utc::now().to_rfc3339())
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                if res.is_none() {
                    return Err("Invalid state transition or task not found for organization".to_string());
                }
                Ok(())
            }
        }
    }

    pub async fn complete_deliberation(&self, organization_id: &str, task_id: &str) -> Result<(), String> {
        match &self.db {
            DbStore::Postgres(pool) => {
                let res = sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'COMPLETED', updated_at = $1
                    WHERE id = $2 AND organization_id = $3 AND status IN ('DELIBERATING', 'RESOLVING_DEPENDENCIES')
                    RETURNING id
                    "#
                )
                .bind(Utc::now())
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;

                if res.is_none() {
                    return Err("Invalid state transition or task not found for organization".to_string());
                }
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let res = sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'COMPLETED', updated_at = ?
                    WHERE id = ? AND organization_id = ? AND status IN ('DELIBERATING', 'RESOLVING_DEPENDENCIES')
                    RETURNING id
                    "#
                )
                .bind(Utc::now().to_rfc3339())
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                if res.is_none() {
                    return Err("Invalid state transition or task not found for organization".to_string());
                }
                Ok(())
            }
        }
    }

    pub async fn fail_deliberation(&self, organization_id: &str, task_id: &str) -> Result<(), String> {
        match &self.db {
            DbStore::Postgres(pool) => {
                let res = sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'FAILED', updated_at = $1
                    WHERE id = $2 AND organization_id = $3
                    RETURNING id
                    "#
                )
                .bind(Utc::now())
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;

                if res.is_none() {
                    return Err("Task not found for organization".to_string());
                }
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let res = sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'FAILED', updated_at = ?
                    WHERE id = ? AND organization_id = ?
                    RETURNING id
                    "#
                )
                .bind(Utc::now().to_rfc3339())
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                if res.is_none() {
                    return Err("Task not found for organization".to_string());
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_db() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
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
                locked_until TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_claim() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        // Insert pending task
        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('t1', 'org1', 'task 1', 'PENDING')"
        )
        .execute(&pool)
        .await
        .unwrap();

        // Claim
        let claimed = sm.claim_for_deliberation("org1", "agent1").await.unwrap();
        assert!(claimed.is_some());
        let claimed_task = claimed.unwrap();
        assert_eq!(claimed_task.id, "t1");
        assert_eq!(claimed_task.status, "DELIBERATING");
        assert_eq!(claimed_task.agent_id.unwrap(), "agent1");

        // Try claim again, should be none since it's no longer pending
        let claimed2 = sm.claim_for_deliberation("org1", "agent2").await.unwrap();
        assert!(claimed2.is_none());
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_resolve_dependencies() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, agent_id) VALUES ('t2', 'org1', 'task 2', 'DELIBERATING', 'agent1')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let deps = json!(["dep1", "dep2"]);
        sm.resolve_dependencies("org1", "t2", deps.clone()).await.unwrap();

        let row: (String, String) = sqlx::query_as("SELECT status, dependencies FROM shared_tasks_decomposition WHERE id = 't2'")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.0, "RESOLVING_DEPENDENCIES");
        assert_eq!(row.1, deps.to_string());
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_resolve_dependencies_invalid_state() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, agent_id) VALUES ('t2b', 'org1', 'task 2', 'COMPLETED', 'agent1')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let deps = json!(["dep1", "dep2"]);
        let res = sm.resolve_dependencies("org1", "t2b", deps.clone()).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Invalid state transition or task not found for organization");
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_resolve_dependencies_cross_tenant() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, agent_id) VALUES ('t2c', 'org1', 'task 2', 'DELIBERATING', 'agent1')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let deps = json!(["dep1", "dep2"]);
        let res = sm.resolve_dependencies("org2", "t2c", deps.clone()).await; // org2 tries to modify org1
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Invalid state transition or task not found for organization");
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_complete() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('t3', 'org1', 'task 3', 'RESOLVING_DEPENDENCIES')"
        )
        .execute(&pool)
        .await
        .unwrap();

        sm.complete_deliberation("org1", "t3").await.unwrap();

        let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 't3'")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.0, "COMPLETED");
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_complete_invalid_state() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('t3b', 'org1', 'task 3', 'PENDING')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let res = sm.complete_deliberation("org1", "t3b").await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Invalid state transition or task not found for organization");
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_fail() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('t4', 'org1', 'task 4', 'DELIBERATING')"
        )
        .execute(&pool)
        .await
        .unwrap();

        sm.fail_deliberation("org1", "t4").await.unwrap();

        let row: (String,) = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = 't4'")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.0, "FAILED");
    }

    #[tokio::test]
    async fn test_deliberation_state_machine_fail_cross_tenant() {
        let pool = setup_db().await;
        let sm = DeliberationStateMachine::new(DbStore::Sqlite(pool.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('t4b', 'org1', 'task 4', 'DELIBERATING')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let res = sm.fail_deliberation("org2", "t4b").await; // cross-tenant
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Task not found for organization");
    }

    #[test]
    fn test_deliberation_state_as_ref() {
        assert_eq!(DeliberationState::Pending.as_ref(), "PENDING");
        assert_eq!(DeliberationState::Deliberating.as_ref(), "DELIBERATING");
        assert_eq!(DeliberationState::ResolvingDependencies.as_ref(), "RESOLVING_DEPENDENCIES");
        assert_eq!(DeliberationState::Claimed.as_ref(), "CLAIMED");
        assert_eq!(DeliberationState::Completed.as_ref(), "COMPLETED");
        assert_eq!(DeliberationState::Failed.as_ref(), "FAILED");
    }
}
