use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_inbox_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);

        // Handle both standard inbox_messages and omni_inbox_messages updates
        sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        let draft_reply = payload.get("generated_response").or_else(|| payload.get("draft_reply")).and_then(|v| v.as_str()).unwrap_or("");
        tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
        sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
<<<<<<< HEAD

        // Fetch message details to dispatch out if it's WhatsApp or SMS
        let row: Option<(String, String)> = sqlx::query_as(
            "SELECT source, sender_id FROM omni_inbox_messages WHERE id = $1 AND tenant_id = $2"
        )
        .bind(inbox_id)
        .bind(tenant_id)
        .fetch_optional(pool)
        .await?;

        if let Some((source, sender_id)) = row {
            if source == "whatsapp" || source == "sms" {
                if let (Ok(account_sid), Ok(auth_token), Ok(from_number)) = (
                    std::env::var("TWILIO_ACCOUNT_SID"),
                    std::env::var("TWILIO_AUTH_TOKEN"),
                    std::env::var("TWILIO_FROM_NUMBER"),
                ) {
                    if !account_sid.trim().is_empty() && !auth_token.trim().is_empty() && !from_number.trim().is_empty() {
                        let provider = crate::integrations::twilio::provider::TwilioProvider::new(account_sid, auth_token);
                        let draft_reply_clone = draft_reply.to_string();

                        tokio::spawn(async move {
                            if source == "whatsapp" {
                                let to = if sender_id.starts_with("whatsapp:") { sender_id.clone() } else { format!("whatsapp:{}", sender_id) };
                                let from = if from_number.starts_with("whatsapp:") { from_number.clone() } else { format!("whatsapp:{}", from_number) };
                                if let Err(e) = provider.send_whatsapp(&to, &from, &draft_reply_clone).await {
                                    tracing::error!("Failed to send WhatsApp reply: {}", e);
                                } else {
                                    tracing::info!("Successfully sent WhatsApp reply to {}", to);
                                }
                            } else {
                                if let Err(e) = provider.send_sms(&sender_id, &from_number, &draft_reply_clone).await {
                                    tracing::error!("Failed to send SMS reply: {}", e);
                                } else {
                                    tracing::info!("Successfully sent SMS reply to {}", sender_id);
                                }
                            }
                        });
                    }
                }
            }
        }
=======
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
    }
    Ok(())
}
