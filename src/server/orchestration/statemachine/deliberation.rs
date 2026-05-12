use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::tasks::SharedTask;
use chrono::Utc;
use sqlx::Row;

pub struct DeliberationStateMachine {
    pub db: Arc<DB>,
    pub mesh: Arc<dyn crate::orchestration::mesh::TeammateMesh>,
}

impl DeliberationStateMachine {
    pub fn new(db: Arc<DB>, mesh: Arc<dyn crate::orchestration::mesh::TeammateMesh>) -> Self {
        Self { db, mesh }
    }

    pub async fn start_deliberation(&self, task_id: &str, agent_id: &str) -> Result<SharedTask, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT id, dependencies FROM shared_tasks_decomposition
                    WHERE id = $1 AND status = 'PENDING'
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Err(format!("Task {} not found or not in PENDING state (or locked)", task_id));
                    }
                };

                let id: String = row.get("id");
                let deps_val: serde_json::Value = row.try_get("dependencies").unwrap_or_else(|_| serde_json::json!([]));
                let deps: Vec<String> = serde_json::from_value(deps_val).unwrap_or_default();

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
                        return Err(format!("Task {} has unmet dependencies", task_id));
                    }
                }

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'DELIBERATING', assigned_agent_id = $1, updated_at = $2
                    WHERE id = $3
                    "#
                )
                .bind(agent_id)
                .bind(now)
                .bind(&id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row_opt = sqlx::query(
                    r#"
                    SELECT id, dependencies FROM shared_tasks_decomposition
                    WHERE id = ? AND status = 'PENDING'
                    "#
                )
                .bind(task_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => {
                        return Err(format!("Task {} not found or not in PENDING state", task_id));
                    }
                };

                let id: String = row.get("id");
                let deps_str: String = row.try_get("dependencies").unwrap_or_else(|_| "[]".to_string());
                let deps: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();

                if !deps.is_empty() {
                    let mut is_ready = true;
                    for dep in deps {
                        let dep_status: Option<String> = sqlx::query_scalar(
                            "SELECT status FROM shared_tasks_decomposition WHERE id = ?"
                        )
                        .bind(&dep)
                        .fetch_optional(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;

                        if dep_status != Some("COMPLETED".to_string()) {
                            is_ready = false;
                            break;
                        }
                    }
                    if !is_ready {
                        return Err(format!("Task {} has unmet dependencies", task_id));
                    }
                }

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'DELIBERATING', assigned_agent_id = ?, updated_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(agent_id)
                .bind(now)
                .bind(&id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        let _ = self.mesh.publish_with_ack("task.deliberation.started", vec![]).await;

        self.get_task(task_id).await
    }

    pub async fn complete_deliberation(&self, task_id: &str, agent_id: &str, log: &str) -> Result<SharedTask, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT id FROM shared_tasks_decomposition
                    WHERE id = $1 AND assigned_agent_id = $2 AND status = 'DELIBERATING'
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(task_id)
                .bind(agent_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if row_opt.is_none() {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    return Err(format!("Task {} not found, not DELIBERATING, or wrong agent (locked)", task_id));
                }

                let deliberation_json: serde_json::Value = serde_json::from_str(log).unwrap_or_else(|_| serde_json::json!(log));

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'DECOMPOSED', deliberation_log = $1, updated_at = $2
                    WHERE id = $3
                    "#
                )
                .bind(&deliberation_json)
                .bind(now)
                .bind(task_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row_opt = sqlx::query(
                    r#"
                    SELECT id FROM shared_tasks_decomposition
                    WHERE id = ? AND assigned_agent_id = ? AND status = 'DELIBERATING'
                    "#
                )
                .bind(task_id)
                .bind(agent_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                if row_opt.is_none() {
                    return Err(format!("Task {} not found, not DELIBERATING, or wrong agent", task_id));
                }

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'DECOMPOSED', deliberation_log = ?, updated_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(log)
                .bind(now)
                .bind(task_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        let _ = self.mesh.publish_with_ack("task.deliberation.completed", vec![]).await;

        self.get_task(task_id).await
    }

    pub async fn fail_deliberation(&self, task_id: &str, agent_id: &str, reason: &str) -> Result<SharedTask, String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT id, payload FROM shared_tasks_decomposition
                    WHERE id = $1 AND assigned_agent_id = $2 AND status = 'DELIBERATING'
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(task_id)
                .bind(agent_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Err(format!("Task {} not found, not DELIBERATING, or wrong agent (locked)", task_id));
                    }
                };

                let payload_val: serde_json::Value = row.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));
                let mut payload = payload_val.as_object().cloned().unwrap_or_default();
                payload.insert("error".to_string(), serde_json::json!(reason));
                payload.insert("failed_at".to_string(), serde_json::json!(now.to_rfc3339()));

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'FAILED', payload = $1, updated_at = $2
                    WHERE id = $3
                    "#
                )
                .bind(serde_json::to_value(payload).unwrap())
                .bind(now)
                .bind(task_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row_opt = sqlx::query(
                    r#"
                    SELECT id, payload FROM shared_tasks_decomposition
                    WHERE id = ? AND assigned_agent_id = ? AND status = 'DELIBERATING'
                    "#
                )
                .bind(task_id)
                .bind(agent_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                let row = match row_opt {
                    Some(r) => r,
                    None => return Err(format!("Task {} not found, not DELIBERATING, or wrong agent", task_id)),
                };

                let payload_str: String = row.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                let mut payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("error".to_string(), serde_json::json!(reason));
                    obj.insert("failed_at".to_string(), serde_json::json!(now.to_rfc3339()));
                }

                sqlx::query(
                    r#"
                    UPDATE shared_tasks_decomposition
                    SET status = 'FAILED', payload = ?, updated_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(payload.to_string())
                .bind(now)
                .bind(task_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        let _ = self.mesh.publish_with_ack("task.deliberation.failed", vec![]).await;

        self.get_task(task_id).await
    }

    async fn get_task(&self, task_id: &str) -> Result<SharedTask, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;
                let row = sqlx::query("SELECT * FROM shared_tasks_decomposition WHERE id = $1")
                    .bind(task_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                let deps_val: serde_json::Value = row.try_get("dependencies").unwrap_or_else(|_| serde_json::json!([]));
                let dependencies: Vec<String> = serde_json::from_value(deps_val).unwrap_or_default();
                let payload_val: serde_json::Value = row.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));

                Ok(SharedTask {
                    id: row.get("id"),
                    organization_id: row.get("organization_id"),
                    mission_id: row.try_get("mission_id").unwrap_or_default(),
                    parent_plan_id: row.try_get("parent_plan_id").unwrap_or_default(),
                    dependencies,
                    title: row.get("title"),
                    description: row.try_get("description").unwrap_or_default(),
                    assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or_default(),
                    status: row.get("status"),
                    priority: row.try_get("priority").unwrap_or_else(|_| "P2".to_string()),
                    payload: payload_val.to_string(),
                    locked_until: row.try_get("locked_until").unwrap_or_default(),
                    ultraplan_phase: row.try_get("ultraplan_phase").unwrap_or_default(),
                    deliberation_log: row.try_get::<Option<serde_json::Value>, _>("deliberation_log").unwrap_or_default().map(|v: serde_json::Value| v.to_string()),
                    depth: row.try_get("depth").unwrap_or_default(),
                    created_at: row.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                    updated_at: row.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
                    action_risk: None,
                    approval_status: row.try_get("approval_status").unwrap_or_default(),
                    proposed_content: row.try_get("proposed_content").unwrap_or_default(),
                })
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row = sqlx::query("SELECT * FROM shared_tasks_decomposition WHERE id = ?")
                    .bind(task_id)
                    .fetch_one(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let deps_str: String = row.try_get("dependencies").unwrap_or_else(|_| "[]".to_string());
                let dependencies: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();

                Ok(SharedTask {
                    id: row.get("id"),
                    organization_id: row.get("organization_id"),
                    mission_id: row.try_get("mission_id").unwrap_or_default(),
                    parent_plan_id: row.try_get("parent_plan_id").unwrap_or_default(),
                    dependencies,
                    title: row.get("title"),
                    description: row.try_get("description").unwrap_or_default(),
                    assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or_default(),
                    status: row.get("status"),
                    priority: row.try_get("priority").unwrap_or_else(|_| "P2".to_string()),
                    payload: row.try_get("payload").unwrap_or_else(|_| "{}".to_string()),
                    locked_until: row.try_get("locked_until").unwrap_or_default(),
                    ultraplan_phase: row.try_get("ultraplan_phase").unwrap_or_default(),
                    deliberation_log: row.try_get("deliberation_log").unwrap_or_default(),
                    depth: row.try_get("depth").unwrap_or_default(),
                    created_at: row.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                    updated_at: row.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
                    action_risk: None,
                    approval_status: row.try_get("approval_status").unwrap_or_default(),
                    proposed_content: row.try_get("proposed_content").unwrap_or_default(),
                })
            }
        }
    }
}
