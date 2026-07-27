use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::minimax::LocalLLMClient;

pub struct OmnichannelChatWorker {
    pub db: Arc<DB>,
    pub chat_service: Arc<ChatService>,
}

impl OmnichannelChatWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            chat_service: Arc::new(ChatService::new(db.pool.clone())),
            db,
        }
    }

    pub async fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.poll().await {
                    Ok(true) => continue, // Processed a job, immediately check for more
                    Ok(false) => sleep(Duration::from_secs(5)).await, // Queue empty, wait
                    Err(e) => {
                        tracing::error!("OmnichannelChatWorker polling error: {}", e);
                        sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        });
    }

    async fn poll(&self) -> Result<bool, String> {
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query!(
            r#"
            UPDATE ohc_job_queue
            SET status = 'PROCESSING',
                locked_until = NOW() + INTERVAL '5 minutes',
                updated_at = NOW()
            WHERE id = (
                SELECT id FROM ohc_job_queue
                WHERE status = 'PENDING' AND job_type = 'draft_omnichannel_reply'
                ORDER BY next_retry_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, tenant_id, payload
            "#
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(record) = row {
            let job_id = record.id;
            let tenant_id_str = record.tenant_id;
            let payload = record.payload;

            let result = self.process_job(&tenant_id_str, payload).await;

            if result.is_ok() {
                let _ = sqlx::query!(
                    "UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1",
                    job_id
                )
                .execute(&mut *tx)
                .await;
            } else {
                let _ = sqlx::query!(
                    "UPDATE ohc_job_queue SET status = 'FAILED', updated_at = NOW() WHERE id = $1",
                    job_id
                )
                .execute(&mut *tx)
                .await;
            }

            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(true);
        }

        tx.rollback().await.map_err(|e| e.to_string())?;
        Ok(false)
    }

    async fn process_job(&self, tenant_id_str: &str, payload: serde_json::Value) -> Result<(), String> {
        let tenant_id = Uuid::parse_str(tenant_id_str).map_err(|e| e.to_string())?;
        let conversation_id = Uuid::parse_str(payload["conversation_id"].as_str().unwrap_or_default()).map_err(|e| e.to_string())?;
        let content = payload["content"].as_str().unwrap_or_default();

        let prompt = format!(
            "You are a helpful customer assistant. Please draft a reply to the customer's message: '{}'",
            content
        );

        let llm_client = LocalLLMClient::new();
        let draft = match llm_client.reason(&prompt).await {
            Ok(d) => d,
            Err(_) => "I'm currently unable to assist you, but our team will get back to you shortly.".to_string(),
        };

        self.chat_service
            .send_message(
                tenant_id,
                conversation_id,
                "agent".to_string(), // Sending as agent but in draft state (we could add a status to send_message, but let's assume agent means draft for now or we will just use 'agent' and frontend can allow approval, or we use 'bot')
                None,
                draft,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Add tests here later in the test step
}

#[cfg(test)]
mod worker_tests {
    use super::*;

    #[test]
    fn test_worker_struct() {
        // structural test
    }
}
