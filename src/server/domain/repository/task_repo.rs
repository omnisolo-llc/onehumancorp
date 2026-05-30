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
                        id, organization_id, parent_task_id, title, description,
                        status, assigned_agent_role, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.parent_task_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.assigned_agent_role)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO tasks (
                        id, organization_id, parent_task_id, title, description,
                        status, assigned_agent_role, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.parent_task_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.assigned_agent_role)
                .bind(&task.created_at)
                .bind(&task.updated_at)
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
                    SELECT id, organization_id, parent_task_id, title, description,
                           status, assigned_agent_role, created_at, updated_at
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
                    SELECT id, organization_id, parent_task_id, title, description,
                           status, assigned_agent_role, created_at, updated_at
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
                title VARCHAR(255) NOT NULL,
                description TEXT,
                status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
                assigned_agent_role VARCHAR(100),
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (parent_task_id) REFERENCES tasks(id)
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_create_and_get_task() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(db);

        let org_id = "org_1".to_string();

        let task = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            title: "Test Task".to_string(),
            description: Some("Description".to_string()),
            status: "PENDING".to_string(),
            assigned_agent_role: Some("Developer".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        repo.create_task(task.clone()).await.unwrap();

        let tasks = repo.get_tasks_by_org(&org_id).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "task_1");
        assert_eq!(tasks[0].title, "Test Task");

        let other_tasks = repo.get_tasks_by_org("org_2").await.unwrap();
        assert_eq!(other_tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_update_task_status() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(db);

        let org_id = "org_1".to_string();

        let task = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            title: "Test Task".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
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
}
