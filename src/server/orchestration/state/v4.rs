use crate::db::{DbStore, DB};
use crate::orchestration::mesh::TeammateMesh;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;

pub struct V4StateMachine {
    db: Arc<DB>,
    mesh: Arc<dyn TeammateMesh>,
}

impl V4StateMachine {
    pub fn new(db: Arc<DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { db, mesh }
    }

    async fn transition_state_v4(
        &self,
        task_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), String> {
        let row = sqlx::query(
            "SELECT status FROM shared_tasks_v4 WHERE id = $1 FOR UPDATE SKIP LOCKED",
        )
        .bind(task_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_state: String = r.get("status");
            if current_state != from_state {
                return Err(format!(
                    "Task {} is in state '{}', expected '{}'",
                    task_id, current_state, from_state
                ));
            }

            let now = Utc::now();
            sqlx::query(
                "UPDATE shared_tasks_v4 SET status = $1, agent_id = $2, updated_at = $3 WHERE id = $4",
            )
            .bind(to_state)
            .bind(agent_id)
            .bind(now)
            .bind(task_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            let trans_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
                VALUES ($1, $2, $3, 'shared_task_v4', $4, $5, $6, $7, $8)
                "#,
            )
            .bind(trans_id)
            .bind(tenant_id)
            .bind(task_id)
            .bind(from_state)
            .bind(to_state)
            .bind(agent_id)
            .bind(reason)
            .bind(now)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            Ok(())
        } else {
            Err(format!("Task {} not found or locked", task_id))
        }
    }

    async fn transition_state_sub_agent(
        &self,
        queue_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        worker_id: Option<&str>,
        reason: Option<&str>,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), String> {
        let row = sqlx::query(
            "SELECT status FROM sub_agent_queue WHERE id = $1 FOR UPDATE SKIP LOCKED",
        )
        .bind(queue_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_state: String = r.get("status");
            if current_state != from_state {
                return Err(format!(
                    "Queue {} is in state '{}', expected '{}'",
                    queue_id, current_state, from_state
                ));
            }

            let now = Utc::now();
            sqlx::query(
                "UPDATE sub_agent_queue SET status = $1, worker_id = $2, updated_at = $3 WHERE id = $4",
            )
            .bind(to_state)
            .bind(worker_id)
            .bind(now)
            .bind(queue_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            let trans_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
                VALUES ($1, $2, $3, 'sub_agent_queue', $4, $5, $6, $7, $8)
                "#,
            )
            .bind(trans_id)
            .bind(tenant_id)
            .bind(queue_id)
            .bind(from_state)
            .bind(to_state)
            .bind(worker_id)
            .bind(reason)
            .bind(now)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            Ok(())
        } else {
            Err(format!("Queue {} not found or locked", queue_id))
        }
    }

    async fn transition_state_v4_sqlite(
        &self,
        task_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), String> {
        let row = sqlx::query(
            "SELECT status FROM shared_tasks_v4 WHERE id = ?",
        )
        .bind(task_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_state: String = r.get("status");
            if current_state != from_state {
                return Err(format!(
                    "Task {} is in state '{}', expected '{}'",
                    task_id, current_state, from_state
                ));
            }

            let now = Utc::now().to_rfc3339();
            sqlx::query(
                "UPDATE shared_tasks_v4 SET status = ?, agent_id = ?, updated_at = ? WHERE id = ?",
            )
            .bind(to_state)
            .bind(agent_id)
            .bind(&now)
            .bind(task_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            let trans_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
                VALUES (?, ?, ?, 'shared_task_v4', ?, ?, ?, ?, ?)
                "#,
            )
            .bind(trans_id)
            .bind(tenant_id)
            .bind(task_id)
            .bind(from_state)
            .bind(to_state)
            .bind(agent_id)
            .bind(reason)
            .bind(&now)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            Ok(())
        } else {
            Err(format!("Task {} not found", task_id))
        }
    }

    async fn transition_state_sub_agent_sqlite(
        &self,
        queue_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        worker_id: Option<&str>,
        reason: Option<&str>,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), String> {
        let row = sqlx::query(
            "SELECT status FROM sub_agent_queue WHERE id = ?",
        )
        .bind(queue_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let current_state: String = r.get("status");
            if current_state != from_state {
                return Err(format!(
                    "Queue {} is in state '{}', expected '{}'",
                    queue_id, current_state, from_state
                ));
            }

            let now = Utc::now().to_rfc3339();
            sqlx::query(
                "UPDATE sub_agent_queue SET status = ?, worker_id = ?, updated_at = ? WHERE id = ?",
            )
            .bind(to_state)
            .bind(worker_id)
            .bind(&now)
            .bind(queue_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            let trans_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO state_machine_transitions (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
                VALUES (?, ?, ?, 'sub_agent_queue', ?, ?, ?, ?, ?)
                "#,
            )
            .bind(trans_id)
            .bind(tenant_id)
            .bind(queue_id)
            .bind(from_state)
            .bind(to_state)
            .bind(worker_id)
            .bind(reason)
            .bind(&now)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            Ok(())
        } else {
            Err(format!("Queue {} not found", queue_id))
        }
    }

    pub async fn transition_task_v4(
        &self,
        task_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_system_context(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                self.transition_state_v4(
                    task_id, tenant_id, from_state, to_state, agent_id, reason, &mut tx,
                )
                .await?;
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(pool) => {
                let lock_key = format!("ohc:lock:{}:task_v4:{}", tenant_id, task_id);
                let _lock_guard = crate::orchestration::state::mod::MeshLockGuard::acquire(
                    self.mesh.clone(),
                    lock_key,
                    "v4_state_manager".to_string(),
                    30,
                )
                .await?;

                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                self.transition_state_v4_sqlite(
                    task_id, tenant_id, from_state, to_state, agent_id, reason, &mut tx,
                )
                .await?;
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn transition_sub_agent_queue(
        &self,
        queue_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        worker_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_system_context(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                self.transition_state_sub_agent(
                    queue_id, tenant_id, from_state, to_state, worker_id, reason, &mut tx,
                )
                .await?;
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(pool) => {
                let lock_key = format!("ohc:lock:{}:queue:{}", tenant_id, queue_id);
                let _lock_guard = crate::orchestration::state::mod::MeshLockGuard::acquire(
                    self.mesh.clone(),
                    lock_key,
                    "v4_state_manager".to_string(),
                    30,
                )
                .await?;

                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                self.transition_state_sub_agent_sqlite(
                    queue_id, tenant_id, from_state, to_state, worker_id, reason, &mut tx,
                )
                .await?;
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }
}
