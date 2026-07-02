use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use uuid::Uuid;
use sqlx::Row;
use serde_json::json;
use tokio::time::timeout;

const DB_OP_TIMEOUT: Duration = Duration::from_secs(2);

pub struct BookingReengagementWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl BookingReengagementWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(10), // Run frequently, jobs are scheduled for 14 days out
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let pool = db.pool.clone();
            loop {
                tokio::time::sleep(interval_duration).await;

                let poll_op = async {
                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                            let row = sqlx::query(
                                r#"
                                SELECT id, tenant_id, payload FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'booking_reengagement_check'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
                                LIMIT 1 FOR UPDATE SKIP LOCKED
                                "#
                            )
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                            if let Some(r) = row {
                                let id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload: serde_json::Value = r.get("payload");

                                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                                    .bind(&id)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                                tx.commit().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(Some((id, tenant_id, payload)))
                            } else {
                                tx.rollback().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(None)
                            }
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                            let row = sqlx::query(
                                r#"
                                SELECT id, tenant_id, payload FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'booking_reengagement_check'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
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
                                let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                                    .bind(&id)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                                tx.commit().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(Some((id, tenant_id, payload)))
                            } else {
                                tx.rollback().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(None)
                            }
                        }
                    }
                };

                let task = match timeout(DB_OP_TIMEOUT, poll_op).await {
                    Ok(Ok(Some(res))) => res,
                    Ok(Ok(None)) => continue,
                    _ => continue,
                };

                let (job_id, tenant_id, payload) = task;
                let customer_id = payload.get("customer_id").and_then(|c| c.as_str()).unwrap_or("");
                let _product_id = payload.get("product_id").and_then(|p| p.as_str()).unwrap_or("");

                // 1. Check if customer is dormant: has historically booked more than once, but not in the last 14 days.
                let is_dormant = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, Option<bool>>(
                            r#"
                            WITH customer_stats AS (
                                SELECT COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                FROM bookings
                                WHERE tenant_id = $1 AND customer_id = $2
                            )
                            SELECT (total_bookings > 1 AND last_booking < CURRENT_TIMESTAMP - INTERVAL '14 days')
                            FROM customer_stats;
                            "#
                        )
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .fetch_one(&pool)
                        .await
                        .unwrap_or(Some(false))
                        .unwrap_or(false)
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         sqlx::query_scalar::<_, Option<bool>>(
                            r#"
                            WITH customer_stats AS (
                                SELECT COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                FROM bookings
                                WHERE tenant_id = ? AND customer_id = ?
                            )
                            SELECT (total_bookings > 1 AND last_booking < datetime('now', '-14 days'))
                            FROM customer_stats;
                            "#
                        )
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .fetch_one(sqlite_pool)
                        .await
                        .unwrap_or(Some(false))
                        .unwrap_or(false)
                    }
                };

                let has_recent_booking = !is_dormant; // Map logic for the rest of the worker

                // 2. If no new booking exists, draft a re-engagement message and push to Agent Feed (shared_tasks).
                if !has_recent_booking {
                     // Get Customer Name (simplified for worker context)
                    let customer_name = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = $1 AND tenant_id = $2")
                            .bind(&customer_id).bind(&tenant_id)
                            .fetch_optional(&pool).await.unwrap_or(None).unwrap_or("Valued Customer".to_string())
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(&customer_id).bind(&tenant_id)
                            .fetch_optional(sqlite_pool).await.unwrap_or(None).unwrap_or("Valued Customer".to_string())
                        }
                    };

                    let drafted_message = format!("Hi {}, I noticed we haven't had a session in a while! Hope everything is going great with your progress. Would you like to jump back in this week? I have some slots available. Here is a quick booking link: [Link]", customer_name);

                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let _ = sqlx::query(
                                r"
                                INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                                VALUES ($1, $2, 'Approve Re-engagement for ' || $3, 'AI detected that ' || $3 || ' is a returning customer who hasn''t booked in 14 days. This follow-up helps maintain momentum.', 'PENDING', 'P1', 'LOW', 'PENDING', $4)
                                "
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(&tenant_id)
                            .bind(&customer_name)
                            .bind(&drafted_message)
                            .execute(&pool)
                            .await;
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             let _ = sqlx::query(
                                r#"
                                INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                                VALUES (?, ?, 'Approve Re-engagement for ' || ?, 'AI detected that ' || ? || ' is a returning customer who hasn''t booked in 14 days. This follow-up helps maintain momentum.', 'PENDING', 'P1', 'LOW', 'PENDING', ?)
                                "#
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(&tenant_id)
                            .bind(&customer_name)
                            .bind(&customer_name)
                            .bind(&drafted_message)
                            .execute(sqlite_pool)
                            .await;
                        }
                    }
                }

                // 3. Mark Job as Completed
                 match &db.store {
                     crate::db::DbStore::Postgres => {
                          let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                          .bind(&job_id).execute(&pool).await;
                     },
                     crate::db::DbStore::Sqlite(sqlite_pool) => {
                           let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                          .bind(&job_id).execute(sqlite_pool).await;
                     }
                 }
            }
        });
    }
}
