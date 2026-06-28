use std::sync::Arc;
use std::time::Duration;
use crate::db::DB;
use serde_json::Value;

pub struct AmbassadorWorker;

impl AmbassadorWorker {
    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let job = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'ambassador_intent'
                    ORDER BY next_retry_at ASC, created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_val: Value = r.get("payload");

                    sqlx::query(
                        "UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1"
                    )
                    .bind(&id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload_val))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload
                    FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'ambassador_intent'
                    ORDER BY next_retry_at ASC, created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    use sqlx::Row;
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
            let _message_id = payload.get("message_id").and_then(|v| v.as_str()).unwrap_or("");
            let source = payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");
            let customer_message = payload.get("content").and_then(|v| v.as_str()).unwrap_or("");
            let thread_id = payload.get("thread_id").and_then(|v| v.as_str()).unwrap_or("");
            let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");

            // RAG Context Gathering
            let (tenant_name, tenant_industry) = match &db.store {
                crate::db::DbStore::Postgres => {
                    let row = sqlx::query("SELECT name, industry FROM tenants WHERE id = $1")
                        .bind(&tenant_id)
                        .fetch_optional(&db.pool)
                        .await
                        .unwrap_or(None);
                    if let Some(r) = row {
                        use sqlx::Row;
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
                        use sqlx::Row;
                        (r.get::<String, _>("name"), r.try_get::<String, _>("industry").unwrap_or_default())
                    } else {
                        ("A business".to_string(), "".to_string())
                    }
                }
            };

            let inventory_context = match &db.store {
                crate::db::DbStore::Postgres => {
                    let rows = sqlx::query("SELECT name, quantity FROM inventory_items WHERE tenant_id = $1 LIMIT 10")
                        .bind(&tenant_id)
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default();
                    let mut ctx = String::new();
                    for r in rows {
                        use sqlx::Row;
                        let name: String = r.get("name");
                        let qty: i32 = r.get("quantity");
                        ctx.push_str(&format!("{}: {} in stock. ", name, qty));
                    }
                    ctx
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let rows = sqlx::query("SELECT name, quantity FROM inventory_items WHERE tenant_id = ? LIMIT 10")
                        .bind(&tenant_id)
                        .fetch_all(sqlite_pool)
                        .await
                        .unwrap_or_default();
                    let mut ctx = String::new();
                    for r in rows {
                        use sqlx::Row;
                        let name: String = r.get("name");
                        let qty: i32 = r.get("quantity");
                        ctx.push_str(&format!("{}: {} in stock. ", name, qty));
                    }
                    ctx
                }
            };

            let business_context = if tenant_industry.is_empty() {
                format!("A business named {}", tenant_name)
            } else {
                format!("A {} business named {}", tenant_industry, tenant_name)
            };

            let prompt = format!(
                "You are The Ambassador, an AI intent orchestrator for {}. \
                Analyze this inbound message from {}: '{}'. \
                Current inventory context: {}. \
                Draft a highly helpful, actionable reply (e.g. confirming availability, offering a booking link).",
                business_context, source, customer_message, inventory_context
            );

            let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let mut drafted_msg = "Thanks for reaching out! We will get back to you soon.".to_string();
            let mut retry_count = 0;
            let max_retries = 3;
            let mut backoff = Duration::from_secs(2);

            while retry_count < max_retries {
                let llm_call = async {
                    match std::env::var("OHC_LLM_PROVIDER").as_deref() {
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
                        tracing::warn!("LLM error in AmbassadorWorker (attempt {}/{}): {}", retry_count, max_retries, e);
                        if retry_count < max_retries {
                            tokio::time::sleep(backoff).await;
                            backoff *= 2;
                        }
                    }
                    Err(_) => {
                        retry_count += 1;
                        tracing::warn!("LLM timeout in AmbassadorWorker (attempt {}/{}): 60s exceeded", retry_count, max_retries);
                        if retry_count < max_retries {
                            tokio::time::sleep(backoff).await;
                            backoff *= 2;
                        }
                    }
                }
            }

            let action_id = format!("act-{}", uuid::Uuid::new_v4());
            let context_summary = if inventory_context.is_empty() { "General Inquiry".to_string() } else { "Inventory Checked".to_string() };

            let action_payload = serde_json::to_string(&serde_json::json!({
                "customer_id": customer_id,
                "context_summary": context_summary,
                "draft_reply": drafted_msg
            })).unwrap();

            match &db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES ($1, $2, $3, 'DRAFT_REPLY', $4, 'pending')")
                        .bind(&action_id)
                        .bind(&tenant_id)
                        .bind(&thread_id)
                        .bind(&action_payload)
                        .execute(&db.pool)
                        .await;
                    let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                        .bind(&job_id)
                        .execute(&db.pool)
                        .await;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES (?, ?, ?, 'DRAFT_REPLY', ?, 'pending')")
                        .bind(&action_id)
                        .bind(&tenant_id)
                        .bind(&thread_id)
                        .bind(&action_payload)
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
