use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct OperationsWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl OperationsWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                loop {
                    match Self::poll(&db).await {
                        Ok(true) => continue, // keep polling until queue is empty
                        Ok(false) => break,
                        Err(e) => {
                            tracing::error!("OperationsWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let task = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE department_tasks
                    SET status = 'IN_PROGRESS', locked_until = $1, updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM department_tasks
                        WHERE status = 'PENDING' AND department = 'operations' AND event_type = 'OrderReceived'
                        AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    )
                    RETURNING id, tenant_id, payload
                    "#
                )
                .bind(Utc::now() + chrono::Duration::minutes(5))
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id"), r.get::<serde_json::Value, _>("payload")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload FROM department_tasks
                    WHERE status = 'PENDING' AND department = 'operations' AND event_type = 'OrderReceived'
                    AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                    sqlx::query(
                        "UPDATE department_tasks SET status = 'IN_PROGRESS', locked_until = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                    )
                    .bind((Utc::now() + chrono::Duration::minutes(5)).to_rfc3339())
                    .bind(&id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        let processed = task.is_some();
        if let Some((id, tenant_id, payload)) = task {
            // Emit OrderProcessed event for Customer Success
            let new_task_id = Uuid::new_v4().to_string();
            let new_payload = json!({
                "original_order": payload,
                "processed_at": Utc::now().to_rfc3339()
            });

            match &db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        r#"
                        INSERT INTO department_tasks (id, tenant_id, department, event_type, payload)
                        VALUES ($1, $2, 'customer_success', 'OrderProcessed', $3)
                        "#
                    )
                    .bind(&new_task_id)
                    .bind(&tenant_id)
                    .bind(&new_payload)
                    .execute(&db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&id)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query(
                        r#"
                        INSERT INTO department_tasks (id, tenant_id, department, event_type, payload)
                        VALUES (?, ?, 'customer_success', 'OrderProcessed', ?)
                        "#
                    )
                    .bind(&new_task_id)
                    .bind(&tenant_id)
                    .bind(new_payload.to_string())
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(processed)
    }
}

pub struct CustomerSuccessWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl CustomerSuccessWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                loop {
                    match Self::poll(&db).await {
                        Ok(true) => continue, // keep polling until queue is empty
                        Ok(false) => break,
                        Err(e) => {
                            tracing::error!("CustomerSuccessWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let task = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE department_tasks
                    SET status = 'IN_PROGRESS', locked_until = $1, updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM department_tasks
                        WHERE status = 'PENDING' AND department = 'customer_success' AND event_type = 'OrderProcessed'
                        AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    )
                    RETURNING id, tenant_id, payload
                    "#
                )
                .bind(Utc::now() + chrono::Duration::minutes(5))
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id"), r.get::<serde_json::Value, _>("payload")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload FROM department_tasks
                    WHERE status = 'PENDING' AND department = 'customer_success' AND event_type = 'OrderProcessed'
                    AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                    sqlx::query(
                        "UPDATE department_tasks SET status = 'IN_PROGRESS', locked_until = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                    )
                    .bind((Utc::now() + chrono::Duration::minutes(5)).to_rfc3339())
                    .bind(&id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        let processed = task.is_some();
        if let Some((id, _tenant_id, payload)) = task {
            // Log drafted confirmation message to the database directly
            let drafted_msg = format!("Drafted confirmation for order: {:?}", payload);

            match &db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', payload = jsonb_set(payload, '{drafted_message}', $1), updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                        .bind(&drafted_msg)
                        .bind(&id)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', payload = json_patch(payload, ?), updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(json!({"drafted_message": drafted_msg}).to_string())
                        .bind(&id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(processed)
    }
}
