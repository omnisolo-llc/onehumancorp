use sqlx::PgPool;
use std::time::Duration;
use crate::agents::builtin::llm::openai::{OpenAIClient, ChatRequest, Message as OpenAIMessage};

pub async fn start_chat_triage_worker(pool: PgPool) {
    let client = OpenAIClient::new(std::env::var("OPENAI_API_KEY").unwrap_or_default());

    loop {
        let result = process_next_pending_message(&pool, &client).await;
        if let Err(e) = result {
            tracing::error!("Chat worker error: {}", e);
            tokio::time::sleep(Duration::from_secs(5)).await;
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn process_next_pending_message(pool: &PgPool, client: &OpenAIClient) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let pending_msg = sqlx::query!(
        r#"
        SELECT id, tenant_id, conversation_id, content
        FROM chat_messages
        WHERE status = 'pending_ai'
        FOR UPDATE SKIP LOCKED
        LIMIT 1
        "#
    )
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(msg) = pending_msg {
        let req = ChatRequest {
            model: "gpt-4o".to_string(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: "You are an AI customer assistant drafting a reply for a business owner. Provide a polite and helpful draft response.".to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: msg.content.clone(),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(500),
            stream: Some(false),
        };

        let draft_reply = match client.chat_completion(&req).await {
            Ok(resp) => {
                resp.choices.first()
                    .map(|c| c.message.content.clone())
                    .unwrap_or_else(|| "Error generating draft.".to_string())
            },
            Err(e) => {
                tracing::error!("LLM Error: {}", e);
                "Error generating draft.".to_string()
            }
        };

        sqlx::query!(
            r#"
            UPDATE chat_messages
            SET status = 'draft', draft_reply = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            draft_reply,
            msg.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }

    Ok(())
}
