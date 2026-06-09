use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tokio::sync::Mutex;

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
    pub ultraplan_phase: Option<String>,
    pub deliberation_log: Option<String>,
    pub depth: Option<i32>,
}

pub struct SharedTaskOrchestrator {
    db: Arc<DB>,
    sqlite_mutex: Mutex<()>,
}

impl SharedTaskOrchestrator {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db, sqlite_mutex: Mutex::new(()) }
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
                        id, organization_id, title, description, status, agent_id,
                        priority, payload, parent_plan_id, dependencies, created_at, updated_at,
                        ultraplan_phase, deliberation_log, depth
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
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
                .bind(&task.parent_plan_id)
                .bind(&task.dependencies)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .bind(&task.ultraplan_phase)
                .bind(&task.deliberation_log)
                .bind(task.depth)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_v4 (
                        id, organization_id, title, description, status, agent_id,
                        priority, payload, parent_plan_id, dependencies, created_at, updated_at,
                        ultraplan_phase, deliberation_log, depth
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
                .bind(&task.parent_plan_id)
                .bind(&task.dependencies)
                .bind(task.created_at.to_rfc3339())
                .bind(task.updated_at.to_rfc3339())
                .bind(&task.ultraplan_phase)
                .bind(&task.deliberation_log)
                .bind(task.depth)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        let mut res = task;
        res.id = task_id;
        Ok(res)
    }

    pub async fn claim_task(&self, organization_id: &str, agent_id: &str) -> Result<Option<SharedTaskV4>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    SELECT st.* FROM shared_tasks_v4 st
                    WHERE (st.status = 'PENDING' OR st.ultraplan_phase = 'APPROVED') AND st.organization_id = $1
                    AND NOT EXISTS (
                        SELECT 1 FROM json_array_elements_text(st.dependencies::json) AS dep_id
                        JOIN shared_tasks_v4 parent ON parent.id = dep_id
                        WHERE parent.status != 'COMPLETED'
                    )
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
                        UPDATE shared_tasks_v4
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

                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at) VALUES ($1, $2, $3, 'task', 'PENDING', 'ASSIGNED', $4, $5)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&task_id)
                    .bind(agent_id)
                    .bind(Utc::now())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Ok(Some(SharedTaskV4 {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        title: row.get("title"),
                        description: row.get("description"),
                        status: "ASSIGNED".to_string(),
                        agent_id: Some(agent_id.to_string()),
                        priority: row.get("priority"),
                        payload: row.get("payload"),
                        parent_plan_id: row.get("parent_plan_id"),
                        dependencies: row.get("dependencies"),
                        created_at: row.get("created_at"),
                        updated_at: Utc::now(),
                        ultraplan_phase: row.get("ultraplan_phase"),
                        deliberation_log: row.get("deliberation_log"),
                        depth: row.get("depth"),
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
                    SELECT st.* FROM shared_tasks_v4 st
                    WHERE (st.status = 'PENDING' OR st.ultraplan_phase = 'APPROVED') AND st.organization_id = ?
                    AND NOT EXISTS (
                        SELECT 1 FROM json_each(st.dependencies) AS dep_id
                        JOIN shared_tasks_v4 parent ON parent.id = dep_id.value
                        WHERE parent.status != 'COMPLETED'
                    )
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
                        UPDATE shared_tasks_v4
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

                    let trans_id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at) VALUES (?, ?, ?, 'task', 'PENDING', 'ASSIGNED', ?, ?)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&task_id)
                    .bind(agent_id)
                    .bind(Utc::now().to_rfc3339())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    let created_str: String = row.get("created_at");
                    let dt_created = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S")
                        .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                        .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(|_| chrono::Utc::now());

                    Ok(Some(SharedTaskV4 {
                        id: task_id,
                        organization_id: row.get("organization_id"),
                        title: row.get("title"),
                        description: row.get("description"),
                        status: "ASSIGNED".to_string(),
                        agent_id: Some(agent_id.to_string()),
                        priority: row.get("priority"),
                        payload: row.get("payload"),
                        parent_plan_id: row.get("parent_plan_id"),
                        dependencies: row.get("dependencies"),
                        created_at: dt_created,
                        updated_at: Utc::now(),
                        ultraplan_phase: row.get("ultraplan_phase"),
                        deliberation_log: row.get("deliberation_log"),
                        depth: row.get("depth"),
                    }))
                } else {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
        }
    }

    pub async fn update_task_status(&self, id: &str, new_status: &str, agent_id: Option<&str>) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let current_status: String = sqlx::query("SELECT status FROM shared_tasks_v4 WHERE id = $1")
                    .bind(id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|row| row.get("status"))
                    .map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_v4
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#
                )
                .bind(new_status)
                .bind(Utc::now())
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(trans_id)
                .bind(id)
                .bind(current_status)
                .bind(new_status)
                .bind(agent_id)
                .bind(Utc::now())
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let current_status: String = sqlx::query("SELECT status FROM shared_tasks_v4 WHERE id = ?")
                    .bind(id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|row| row.get("status"))
                    .map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_v4
                    SET status = ?, updated_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(new_status)
                .bind(Utc::now().to_rfc3339())
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at) VALUES (?, ?, ?, 'task', ?, ?, ?, ?)
                    "#
                )
                .bind(trans_id)
                .bind(id)
                .bind(current_status)
                .bind(new_status)
                .bind(agent_id)
                .bind(Utc::now().to_rfc3339())
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn complete_task(&self, id: &str, agent_id: Option<&str>) -> Result<(), String> {
        self.update_task_status(id, "COMPLETED", agent_id).await
    }

    pub async fn list_tasks(&self, organization_id: &str) -> Result<Vec<SharedTaskV4>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let rows = sqlx::query("SELECT * FROM shared_tasks_v4 WHERE organization_id = $1 ORDER BY created_at DESC")
                    .bind(organization_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let tasks = rows.into_iter().map(|row| {
                    SharedTaskV4 {
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
                        ultraplan_phase: row.get("ultraplan_phase"),
                        deliberation_log: row.get("deliberation_log"),
                        depth: row.get("depth"),
                    }
                }).collect();

                Ok(tasks)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let rows = sqlx::query("SELECT * FROM shared_tasks_v4 WHERE organization_id = ? ORDER BY created_at DESC")
                    .bind(organization_id)
                    .fetch_all(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let tasks = rows.into_iter().map(|row| {
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

                    SharedTaskV4 {
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
                        ultraplan_phase: row.get("ultraplan_phase"),
                        deliberation_log: row.get("deliberation_log"),
                        depth: row.get("depth"),
                    }
                }).collect();

                Ok(tasks)
            }
        }
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
                    ultraplan_phase: row.get("ultraplan_phase"),
                    deliberation_log: row.get("deliberation_log"),
                    depth: row.get("depth"),
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
                    ultraplan_phase: row.get("ultraplan_phase"),
                    deliberation_log: row.get("deliberation_log"),
                    depth: row.get("depth"),
                })
            }
        }
    }
}
