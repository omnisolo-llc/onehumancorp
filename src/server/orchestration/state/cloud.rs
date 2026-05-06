use crate::tasks::SharedTask;
use crate::db::DB;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::Row;
use chrono::Utc;



use crate::orchestration::mesh::TeammateMesh;
use super::MeshLockGuard;

pub struct CloudStateManager {
    db: Arc<DB>,
    mesh: Arc<dyn TeammateMesh>,
}

impl CloudStateManager {
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
    ) -> Result<(), String> {
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

        // 1. Verify current state with FOR UPDATE
        let row = sqlx::query(
            "SELECT status, dependencies, payload, tenant_id FROM swarm_tasks WHERE id = $1::uuid FOR UPDATE"
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

        let tenant_id_db: String = row.try_get("tenant_id").unwrap_or_else(|_| "system".to_string());

        // DAG validation
        if to_state == "EXECUTING" {
            let deps_val: serde_json::Value = row.try_get("dependencies").unwrap_or_else(|_| serde_json::json!([]));
            let dependencies: Vec<String> = serde_json::from_value(deps_val).unwrap_or_default();

            if !dependencies.is_empty() {
                // Parse UUIDs carefully, return an error if a dependency id is invalid
                let mut dep_uuids = Vec::with_capacity(dependencies.len());
                for dep in &dependencies {
                    let parsed = uuid::Uuid::parse_str(dep).map_err(|_| format!("Invalid dependency UUID: {}", dep))?;
                    dep_uuids.push(parsed);
                }

                let completed_count: i64 = sqlx::query_scalar(
                    "SELECT count(*) FROM swarm_tasks WHERE id = ANY($1) AND status = 'COMPLETED'"
                )
                .bind(&dep_uuids)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if completed_count as usize != dep_uuids.len() {
                    return Err(format!("Not all dependencies are COMPLETED (found {}, expected {})", completed_count, dep_uuids.len()));
                }
            }
        }

        // 2. Update state
        let now = Utc::now();
        sqlx::query(
            "UPDATE swarm_tasks SET status = $1, assigned_agent_id = $2, updated_at = $3 WHERE id = $4::uuid"
        )
        .bind(to_state)
        .bind(agent_id)
        .bind(now)
        .bind(task_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 3. Record transition
        let trans_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
            VALUES ($1, $2, $3, 'swarm_task', $4, $5, $6, $7, $8)
            "#
        )
        .bind(trans_id)
        .bind(&tenant_id_db)
        .bind(task_id)
        .bind(from_state)
        .bind(to_state)
        .bind(agent_id)
        .bind(reason)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[async_trait]
impl crate::orchestration::state::StateManager for CloudStateManager {
    async fn transition_state(
        &self,
        task_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<(), String> {
        let lock_key = format!("ohc:lock:{}:task:{}", tenant_id, task_id);

        let acquire_future = MeshLockGuard::acquire(self.mesh.clone(), lock_key.clone(), "cloud_state_manager".to_string(), 30);
        let lock_guard = match tokio::time::timeout(std::time::Duration::from_secs(2), acquire_future).await {
            Ok(Ok(guard)) => guard,
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err("Timeout acquiring lock".to_string()),
        };

        // Will drop automatically when block exits
        self.transition_state_inner(task_id, tenant_id, from_state, to_state, agent_id, reason, &lock_guard).await
    }

    async fn pull_available_tasks(&self, limit: i64) -> Result<Vec<SharedTask>, String> {
        let lock_key = "ohc:lock:system:pull_tasks".to_string();
        let acquire_future = MeshLockGuard::acquire(self.mesh.clone(), lock_key.clone(), "cloud_state_manager".to_string(), 30);
        let _lock_guard = match tokio::time::timeout(std::time::Duration::from_secs(2), acquire_future).await {
            Ok(Ok(guard)) => guard,
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                tracing::warn!("Lock timeout in CloudStateManager::pull_available_tasks, fail-safing to empty list.");
                return Ok(vec![]);
            }
        };

        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

        let rows_future = sqlx::query(
            r#"
            SELECT t.*
            FROM swarm_tasks t
            WHERE t.status = 'PENDING'
              AND NOT EXISTS (
                  SELECT 1
                  FROM jsonb_array_elements_text(t.dependencies) as dep_id
                  JOIN swarm_tasks dep ON dep.id::text = dep_id
                  WHERE dep.status != 'COMPLETED'
              )
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#
        )
        .bind(limit)
        .fetch_all(&mut *tx);

        let rows = match tokio::time::timeout(std::time::Duration::from_secs(2), rows_future).await {
            Ok(Ok(rows)) => rows,
            Ok(Err(e)) => return Err(e.to_string()),
            Err(_) => {
                tracing::warn!("Database timeout in CloudStateManager::pull_available_tasks, fail-safing to empty list.");
                return Ok(vec![]);
            }
        };

        let mut tasks = Vec::new();
        let mut task_ids = Vec::new();

        for row in &rows {
            let id: uuid::Uuid = row.get("id");
            let id_str = id.to_string();

            let tenant_id: String = row.try_get("tenant_id").unwrap_or_else(|_| "system".to_string());
            task_ids.push((id_str.clone(), tenant_id.clone()));

            let deps_val: serde_json::Value = row.try_get("dependencies").unwrap_or_else(|_| serde_json::json!([]));
            let dependencies: Vec<String> = serde_json::from_value(deps_val).unwrap_or_default();

            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));
            let payload = payload_val.to_string();

            tasks.push(SharedTask {
                id: id_str,
                organization_id: tenant_id,
                mission_id: row.get("mission_id"),
                parent_plan_id: row.try_get("parent_plan_id").unwrap_or_default(),
                dependencies,
                title: row.get("title"),
                description: row.try_get("description").unwrap_or_default(),
                assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or_default(),
                status: row.get("status"),
                priority: row.try_get("priority").unwrap_or_else(|_| "P2".to_string()),
                payload,
                locked_until: row.try_get("locked_until").unwrap_or_default(),
                ultraplan_phase: None,
                deliberation_log: None,
                depth: None,
                created_at: row.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
                action_risk: None,
                approval_status: None,
                proposed_content: None,
            });
        }

        if !task_ids.is_empty() {
            let now = Utc::now();
            for (id_str, tenant_id) in task_ids {
                sqlx::query(
                    "UPDATE swarm_tasks SET status = 'IN_PROGRESS', updated_at = $1 WHERE id = $2::uuid"
                )
                .bind(now)
                .bind(&id_str)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let trans_id = uuid::Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, occurred_at)
                    VALUES ($1, $2, $3, 'swarm_task', 'PENDING', 'IN_PROGRESS', $4)
                    "#
                )
                .bind(trans_id)
                .bind(&tenant_id)
                .bind(&id_str)
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        for t in &mut tasks {
            t.status = "IN_PROGRESS".to_string();
        }

        Ok(tasks)
    }
}
