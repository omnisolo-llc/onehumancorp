use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use sqlx::Row;
use serde_json::Value;

pub struct AutoResponderWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl AutoResponderWorker {
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
                            ::server_telemetry::record_error_signal("[bug] AutoResponderWorker error");
                            tracing::error!("AutoResponderWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let job = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM ohc_job_queue
                        WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'auto_responder'
                        ORDER BY next_retry_at ASC, created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    ) RETURNING id, tenant_id, payload
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id"), r.get::<Value, _>("payload")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'auto_responder'
                    ORDER BY next_retry_at ASC, created_at ASC
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
                    let payload: Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({}));

                    sqlx::query(
                        "UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                    )
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

        if let Some((job_id, tenant_id, payload)) = job {
            let message_id = payload.get("message_id").and_then(|v| v.as_str()).unwrap_or("");
            let source = payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");
            let customer_message = payload.get("content").and_then(|v| v.as_str()).unwrap_or("");

            // Retrieve business context
            let (tenant_name, tenant_industry) = match &db.store {
                crate::db::DbStore::Postgres => {
                    let row = sqlx::query("SELECT name, industry FROM tenants WHERE id = $1")
                        .bind(&tenant_id)
                        .fetch_optional(&db.pool)
                        .await
                        .unwrap_or(None);
                    if let Some(r) = row {
                        (r.get::<String, _>("name"), r.try_get::<String, _>("industry").unwrap_or_default())
                    } else {
                        ("A business".to_string(), "".to_string())
                    }
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let row = sqlx::query("SELECT name, industry FROM tenants WHERE id = ?")
                        .bind(&tenant_id)
                        .fetch_optional(sqlite_pool)
                        .await
                        .unwrap_or(None);
                    if let Some(r) = row {
                        (r.get::<String, _>("name"), r.try_get::<String, _>("industry").unwrap_or_default())
                    } else {
                        ("A business".to_string(), "".to_string())
                    }
                }
            };

            let business_context = if tenant_industry.is_empty() {
                format!("A business named {}", tenant_name)
            } else {
                format!("A {} business named {}", tenant_industry, tenant_name)
            };

            let prompt = format!(
                "You are the Customer & Relationship Assistant for {}. Draft a short, friendly chat reply to this customer message received via {}: '{}'.",
                business_context, source, customer_message
            );
            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let mut drafted_msg = "Thanks for reaching out! We will get back to you soon.".to_string();
            let mut retry_count = 0;
            let max_retries = 3;
            let mut backoff = Duration::from_secs(2);

            while retry_count < max_retries {
                let llm_call = async {
                    match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                        .as_deref()
                    {
                        Ok("minimax") => {
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            if !api_key.is_empty() {
                                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await
                            } else {
                                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await
                            }
                        }
                        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await,
                    }
                };

                match tokio::time::timeout(Duration::from_secs(60), llm_call).await {
                    Ok(Ok(reply)) => {
                        drafted_msg = reply;
                        break;
                    }
                    Ok(Err(e)) => {
                        retry_count += 1;
                        tracing::warn!("LLM error in AutoResponderWorker (attempt {}/{}): {}", retry_count, max_retries, e);
                        if retry_count < max_retries {
                            tokio::time::sleep(backoff).await;
                            backoff *= 2;
                        }
                    }
                    Err(_) => {
                        retry_count += 1;
                        tracing::warn!("LLM timeout in AutoResponderWorker (attempt {}/{}): 60s exceeded", retry_count, max_retries);
                        if retry_count < max_retries {
                            tokio::time::sleep(backoff).await;
                            backoff *= 2;
                        }
                    }
                }
            }

            match &db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("UPDATE inbox_messages SET status = 'auto_replied', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(&drafted_msg)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(&db.pool)
                        .await;
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&db.pool)
                        .await;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query("UPDATE inbox_messages SET status = 'auto_replied', draft_reply = ? WHERE id = ? AND tenant_id = ?")
                        .bind(&drafted_msg)
                        .bind(&message_id)
                        .bind(&tenant_id)
                        .execute(sqlite_pool)
                        .await;
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&job_id)
                        .execute(sqlite_pool)
                        .await;
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

    async fn setup_test_db() -> Arc<DB> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let _ = sqlx::query("CREATE TABLE tenants (id TEXT PRIMARY KEY, name TEXT, industry TEXT);").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, next_retry_at TIMESTAMP, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE inbox_messages (id TEXT PRIMARY KEY, tenant_id TEXT, source TEXT, content TEXT, draft_reply TEXT, status TEXT);").execute(&pool).await;

        let db = DB {
            pool: pool.clone(),
            store: DbStore::Sqlite(pool),
            read_pool: None,
        };
        Arc::new(db)
    }

    #[tokio::test]
    async fn test_worker_dequeues_job_and_updates_inbox() {
        let db = setup_test_db().await;
        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant1', 'Maya Bakery', 'Bakery')")
                .execute(pool).await.unwrap();

            sqlx::query("INSERT INTO inbox_messages (id, tenant_id, source, content, status) VALUES ('msg1', 'tenant1', 'instagram', 'Do you make vegan cakes?', 'unread')")
                .execute(pool).await.unwrap();

            let payload = serde_json::json!({
                "message_id": "msg1",
                "source": "instagram",
                "content": "Do you make vegan cakes?"
            });

            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ('job1', 'tenant1', 'auto_responder', ?, 'PENDING', CURRENT_TIMESTAMP)")
                .bind(payload.to_string())
                .execute(pool).await.unwrap();
        }

        let processed = AutoResponderWorker::poll(&db).await.unwrap();
        assert!(processed);

        if let DbStore::Sqlite(pool) = &db.store {
            let row = sqlx::query("SELECT status, draft_reply FROM inbox_messages WHERE id = 'msg1'")
                .fetch_one(pool).await.unwrap();
            let status: String = row.get("status");
            let draft_reply: String = row.get("draft_reply");

            assert_eq!(status, "auto_replied");
            assert!(!draft_reply.is_empty());

            let job_status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = 'job1'")
                .fetch_one(pool).await.unwrap();
            assert_eq!(job_status, "COMPLETED");
        }
    }
}
