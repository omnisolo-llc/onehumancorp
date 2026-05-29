use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use ::server_ohc::orchestration::TeammateMeshEvent;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAvailablePayload {
    pub task_id: String,
    pub required_skills: Vec<String>,
    pub details: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskClaimPayload {
    pub task_id: String,
    pub agent_id: String,
    pub capability_score: i32,
}

pub struct DynamicTaskRouter {
    db: Arc<DB>,
    mesh: Arc<dyn TeammateMesh>,
}

impl DynamicTaskRouter {
    pub fn new(db: Arc<DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { db, mesh }
    }

    pub async fn start_listener(self: Arc<Self>) -> Result<(), String> {
        let router = self.clone();
        let _ = self.mesh.subscribe("task.claim", Box::new(move |msg| {
            let payload_bytes = msg.payload;
            if let Ok(payload) = serde_json::from_slice::<TaskClaimPayload>(&payload_bytes) {
                let router_clone = router.clone();
                tokio::spawn(async move {
                    let _ = router_clone.handle_claim(payload).await;
                });
            }
        })).await?;
        Ok(())
    }

    pub async fn broadcast_task_available(&self, payload: TaskAvailablePayload) -> Result<(), String> {
        let payload_bytes = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;

        let event = TeammateMeshEvent {
            agent_id: "system".to_string(),
            action: "task.available".to_string(),
            status: "ok".to_string(),
            payload: payload_bytes,
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        // We can just use the mesh's publish method since it takes a bytes payload or we use TeammateMeshEvent directly
        // Wait, mesh.publish takes (topic: &str, payload: Vec<u8>) for some traits and TeammateMeshEvent for others. Let's check MeshTransport vs TeammateMesh
        // From mesh.rs: `async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String>;`
        let event_bytes = serde_json::to_vec(&event).unwrap_or_default(); // But we probably want to send payload if TeammateMeshEvent is expected.
        self.mesh.publish("task.available", event_bytes).await
    }

    pub async fn handle_claim(&self, payload: TaskClaimPayload) -> Result<bool, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    SELECT claim_status FROM shared_tasks
                    WHERE id = $1
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .bind(&payload.task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let status: Option<String> = r.get("claim_status");
                    if status.as_deref() != Some("CLAIMED") {
                        sqlx::query(
                            r#"
                            UPDATE shared_tasks
                            SET claimed_by = $1, claim_status = 'CLAIMED', updated_at = $2
                            WHERE id = $3
                            "#
                        )
                        .bind(&payload.agent_id)
                        .bind(Utc::now())
                        .bind(&payload.task_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Ok(true);
                    }
                }

                tx.rollback().await.unwrap_or_default();
                Ok(false)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row = sqlx::query(
                    r#"
                    SELECT claim_status FROM shared_tasks
                    WHERE id = ?
                    "#
                )
                .bind(&payload.task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let result = sqlx::query(
                    r#"
                    UPDATE shared_tasks
                    SET claimed_by = ?, claim_status = 'CLAIMED', updated_at = ?
                    WHERE id = ? AND (claim_status != 'CLAIMED' OR claim_status IS NULL)
                    "#
                )
                .bind(&payload.agent_id)
                .bind(Utc::now().to_rfc3339())
                .bind(&payload.task_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if result.rows_affected() > 0 {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    return Ok(true);
                }

                tx.rollback().await.unwrap_or_default();
                Ok(false)
            }
        }
    }
}
