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

        // Fetch message details to dispatch out if it's WhatsApp or SMS
        let row: Option<(String, String)> = sqlx::query_as(
            "SELECT source, sender_id FROM omni_inbox_messages WHERE id = $1 AND tenant_id = $2"
        )
        .bind(inbox_id)
        .bind(tenant_id)
        .fetch_optional(pool)
        .await?;

        if let Some((source, sender_id)) = row {
            if source == "whatsapp" {
                let twilio_creds: Result<(String, String, String), sqlx::Error> = sqlx::query_as(
                    "SELECT bot_token, api_token, from_phone FROM integration_credentials WHERE integration_id = 'whatsapp' AND tenant_id = $1 LIMIT 1"
                )
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
                .map(|opt| opt.unwrap_or_default());

                if let Ok((account_sid, auth_token, from_number)) = twilio_creds {
                    if !account_sid.trim().is_empty() && !auth_token.trim().is_empty() && !from_number.trim().is_empty() {
                        let provider = crate::integrations::twilio::provider::TwilioProvider::new(account_sid, auth_token);
                        let draft_reply_clone = draft_reply.to_string();
                        let sender_id_clone = sender_id.to_string();
                        tokio::spawn(async move {
                            if let Err(e) = provider.send_whatsapp(&sender_id_clone, &from_number, &draft_reply_clone).await {
                                tracing::error!("Failed to send Twilio WhatsApp reply: {}", e);
                            } else {
                                tracing::info!("Successfully sent Twilio WhatsApp reply to {}", sender_id_clone);
                            }
                        });
                        return Ok(());
                    }
                }
                let registry = crate::integrations::registry::IntegrationsRegistry::new();
                let draft_reply_clone = draft_reply.to_string();
                let sender_id_clone = sender_id.to_string();
                tokio::spawn(async move {
                    if let Err(e) = registry.send_whatsapp("whatsapp_cloud_api", &sender_id_clone, "omni", &draft_reply_clone).await {
                        tracing::error!("Failed to send WhatsApp reply: {}", e);
                    } else {
                        tracing::info!("Successfully sent WhatsApp reply to {}", sender_id_clone);
                    }
                });
            } else if source == "instagram" || source == "facebook" {
                let meta_creds: Result<String, sqlx::Error> = sqlx::query_scalar(
                    "SELECT api_token FROM integration_credentials WHERE integration_id = 'meta' AND tenant_id = $1 LIMIT 1"
                )
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
                .map(|opt| opt.unwrap_or_default());

                if let Ok(api_token) = meta_creds {
                    if !api_token.trim().is_empty() {
                        let registry = crate::integrations::registry::IntegrationsRegistry::new();
                        let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
                            integration_id: "meta".to_string(),
                            base_url: "".to_string(),
                            bot_token: "".to_string(),
                            chat_id: "".to_string(),
                            webhook_url: "".to_string(),
                            api_token: api_token.clone(),
                            from_phone: "".to_string(),
                        };
                        let _ = registry.connect("meta", "", creds);

                        let draft_reply_clone = draft_reply.to_string();
                        let sender_id_clone = sender_id.to_string();
                        let source_clone = source.clone();
                        tokio::spawn(async move {
                            if let Err(e) = registry.send_message("meta", &source_clone, &sender_id_clone, &draft_reply_clone).await {
                                tracing::error!("Failed to send {} reply: {}", source_clone, e);
                            } else {
                                tracing::info!("Successfully sent {} reply to {}", source_clone, sender_id_clone);
                            }
                        });
                    }
                }
            } else if source == "sms" {
                let twilio_creds: Result<(String, String, String), sqlx::Error> = sqlx::query_as(
                    "SELECT bot_token, api_token, from_phone FROM integration_credentials WHERE integration_id = 'twilio' AND tenant_id = $1 LIMIT 1"
                )
                .bind(tenant_id)
                .fetch_one(pool)
                .await;

                let (account_sid, auth_token, from_number) = match twilio_creds {
                    Ok(creds) => creds,
                    Err(_) => {
                        (
                            std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_default(),
                            std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default(),
                            std::env::var("TWILIO_FROM_NUMBER").unwrap_or_default(),
                        )
                    }
                };

                if !account_sid.trim().is_empty() && !auth_token.trim().is_empty() && !from_number.trim().is_empty() {
                    let provider = crate::integrations::twilio::provider::TwilioProvider::new(account_sid, auth_token);
                    let draft_reply_clone = draft_reply.to_string();

                    tokio::spawn(async move {
                        if let Err(e) = provider.send_sms(&sender_id, &from_number, &draft_reply_clone).await {
                            tracing::error!("Failed to send SMS reply: {}", e);
                        } else {
                            tracing::info!("Successfully sent SMS reply to {}", sender_id);
                        }
                    });
                }
            }
        }
    }
    Ok(())
}
