use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTaskV4 {
    pub id: String,
    pub organization_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub agent_id: Option<String>,
    pub priority: String,
    pub payload: Option<String>,
    pub parent_plan_id: Option<String>,
    pub dependencies: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SharedTaskOrchestrator {
    db: Arc<DB>,
}

impl SharedTaskOrchestrator {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_task(&self, task: SharedTaskV4) -> Result<SharedTaskV4, String> {
        let task_id = if task.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            task.id.clone()
        };

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_v4 (
                        tenant_id, id, organization_id, title, description, status, agent_id,
                        priority, payload, parent_plan_id, dependencies, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                    "#
                )
                .bind(&task.organization_id)
                .bind(&task_id)
                .bind(&task.organization_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.agent_id)
                .bind(&task.priority)
                .bind(&task.payload)
                .bind(&task.parent_plan_id)
                .bind(&task.dependencies)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_v4 (
                        tenant_id, id, organization_id, title, description, status, agent_id,
                        priority, payload, parent_plan_id, dependencies, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&task.organization_id)
                .bind(&task_id)
                .bind(&task.organization_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.agent_id)
                .bind(&task.priority)
                .bind(&task.payload)
                .bind(&task.parent_plan_id)
                .bind(&task.dependencies)
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

    pub async fn get_task(&self, id: &str) -> Result<SharedTaskV4, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query("SELECT * FROM shared_tasks_v4 WHERE id = $1")
                    .bind(id)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(SharedTaskV4 {
                    id: row.get("id"),
                    organization_id: row.get("organization_id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    status: row.get("status"),
                    agent_id: row.get("agent_id"),
                    priority: row.get("priority"),
                    payload: row.get("payload"),
                    parent_plan_id: row.get("parent_plan_id"),
                    dependencies: row.get("dependencies"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row = sqlx::query("SELECT * FROM shared_tasks_v4 WHERE id = ?")
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

                Ok(SharedTaskV4 {
                    id: row.get("id"),
                    organization_id: row.get("organization_id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    status: row.get("status"),
                    agent_id: row.get("agent_id"),
                    priority: row.get("priority"),
                    payload: row.get("payload"),
                    parent_plan_id: row.get("parent_plan_id"),
                    dependencies: row.get("dependencies"),
                    created_at: dt_created,
                    updated_at: dt_updated,
                })
            }
        }
    }
}
