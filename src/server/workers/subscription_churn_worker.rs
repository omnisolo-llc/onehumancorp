use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct SubscriptionChurnWorker {
    db: Arc<DB>,
}

impl SubscriptionChurnWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue immediately
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                    Err(e) => {
                        tracing::error!("SubscriptionChurnWorker error: {}", e);
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
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'subscription_churn_check'
                    ORDER BY next_retry_at ASC, created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload: serde_json::Value = serde_json::from_str(r.get("payload")).unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                        .bind(&id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Some((id, tenant_id, payload))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    None
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'subscription_churn_check'
                    ORDER BY next_retry_at ASC, created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Some((id, tenant_id, payload))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    None
                }
            }
        };

        if let Some((job_id, tenant_id, payload)) = job {
            if let (Some(subscriber_id), Some(customer_id)) = (payload.get("subscriber_id").and_then(|v| v.as_str()), payload.get("customer_id").and_then(|v| v.as_str())) {

                let mut health_score: i32 = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar("SELECT health_score FROM subscribers WHERE id = $1")
                            .bind(subscriber_id)
                            .fetch_one(&self.db.pool)
                            .await
                            .unwrap_or(100)
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_scalar("SELECT health_score FROM subscribers WHERE id = ?")
                            .bind(subscriber_id)
                            .fetch_one(sqlite_pool)
                            .await
                            .unwrap_or(100)
                    }
                };

                // Assess recent activity: bookings
                let bookings_count: i64 = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, i64>(
                            "SELECT COUNT(*) FROM bookings WHERE tenant_id = $1 AND customer_id = $2 AND created_at >= NOW() - INTERVAL '30 days'"
                        )
                        .bind(&tenant_id)
                        .bind(uuid::Uuid::parse_str(customer_id).unwrap_or_default())
                        .fetch_one(&self.db.pool)
                        .await
                        .unwrap_or(0)
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_scalar::<_, i64>(
                            "SELECT COUNT(*) FROM bookings WHERE tenant_id = ? AND customer_id = ? AND created_at >= datetime('now', '-30 days')"
                        )
                        .bind(&tenant_id)
                        .bind(uuid::Uuid::parse_str(customer_id).unwrap_or_default())
                        .fetch_one(sqlite_pool)
                        .await
                        .unwrap_or(0)
                    }
                };

                if bookings_count == 0 {
                    health_score -= 30; // Deduct if no recent bookings
                }

                let (customer_name, _email) = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_as::<_, (String, Option<String>)>("SELECT name, email FROM customers WHERE id = $1 AND tenant_id = $2")
                            .bind(uuid::Uuid::parse_str(customer_id).unwrap_or_default())
                            .bind(&tenant_id)
                            .fetch_optional(&self.db.pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| ("Customer".to_string(), None))
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_as::<_, (String, Option<String>)>("SELECT name, email FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(uuid::Uuid::parse_str(customer_id).unwrap_or_default())
                            .bind(&tenant_id)
                            .fetch_optional(sqlite_pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| ("Customer".to_string(), None))
                    }
                };

                // Update health score in DB
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE subscribers SET health_score = $1, last_health_check_at = NOW() WHERE id = $2")
                            .bind(health_score)
                            .bind(subscriber_id)
                            .execute(&self.db.pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query("UPDATE subscribers SET health_score = ?, last_health_check_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(health_score)
                            .bind(subscriber_id)
                            .execute(sqlite_pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                }

                if health_score < 50 {
                    // LLM drafts a win-back message
                    let prompt = format!(
                        "You are the Customer Success Ambassador. \
                        Draft a friendly, personalized win-back message for {name} who is an at-risk subscriber (has not booked anything in 30 days). \
                        Offer them a free 15-minute consultation to get back on track or a 10% discount on their next package to keep the momentum going.",
                        name = customer_name
                    );

                    let draft = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                        Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| format!("Hi {}, we noticed you haven't booked a lesson in a few weeks. Is everything okay? We'd love to offer you 10% off your next package to keep the momentum going!", customer_name)),
                        Ok("minimax") => {
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await.unwrap_or_else(|_| format!("Hi {}, we noticed you haven't booked a lesson in a few weeks. Is everything okay? We'd love to offer you 10% off your next package to keep the momentum going!", customer_name))
                        }
                        _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| format!("Hi {}, we noticed you haven't booked a lesson in a few weeks. Is everything okay? We'd love to offer you 10% off your next package to keep the momentum going!", customer_name)),
                    };

                    let proposed_action = serde_json::json!({
                        "action_type": "send_message",
                        "draft_action": draft,
                        "customer_id": customer_id
                    });

                    let context_payload = serde_json::json!({
                        "description": format!("The Ambassador identified {} as an at-risk subscriber (Health Score: {}).", customer_name, health_score),
                        "feature_type": "subscription_churn_winback",
                        "health_score": health_score
                    });

                    let agent_feed_item_id = Uuid::new_v4().to_string();

                    match &self.db.store {
                        crate::db::DbStore::Postgres => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
                            )
                            .bind(&agent_feed_item_id)
                            .bind(&tenant_id)
                            .bind("Customer Success")
                            .bind(context_payload.to_string())
                            .bind(proposed_action.to_string())
                            .execute(&self.db.pool)
                            .await;
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
                            )
                            .bind(&agent_feed_item_id)
                            .bind(&tenant_id)
                            .bind("Customer Success")
                            .bind(context_payload.to_string())
                            .bind(proposed_action.to_string())
                            .execute(sqlite_pool)
                            .await;
                        }
                    }
                }
            }

            // Mark job as completed
            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                        .bind(&job_id)
                        .execute(&self.db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let sqlite_pool = crate::db::create_sqlite_pool_for_test().await;
        let pool = crate::db::create_dummy_pg_pool().await;
        let db = DB {
            pool,
            store: DbStore::Sqlite(sqlite_pool.clone()),
        };

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS subscribers (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, health_score INTEGER, last_health_check_at TEXT, status TEXT);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS bookings (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, created_at TEXT);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS customers (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT, email TEXT);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, next_retry_at TEXT, created_at TEXT, updated_at TEXT);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_feed_items (id TEXT PRIMARY KEY, tenant_id TEXT, event_source TEXT, context_payload TEXT, proposed_action TEXT, lifecycle_state TEXT, created_at TEXT, updated_at TEXT);").execute(&sqlite_pool).await;

        Some(Arc::new(db))
    }

    #[tokio::test]
    async fn test_subscription_churn_worker() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };

        let pool = match &db.store {
            DbStore::Sqlite(p) => p.clone(),
            _ => panic!("Expected Sqlite store"),
        };

        let tenant_id = "tenant-test";
        let sub_id = "sub-test";
        let cus_id = Uuid::new_v4().to_string();

        let _ = sqlx::query("INSERT INTO subscribers (id, tenant_id, customer_id, health_score, status) VALUES (?, ?, ?, 100, 'ACTIVE')")
            .bind(sub_id).bind(tenant_id).bind(&cus_id).execute(&pool).await.unwrap();

        let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, 'Alex')")
            .bind(&cus_id).bind(tenant_id).execute(&pool).await.unwrap();

        let payload = serde_json::json!({
            "subscriber_id": sub_id,
            "customer_id": cus_id
        });

        let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ('job-1', ?, 'subscription_churn_check', ?, 'PENDING', CURRENT_TIMESTAMP)")
            .bind(tenant_id).bind(payload.to_string()).execute(&pool).await.unwrap();

        let worker = SubscriptionChurnWorker::new(db.clone());
        let res = worker.poll().await.unwrap();
        assert!(res); // job processed

        let health: i32 = sqlx::query_scalar("SELECT health_score FROM subscribers WHERE id = ?")
            .bind(sub_id).fetch_one(&pool).await.unwrap();
        assert_eq!(health, 70); // 100 - 30 because no bookings

        // Wait for it to fall below 50, let's just insert one with initial health 60
        let sub_id2 = "sub-test2";
        let cus_id2 = Uuid::new_v4().to_string();

        let _ = sqlx::query("INSERT INTO subscribers (id, tenant_id, customer_id, health_score, status) VALUES (?, ?, ?, 60, 'ACTIVE')")
            .bind(sub_id2).bind(tenant_id).bind(&cus_id2).execute(&pool).await.unwrap();

        let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, 'Alex2')")
            .bind(&cus_id2).bind(tenant_id).execute(&pool).await.unwrap();

        let payload2 = serde_json::json!({
            "subscriber_id": sub_id2,
            "customer_id": cus_id2
        });

        let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ('job-2', ?, 'subscription_churn_check', ?, 'PENDING', CURRENT_TIMESTAMP)")
            .bind(tenant_id).bind(payload2.to_string()).execute(&pool).await.unwrap();

        let worker2 = SubscriptionChurnWorker::new(db.clone());
        worker2.poll().await.unwrap();

        let health2: i32 = sqlx::query_scalar("SELECT health_score FROM subscribers WHERE id = ?")
            .bind(sub_id2).fetch_one(&pool).await.unwrap();
        assert_eq!(health2, 30); // 60 - 30 = 30 < 50

        // Agent feed item should be created
        let agent_feed_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = ? AND event_source = 'Customer Success'")
            .bind(tenant_id).fetch_one(&pool).await.unwrap();
        assert_eq!(agent_feed_count, 1);
    }
}
