use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTask {
    pub id: String,
    pub organization_id: String,
    pub parent_plan_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_agent_id: Option<String>,
    pub dependencies: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct TasksDB {
    db: Arc<DB>,
    sqlite_mutex: Mutex<()>,
}

impl TasksDB {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db, sqlite_mutex: Mutex::new(()) }
    }

    pub async fn claim_task(&self, organization_id: &str, agent_id: &str) -> Result<Option<SharedTask>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    SELECT * FROM shared_tasks
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
                        UPDATE shared_tasks
                        SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
                        WHERE id = $2
                        "#
                    )
                    .bind(agent_id)
                    .bind(&task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Ok(Some(SharedTask {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        parent_plan_id: row.get("parent_plan_id"),
                        title: row.get("title"),
                        description: row.get("description"),
                        status: "ASSIGNED".to_string(),
                        assigned_agent_id: Some(agent_id.to_string()),
                        dependencies: row.get("dependencies"),
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
                    SELECT * FROM shared_tasks
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
                        UPDATE shared_tasks
                        SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
                        WHERE id = ?
                        "#
                    )
                    .bind(agent_id)
                    .bind(&task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    let created_str: String = row.get("created_at");
                    let dt_created = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S")
                        .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                        .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(|_| chrono::Utc::now());


                    Ok(Some(SharedTask {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        parent_plan_id: row.get("parent_plan_id"),
                        title: row.get("title"),
                        description: row.get("description"),
                        status: "ASSIGNED".to_string(),
                        assigned_agent_id: Some(agent_id.to_string()),
                        dependencies: serde_json::from_str(row.get("dependencies")).unwrap_or_default(),
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
}
