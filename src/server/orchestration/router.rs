use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use sqlx::Row;
use serde_json::json;

pub struct DynamicTaskRouter {
    pub db: Arc<DB>,
    pub mesh: Arc<dyn TeammateMesh>,
}

impl DynamicTaskRouter {
    pub fn new(db: Arc<DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { db, mesh }
    }

    pub async fn broadcast_task_available(&self, task_id: &str) -> Result<(), String> {
        let payload = json!({
            "event": "task.available",
            "task_id": task_id
        }).to_string().into_bytes();

        self.mesh.publish("mesh:tasks", payload).await
    }

    pub async fn claim_task(&self, task_id: &str, agent_id: &str) -> Result<bool, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                // Skip locked allows multiple agents to try claiming tasks concurrently without deadlocking
                let row = sqlx::query(
                    "SELECT id, claim_status FROM shared_tasks
                     WHERE id = $1 AND claim_status = 'UNCLAIMED'
                     FOR UPDATE SKIP LOCKED"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if row.is_none() {
                    // Task either doesn't exist, is already claimed, or is locked by another agent
                    let _ = tx.rollback().await;
                    return Ok(false);
                }

                // Claim the task
                sqlx::query(
                    "UPDATE shared_tasks SET claimed_by = $1, claim_status = 'CLAIMED', updated_at = CURRENT_TIMESTAMP WHERE id = $2"
                )
                .bind(agent_id)
                .bind(task_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(true)
            }
            DbStore::Sqlite(sqlite_pool) => {
                // For SQLite, rely on application-level locks using acquire_lock in the mesh,
                // or just standard transactions which are serialized in SQLite

                // Attempt to acquire mesh lock for this task
                let lock_key = format!("task_claim_{}", task_id);
                if !self.mesh.acquire_lock(&lock_key, agent_id, 10).await.unwrap_or(false) {
                    return Ok(false);
                }

                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    "SELECT id, claim_status FROM shared_tasks WHERE id = ?"
                )
                .bind(task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let claim_status: Option<String> = r.try_get("claim_status").unwrap_or(None);
                    if claim_status.as_deref() == Some("UNCLAIMED") || claim_status.is_none() {
                        sqlx::query(
                            "UPDATE shared_tasks SET claimed_by = ?, claim_status = 'CLAIMED', updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                        )
                        .bind(agent_id)
                        .bind(task_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;

                        let _ = self.mesh.release_lock(&lock_key, agent_id).await;
                        return Ok(true);
                    }
                }

                let _ = tx.rollback().await;
                let _ = self.mesh.release_lock(&lock_key, agent_id).await;
                Ok(false)
            }
        }
    }
}
