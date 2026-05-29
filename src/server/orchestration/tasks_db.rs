use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::tasks::SharedTask;
use chrono::Utc;
use sqlx::Row;
use std::time::Instant;

pub struct TasksDB {
    db: Arc<DB>,
    sqlite_mu: tokio::sync::Mutex<()>,
}

impl TasksDB {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            sqlite_mu: tokio::sync::Mutex::new(()),
        }
    }

    pub async fn claim_task(&self, organization_id: &str, agent_id: &str) -> Result<Option<SharedTask>, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT t.id, t.organization_id, t.parent_plan_id, t.title, t.description, t.status, t.assigned_agent_id, t.dependencies::text, t.created_at, t.updated_at
                    FROM shared_tasks t
                    WHERE t.status = 'PENDING' AND t.organization_id = $1
                    AND (t.approval_status IS NULL OR t.approval_status != 'PENDING')
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
                .map_err(|e| e.to_string())?;

                if let Some(r) = row_opt {
                    let id: String = r.get("id");

                    sqlx::query(
                        "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3"
                    )
                    .bind(agent_id)
                    .bind(now)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

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
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Ok(Some(SharedTask {
                        id,
                        organization_id: r.get("organization_id"),
                        parent_plan_id: r.try_get("parent_plan_id").unwrap_or_default(),
                        title: r.get("title"),
                        description: r.try_get("description").unwrap_or_default(),
                        status: "IN_PROGRESS".to_string(),
                        dependencies: serde_json::from_str(r.get("dependencies")).unwrap_or_default(),
                        assigned_agent_id: Some(agent_id.to_string()),
                        created_at: r.get("created_at"),
                        updated_at: now,
                        mission_id: "".to_string(), // These fields are required by SharedTask struct but not directly returned by the above query
                        priority: "P2".to_string(), // Default value
                        payload: "{}".to_string(),
                        locked_until: None,
                        ultraplan_phase: None,
                        deliberation_log: None,
                        depth: None,
                        action_risk: None,
                        approval_status: None,
                        proposed_content: None,
                    }))
                } else {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let lock_result = self.sqlite_mu.try_lock();
                let _lock = match lock_result {
                    Ok(guard) => guard,
                    Err(_) => {
                        let _ = crate::telemetry::record_sqlite_lock_contention(&self.db.pool, "ClaimTask").await;
                        self.sqlite_mu.lock().await
                    }
                };

                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    UPDATE shared_tasks
                    SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = ?
                    WHERE id = (
                        SELECT t.id
                        FROM shared_tasks t
                        WHERE t.status = 'PENDING' AND t.organization_id = ?
                        AND (t.approval_status IS NULL OR t.approval_status != 'PENDING')
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
                .map_err(|e| e.to_string())?;

                if let Some(r) = row_opt {
                    let created_str_opt: Option<String> = r.try_get("created_at").unwrap_or(None);
                    let dt_created = if let Some(created_str) = created_str_opt {
                        chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now())
                    } else {
                        chrono::Utc::now()
                    };

                    let task = SharedTask {
                        id: r.get("id"),
                        organization_id: r.get("organization_id"),
                        parent_plan_id: r.try_get("parent_plan_id").unwrap_or_default(),
                        title: r.get("title"),
                        description: r.try_get("description").unwrap_or_default(),
                        status: r.get("status"),
                        dependencies: serde_json::from_str(r.get::<&str, _>("dependencies")).unwrap_or_default(),
                        assigned_agent_id: r.try_get("assigned_agent_id").unwrap_or_default(),
                        created_at: dt_created,
                        updated_at: now,
                        mission_id: "".to_string(), // These fields are required by SharedTask struct but not directly returned by the above query
                        priority: "P2".to_string(), // Default value
                        payload: "{}".to_string(),
                        locked_until: None,
                        ultraplan_phase: None,
                        deliberation_log: None,
                        depth: None,
                        action_risk: None,
                        approval_status: None,
                        proposed_content: None,
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
                    .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;

                    Ok(Some(task))
                } else {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
        }
    }
}
