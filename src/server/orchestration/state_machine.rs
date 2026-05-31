use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use chrono::Utc;
use uuid::Uuid;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "PENDING",
            TaskStatus::InProgress => "IN_PROGRESS",
            TaskStatus::Blocked => "BLOCKED",
            TaskStatus::Completed => "COMPLETED",
            TaskStatus::Failed => "FAILED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(TaskStatus::Pending),
            "IN_PROGRESS" => Some(TaskStatus::InProgress),
            "BLOCKED" => Some(TaskStatus::Blocked),
            "COMPLETED" => Some(TaskStatus::Completed),
            "FAILED" => Some(TaskStatus::Failed),
            _ => None,
        }
    }
}

pub struct StateMachine {
    db: Arc<DB>,
    sqlite_mutex: Mutex<()>,
}

impl StateMachine {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db, sqlite_mutex: Mutex::new(()) }
    }

    fn increment_transition_metric(status: &TaskStatus) {
        crate::telemetry::get_tasks_transitions_total().add(1, &[]);
        match status {
            TaskStatus::Completed => {
                crate::telemetry::get_tasks_completed_total().add(1, &[]);
            }
            TaskStatus::Failed => {
                crate::telemetry::get_tasks_failed_total().add(1, &[]);
            }
            _ => {}
        }
    }

    pub async fn create_task(&self, task_id: Option<String>, organization_id: &str, title: &str, description: Option<&str>, dependencies: Vec<String>) -> Result<String, String> {
        let tid = task_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let now = Utc::now();

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                sqlx::query(
                    "INSERT INTO shared_tasks (id, organization_id, title, description, status, created_at, updated_at) VALUES ($1, $2, $3, $4, 'PENDING', $5, $6)"
                )
                .bind(&tid)
                .bind(organization_id)
                .bind(title)
                .bind(description)
                .bind(now)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                for dep in dependencies {
                    sqlx::query(
                        "INSERT INTO shared_task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)"
                    )
                    .bind(&tid)
                    .bind(&dep)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                sqlx::query(
                    "INSERT INTO shared_tasks (id, organization_id, title, description, status, created_at, updated_at) VALUES (?, ?, ?, ?, 'PENDING', ?, ?)"
                )
                .bind(&tid)
                .bind(organization_id)
                .bind(title)
                .bind(description)
                .bind(now.to_rfc3339())
                .bind(now.to_rfc3339())
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                for dep in dependencies {
                    sqlx::query(
                        "INSERT INTO shared_task_dependencies (task_id, depends_on_task_id) VALUES (?, ?)"
                    )
                    .bind(&tid)
                    .bind(&dep)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }

                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }

        Self::increment_transition_metric(&TaskStatus::Pending);

        Ok(tid)
    }

    pub async fn claim_task(&self, task_id: &str, agent_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE SKIP LOCKED"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: String = r.get("status");
                    if status != "PENDING" && status != "BLOCKED" {
                        return Err(format!("Cannot claim task in {} state", status));
                    }

                    // Check dependencies
                    let incomplete_deps: i64 = sqlx::query(
                        r#"
                        SELECT COUNT(*) FROM shared_task_dependencies std
                        JOIN shared_tasks st ON std.depends_on_task_id = st.id
                        WHERE std.task_id = $1 AND st.status != 'COMPLETED'
                        "#
                    )
                    .bind(task_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?
                    .get(0);

                    if incomplete_deps > 0 {
                        return Err("Cannot claim task, dependencies are not completed".to_string());
                    }

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3"
                    )
                    .bind(agent_id)
                    .bind(now)
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Self::increment_transition_metric(&TaskStatus::InProgress);
                    Ok(())
                } else {
                    Err("Task not found or locked".to_string())
                }
            }
            DbStore::Sqlite(pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT status FROM shared_tasks WHERE id = ?"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: String = r.get("status");
                    if status != "PENDING" && status != "BLOCKED" {
                        return Err(format!("Cannot claim task in {} state", status));
                    }

                    // Check dependencies
                    let incomplete_deps: i64 = sqlx::query(
                        r#"
                        SELECT COUNT(*) FROM shared_task_dependencies std
                        JOIN shared_tasks st ON std.depends_on_task_id = st.id
                        WHERE std.task_id = ? AND st.status != 'COMPLETED'
                        "#
                    )
                    .bind(task_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?
                    .get(0);

                    if incomplete_deps > 0 {
                        return Err("Cannot claim task, dependencies are not completed".to_string());
                    }

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = ? WHERE id = ?"
                    )
                    .bind(agent_id)
                    .bind(now.to_rfc3339())
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Self::increment_transition_metric(&TaskStatus::InProgress);
                    Ok(())
                } else {
                    Err("Task not found".to_string())
                }
            }
        }
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: String = r.get("status");
                    if status != "IN_PROGRESS" {
                        return Err(format!("Cannot complete task in {} state", status));
                    }

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'COMPLETED', updated_at = $1 WHERE id = $2"
                    )
                    .bind(now)
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Self::increment_transition_metric(&TaskStatus::Completed);
                    Ok(())
                } else {
                    Err("Task not found".to_string())
                }
            }
            DbStore::Sqlite(pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT status FROM shared_tasks WHERE id = ?"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: String = r.get("status");
                    if status != "IN_PROGRESS" {
                        return Err(format!("Cannot complete task in {} state", status));
                    }

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'COMPLETED', updated_at = ? WHERE id = ?"
                    )
                    .bind(now.to_rfc3339())
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Self::increment_transition_metric(&TaskStatus::Completed);
                    Ok(())
                } else {
                    Err("Task not found".to_string())
                }
            }
        }
    }

    pub async fn block_task(&self, task_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: String = r.get("status");
                    if status != "IN_PROGRESS" && status != "PENDING" {
                        return Err(format!("Cannot block task in {} state", status));
                    }

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'BLOCKED', updated_at = $1 WHERE id = $2"
                    )
                    .bind(now)
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Self::increment_transition_metric(&TaskStatus::Blocked);
                    Ok(())
                } else {
                    Err("Task not found".to_string())
                }
            }
            DbStore::Sqlite(pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT status FROM shared_tasks WHERE id = ?"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: String = r.get("status");
                    if status != "IN_PROGRESS" && status != "PENDING" {
                        return Err(format!("Cannot block task in {} state", status));
                    }

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'BLOCKED', updated_at = ? WHERE id = ?"
                    )
                    .bind(now.to_rfc3339())
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Self::increment_transition_metric(&TaskStatus::Blocked);
                    Ok(())
                } else {
                    Err("Task not found".to_string())
                }
            }
        }
    }
}
