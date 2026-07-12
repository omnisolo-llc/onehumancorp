use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{Task, TaskDependency};
use chrono::Utc;

use tokio::sync::Mutex;

pub struct TaskRepository {
    db: Arc<DB>,
    sqlite_mutex: Mutex<()>,
}

impl TaskRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            sqlite_mutex: Mutex::new(()),
        }
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

    pub async fn create_task_dependency(&self, dependency: TaskDependency) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO task_dependencies (task_id, depends_on_task_id, tenant_id)
                    VALUES ($1, $2, 'default_tenant')
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(&dependency.task_id)
                .bind(&dependency.depends_on_task_id)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO task_dependencies (task_id, depends_on_task_id, tenant_id)
                    VALUES (?, ?, 'default_tenant')
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(&dependency.task_id)
                .bind(&dependency.depends_on_task_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_task_dependencies(&self, task_id: &str) -> Result<Vec<TaskDependency>, String> {
        let deps = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, TaskDependency>(
                    r#"
                    SELECT task_id, depends_on_task_id
                    FROM task_dependencies
                    WHERE task_id = $1
                    "#
                )
                .bind(task_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, TaskDependency>(
                    r#"
                    SELECT task_id, depends_on_task_id
                    FROM task_dependencies
                    WHERE task_id = ?
                    "#
                )
                .bind(task_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?
            }
        };
        Ok(deps)
    }

    pub async fn claim_task(&self, organization_id: &str, assigned_agent_role: &str) -> Result<Option<Task>, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT t.id FROM tasks t
                    WHERE t.status = 'PENDING' AND t.organization_id = $1
                    AND NOT EXISTS (
                        SELECT 1
                        FROM task_dependencies td
                        JOIN tasks parent ON parent.id = td.depends_on_task_id
                        WHERE td.task_id = t.id AND parent.status != 'DONE'
                    )
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#,
                )
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Ok(None);
                    }
                };

                let id: String = sqlx::Row::get(&row, "id");

                sqlx::query(
                    r#"
                    UPDATE tasks
                    SET status = 'CLAIMED', assigned_agent_role = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                )
                .bind(assigned_agent_role)
                .bind(now)
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let task = sqlx::query_as::<_, Task>(
                    r#"
                    SELECT id, organization_id, parent_task_id, title, description,
                           status, assigned_agent_role, created_at, updated_at
                    FROM tasks
                    WHERE id = $1
                    "#
                )
                .bind(&id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(Some(task))
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT t.id FROM tasks t
                    WHERE t.status = 'PENDING' AND t.organization_id = ?
                    AND NOT EXISTS (
                        SELECT 1
                        FROM task_dependencies td
                        JOIN tasks parent ON parent.id = td.depends_on_task_id
                        WHERE td.task_id = t.id AND parent.status != 'DONE'
                    )
                    LIMIT 1
                    "#,
                )
                .bind(organization_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Ok(None);
                    }
                };

                let id: String = sqlx::Row::get(&row, "id");

                sqlx::query(
                    r#"
                    UPDATE tasks
                    SET status = 'CLAIMED', assigned_agent_role = ?, updated_at = ?
                    WHERE id = ?
                    "#,
                )
                .bind(assigned_agent_role)
                .bind(now)
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let task = sqlx::query_as::<_, Task>(
                    r#"
                    SELECT id, organization_id, parent_task_id, title, description,
                           status, assigned_agent_role, created_at, updated_at
                    FROM tasks
                    WHERE id = ?
                    "#
                )
                .bind(&id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(Some(task))
            }
        }
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

        sqlx::query(
            r#"
            CREATE TABLE task_dependencies (
                tenant_id TEXT DEFAULT 'default_tenant',
                task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
                depends_on_task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
                PRIMARY KEY (task_id, depends_on_task_id)
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = crate::db::secure_pg_pool_options()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        // create table in pg test pool if not exists (although normally migrations would be run)
        let _ = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS task_dependencies (
                tenant_id TEXT DEFAULT 'default_tenant',
                task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
                depends_on_task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
                PRIMARY KEY (task_id, depends_on_task_id)
            );
            "#
        )
        .execute(&pg_pool)
        .await;

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

    #[tokio::test]
    async fn test_task_dependencies() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(db);

        let org_id = "org_1".to_string();

        let task1 = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            title: "Task 1".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_task(task1).await.unwrap();

        let task2 = Task {
            id: "task_2".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            title: "Task 2".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_task(task2).await.unwrap();

        repo.create_task_dependency(TaskDependency {
            task_id: "task_2".to_string(),
            depends_on_task_id: "task_1".to_string(),
        }).await.unwrap();

        let deps = repo.get_task_dependencies("task_2").await.unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].depends_on_task_id, "task_1");
    }

    #[tokio::test]
    async fn test_claim_task() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(db);

        let org_id = "org_1".to_string();

        let task1 = Task {
            id: "task_1".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            title: "Task 1".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_task(task1).await.unwrap();

        let task2 = Task {
            id: "task_2".to_string(),
            organization_id: org_id.clone(),
            parent_task_id: None,
            title: "Task 2".to_string(),
            description: None,
            status: "PENDING".to_string(),
            assigned_agent_role: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_task(task2).await.unwrap();

        repo.create_task_dependency(TaskDependency {
            task_id: "task_2".to_string(),
            depends_on_task_id: "task_1".to_string(),
        }).await.unwrap();

        // task_2 cannot be claimed because task_1 is not DONE
        let claimed_task_2 = repo.claim_task(&org_id, "agent_1").await.unwrap();
        assert!(claimed_task_2.is_some()); // wait, it might claim task_1 instead

        let claimed = claimed_task_2.unwrap();
        assert_eq!(claimed.id, "task_1"); // should claim task_1 which has no unfulfilled dependencies
        assert_eq!(claimed.status, "CLAIMED");
        assert_eq!(claimed.assigned_agent_role.unwrap(), "agent_1");

        // now there are no pending tasks without dependencies
        let claimed_none = repo.claim_task(&org_id, "agent_2").await.unwrap();
        assert!(claimed_none.is_none());

        // finish task 1
        repo.update_task_status(&org_id, "task_1", "DONE").await.unwrap();

        // now task 2 can be claimed
        let claimed_task_2_now = repo.claim_task(&org_id, "agent_2").await.unwrap();
        assert!(claimed_task_2_now.is_some());
        assert_eq!(claimed_task_2_now.unwrap().id, "task_2");
    }
}
