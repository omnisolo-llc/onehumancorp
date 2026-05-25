use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::Task;
use chrono::Utc;

pub struct TaskRepository {
    db: Arc<DB>,
}

impl TaskRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_task(&self, task: Task) -> Result<Task, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO tasks (
                        id, organization_id, parent_task_id, epic_id, title, description,
                        status, assigned_agent_role, payload, created_at, updated_at, locked_by, locked_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.parent_task_id)
                .bind(&task.epic_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.assigned_agent_role)
                .bind(&task.payload)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .bind(&task.locked_by)
                .bind(&task.locked_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO tasks (
                        id, organization_id, parent_task_id, epic_id, title, description,
                        status, assigned_agent_role, payload, created_at, updated_at, locked_by, locked_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.parent_task_id)
                .bind(&task.epic_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.assigned_agent_role)
                .bind(&task.payload)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .bind(&task.locked_by)
                .bind(&task.locked_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(task)
    }

    pub async fn get_tasks_by_org(&self, organization_id: &str) -> Result<Vec<Task>, String> {
        let tasks = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, Task>(
                    r#"
                    SELECT id, organization_id, parent_task_id, epic_id, title, description,
                           status, assigned_agent_role, payload, created_at, updated_at, locked_by, locked_at
                    FROM tasks
                    WHERE organization_id = $1
                    "#
                )
                .bind(organization_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Task>(
                    r#"
                    SELECT id, organization_id, parent_task_id, epic_id, title, description,
                           status, assigned_agent_role, payload, created_at, updated_at, locked_by, locked_at
                    FROM tasks
                    WHERE organization_id = ?
                    "#
                )
                .bind(organization_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?
            }
        };
        Ok(tasks)
    }

    pub async fn update_task_status(&self, organization_id: &str, task_id: &str, new_status: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let result = sqlx::query(
                    r#"
                    UPDATE tasks
                    SET status = $1, updated_at = $2
                    WHERE id = $3 AND organization_id = $4
                    RETURNING id
                    "#
                )
                .bind(new_status)
                .bind(now)
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                if result.is_none() {
                    return Err("Task not found or does not belong to organization".to_string());
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let result = sqlx::query(
                    r#"
                    UPDATE tasks
                    SET status = ?, updated_at = ?
                    WHERE id = ? AND organization_id = ?
                    RETURNING id
                    "#
                )
                .bind(new_status)
                .bind(now)
                .bind(task_id)
                .bind(organization_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                if result.is_none() {
                    return Err("Task not found or does not belong to organization".to_string());
                }
            }
        }
        Ok(())
    }

    pub async fn get_next_available_task(&self, organization_id: &str, agent_id: &str) -> Result<Option<Task>, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query_as::<_, Task>(
                    r#"
                    SELECT id, organization_id, parent_task_id, epic_id, title, description,
                           status, assigned_agent_role, payload, created_at, updated_at, locked_by, locked_at
                    FROM tasks
                    WHERE organization_id = $1 AND status = 'PENDING'
                    AND (locked_by IS NULL OR locked_at IS NULL OR locked_at < NOW() - INTERVAL '5 minutes')
                    AND NOT EXISTS (
                        SELECT 1 FROM task_dependencies td
                        JOIN tasks parent ON parent.id = td.depends_on_task_id
                        WHERE td.task_id = tasks.id AND parent.status != 'COMPLETED'
                    )
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(mut task) = row {
                    sqlx::query(
                        r#"
                        UPDATE tasks
                        SET status = 'IN_PROGRESS', locked_by = $1, locked_at = $2, updated_at = $2
                        WHERE id = $3
                        "#
                    )
                    .bind(agent_id)
                    .bind(now)
                    .bind(&task.id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    task.status = "IN_PROGRESS".to_string();
                    task.locked_by = Some(agent_id.to_string());
                    task.locked_at = Some(now);
                    task.updated_at = Some(now);
                    Ok(Some(task))
                } else {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                // SQLite doesn't have FOR UPDATE SKIP LOCKED, so we use an atomic UPDATE RETURNING
                let row = sqlx::query_as::<_, Task>(
                    r#"
                    UPDATE tasks
                    SET status = 'IN_PROGRESS', locked_by = ?, locked_at = ?, updated_at = ?
                    WHERE id = (
                        SELECT t.id
                        FROM tasks t
                        WHERE t.organization_id = ? AND t.status = 'PENDING'
                        AND (t.locked_by IS NULL OR t.locked_at IS NULL OR datetime(t.locked_at) < datetime(?, '-5 minutes'))
                        AND NOT EXISTS (
                            SELECT 1 FROM task_dependencies td
                            JOIN tasks parent ON parent.id = td.depends_on_task_id
                            WHERE td.task_id = t.id AND parent.status != 'COMPLETED'
                        )
                        LIMIT 1
                    )
                    RETURNING id, organization_id, parent_task_id, epic_id, title, description,
                              status, assigned_agent_role, payload, created_at, updated_at, locked_by, locked_at
                    "#
                )
                .bind(agent_id)
                .bind(now)
                .bind(now)
                .bind(organization_id)
                .bind(now)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                Ok(row)
            }
        }
    }

    pub async fn create_task_dependency(&self, task_id: &str, depends_on_task_id: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO task_dependencies (task_id, depends_on_task_id)
                    VALUES ($1, $2)
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(task_id)
                .bind(depends_on_task_id)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT OR IGNORE INTO task_dependencies (task_id, depends_on_task_id)
                    VALUES (?, ?)
                    "#
                )
                .bind(task_id)
                .bind(depends_on_task_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE tasks (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                parent_task_id TEXT,
                epic_id TEXT,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
                assigned_agent_role VARCHAR(100),
                payload TEXT,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                locked_by TEXT,
                locked_at TIMESTAMPTZ,
                FOREIGN KEY (parent_task_id) REFERENCES tasks(id)
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE task_dependencies (
                task_id TEXT NOT NULL,
                depends_on_task_id TEXT NOT NULL,
                PRIMARY KEY (task_id, depends_on_task_id),
                FOREIGN KEY (task_id) REFERENCES tasks(id),
                FOREIGN KEY (depends_on_task_id) REFERENCES tasks(id)
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_create_and_get_task() {
        let db: Arc<DB> = setup_test_db().await;
        let repo = TaskRepository::new(db);

        let org_id = "org_1".to_string();

        let task = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            epic_id: Some("epic_1".to_string()),
            title: "Test Task".to_string(),
            description: Some("Description".to_string()),
            status: "PENDING".to_string(),
            assigned_agent_role: Some("Developer".to_string()),
            payload: Some("{}".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            locked_by: None,
            locked_at: None,
        };

        repo.create_task(task.clone()).await.unwrap();

        let tasks = repo.get_tasks_by_org(&org_id).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "task_1");
        assert_eq!(tasks[0].title, "Test Task");
        assert_eq!(tasks[0].epic_id, Some("epic_1".to_string()));

        let other_tasks = repo.get_tasks_by_org("org_2").await.unwrap();
        assert_eq!(other_tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_update_task_status() {
        let db: Arc<DB> = setup_test_db().await;
        let repo = TaskRepository::new(db);

        let org_id = "org_1".to_string();

        let task = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            epic_id: None,
            title: "Test Task".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            payload: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            locked_by: None,
            locked_at: None,
        };

        repo.create_task(task).await.unwrap();

        repo.update_task_status(&org_id, "task_1", "IN_PROGRESS").await.unwrap();

        let tasks = repo.get_tasks_by_org(&org_id).await.unwrap();
        assert_eq!(tasks[0].status, "IN_PROGRESS");

        let result = repo.update_task_status("wrong_org", "task_1", "COMPLETED").await;
        assert!(result.is_err());

        let tasks_after = repo.get_tasks_by_org(&org_id).await.unwrap();
        assert_eq!(tasks_after[0].status, "IN_PROGRESS");
    }

    #[tokio::test]
    async fn test_get_next_available_task() {
        let db: Arc<DB> = setup_test_db().await;
        let repo = TaskRepository::new(db.clone());

        let org_id = "org_1".to_string();
        let agent_id = "agent_1".to_string();

        let task1 = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            epic_id: None,
            title: "Task 1".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            payload: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            locked_by: None,
            locked_at: None,
        };

        repo.create_task(task1).await.unwrap();

        // Should return the only available task
        let next_task = repo.get_next_available_task(&org_id, &agent_id).await.unwrap();
        assert!(next_task.is_some());
        let claimed_task = next_task.unwrap();
        assert_eq!(claimed_task.id, "task_1");
        assert_eq!(claimed_task.status, "IN_PROGRESS");
        assert_eq!(claimed_task.locked_by, Some(agent_id.clone()));

        // Should return None since task is IN_PROGRESS
        let next_task_none = repo.get_next_available_task(&org_id, &agent_id).await.unwrap();
        assert!(next_task_none.is_none());

        // Create dependent task
        let task2 = Task {
            id: "task_2".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            epic_id: None,
            title: "Task 2".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            payload: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            locked_by: None,
            locked_at: None,
        };
        repo.create_task(task2).await.unwrap();

        repo.create_task_dependency("task_2", "task_1").await.unwrap();

        // Should return None because task_2 depends on task_1 which is not completed
        let blocked_task = repo.get_next_available_task(&org_id, &agent_id).await.unwrap();
        assert!(blocked_task.is_none());

        // Complete task_1
        repo.update_task_status(&org_id, "task_1", "COMPLETED").await.unwrap();

        // Now task_2 should be available
        let unblocked_task = repo.get_next_available_task(&org_id, &agent_id).await.unwrap();
        assert!(unblocked_task.is_some());
        assert_eq!(unblocked_task.unwrap().id, "task_2");
    }
}
