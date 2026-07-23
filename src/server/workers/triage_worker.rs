use crate::db::DbStore;
use crate::db::DB;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use sqlx::Row;

pub async fn start_triage_worker(db: Arc<DB>) {
    loop {
        match process_next_triage_item(&db).await {
            Ok(true) => {
                // Processed an item, loop immediately to check for more
                continue;
            }
            Ok(false) => {
                // No items found, sleep
            }
            Err(e) => {
                tracing::error!("Triage worker error: {:?}", e);
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}

async fn process_next_triage_item(db: &DB) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match &db.store {
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await?;
            let row = sqlx::query(
                r#"
                SELECT id, tenant_id, context, source
                FROM triage_items
                WHERE status = 'pending'
                AND id NOT IN (SELECT triage_item_id FROM triage_proposed_actions)
                FOR UPDATE SKIP LOCKED
                LIMIT 1
                "#
            ).fetch_optional(&mut *tx).await?;

            if let Some(r) = row {
                let triage_id: String = r.try_get("id").unwrap_or_default();
                let tenant_id: String = r.try_get("tenant_id").unwrap_or_default();
                let context: String = r.try_get("context").unwrap_or_default();

                let prompt = format!("You are a helpful assistant. The customer said: {}. Provide a brief draft reply.", context);
                let llm_reply = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                    Ok("minimax") => crate::minimax::MinimaxClient::new(std::env::var("MINIMAX_API_KEY").unwrap_or_default()).reason(&prompt).await.unwrap_or_else(|_| "We are looking into this.".to_string()),
                    _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| "We are looking into this.".to_string()),
                };

                let draft_reply = format!("AI Drafted Reply: {}", llm_reply);
                let payload = serde_json::json!({ "reply": draft_reply });
                let payload_str = payload.to_string();
                let action_id = format!("action-{}", uuid::Uuid::new_v4());

                sqlx::query(
                    r#"
                    INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload)
                    VALUES ($1, $2, $3, 'draft_reply', $4)
                    "#
                )
                .bind(action_id)
                .bind(triage_id)
                .bind(tenant_id)
                .bind(payload_str)
                .execute(&mut *tx).await?;

                tx.commit().await?;
                return Ok(true);
            }
            tx.commit().await?;
            return Ok(false);
        },
        crate::db::DbStore::Sqlite(pool) => {
            let mut tx = pool.begin().await?;
            let row = sqlx::query(
                r#"
                SELECT id, tenant_id, context, source
                FROM triage_items
                WHERE status = 'pending'
                AND id NOT IN (SELECT triage_item_id FROM triage_proposed_actions)
                LIMIT 1
                "#
            ).fetch_optional(&mut *tx).await?;

            if let Some(r) = row {
                let triage_id: String = r.try_get("id").unwrap_or_default();
                let tenant_id: String = r.try_get("tenant_id").unwrap_or_default();
                let context: String = r.try_get("context").unwrap_or_default();

                let prompt = format!("You are a helpful assistant. The customer said: {}. Provide a brief draft reply.", context);
                let llm_reply = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                    Ok("minimax") => crate::minimax::MinimaxClient::new(std::env::var("MINIMAX_API_KEY").unwrap_or_default()).reason(&prompt).await.unwrap_or_else(|_| "We are looking into this.".to_string()),
                    _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| "We are looking into this.".to_string()),
                };

                let draft_reply = format!("AI Drafted Reply: {}", llm_reply);
                let payload = serde_json::json!({ "reply": draft_reply });
                let payload_str = payload.to_string();
                let action_id = format!("action-{}", uuid::Uuid::new_v4());

                sqlx::query(
                    r#"
                    INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload)
                    VALUES (?, ?, ?, 'draft_reply', ?)
                    "#
                )
                .bind(action_id)
                .bind(triage_id)
                .bind(tenant_id)
                .bind(payload_str)
                .execute(&mut *tx).await?;

                tx.commit().await?;
                return Ok(true);
            }
            tx.commit().await?;
            return Ok(false);
        }
    }
}