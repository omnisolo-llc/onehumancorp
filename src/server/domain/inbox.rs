use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_inbox_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut draft_reply_str = String::new();
    let mut sender_id_str = String::new();
    let mut source_str = String::new();

    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);

        // Handle both standard inbox_messages and omni_inbox_messages updates
        sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        let draft_reply = payload.get("generated_response").or_else(|| payload.get("draft_reply")).and_then(|v| v.as_str()).unwrap_or("");
        draft_reply_str = draft_reply.to_string();

        tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
        sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        // Fallback fetch sender details from omni_inbox_messages if not in payload
        sender_id_str = payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        source_str = payload.get("source").and_then(|v| v.as_str()).unwrap_or("").to_string();

        if sender_id_str.is_empty() || source_str.is_empty() {
            use sqlx::Row;
            if let Ok(Some(row)) = sqlx::query("SELECT sender_id, source FROM omni_inbox_messages WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
            {
                if sender_id_str.is_empty() {
                    sender_id_str = row.try_get("sender_id").unwrap_or_else(|_| "".to_string());
                }
                if source_str.is_empty() {
                    source_str = row.try_get("source").unwrap_or_else(|_| "".to_string());
                }
            }
        }
    }

    if !sender_id_str.is_empty() && (source_str == "whatsapp" || source_str == "instagram_dm" || source_str == "instagram") {
        use sqlx::Row;

        // Try Twilio first
        let twilio_query: Result<Option<sqlx::postgres::PgRow>, _> = sqlx::query("SELECT twilio_whatsapp_config FROM settings WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_optional(pool)
            .await;

        let mut handled = false;

        if let Ok(Some(row)) = twilio_query {
            let twilio_val: Option<serde_json::Value> = row.try_get("twilio_whatsapp_config").unwrap_or(None);
            if let Some(twilio_cfg) = twilio_val {
                if let (Some(sid), Some(token), Some(from)) = (
                    twilio_cfg.get("bot_token").and_then(|v| v.as_str()),
                    twilio_cfg.get("api_token").and_then(|v| v.as_str()),
                    twilio_cfg.get("from_phone").and_then(|v| v.as_str())
                ) {
                    if !sid.is_empty() && !token.is_empty() {
                        tracing::info!("Sending WhatsApp message via Twilio for tenant {}", tenant_id);
                        let client = crate::integrations::twilio::provider::TwilioProvider::new(sid.to_string(), token.to_string());
                        if let Err(e) = client.send_whatsapp(&sender_id_str, from, &draft_reply_str).await {
                            tracing::error!("Twilio send_whatsapp failed: {}", e);
                        } else {
                            handled = true;
                        }
                    }
                }
            }
        }

        // Fallback to Meta Cloud API
        if !handled {
            let meta_query: Result<Option<sqlx::postgres::PgRow>, _> = sqlx::query("SELECT meta_whatsapp_config FROM settings WHERE tenant_id = $1")
                .bind(tenant_id)
                .fetch_optional(pool)
                .await;

            if let Ok(Some(row)) = meta_query {
                let meta_val: Option<serde_json::Value> = row.try_get("meta_whatsapp_config").unwrap_or(None);
                if let Some(meta_cfg) = meta_val {
                    if let Some(token) = meta_cfg.get("api_token").and_then(|v| v.as_str()) {
                        if !token.is_empty() {
                            tracing::info!("Sending WhatsApp message via Meta for tenant {}", tenant_id);
                            // We need to use RealMetaClient directly because we need MetaClientWrapper's send_message
                            use crate::integrations::meta::client::MetaClientWrapper;
                            let client = crate::integrations::meta::client::RealMetaClient::new(token.to_string());
                            let platform = if source_str == "instagram" || source_str == "instagram_dm" { "instagram" } else { "whatsapp" };
                            if let Err(e) = client.send_message(platform, &sender_id_str, &draft_reply_str).await {
                                tracing::error!("Meta send_message failed: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
