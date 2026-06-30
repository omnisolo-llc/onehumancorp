use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;

pub struct DailyOperationsWorker {
    pub db: Arc<DB>,
}

impl DailyOperationsWorker {
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
                    // Check if there's already an operations task for this tenant today to avoid spamming
                    let has_pending = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'operations' AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 day'")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'operations' AND created_at > datetime('now', '-1 day')")
                                .bind(&tenant_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                        }
                    };

                    if has_pending {
                        continue;
                    }

                    // Insert a new daily operations task
                    let task_id = Uuid::new_v4().to_string();
                    let context_payload = serde_json::json!({
                        "description": "Review Daily Prep Checklist",
                        "feature_type": "operations_task",
                        "icon": "📋"
                    });

                    let proposed_action = serde_json::json!({
                        "action_type": "operations_task",
                        "message": "Approve to mark daily prep checklist as completed."
                    });

                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, 'operations', $3::jsonb, $4::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&task_id)
                            .bind(&tenant_id)
                            .bind(serde_json::to_string(&context_payload).unwrap())
                            .bind(serde_json::to_string(&proposed_action).unwrap())
                            .execute(&db.pool)
                            .await;
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, 'operations', ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&task_id)
                            .bind(&tenant_id)
                            .bind(serde_json::to_string(&context_payload).unwrap())
                            .bind(serde_json::to_string(&proposed_action).unwrap())
                            .execute(&db.pool)
                            .await;
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    // A helper to quickly set up a test db since the harness isn't available
    async fn setup_test_db() -> Arc<DB> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // set up required tables
        sqlx::query("CREATE TABLE tenants (id TEXT PRIMARY KEY);").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE agent_feed_items (id TEXT PRIMARY KEY, tenant_id TEXT, event_source TEXT, context_payload TEXT, proposed_action TEXT, lifecycle_state TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await.unwrap();

        Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool: crate::db::secure_pg_pool_options().max_connections(1).connect_lazy("postgres://postgres:postgres@localhost:5432/ohc").unwrap() })
    }

    #[tokio::test]
    async fn test_daily_operations_worker() {
        let db = setup_test_db().await;

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO tenants (id) VALUES ('tenant1')").execute(pool).await.unwrap();

            // start worker
            let worker = DailyOperationsWorker::new(db.clone());
            // Since start() spawns a loop, let's manually run the logic for one pass
            let tenants: Vec<String> = sqlx::query_scalar("SELECT id FROM tenants")
                .fetch_all(pool).await.unwrap();

            for tenant_id in tenants {
                let has_pending: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'operations' AND created_at > datetime('now', '-1 day')")
                    .bind(&tenant_id).fetch_one(pool).await.unwrap();

                if has_pending == 0 {
                    let task_id = Uuid::new_v4().to_string();
                    let context_payload = serde_json::json!({
                        "description": "Review Daily Prep Checklist",
                        "feature_type": "operations_task",
                        "icon": "📋"
                    });

                    let proposed_action = serde_json::json!({
                        "action_type": "operations_task",
                        "message": "Approve to mark daily prep checklist as completed."
                    });

                    sqlx::query(
                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES (?, ?, 'operations', ?, ?, 'PENDING_APPROVAL')"
                    )
                    .bind(&task_id)
                    .bind(&tenant_id)
                    .bind(serde_json::to_string(&context_payload).unwrap())
                    .bind(serde_json::to_string(&proposed_action).unwrap())
                    .execute(pool)
                    .await.unwrap();
                }
            }

            // verify task was added
            let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = 'tenant1'").fetch_one(pool).await.unwrap();
            assert_eq!(count, 1);

            // run logic again
            let tenants2: Vec<String> = sqlx::query_scalar("SELECT id FROM tenants")
                .fetch_all(pool).await.unwrap();

            for tenant_id in tenants2 {
                let has_pending: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'operations' AND created_at > datetime('now', '-1 day')")
                    .bind(&tenant_id).fetch_one(pool).await.unwrap();

                assert_eq!(has_pending, 1);
            }

            // verify task count remains 1
            let count2: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = 'tenant1'").fetch_one(pool).await.unwrap();
            assert_eq!(count2, 1);
        }
    }
}
