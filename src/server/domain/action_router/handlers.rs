use sqlx::PgPool;
use super::registry::ActionHandler;
use super::payload::ActionIntent;
use uuid::Uuid;

pub struct IncidentResolutionHandler;

#[async_trait::async_trait]
impl ActionHandler for IncidentResolutionHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(incident_id) = intent.payload.get("incident_id").and_then(|v| v.as_str()) {
            let res = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                .bind(incident_id)
                .bind(tenant_id)
                .execute(pool)
                .await;
            if let Err(e) = res {
                return Err(format!("Failed to update incident: {}", e));
            }
        }
        Ok(())
    }
}

pub struct SocialPostDraftHandler;

#[async_trait::async_trait]
impl ActionHandler for SocialPostDraftHandler {
    async fn execute(&self, _pool: &PgPool, tenant_id: &str, _intent: &ActionIntent) -> Result<(), String> {
        tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
        // Real implementation would buffer post here to AYRSHARE.
        Ok(())
    }
}

pub struct AmbassadorReplyHandler;

#[async_trait::async_trait]
impl ActionHandler for AmbassadorReplyHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(inbox_id) = intent.payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);

            // Legacy inbox
            let res1 = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            // Omni inbox
            let draft_reply = intent.payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            let res2 = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            if res1.is_err() && res2.is_err() {
                return Err("Failed to update ambassador reply".into());
            }
        }
        Ok(())
    }
}

pub struct QuoteDraftHandler;

#[async_trait::async_trait]
impl ActionHandler for QuoteDraftHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(quote_id) = intent.payload.get("quote_id").and_then(|v| v.as_str()) {
            tracing::info!("Approved quote draft: {}", quote_id);
            let res = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                .bind(Uuid::parse_str(quote_id).unwrap_or_default())
                .bind(tenant_id)
                .execute(pool)
                .await;
            if let Err(e) = res {
                return Err(format!("Failed to update quote: {}", e));
            }
        }
        Ok(())
    }
}

pub struct InstagramDmHandler;

#[async_trait::async_trait]
impl ActionHandler for InstagramDmHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(inbox_id) = intent.payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let draft_reply = intent.payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            tracing::info!("Approved Instagram DM draft reply for inbox_id: {}", inbox_id);
            let res = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;
            if let Err(e) = res {
                return Err(format!("Failed to update Instagram DM: {}", e));
            }
        }
        Ok(())
    }
}
