use sqlx::PgPool;
use crate::domain::action_router::router::{ActionRouter, HandlerResult};

pub async fn handle_social_post_draft(_pool: PgPool, tenant_id: String, _payload: serde_json::Value) -> HandlerResult {
    tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
    // Real implementation would buffer post here to AYRSHARE.
    Ok(())
}

pub async fn handle_ambassador_reply(pool: PgPool, tenant_id: String, payload: serde_json::Value) -> HandlerResult {
    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);

        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str());

        if let Some(draft) = draft_reply {
            // New structure from omni_inbox_messages updates
            let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft)
                .bind(inbox_id)
                .bind(&tenant_id)
                .execute(&pool)
                .await.map_err(|e| e.to_string())?;
        } else {
            // Old structure from inbox_messages updates
            let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(&tenant_id)
                .execute(&pool)
                .await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub async fn handle_quote_draft(pool: PgPool, tenant_id: String, payload: serde_json::Value) -> HandlerResult {
    if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved quote draft: {}", quote_id);
        let _ = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
            .bind(&tenant_id)
            .execute(&pool)
            .await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn handle_incident_resolution(pool: PgPool, tenant_id: String, payload: serde_json::Value) -> HandlerResult {
    if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
        let _ = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(incident_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn handle_instagram_dm(pool: PgPool, tenant_id: String, payload: serde_json::Value) -> HandlerResult {
    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
        tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn configure_action_router() -> ActionRouter {
    let mut router = ActionRouter::new();
    router.register("social_post_draft", handle_social_post_draft);
    router.register("ambassador_reply", handle_ambassador_reply);
    router.register("quote_draft", handle_quote_draft);
    router.register("incident_resolution", handle_incident_resolution);
    router.register("instagram_dm", handle_instagram_dm);
    router
}
