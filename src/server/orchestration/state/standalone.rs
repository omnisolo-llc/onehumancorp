use super::StateManager;
use crate::tasks::SharedTask;
use crate::db::{DB, DbStore};
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::Row;
use chrono::Utc;

use crate::orchestration::mesh::TeammateMesh;
use super::MeshLockGuard;

pub struct StandaloneStateManager {
    db: Arc<DB>,
    mesh: Arc<dyn TeammateMesh>,
}

impl StandaloneStateManager {
    pub fn new(db: Arc<DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { db, mesh }
    }

    async fn transition_state_inner(
        &self,
        task_id: &str,
        _tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
        _lock_guard: &MeshLockGuard,
        sqlite_pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<(), String> {
        let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Verify current state
        let row = sqlx::query(
            "SELECT status, dependencies, tenant_id FROM swarm_tasks WHERE id = ?"
        )
        .bind(task_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let row = match row {
            Some(r) => r,
            None => return Err(format!("Task {} not found", task_id)),
        };

        let current_state: String = row.get("status");

        if current_state != from_state {
            return Err(format!(
                "Task {} is in state '{}', expected '{}'",
                task_id, current_state, from_state
            ));
        }

        let tenant_id: String = row.try_get("tenant_id").unwrap_or_else(|_| "system".to_string());

        // DAG validation
        if to_state == "EXECUTING" {
            let deps_str: String = row.try_get("dependencies").unwrap_or_else(|_| "[]".to_string());
            let dependencies: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();
            if !dependencies.is_empty() {
                let placeholders = dependencies.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                let query = format!(
                    "SELECT count(*) FROM swarm_tasks WHERE id IN ({}) AND status = 'COMPLETED'",
                    placeholders
                );

                let mut db_query = sqlx::query_scalar::<_, i32>(&query);
                for dep in &dependencies {
                    db_query = db_query.bind(dep);
                }

                let completed_count: i32 = db_query
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                if completed_count as usize != dependencies.len() {
                    return Err(format!("Not all dependencies are COMPLETED (found {}, expected {})", completed_count, dependencies.len()));
                }
            }
        }

        // 2. Update state
        let now = Utc::now();
        sqlx::query(
            "UPDATE swarm_tasks SET status = ?, assigned_agent_id = ?, updated_at = ? WHERE id = ?"
        )
        .bind(to_state)
        .bind(agent_id)
        .bind(now.to_rfc3339())
        .bind(task_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 3. Record transition
        let trans_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
            VALUES (?, ?, ?, 'swarm_task', ?, ?, ?, ?, ?)
            "#
        )
        .bind(trans_id)
        .bind(&tenant_id)
        .bind(task_id)
        .bind(from_state)
        .bind(to_state)
        .bind(agent_id)
        .bind(reason)
        .bind(now.to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[async_trait]
impl StateManager for StandaloneStateManager {
    async fn transition_state(
        &self,
        task_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<(), String> {
        let sqlite_pool = match &self.db.store {
            DbStore::Sqlite(pool) => pool,
            _ => return Err("StandaloneStateManager requires DbStore::Sqlite".to_string()),
        };

        let lock_key = format!("ohc:lock:{}:task:{}", tenant_id, task_id);

        let transition_future = async {
            let lock_guard = MeshLockGuard::acquire(self.mesh.clone(), lock_key.clone(), "standalone_state_manager".to_string(), 30).await?;
            self.transition_state_inner(task_id, tenant_id, from_state, to_state, agent_id, reason, &lock_guard, sqlite_pool).await
        };

        match tokio::time::timeout(std::time::Duration::from_secs(60), transition_future).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Timeout acquiring lock or writing database transition".to_string()),
        }
    }

    async fn pull_available_tasks(&self, limit: i64) -> Result<Vec<SharedTask>, String> {
        let sqlite_pool = match &self.db.store {
            DbStore::Sqlite(pool) => pool,
            _ => return Err("StandaloneStateManager requires DbStore::Sqlite".to_string()),
        };

        let lock_key = "ohc:lock:system:pull_tasks".to_string();
        let acquire_and_fetch = async {
            let lock_guard = MeshLockGuard::acquire(self.mesh.clone(), lock_key.clone(), "standalone_state_manager".to_string(), 30).await?;

            let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

            let now_rfc = Utc::now().to_rfc3339();
            let rows = sqlx::query(
                r#"
                UPDATE swarm_tasks
                SET status = 'IN_PROGRESS', updated_at = ?
                WHERE id IN (
                    SELECT t.id
                    FROM swarm_tasks t
                    WHERE t.status = 'PENDING'
                    AND NOT EXISTS (
                        SELECT 1
                        FROM json_each(t.dependencies) as dep_id
                        JOIN swarm_tasks dep ON dep.id = dep_id.value
                        WHERE dep.status != 'COMPLETED'
                    )
                    LIMIT ?
                )
                RETURNING *
                "#
            )
            .bind(now_rfc)
            .bind(limit)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            Ok::<_, String>((lock_guard, tx, rows))
        };

        let (_lock_guard, mut tx, rows) = match tokio::time::timeout(std::time::Duration::from_secs(60), acquire_and_fetch).await {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => {
                if e.contains("Timeout acquiring lock") {
                    tracing::warn!("Lock timeout in StandaloneStateManager::pull_available_tasks, fail-safing to empty list.");
                    return Ok(vec![]);
                }
                return Err(e);
            },
            Err(_) => {
                tracing::warn!("Database/Lock timeout in StandaloneStateManager::pull_available_tasks, fail-safing to empty list.");
                return Ok(vec![]);
            }
        };

        let mut tasks = Vec::new();
        let mut task_ids = Vec::new();

        for row in rows {
            let id: String = row.get("id");
            let deps_str: String = row.try_get("dependencies").unwrap_or_else(|_| "[]".to_string());
            let dependencies: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();

            // Check dependencies again explicitly in Rust just to be perfectly safe
            let mut all_completed = true;
            if !dependencies.is_empty() {
                let placeholders = dependencies.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                let query = format!(
                    "SELECT count(*) FROM swarm_tasks WHERE id IN ({}) AND status = 'COMPLETED'",
                    placeholders
                );

                let mut db_query = sqlx::query_scalar::<_, i32>(&query);
                for dep in &dependencies {
                    db_query = db_query.bind(dep);
                }

                let completed_count: i32 = db_query
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                if completed_count as usize != dependencies.len() {
                    all_completed = false;
                }
            }

            if all_completed {
                let tenant_id: String = row.try_get("tenant_id").unwrap_or_else(|_| "system".to_string());
                task_ids.push((id.clone(), tenant_id.clone()));

                let payload_str: String = row.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                let created_at: String = row.try_get("created_at").unwrap_or_else(|_| Utc::now().to_rfc3339());
                let dt_created = chrono::DateTime::parse_from_rfc3339(&created_at).unwrap_or_default().with_timezone(&Utc);
                let updated_at: String = row.try_get("updated_at").unwrap_or_else(|_| Utc::now().to_rfc3339());
                let dt_updated = chrono::DateTime::parse_from_rfc3339(&updated_at).unwrap_or_default().with_timezone(&Utc);

                let t = SharedTask {
                    id: id.clone(),
                    organization_id: tenant_id,
                    mission_id: row.get("mission_id"),
                    parent_plan_id: row.try_get("parent_plan_id").unwrap_or_default(),
                    dependencies,
                    title: row.get("title"),
                    description: row.try_get("description").unwrap_or_default(),
                    assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or_default(),
                    status: row.get("status"),
                    priority: row.try_get("priority").unwrap_or_else(|_| "P2".to_string()),
                    payload: payload_str,
                    locked_until: None,
                    ultraplan_phase: None,
                    deliberation_log: None,
                    depth: None,
                    created_at: dt_created,
                    updated_at: dt_updated,
                    action_risk: None,
                    approval_status: None,
                    proposed_content: None,
                };
                tasks.push(t);
                if tasks.len() as i64 >= limit {
                    break;
                }
            }
        }

        // Transitions are recorded after the UPDATE ... RETURNING
        if !task_ids.is_empty() {
            let now = Utc::now();
            let now_rfc = now.to_rfc3339();
            for (id_str, tenant_id) in task_ids {
                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, occurred_at)
                    VALUES (?, ?, ?, 'swarm_task', 'PENDING', 'IN_PROGRESS', ?)
                    "#
                )
                .bind(trans_id)
                .bind(&tenant_id)
                .bind(&id_str)
                .bind(&now_rfc)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        // Update the returned tasks statuses to match what we committed (IN_PROGRESS)
        for t in &mut tasks {
            t.status = "IN_PROGRESS".to_string();
        }

        Ok(tasks)
    }
}
