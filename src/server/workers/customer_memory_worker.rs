use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct CustomerMemoryWorker {
    db: Arc<DB>,
}

impl CustomerMemoryWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            tracing::info!("Starting CustomerMemoryWorker for Agentic Omnichannel Customer Context Memory Architecture...");
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue immediately
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                    Err(e) => {
                        tracing::error!("CustomerMemoryWorker error: {}", e);
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
                    SELECT job_id, tenant_id, interaction_event_id
                    FROM interaction_event_jobs
                    WHERE status = 'pending'
                    ORDER BY created_at ASC
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let job_id: Uuid = r.get("job_id");
                    let tenant_id: String = r.get("tenant_id");
                    let event_id: Uuid = r.get("interaction_event_id");

                    sqlx::query("UPDATE interaction_event_jobs SET status = 'processing', updated_at = NOW() WHERE job_id = $1")
                        .bind(job_id)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((job_id, tenant_id, event_id))
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
                    SELECT job_id, tenant_id, interaction_event_id
                    FROM interaction_event_jobs
                    WHERE status = 'pending'
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let job_id: Uuid = r.get("job_id");
                    let tenant_id: String = r.get("tenant_id");
                    let event_id: Uuid = r.get("interaction_event_id");

                    sqlx::query("UPDATE interaction_event_jobs SET status = 'processing', updated_at = CURRENT_TIMESTAMP WHERE job_id = ?")
                        .bind(job_id)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((job_id, tenant_id, event_id))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        if let Some((job_id, tenant_id, event_id)) = job {
            // Get event details
            let mut event_data: Option<(String, String, String)> = None;
            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;
                    let row = sqlx::query("SELECT customer_id, channel, raw_content FROM interaction_events WHERE id = $1")
                        .bind(event_id)
                        .fetch_optional(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                    if let Some(r) = row {
                        event_data = Some((r.get("customer_id"), r.get("channel"), r.get("raw_content")));
                    }
                },
                crate::db::DbStore::Sqlite(pool) => {
                    let row = sqlx::query("SELECT customer_id, channel, raw_content FROM interaction_events WHERE id = ?")
                        .bind(event_id)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                    if let Some(r) = row {
                        event_data = Some((r.get("customer_id"), r.get("channel"), r.get("raw_content")));
                    }
                }
            };

            if let Some((customer_id, channel, raw_content)) = event_data {
                let prompt = format!(
                    "You are an AI tasked with extracting structured memory context from customer interactions.\nChannel: {}\nMessage: {}\nExtract any important facts, preferences, constraints, or status updates regarding this customer. Output strictly as JSON. Example: {{\"preferences\":[\"vegan\"], \"status\":\"waiting for quote\"}}",
                    channel, raw_content
                );

                let max_retries = 3;
                let mut retry_count = 0;
                let mut context_graph = serde_json::json!({
                    "recent_interactions": [
                        { "channel": channel, "content": raw_content }
                    ]
                });

                while retry_count < max_retries {
                    let llm_call = async {
                        match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                            Ok("minimax") => {
                                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                                if !api_key.is_empty() {
                                    crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
                                } else {
                                    crate::minimax::LocalLLMClient::new().reason(&prompt).await
                                }
                            }
                            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
                        }
                    };

                    match tokio::time::timeout(Duration::from_secs(30), llm_call).await {
                        Ok(Ok(reply)) => {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&reply) {
                                if parsed.is_object() {
                                    context_graph = parsed;
                                    break;
                                }
                            }
                            retry_count += 1;
                            if retry_count < max_retries {
                                tokio::time::sleep(Duration::from_secs(1)).await;
                            }
                        }
                        _ => {
                            retry_count += 1;
                            if retry_count < max_retries {
                                tokio::time::sleep(Duration::from_secs(1)).await;
                            }
                        }
                    }
                }

                // Merge context_graph into customer_memory_context table
                let upsert_res: Result<(), String> = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;

                        let current_context = sqlx::query("SELECT context_graph FROM customer_memory_context WHERE tenant_id = $1 AND customer_id = $2")
                            .bind(&tenant_id).bind(&customer_id).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                        let mut final_context = context_graph;
                        if let Some(r) = current_context {
                            if let Ok(val) = r.try_get::<serde_json::Value, _>("context_graph") {
                                // Merge logic could be more sophisticated, but simple replacement/merge
                                if let (Some(obj1), Some(obj2)) = (val.as_object(), final_context.as_object_mut()) {
                                    for (k, v) in obj1 {
                                        if !obj2.contains_key(k) {
                                            obj2.insert(k.clone(), v.clone());
                                        }
                                    }
                                }
                            }
                        }

                        let res = sqlx::query(
                            r#"
                            INSERT INTO customer_memory_context (id, tenant_id, customer_id, context_graph, created_at, updated_at)
                            VALUES ($1, $2, $3, $4, NOW(), NOW())
                            ON CONFLICT (tenant_id, customer_id)
                            DO UPDATE SET context_graph = EXCLUDED.context_graph, updated_at = NOW()
                            "#
                        )
                        .bind(Uuid::new_v4())
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .bind(final_context)
                        .execute(&mut *tx)
                        .await;

                        tx.commit().await.map_err(|e| e.to_string())?;
                        res.map(|_| ()).map_err(|e| e.to_string())
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                        let current_context = sqlx::query("SELECT context_graph FROM customer_memory_context WHERE tenant_id = ? AND customer_id = ?")
                            .bind(&tenant_id).bind(&customer_id).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

                        let mut final_context = context_graph;
                        if let Some(r) = current_context {
                            if let Ok(val) = r.try_get::<serde_json::Value, _>("context_graph") {
                                if let (Some(obj1), Some(obj2)) = (val.as_object(), final_context.as_object_mut()) {
                                    for (k, v) in obj1 {
                                        if !obj2.contains_key(k) {
                                            obj2.insert(k.clone(), v.clone());
                                        }
                                    }
                                }
                            }
                        }

                        let res = sqlx::query(
                            r#"
                            INSERT INTO customer_memory_context (id, tenant_id, customer_id, context_graph, created_at, updated_at)
                            VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                            ON CONFLICT (tenant_id, customer_id)
                            DO UPDATE SET context_graph = EXCLUDED.context_graph, updated_at = CURRENT_TIMESTAMP
                            "#
                        )
                        .bind(Uuid::new_v4())
                        .bind(&tenant_id)
                        .bind(&customer_id)
                        .bind(final_context.to_string())
                        .execute(&mut *tx)
                        .await;

                        tx.commit().await.map_err(|e| e.to_string())?;
                        res.map(|_| ()).map_err(|e| e.to_string())
                    }
                };

                if let Err(e) = upsert_res {
                    tracing::error!("Failed to upsert customer_memory_context: {}", e);
                }

                // Update job status
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("UPDATE interaction_event_jobs SET status = 'completed', updated_at = NOW() WHERE job_id = $1")
                            .bind(job_id)
                            .execute(&self.db.pool).await;
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        let _ = sqlx::query("UPDATE interaction_event_jobs SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE job_id = ?")
                            .bind(job_id)
                            .execute(pool).await;
                    }
                }
            } else {
                // Event not found, mark job as failed
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("UPDATE interaction_event_jobs SET status = 'failed', updated_at = NOW() WHERE job_id = $1")
                            .bind(job_id)
                            .execute(&self.db.pool).await;
                    },
                    crate::db::DbStore::Sqlite(pool) => {
                        let _ = sqlx::query("UPDATE interaction_event_jobs SET status = 'failed', updated_at = CURRENT_TIMESTAMP WHERE job_id = ?")
                            .bind(job_id)
                            .execute(pool).await;
                    }
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
