use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTask {
    pub id: String,
    pub organization_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub agent_id: Option<String>,
    pub priority: String,
    pub payload: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SharedTaskOrchestrator {
    db: Arc<DB>,
    sqlite_mutex: Mutex<()>,
}

impl SharedTaskOrchestrator {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db, sqlite_mutex: Mutex::new(()) }
    }

    pub async fn create_task(&self, task: SharedTask) -> Result<SharedTask, String> {
        let task_id = if task.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            task.id.clone()
        };

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks (
                        id, organization_id, title, description, status, agent_id,
                        priority, payload, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                    "#
                )
                .bind(&task_id)
                .bind(&task.organization_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.agent_id)
                .bind(&task.priority)
                .bind(&task.payload)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let payload_str = task.payload.as_ref().map(|v| v.to_string());
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks (
                        id, organization_id, title, description, status, agent_id,
                        priority, payload, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&task_id)
                .bind(&task.organization_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.agent_id)
                .bind(&task.priority)
                .bind(&payload_str)
                .bind(task.created_at.to_rfc3339())
                .bind(task.updated_at.to_rfc3339())
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        let mut res = task;
        res.id = task_id;
        Ok(res)
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
                        SET status = 'ASSIGNED', agent_id = $1, updated_at = $2
                        WHERE id = $3
                        "#
                    )
                    .bind(agent_id)
                    .bind(Utc::now())
                    .bind(&task_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Ok(Some(SharedTask {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        title: row.get("title"),
                        description: row.get("description"),
                        status: "ASSIGNED".to_string(),
                        agent_id: Some(agent_id.to_string()),
                        priority: row.get("priority"),
                        payload: row.get("payload"),
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
                        SET status = 'ASSIGNED', agent_id = ?, updated_at = ?
                        WHERE id = ?
                        "#
                    )
                    .bind(agent_id)
                    .bind(Utc::now().to_rfc3339())
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
                        title: row.get("title"),
                        description: row.get("description"),
                        status: "ASSIGNED".to_string(),
                        agent_id: Some(agent_id.to_string()),
                        priority: row.get("priority"),
                        payload: row.get::<Option<String>, _>("payload").map(|s| serde_json::from_str(&s).unwrap_or(serde_json::json!({}))),
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

    pub async fn get_task(&self, id: &str) -> Result<SharedTask, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query("SELECT * FROM shared_tasks WHERE id = $1")
                    .bind(id)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(SharedTask {
                    id: row.get("id"),
                    organization_id: row.get("organization_id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    status: row.get("status"),
                    agent_id: row.get("agent_id"),
                    priority: row.get("priority"),
                    payload: row.get("payload"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row = sqlx::query("SELECT * FROM shared_tasks WHERE id = ?")
                    .bind(id)
                    .fetch_one(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let created_str: String = row.get("created_at");
                let dt_created = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S")
                    .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                    .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
                    .unwrap_or_else(|_| chrono::Utc::now());

                let updated_str: String = row.get("updated_at");
                let dt_updated = chrono::NaiveDateTime::parse_from_str(&updated_str, "%Y-%m-%d %H:%M:%S")
                    .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                    .or_else(|_| chrono::DateTime::parse_from_rfc3339(&updated_str).map(|d| d.with_timezone(&chrono::Utc)))
                    .unwrap_or_else(|_| chrono::Utc::now());

                Ok(SharedTask {
                    id: row.get("id"),
                    organization_id: row.get("organization_id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    status: row.get("status"),
                    agent_id: row.get("agent_id"),
                    priority: row.get("priority"),
                    payload: row.get::<Option<String>, _>("payload").map(|s| serde_json::from_str(&s).unwrap_or(serde_json::json!({}))),
                    created_at: dt_created,
                    updated_at: dt_updated,
                })
            }
        }
    }
}
