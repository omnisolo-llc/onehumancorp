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
                let draft_reply_clone = draft_reply.to_string();
                let sender_id_clone = sender_id.to_string();
                let tenant_id_clone = tenant_id.to_string();
                let pool_clone = pool.clone();
                tokio::spawn(async move {
                    // Find the best integration id for whatsapp.
                    let integration_id: Result<String, sqlx::Error> = sqlx::query_scalar(
                        "SELECT integration_id FROM integration_credentials WHERE tenant_id = $1 AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                    )
                    .bind(&tenant_id_clone)
                    .fetch_optional(&pool_clone)
                    .await
                    .map(|opt| opt.unwrap_or_else(|| "whatsapp_cloud_api".to_string()));

                    let id_to_use = integration_id.unwrap_or_else(|_| "whatsapp_cloud_api".to_string());

                    let registry = crate::integrations::registry::IntegrationsRegistry::new();

                    // We need a from_phone if sending via twilio/whatsapp abstraction
                    let from_number = sqlx::query_scalar::<_, String>(
                        "SELECT from_phone FROM integration_credentials WHERE tenant_id = $1 AND integration_id = $2 LIMIT 1"
                    )
                    .bind(&tenant_id_clone)
                    .bind(&id_to_use)
                    .fetch_optional(&pool_clone)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();

                    // If not found in DB, fallback to ENV for twilio
                    let effective_from = if from_number.is_empty() && id_to_use == "twilio" {
                        std::env::var("TWILIO_FROM_NUMBER").unwrap_or_else(|_| "omni".to_string())
                    } else if from_number.is_empty() {
                        "omni".to_string()
                    } else {
                        from_number
                    };

                    // Re-register it in memory just in case (optional, but good practice given how OHC integration registry works)
                    if id_to_use == "twilio" || id_to_use == "whatsapp" {
                         let twilio_creds: Result<(String, String), sqlx::Error> = sqlx::query_as(
                             "SELECT bot_token, api_token FROM integration_credentials WHERE integration_id = $1 AND tenant_id = $2 LIMIT 1"
                         )
                         .bind(&id_to_use)
                         .bind(&tenant_id_clone)
                         .fetch_one(&pool_clone)
                         .await;

                         if let Ok((account_sid, auth_token)) = twilio_creds {
                             let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
                                 integration_id: id_to_use.clone(),
                                 base_url: "https://api.twilio.com".to_string(),
                                 bot_token: account_sid,
                                 chat_id: "".to_string(),
                                 webhook_url: "".to_string(),
                                 api_token: auth_token,
                                 from_phone: effective_from.clone(),
                             };
                             let _ = registry.connect(&id_to_use, "https://api.twilio.com", creds);
                         }
                    }

                    if let Err(e) = registry.send_whatsapp(&id_to_use, &sender_id_clone, &effective_from, &draft_reply_clone).await {
                        tracing::error!("Failed to send WhatsApp reply using {}: {}", id_to_use, e);
                    } else {
                        tracing::info!("Successfully sent WhatsApp reply to {} using {}", sender_id_clone, id_to_use);
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
