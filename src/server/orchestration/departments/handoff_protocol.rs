use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::departments::types::TaskEnvelope;
use chrono::Utc;
use serde_json::json;

pub struct DepartmentHandoffManager {
    db: Arc<DB>,
}

impl DepartmentHandoffManager {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_envelope(
        &self,
        tenant_id: &str,
        initial_department: &str,
        payload: serde_json::Value,
    ) -> Result<TaskEnvelope, String> {
        let envelope_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let routing_history = json!([
            {
                "department": initial_department,
                "timestamp": now.to_rfc3339()
            }
        ]);

        let envelope = TaskEnvelope {
            id: envelope_id.clone(),
            tenant_id: tenant_id.to_string(),
            current_department: initial_department.to_string(),
            status: "PENDING".to_string(),
            payload: payload.clone(),
            routing_history: routing_history.clone(),
            created_at: Some(now),
            updated_at: Some(now),
        };

        match &self.db.store {
            DbStore::Postgres => {
                let res = sqlx::query(
                    "INSERT INTO task_envelopes (id, tenant_id, current_department, status, payload, routing_history, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
                )
                .bind(&envelope.id)
                .bind(&envelope.tenant_id)
                .bind(&envelope.current_department)
                .bind(&envelope.status)
                .bind(&envelope.payload)
                .bind(&envelope.routing_history)
                .bind(now)
                .bind(now)
                .execute(&self.db.pool).await;

                if let Err(e) = res {
                    return Err(format!("Failed to create task envelope: {}", e));
                }
            }
            DbStore::Sqlite(pool) => {
                let payload_str = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());
                let routing_str = serde_json::to_string(&routing_history).unwrap_or_else(|_| "[]".to_string());
                let res = sqlx::query(
                    "INSERT INTO task_envelopes (id, tenant_id, current_department, status, payload, routing_history, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&envelope.id)
                .bind(&envelope.tenant_id)
                .bind(&envelope.current_department)
                .bind(&envelope.status)
                .bind(&payload_str)
                .bind(&routing_str)
                .bind(now.to_rfc3339())
                .bind(now.to_rfc3339())
                .execute(pool).await;

                if let Err(e) = res {
                    return Err(format!("Failed to create task envelope (SQLite): {}", e));
                }
            }
        }

        Ok(envelope)
    }

    pub async fn route_envelope(
        &self,
        envelope_id: &str,
        tenant_id: &str,
        next_department: &str,
    ) -> Result<(), String> {
        let now = Utc::now();
        let new_history_entry = json!({
            "department": next_department,
            "timestamp": now.to_rfc3339()
        });

        match &self.db.store {
            DbStore::Postgres => {
                let res = sqlx::query(
                    "UPDATE task_envelopes SET current_department = $1, routing_history = routing_history || $2::jsonb, updated_at = $3 WHERE id = $4 AND tenant_id = $5"
                )
                .bind(next_department)
                .bind(&new_history_entry)
                .bind(now)
                .bind(envelope_id)
                .bind(tenant_id)
                .execute(&self.db.pool).await;

                if let Err(e) = res {
                    return Err(format!("Failed to route envelope: {}", e));
                }
            }
            DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT routing_history FROM task_envelopes WHERE id = ? AND tenant_id = ?")
                    .bind(envelope_id)
                    .bind(tenant_id)
                    .fetch_one(pool).await;

                if let Ok(r) = row {
                    use sqlx::Row;
                    let mut history_arr: Vec<serde_json::Value> = vec![];
                    if let Ok(history_str) = r.try_get::<String, _>("routing_history") {
                        if let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(&history_str) {
                            history_arr = parsed;
                        }
                    }
                    history_arr.push(new_history_entry);
                    let new_history_str = serde_json::to_string(&history_arr).unwrap_or_default();

                    let update_res = sqlx::query(
                        "UPDATE task_envelopes SET current_department = ?, routing_history = ?, updated_at = ? WHERE id = ? AND tenant_id = ?"
                    )
                    .bind(next_department)
                    .bind(new_history_str)
                    .bind(now.to_rfc3339())
                    .bind(envelope_id)
                    .bind(tenant_id)
                    .execute(pool).await;

                    if let Err(e) = update_res {
                        return Err(format!("Failed to route envelope (SQLite): {}", e));
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn approve_envelope(&self, envelope_id: &str, tenant_id: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let res = sqlx::query(
                    "UPDATE task_envelopes SET status = 'COMPLETED', updated_at = $1 WHERE id = $2 AND tenant_id = $3"
                )
                .bind(now)
                .bind(envelope_id)
                .bind(tenant_id)
                .execute(&self.db.pool).await;

                if let Err(e) = res {
                    return Err(format!("Failed to approve envelope: {}", e));
                }
            }
            DbStore::Sqlite(pool) => {
                let res = sqlx::query(
                    "UPDATE task_envelopes SET status = 'COMPLETED', updated_at = ? WHERE id = ? AND tenant_id = ?"
                )
                .bind(now.to_rfc3339())
                .bind(envelope_id)
                .bind(tenant_id)
                .execute(pool).await;

                if let Err(e) = res {
                    return Err(format!("Failed to approve envelope (SQLite): {}", e));
                }
            }
        }
        Ok(())
    }
}
