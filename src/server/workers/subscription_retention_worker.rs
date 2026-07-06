use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct SubscriptionRetentionWorker {
    db: Arc<DB>,
}

impl SubscriptionRetentionWorker {
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
                        tracing::error!("SubscriptionRetentionWorker error: {}", e);
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
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'subscription_retention'
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
                    let payload: serde_json::Value = r.try_get::<sqlx::types::Json<serde_json::Value>, _>("payload")
                        .map(|j| j.0)
                        .unwrap_or_else(|_| serde_json::json!({}));

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
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'subscription_retention'
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
                    let payload: serde_json::Value = r.try_get::<sqlx::types::Json<serde_json::Value>, _>("payload")
                        .map(|j| j.0)
                        .unwrap_or_else(|_| serde_json::json!({}));

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

        if let Some((job_id, tenant_id_str, payload)) = job {
            if let Some(customer_id_str) = payload.get("customer_id").and_then(|v| v.as_str()) {
                let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|e| e.to_string())?;
                let customer_id = Uuid::parse_str(customer_id_str).map_err(|e| e.to_string())?;

                let customer_name = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = $1 AND tenant_id = $2")
                            .bind(&customer_id)
                            .bind(&tenant_id_str)
                            .fetch_optional(&self.db.pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string())
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_scalar::<_, String>("SELECT name FROM customers WHERE id = ? AND tenant_id = ?")
                            .bind(customer_id_str)
                            .bind(&tenant_id_str)
                            .fetch_optional(sqlite_pool)
                            .await
                            .unwrap_or_default()
                            .unwrap_or_else(|| "Customer".to_string())
                    }
                };

                // Fetch context for RAG
                let ledger_entries: Vec<(String, serde_json::Value)> = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_as(
                            "SELECT action_type, state_change FROM ohc_universal_ledger WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5"
                        )
                        .bind(&tenant_id_str)
                        .fetch_all(&self.db.pool)
                        .await
                        .unwrap_or_default()
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_as(
                            "SELECT action_type, json(state_change) FROM ohc_universal_ledger WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 5"
                        )
                        .bind(&tenant_id_str)
                        .fetch_all(sqlite_pool)
                        .await
                        .unwrap_or_default()
                    }
                };

                let context = serde_json::json!({
                    "customer_name": customer_name,
                    "recent_history": ledger_entries.iter().map(|(_, sc)| sc.clone()).collect::<Vec<_>>()
                });

                // LLM call to draft a personalized message with RAG context
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                let prompt = format!(
                    "You are the Customer Success Ambassador. Draft a short personalized win-back message for an at-risk subscriber named {}. They haven't booked in 21 days and their subscription renews soon. Use this context about their recent interactions: {}. Offer them a 10% discount or a free 15-minute consultation to get back on track.",
                    customer_name, context
                );

                let generated_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                    Ok("minimax") => {
                        crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await.unwrap_or_else(|_| format!("Hi {}, it's been a while! We'd love to offer you a free 15-minute consultation to get back on track.", customer_name))
                    }
                    _ => {
                        crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| format!("Hi {}, it's been a while! We'd love to offer you a free 15-minute consultation to get back on track.", customer_name))
                    }
                };

                let work_item_id = Uuid::new_v4();
                let draft_id = Uuid::new_v4();
                let source = "Subscription Retention";
                let payload_json = serde_json::json!({
                    "feature_type": "subscription_retention",
                    "reasoning": "Health Score dropped due to no bookings in 21 days."
                });

                // Create work_item and agent_draft
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                        // Set tenant context for RLS
                        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id_str).await.map_err(|e| e.to_string())?;

                        sqlx::query(
                            "INSERT INTO work_item (id, tenant_id, customer_id, source, payload, status) VALUES ($1, $2, $3, $4, $5, 'PENDING')"
                        )
                        .bind(work_item_id)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(source)
                        .bind(sqlx::types::Json(payload_json))
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        sqlx::query(
                            "INSERT INTO agent_draft (id, work_item_id, response, status) VALUES ($1, $2, $3, 'DRAFT')"
                        )
                        .bind(draft_id)
                        .bind(work_item_id)
                        .bind(generated_response.clone())
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                        sqlx::query(
                            "INSERT INTO work_item (id, tenant_id, customer_id, source, payload, status) VALUES (?, ?, ?, ?, ?, 'PENDING')"
                        )
                        .bind(work_item_id)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(source)
                        .bind(sqlx::types::Json(payload_json))
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        sqlx::query(
                            "INSERT INTO agent_draft (id, work_item_id, response, status) VALUES (?, ?, ?, 'DRAFT')"
                        )
                        .bind(draft_id)
                        .bind(work_item_id)
                        .bind(generated_response.clone())
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;
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
