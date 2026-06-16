use crate::domain::action_router_router::ActionHandler;
use sqlx::PgPool;
use serde_json::Value;

pub struct IncidentResolutionHandler;

#[async_trait::async_trait]
impl ActionHandler for IncidentResolutionHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, payload: &Value) -> Result<(), String> {
        if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
            sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                .bind(incident_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
            Ok(())
        } else {
            Err("Missing incident_id".to_string())
        }
    }
}

pub struct SocialPostDraftHandler;

#[async_trait::async_trait]
impl ActionHandler for SocialPostDraftHandler {
    async fn execute(&self, _pool: &PgPool, tenant_id: &str, _payload: &Value) -> Result<(), String> {
        tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
        // Real implementation would buffer post here to AYRSHARE.
        Ok(())
    }
}

pub struct AmbassadorReplyHandler;

#[async_trait::async_trait]
impl ActionHandler for AmbassadorReplyHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, payload: &Value) -> Result<(), String> {
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);
            sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
            Ok(())
        } else {
            Err("Missing inbox_message_id".to_string())
        }
    }
}

pub struct QuoteDraftHandler;

#[async_trait::async_trait]
impl ActionHandler for QuoteDraftHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, payload: &Value) -> Result<(), String> {
        if let Some(quote_id_str) = payload.get("quote_id").and_then(|v| v.as_str()) {
            if let Ok(quote_id) = uuid::Uuid::parse_str(quote_id_str) {
                tracing::info!("Approved quote draft: {}", quote_id);
                sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                    .bind(quote_id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Database error: {}", e))?;
                Ok(())
            } else {
                Err("Invalid quote_id UUID format".to_string())
            }
        } else {
            Err("Missing quote_id".to_string())
        }
    }
}

pub struct InstagramDmHandler;

#[async_trait::async_trait]
impl ActionHandler for InstagramDmHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, payload: &Value) -> Result<(), String> {
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
            sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
            Ok(())
        } else {
            Err("Missing inbox_message_id".to_string())
        }
    }
}
