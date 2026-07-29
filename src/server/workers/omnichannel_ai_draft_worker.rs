use sqlx::PgPool;
use uuid::Uuid;
use std::time::Duration;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use std::sync::Arc;

pub async fn start_omnichannel_ai_draft_worker(pool: PgPool, orchestrator: Arc<DepartmentOrchestrator>) {
    tokio::spawn(async move {
        loop {
            // Find a pending message to draft a reply for
            let result = sqlx::query!(
                r#"
                WITH next_message AS (
                    SELECT m.id, m.tenant_id, m.content, m.sender_id, m.conversation_id
                    FROM chat_messages m
                    WHERE m.status = 'pending_draft'
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                )
                UPDATE chat_messages
                SET status = 'drafting', updated_at = NOW()
                FROM next_message
                WHERE chat_messages.id = next_message.id
                RETURNING next_message.id, next_message.tenant_id, next_message.content, next_message.sender_id, next_message.conversation_id
                "#
            )
            .fetch_optional(&pool)
            .await;

            match result {
                Ok(Some(row)) => {
                    let tenant_id_str = row.tenant_id.to_string();
                    let description = format!("Draft AI reply for chat message {}", row.id);
                    let payload = serde_json::json!({
                        "message_id": row.id,
                        "conversation_id": row.conversation_id,
                        "content": row.content,
                        "sender_id": row.sender_id,
                        "action": "draft_reply"
                    });

                    // Trigger Ambassador agent via orchestrator
                    match orchestrator.execute_action(
                        DepartmentType::CustomerSuccess, // Assuming Ambassador is here
                        description,
                        tenant_id_str,
                        ActionRisk::DraftForReview,
                        payload,
                    ).await {
                        Ok(_) => {
                            let _ = sqlx::query!(
                                r#"
                                UPDATE chat_messages
                                SET status = 'draft_ready', updated_at = NOW()
                                WHERE id = $1
                                "#,
                                row.id
                            ).execute(&pool).await;
                        }
                        Err(e) => {
                            eprintln!("Failed to draft reply: {:?}", e);
                            let _ = sqlx::query!(
                                r#"
                                UPDATE chat_messages
                                SET status = 'pending_draft', updated_at = NOW()
                                WHERE id = $1
                                "#,
                                row.id
                            ).execute(&pool).await;
                        }
                    }
                }
                Ok(None) => {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
                Err(e) => {
                    eprintln!("Error in omnichannel AI draft worker: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}
