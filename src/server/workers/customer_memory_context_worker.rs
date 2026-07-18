use crate::db::DB;
use sqlx::Row;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use serde_json::Value;

pub struct CustomerMemoryContextWorker {
    db: Arc<DB>,
}

impl CustomerMemoryContextWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn run(&self) {
        loop {
            if let Err(e) = self.process_next_job().await {
                tracing::error!("CustomerMemoryContextWorker error: {}", e);
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    async fn process_next_job(&self) -> Result<(), String> {
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query(
            "SELECT job_id, tenant_id, interaction_event_id FROM interaction_event_jobs WHERE status = 'pending' ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED"
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let job_id: Uuid = r.get("job_id");
            let tenant_id: String = r.get("tenant_id");
            let interaction_event_id: Uuid = r.get("interaction_event_id");

            // Fix RLS violation: Must set app.current_tenant context for this transaction
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;

            sqlx::query("UPDATE interaction_event_jobs SET status = 'processing', updated_at = CURRENT_TIMESTAMP WHERE job_id = $1")
                .bind(job_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            // Get the interaction details
            let interaction_row = sqlx::query(
                "SELECT customer_id, channel, raw_content, created_at FROM interaction_events WHERE id = $1 AND tenant_id = $2"
            )
            .bind(interaction_event_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            if let Some(i_row) = interaction_row {
                let customer_id: String = i_row.get("customer_id");
                let channel: String = i_row.get("channel");
                let raw_content: String = i_row.get("raw_content");
                let created_at: chrono::DateTime<chrono::Utc> = i_row.get("created_at");

                // Get or create context_graph
                let context_graph_row = sqlx::query(
                    "SELECT context_graph FROM customer_memory_context WHERE tenant_id = $1 AND customer_id = $2 FOR UPDATE"
                )
                .bind(&tenant_id)
                .bind(&customer_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let mut context_graph = if let Some(cg_row) = context_graph_row {
                    let cg: Value = cg_row.get("context_graph");
                    cg
                } else {
                    serde_json::json!({
                        "interactions": [],
                        "facts": [],
                    })
                };

                // Update graph with new interaction
                if let Some(interactions) = context_graph.get_mut("interactions").and_then(|i| i.as_array_mut()) {
                    interactions.push(serde_json::json!({
                        "id": interaction_event_id,
                        "channel": channel,
                        "content": raw_content,
                        "timestamp": created_at.to_rfc3339()
                    }));
                } else {
                    context_graph["interactions"] = serde_json::json!([
                        {
                            "id": interaction_event_id,
                            "channel": channel,
                            "content": raw_content,
                            "timestamp": created_at.to_rfc3339()
                        }
                    ]);
                }

                // Upsert back
                sqlx::query(
                    "INSERT INTO customer_memory_context (id, tenant_id, customer_id, context_graph, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                    ON CONFLICT (tenant_id, customer_id) DO UPDATE SET context_graph = EXCLUDED.context_graph, updated_at = CURRENT_TIMESTAMP"
                )
                .bind(Uuid::new_v4())
                .bind(&tenant_id)
                .bind(&customer_id)
                .bind(&context_graph)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                // Now try to update `customers.profile_summary` so `ContextCard` doesn't break
                // In a real app we would summarize via LLM, here we just do some simple counting and basic tagging
                let num_interactions = context_graph.get("interactions").and_then(|i| i.as_array()).map(|a| a.len()).unwrap_or(0);

                // Very basic tagging based on raw_content to simulate LLM fact extraction
                let mut preferences = Vec::new();
                let lower_content = raw_content.to_lowercase();
                if lower_content.contains("vegan") {
                    preferences.push("Vegan".to_string());
                }
                if lower_content.contains("urgent") {
                    preferences.push("Urgent Buyer".to_string());
                }

                let mut segments = Vec::new();
                if num_interactions > 2 {
                    segments.push("Frequent Contact".to_string());
                }

                let summary_text = format!("Customer has {} recorded interaction(s).", num_interactions);

                // Read existing profile summary if any to append to it
                let current_summary_row = sqlx::query("SELECT profile_summary FROM customers WHERE id = $1 AND tenant_id = $2")
                    .bind(&customer_id)
                    .bind(&tenant_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                let mut new_summary = serde_json::json!({
                    "total_interactions": num_interactions,
                    "last_interaction": created_at.to_rfc3339(),
                    "segments": segments,
                    "preferences": preferences,
                    "summary": summary_text
                });

                if let Some(r) = current_summary_row {
                    if let Ok(current_summary) = r.try_get::<sqlx::types::Json<Value>, _>("profile_summary") {
                        let mut curr_prefs = current_summary.0.get("preferences").and_then(|p| p.as_array()).map(|a| a.clone()).unwrap_or_default();
                        for pref in preferences {
                            let val = serde_json::json!(pref);
                            if !curr_prefs.contains(&val) {
                                curr_prefs.push(val);
                            }
                        }

                        let mut curr_segs = current_summary.0.get("segments").and_then(|s| s.as_array()).map(|a| a.clone()).unwrap_or_default();
                        for seg in segments {
                            let val = serde_json::json!(seg);
                            if !curr_segs.contains(&val) {
                                curr_segs.push(val);
                            }
                        }

                        new_summary = serde_json::json!({
                            "total_interactions": num_interactions,
                            "last_interaction": created_at.to_rfc3339(),
                            "segments": curr_segs,
                            "preferences": curr_prefs,
                            "summary": summary_text
                        });
                    }
                }

                sqlx::query("UPDATE customers SET profile_summary = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(sqlx::types::Json(new_summary))
                    .bind(&customer_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }

            sqlx::query("UPDATE interaction_event_jobs SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE job_id = $1")
                .bind(job_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;
        } else {
            tx.rollback().await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
