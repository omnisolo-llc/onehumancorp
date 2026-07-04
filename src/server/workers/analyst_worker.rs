use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct AnalystWorker {
    db: Arc<DB>,
}

impl AnalystWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            tracing::info!("Starting AnalystWorker for proactive insights feed...");
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue immediately
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(5000)).await; // Poll every 5s
                    }
                    Err(e) => {
                        tracing::error!("AnalystWorker error: {}", e);
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<bool, String> {
        let job = match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'analyst_weekly_summary'
                    ORDER BY next_retry_at ASC, created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let job_id: String = r.get("id");
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                        .bind(&job_id)
                        .execute(&mut *tx).await;
                    tx.commit().await.map_err(|e| e.to_string())?;
                    let t_id: String = r.get("tenant_id");
                    Some((job_id, t_id))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    None
                }
            },
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'analyst_weekly_summary'
                    ORDER BY next_retry_at ASC, created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let job_id: String = r.get("id");
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(&mut *tx).await;
                    tx.commit().await.map_err(|e| e.to_string())?;
                    let t_id: String = r.get("tenant_id");
                    Some((job_id, t_id))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    None
                }
            }
        };

        if let Some((job_id, tenant_id)) = job {

            // Execute analytics query (mock LLM processing logic)
            // Typically we would aggregate `orders`, `order_items` here and call an LLM API
            // For now, simulate the Analyst capability and generate an insight record.

            let intent = "weekly_analytics".to_string();
            let summary_text = "Weekly Summary: Revenue up 10%. 'Summer Hat' is your top seller.";
            let action_type = "Draft Restock";
            let agent_feed_item_id = format!("afi-{}", Uuid::new_v4());

            let mut db_success = false;

            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    if let Err(e) = sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                    )
                    .bind::<&_>(&agent_feed_item_id)
                    .bind::<&_>(&tenant_id)
                    .bind::<&_>(&intent)
                    .bind(serde_json::json!({
                        "summary": summary_text,
                        "feature_type": "analyst_insight",
                    }))
                    .bind(serde_json::json!({
                        "action_type": action_type,
                        "feature_type": "analyst_insight"
                    }))
                    .execute(&self.db.pool).await {
                        tracing::error!("Failed to insert agent feed item: {}", e);
                    } else {
                        db_success = true;
                    }

                    if db_success {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                            .bind::<&_>(&job_id)
                            .execute(&self.db.pool).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1")
                            .bind::<&_>(&job_id)
                            .execute(&self.db.pool).await;
                    }
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    if let Err(e) = sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                    )
                    .bind::<&_>(&agent_feed_item_id)
                    .bind::<&_>(&tenant_id)
                    .bind::<&_>(&intent)
                    .bind::<&_>(&serde_json::json!({
                        "summary": summary_text,
                        "feature_type": "analyst_insight",
                    }).to_string())
                    .bind::<&_>(&serde_json::json!({
                        "action_type": action_type,
                        "feature_type": "analyst_insight"
                    }).to_string())
                    .execute(&*sqlite_pool).await {
                        tracing::error!("Failed to insert agent feed item (SQLite): {}", e);
                    } else {
                        db_success = true;
                    }

                    if db_success {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind::<&_>(&job_id)
                            .execute(&*sqlite_pool).await;
                    } else {
                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind::<&_>(&job_id)
                            .execute(&*sqlite_pool).await;
                    }
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
