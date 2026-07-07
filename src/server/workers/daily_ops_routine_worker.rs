use std::sync::Arc;
use crate::db::DB;

use uuid::Uuid;
use serde_json::json;

pub struct DailyOpsRoutineWorker {
    pub db: Arc<DB>,
}

impl DailyOpsRoutineWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // Check every minute in dev (or configurable)
            loop {
                interval.tick().await;

                // 1. Get all active tenants
                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                    }
                };

                for tenant_id in tenants {
                    // Check if there's already a pending daily prep task for this tenant today to avoid spamming
                    let has_pending = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'Operations Agent' AND lifecycle_state = 'PENDING_APPROVAL' AND proposed_action->>'action_type' = 'Daily Prep Checklist' AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 day'")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool).await.unwrap_or(0) > 0
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'Operations Agent' AND lifecycle_state = 'PENDING_APPROVAL' AND json_extract(proposed_action, '$.action_type') = 'Daily Prep Checklist' AND created_at > datetime('now', '-1 day')")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool).await.unwrap_or(0) > 0
                        }
                    };

                    // Find dormant customers and insert into ohc_job_queue
                    let dormant_customers: Vec<String> = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar(
                                r#"
                                WITH customer_stats AS (
                                    SELECT customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                    FROM bookings
                                    WHERE tenant_id = $1
                                    GROUP BY customer_id
                                )
                                SELECT customer_id
                                FROM customer_stats
                                WHERE total_bookings > 1 AND last_booking < CURRENT_TIMESTAMP - INTERVAL '14 days'
                                AND customer_id NOT IN (
                                    SELECT payload->>'customer_id'
                                    FROM ohc_job_queue
                                    WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND status IN ('PENDING', 'PROCESSING', 'COMPLETED') AND created_at > CURRENT_TIMESTAMP - INTERVAL '14 days'
                                )
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool).await.unwrap_or_default()
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar(
                                r#"
                                WITH customer_stats AS (
                                    SELECT customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                    FROM bookings
                                    WHERE tenant_id = $1
                                    GROUP BY customer_id
                                )
                                SELECT customer_id
                                FROM customer_stats
                                WHERE total_bookings > 1 AND last_booking < datetime('now', '-14 days')
                                AND customer_id NOT IN (
                                    SELECT json_extract(payload, '$.customer_id')
                                    FROM ohc_job_queue
                                    WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND status IN ('PENDING', 'PROCESSING', 'COMPLETED') AND created_at > datetime('now', '-14 days')
                                )
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool).await.unwrap_or_default()
                        }
                    };

                    for dormant_customer in dormant_customers {
                        let payload = json!({"customer_id": dormant_customer});
                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING', CURRENT_TIMESTAMP)"
                                )
                                .bind(Uuid::new_v4().to_string())
                                .bind(&tenant_id)
                                .bind(sqlx::types::Json(payload))
                                .execute(&db.pool).await;
                            },
                            crate::db::DbStore::Sqlite(_) => {
                                let payload_str = payload.to_string();
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'booking_reengagement_check', ?, 'PENDING', CURRENT_TIMESTAMP)"
                                )
                                .bind(Uuid::new_v4().to_string())
                                .bind(&tenant_id)
                                .bind(payload_str)
                                .execute(&db.pool).await;
                            }
                        }
                    }

                    if !has_pending {
                        let context_payload = json!({
                            "feature_type": "daily_prep_checklist",
                            "description": "Daily Prep Checklist",
                        });

                        let proposed_action = json!({
                            "action_type": "Daily Prep Checklist",
                            "message": "Review Daily Prep Checklist"
                        });

                        let item_id = Uuid::new_v4().to_string();
                        let event_source = "Operations Agent".to_string();
                        let lifecycle_state = "PENDING_APPROVAL".to_string();
                        let created_at = chrono::Utc::now();
                        let updated_at = chrono::Utc::now();

                        let conn_result = db.pool.acquire().await;
                        if let Ok(mut conn) = conn_result {
                             let _ = ::server_common::auth_utils::set_org_context(&mut *conn, &tenant_id).await;
                             let context_payload_json = sqlx::types::Json(context_payload);
                             let proposed_action_json = sqlx::types::Json(proposed_action);
                             let _ = sqlx::query(
                                 r#"
                                 INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                                 "#
                             )
                             .bind(&item_id)
                             .bind(&tenant_id)
                             .bind(&event_source)
                             .bind(&context_payload_json)
                             .bind(&proposed_action_json)
                             .bind(&lifecycle_state)
                             .bind(&created_at)
                             .bind(&updated_at)
                             .execute(&mut *conn)
                             .await;

                             let task_id = Uuid::new_v4().to_string();
                             let _ = sqlx::query(
                                 r#"
                                 INSERT INTO staff_tasks (id, tenant_id, description, priority, status)
                                 VALUES ($1, $2, $3, $4, $5)
                                 "#
                             )
                             .bind(&task_id)
                             .bind(&tenant_id)
                             .bind("Review Daily Prep Checklist")
                             .bind(1)
                             .bind("pending")
                             .execute(&mut *conn)
                             .await;

                             let summary_id = Uuid::new_v4().to_string();
                             let metrics_json = sqlx::types::Json(json!({"completed_tasks": 0, "prep_status": "started"}));
                             let _ = sqlx::query(
                                 r#"
                                 INSERT INTO shift_summaries (id, tenant_id, shift_id, staff_id, summary_text, metrics)
                                 VALUES ($1, $2, $3, $4, $5, $6)
                                 "#
                             )
                             .bind(&summary_id)
                             .bind(&tenant_id)
                             .bind("shift-initial")
                             .bind("system")
                             .bind("Daily prep shift started.")
                             .bind(&metrics_json)
                             .execute(&mut *conn)
                             .await;
                        }
                    }
                }
            }
        });
    }
}
