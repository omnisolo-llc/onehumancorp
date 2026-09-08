use crate::db::DB;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub struct ProactiveOperationsWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl ProactiveOperationsWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60), // Check every minute in dev
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;

                // 1. Get all active tenants
                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => sqlx::query_scalar("SELECT id FROM tenants")
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default(),
                    crate::db::DbStore::Sqlite(_) => sqlx::query_scalar("SELECT id FROM tenants")
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default(),
                };

                for tenant_id in tenants {
                    // Check if there's already a pending proactive operations task for this tenant today
                    let has_pending = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'operations' AND context_payload->>'feature_type' = 'proactive_ops' AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 day'")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'operations' AND json_extract(context_payload, '$.feature_type') = 'proactive_ops' AND created_at > datetime('now', '-1 day')")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                        }
                    };

                    if has_pending {
                        continue;
                    }

                    // Generate the 3 specific CUJ tasks
                    let tasks = vec![
                        (
                            "Review Daily Prep Checklist",
                            "Review Checklist",
                            "mark_complete",
                        ),
                        (
                            "Follow up on delayed supplier delivery from yesterday",
                            "Assign to Staff",
                            "assign_to_staff",
                        ),
                        (
                            "Staffing alert: Only 1 person scheduled for closing shift.",
                            "Draft Schedule Request",
                            "draft_schedule_request",
                        ),
                    ];

                    for (description, action_msg, action_type) in tasks {
                        let task_id = Uuid::new_v4().to_string();
                        let context_payload = json!({
                            "description": description,
                            "feature_type": "proactive_ops"
                        });
                        let proposed_action = json!({
                            "message": action_msg,
                            "action_type": action_type,
                            "feature_type": "proactive_ops"
                        });

                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query(
                                    "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES ($1, $2, $3, $4, $5, $6)"
                                )
                                .bind(&task_id)
                                .bind(&tenant_id)
                                .bind("operations")
                                .bind(context_payload)
                                .bind(proposed_action)
                                .bind("PENDING_APPROVAL")
                                .execute(&db.pool)
                                .await;
                            }
                            crate::db::DbStore::Sqlite(_) => {
                                let _ = sqlx::query(
                                    "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES (?, ?, ?, ?, ?, ?)"
                                )
                                .bind(&task_id)
                                .bind(&tenant_id)
                                .bind("operations")
                                .bind(context_payload)
                                .bind(proposed_action)
                                .bind("PENDING_APPROVAL")
                                .execute(&db.pool)
                                .await;
                            }
                        }
                    }
                }
            }
        });
    }
}
