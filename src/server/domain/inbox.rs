use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_inbox_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);

        // Determine message source to handle integration sending
        let row: Option<(String, String)> = sqlx::query_as("SELECT source, sender_id FROM inbox_messages WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .fetch_optional(pool)
            .await?;

        // Handle both standard inbox_messages and omni_inbox_messages updates
        sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
        tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
        sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        if let Some((source, sender_id)) = row {
            if source == "whatsapp" {
                tracing::info!("Sending WhatsApp message via WhatsApp Cloud API for tenant: {}", tenant_id);
                // For this implementation context, we use a mock credentials fallback as configured by the backend architecture
                // In a production setup, we would fetch the token and phone number ID from a settings/credentials table.
                let token = std::env::var("WHATSAPP_API_TOKEN").unwrap_or_else(|_| "test_whatsapp_token".to_string());
                let phone_id = std::env::var("WHATSAPP_PHONE_ID").unwrap_or_else(|_| "test_phone_id".to_string());

                let whatsapp_provider = crate::integrations::whatsapp::WhatsAppProvider::new(token, phone_id);
                match whatsapp_provider.send_message(&sender_id, draft_reply).await {
                    Ok(_) => tracing::info!("WhatsApp message sent successfully to {}", sender_id),
                    Err(e) => tracing::error!("Failed to send WhatsApp message: {}", e),
                }
            }
        }
    }
    Ok(())
}
