use sqlx::Row;
use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::tasks::SharedTask;
use chrono::Utc;

pub struct TaskDecompositionService {
    db: Arc<DB>,
}

impl TaskDecompositionService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_task(&self, task: SharedTask) -> Result<SharedTask, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let deps = serde_json::to_value(&task.dependencies).map_err(|e| e.to_string())?;
                let payload = serde_json::from_str::<serde_json::Value>(&task.payload).unwrap_or(serde_json::json!({}));
                let deliberation = serde_json::from_str::<serde_json::Value>(task.deliberation_log.as_deref().unwrap_or("[]")).unwrap_or(serde_json::json!([]));

                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_decomposition (
                        id, organization_id, mission_id, parent_plan_id, dependencies,
                        title, description, status, priority, payload, deliberation_log,
                        depth, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.mission_id)
                .bind(&task.parent_plan_id)
                .bind(&deps)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.priority)
                .bind(&payload)
                .bind(&deliberation)
                .bind(task.depth)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let deps_str = serde_json::to_string(&task.dependencies).map_err(|e| e.to_string())?;
                let payload_str = if task.payload.is_empty() { "{}" } else { &task.payload };
                let deliberation_str = task.deliberation_log.as_deref().unwrap_or("[]");

                sqlx::query(
                    r#"
                    INSERT INTO shared_tasks_decomposition (
                        id, organization_id, mission_id, parent_plan_id, dependencies,
                        title, description, status, priority, payload, deliberation_log,
                        depth, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&task.id)
                .bind(&task.organization_id)
                .bind(&task.mission_id)
                .bind(&task.parent_plan_id)
                .bind(&deps_str)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.priority)
                .bind(payload_str)
                .bind(deliberation_str)
                .bind(task.depth)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(task)
    }

    pub async fn claim_task(&self, agent_id: &str) -> Result<Option<SharedTask>, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                // Use FOR UPDATE SKIP LOCKED
                let row_opt = sqlx::query(
                    r#"
                    SELECT st.id, st.dependencies FROM shared_tasks_decomposition st
                    WHERE st.status = 'PENDING'
                    AND NOT EXISTS (SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep_id JOIN shared_tasks_decomposition parent ON parent.id::text = dep_id WHERE parent.status != 'COMPLETED')
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => return Ok(None)
                };

                let id: String = row.get("id");
                let mut _skip = false;
                let deps_val: serde_json::Value = row.get("dependencies");
                let deps: Vec<String> = serde_json::from_value(deps_val).unwrap_or_default();

                // DAG Dependency check
                if !deps.is_empty() {
                    let mut is_ready = true;
                    for dep in deps {
                        let dep_status: Option<String> = sqlx::query_scalar(
                            "SELECT status FROM shared_tasks_decomposition WHERE id = $1"
                        )
                        .bind(&dep)
                        .fetch_optional(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        if dep_status != Some("COMPLETED".to_string()) {
                            is_ready = false;
                            break;
                        }
                    }
                    if !is_ready {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Ok(None);
                    }
                }

                // Transition state
                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'EXECUTING', assigned_agent_id = $1, updated_at = $2
                    WHERE id = $3
                    "#
                )
                .bind(agent_id)
                .bind(now)
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                // Record transition
                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                    VALUES ($1, $2, 'PENDING', 'EXECUTING', $3, $4)
                    "#
                )
                .bind(trans_id)
                .bind(&id)
                .bind(agent_id)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let task = self.get_task_pg(&mut tx, &id).await?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(Some(task))
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT st.id, st.dependencies FROM shared_tasks_decomposition st
                    WHERE st.status = 'PENDING'
                    AND NOT EXISTS (
                        SELECT 1
                        FROM json_each(st.dependencies) AS dep_id
                        JOIN shared_tasks_decomposition parent ON parent.id = dep_id.value
                        WHERE parent.status != 'COMPLETED'
                    )
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => return Ok(None)
                };

                let id: String = row.get("id");
                let deps_str: String = row.get("dependencies");
                let deps: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();

                if !deps.is_empty() {
                    let mut is_ready = true;
                    for dep in deps {
                        let dep_status: Option<String> = sqlx::query_scalar(
                            "SELECT status FROM shared_tasks_decomposition WHERE id = ?"
                        )
                        .bind(&dep)
                        .fetch_optional(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        if dep_status != Some("COMPLETED".to_string()) {
                            is_ready = false;
                            break;
                        }
                    }
                    if !is_ready {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Ok(None);
                    }
                }

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'EXECUTING', assigned_agent_id = ?, updated_at = ?
                    WHERE id = ?
                    "#
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
                    VALUES (?, ?, 'PENDING', 'EXECUTING', ?, ?)
                    "#
                )
                .bind(trans_id)
                .bind(&id)
                .bind(agent_id)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let task = self.get_task_sqlite(&mut tx, &id).await?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(Some(task))
            }
        }
    }

    async fn get_task_pg(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, id: &str) -> Result<SharedTask, String> {
        let row = sqlx::query(
            "SELECT * FROM shared_tasks_decomposition WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        let deps_val: serde_json::Value = row.get("dependencies");
        let deps: Vec<String> = serde_json::from_value(deps_val).unwrap_or_default();
        let payload_val: serde_json::Value = row.get("payload");
        let payload = payload_val.to_string();
        let delib_val: serde_json::Value = row.get("deliberation_log");
        let deliberation_log = Some(delib_val.to_string());

        Ok(SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            mission_id: row.get("mission_id"),
            parent_plan_id: row.get("parent_plan_id"),
            dependencies: deps,
            title: row.get("title"),
            description: row.get("description"),
            assigned_agent_id: row.get("assigned_agent_id"),
            status: row.get("status"),
            priority: row.get("priority"),
            payload,
            locked_until: row.get("locked_until"),
            ultraplan_phase: row.get("ultraplan_phase"),
            deliberation_log,
            depth: row.get("depth"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            action_risk: row.get("action_risk"),
            approval_status: row.get("approval_status"),
            proposed_content: row.get("proposed_content"),
        })
    }

    async fn get_task_sqlite(&self, tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, id: &str) -> Result<SharedTask, String> {
        let row = sqlx::query(
            "SELECT * FROM shared_tasks_decomposition WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        let deps_str: String = row.get("dependencies");
        let deps: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();
        let payload: String = row.get("payload");
        let deliberation_log: Option<String> = row.get("deliberation_log");

        let created_at: String = row.get("created_at");
        let dt_created = chrono::DateTime::parse_from_rfc3339(&created_at).unwrap_or_default().with_timezone(&Utc);
        let updated_at: String = row.get("updated_at");
        let dt_updated = chrono::DateTime::parse_from_rfc3339(&updated_at).unwrap_or_default().with_timezone(&Utc);

        Ok(SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            mission_id: row.get("mission_id"),
            parent_plan_id: row.get("parent_plan_id"),
            dependencies: deps,
            title: row.get("title"),
            description: row.get("description"),
            assigned_agent_id: row.get("assigned_agent_id"),
            status: row.get("status"),
            priority: row.get("priority"),
            payload,
            locked_until: None,
            ultraplan_phase: row.get("ultraplan_phase"),
            deliberation_log,
            depth: row.get("depth"),
            created_at: dt_created,
            updated_at: dt_updated,
            action_risk: row.get("action_risk"),
            approval_status: row.get("approval_status"),
            proposed_content: row.get("proposed_content"),
        })
    }


    pub async fn fail_task(&self, task_id: &str, agent_id: &str, reason: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let old_status: Option<String> = sqlx::query_scalar(
                    "SELECT status FROM shared_tasks_decomposition WHERE id = $1 FOR UPDATE"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let old_status = match old_status {
                    Some(s) => s,
                    None => return Err("Task not found".to_string())
                };

                let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap_or_else(|_| "{}".to_string());

                sqlx::query(
                    "UPDATE shared_tasks_decomposition SET status = 'FAILED', payload = COALESCE(payload, '{}'::jsonb) || $1::jsonb, updated_at = $2 WHERE id = $3"
                )
                .bind(payload_update)
                .bind(now)
                .bind(task_id)
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
                .bind(task_id)
                .bind(old_status)
                .bind("FAILED")
                .bind(agent_id)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            },
            DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let old_status: Option<String> = sqlx::query_scalar(
                    "SELECT status FROM shared_tasks_decomposition WHERE id = ?"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let old_status = match old_status {
                    Some(s) => s,
                    None => return Err("Task not found".to_string())
                };

                // SQLite json patching
                let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap_or_else(|_| "{}".to_string());
                sqlx::query(
                    "UPDATE shared_tasks_decomposition SET status = 'FAILED', payload = json_patch(COALESCE(payload, '{}'), ?), updated_at = ? WHERE id = ?"
                )
                .bind(payload_update)
                .bind(now.to_rfc3339())
                .bind(task_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(trans_id)
                .bind(task_id)
                .bind(old_status)
                .bind("FAILED")
                .bind(agent_id)
                .bind(now.to_rfc3339())
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn update_status(&self, id: &str, new_status: &str, agent_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let old_status: Option<String> = sqlx::query_scalar(
                    "SELECT status FROM shared_tasks_decomposition WHERE id = $1 FOR UPDATE"
                )
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let old_status = match old_status {
                    Some(s) => s,
                    None => return Err("Task not found".to_string())
                };

                sqlx::query(
                    "UPDATE shared_tasks_decomposition SET status = $1, updated_at = $2 WHERE id = $3"
                )
                .bind(new_status)
                .bind(now)
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
                .bind(old_status)
                .bind(new_status)
                .bind(agent_id)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let old_status: Option<String> = sqlx::query_scalar(
                    "SELECT status FROM shared_tasks_decomposition WHERE id = ?"
                )
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let old_status = match old_status {
                    Some(s) => s,
                    None => return Err("Task not found".to_string())
                };

                sqlx::query(
                    "UPDATE shared_tasks_decomposition SET status = ?, updated_at = ? WHERE id = ?"
                )
                .bind(new_status)
                .bind(now)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(trans_id)
                .bind(id)
                .bind(old_status)
                .bind(new_status)
                .bind(agent_id)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
