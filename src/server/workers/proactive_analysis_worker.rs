use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl ProactiveAnalysisWorker {
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
            let pool = db.pool.clone();
            loop {
                tokio::time::sleep(interval_duration).await;

                let _ = async {
                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let mut tx = match pool.begin().await {
                                Ok(tx) => tx,
                                Err(e) => {
                                    tracing::error!("proactive_analysis_worker: failed to begin tx: {}", e);
                                    return;
                                }
                            };

                            let row = match sqlx::query(
                                r#"
                                SELECT id, tenant_id FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'proactive_analysis_check'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
                                LIMIT 1 FOR UPDATE SKIP LOCKED
                                "#
                            )
                            .fetch_optional(&mut *tx)
                            .await {
                                Ok(r) => r,
                                Err(e) => {
                                    tracing::error!("proactive_analysis_worker: failed to fetch job: {}", e);
                                    let _ = tx.rollback().await;
                                    return;
                                }
                            };

                            if let Some(r) = row {
                                let job_id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");

                                let context_payload = json!({
                                    "title": "Needs Attention Today",
                                    "message": "You have 2 estimates pending from yesterday. Tap to review drafted follow-up messages.",
                                });
                                let proposed_action = json!({
                                    "action_type": "review_drafts",
                                    "feature_type": "proactive_follow_up"
                                });

                                let feed_item_id = Uuid::new_v4().to_string();

                                let insert_res = sqlx::query(
                                    r#"
                                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
                                    VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL')
                                    "#
                                )
                                .bind(&feed_item_id)
                                .bind(&tenant_id)
                                .bind("proactive_context_agent")
                                .bind(context_payload)
                                .bind(proposed_action)
                                .execute(&mut *tx)
                                .await;

                                if let Err(e) = insert_res {
                                    tracing::error!("proactive_analysis_worker: failed to create feed item: {}", e);
                                    let _ = tx.rollback().await;
                                    return;
                                }

                                let update_job = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                                    .bind(&job_id)
                                    .execute(&mut *tx)
                                    .await;

                                if let Err(e) = update_job {
                                    tracing::error!("proactive_analysis_worker: failed to update job status: {}", e);
                                    let _ = tx.rollback().await;
                                    return;
                                }

                                let _ = tx.commit().await;
                            } else {
                                let _ = tx.rollback().await;
                            }
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            // no-op for now, sqlite fallback for testing
                        }
                    }
                }.await;
            }
        });
    }
}
