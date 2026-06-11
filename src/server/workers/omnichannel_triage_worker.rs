use std::sync::Arc;
use std::time::Duration;
use crate::db::DB;
use sqlx::Row;
use uuid::Uuid;

pub struct OmnichannelTriageWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl OmnichannelTriageWorker {
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
                        Ok(true) => continue, // keep polling until no more new message threads
                        Ok(false) => break,
                        Err(e) => {
                            tracing::error!("OmnichannelTriageWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let item = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                // Find a message thread that is pending and has no agent draft
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, context, source FROM message_threads t
                    WHERE t.status = 'pending'
                      AND NOT EXISTS (
                          SELECT 1 FROM agent_drafts a WHERE a.message_thread_id = t.id
                      )
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
                    let context: Option<String> = r.try_get("context").unwrap_or(None);
                    let source: Option<String> = r.try_get("source").unwrap_or(None);
                    Some((tx, id, tenant_id, context, source))
                } else {
                    None
                }
            },
            _ => None,
        };

        if let Some((mut tx, id, tenant_id, _context, source)) = item {
            // Simulated LLM drafting. In a real scenario, this would call `crate::api::agents::translation::generate_inbox_draft_reply` or similar.
            let reply_text = format!("Draft reply for your request from {}: Sure, we can help with that!", source.unwrap_or_else(|| "Unknown".to_string()));

            let draft_id = Uuid::new_v4().to_string();

            sqlx::query(
                "INSERT INTO agent_drafts (id, message_thread_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(&draft_id)
            .bind(&id)
            .bind(&tenant_id)
            .bind("Draft Reply")
            .bind(&reply_text)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(true);
        }

        Ok(false)
    }
}
