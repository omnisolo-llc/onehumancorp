
use crate::tasks::SharedTask;
use crate::db::DB;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::Row;
use chrono::Utc;
use redis::AsyncCommands;

struct RedisLockGuard {
    client: Option<redis::Client>,
    key: String,
}

impl RedisLockGuard {
    async fn acquire(client: Option<&redis::Client>, key: String) -> Result<Self, String> {
        if let Some(c) = client {
            let mut conn = c.get_multiplexed_async_connection().await.map_err(|e| format!("Failed to connect to Redis: {}", e))?;
            let acquired: redis::RedisResult<Option<String>> = redis::cmd("SET")
                .arg(&key)
                .arg("locked")
                .arg("NX")
                .arg("EX")
                .arg(30)
                .query_async(&mut conn)
                .await;

            match acquired {
                Ok(Some(resp)) if resp == "OK" => {
                    Ok(Self { client: Some(c.clone()), key })
                }
                _ => {
                    Err(format!("Task {} is currently locked via Redis", key))
                }
            }
        } else {
            Ok(Self { client: None, key })
        }
    }
}

impl Drop for RedisLockGuard {
    fn drop(&mut self) {
        if let Some(client) = self.client.take() {
            let key = self.key.clone();
            tokio::spawn(async move {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: redis::RedisResult<()> = conn.del(&key).await;
                }
            });
        }
    }
}

pub struct CloudStateManager {
    db: Arc<DB>,
    redis_client: Option<redis::Client>,
}

impl CloudStateManager {
    pub fn new(db: Arc<DB>, redis_client: Option<redis::Client>) -> Self {
        Self { db, redis_client }
    }

    async fn transition_state_inner(
        &self,
        task_id: &str,
        _tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
        _lock_guard: &RedisLockGuard,
    ) -> Result<(), String> {
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

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

        let lock_guard = RedisLockGuard::acquire(self.redis_client.as_ref(), lock_key).await?;

        // Will drop automatically when block exits
        self.transition_state_inner(task_id, tenant_id, from_state, to_state, agent_id, reason, &lock_guard).await
    }

    async fn pull_available_tasks(&self, limit: i64) -> Result<Vec<SharedTask>, String> {
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

        let rows = sqlx::query(
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
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

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
