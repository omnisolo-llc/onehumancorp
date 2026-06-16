
use serde_json::Value;
use sqlx::PgPool;

pub async fn handle_ambassador_reply(tenant_id: String, payload: Value, pool: PgPool) -> Result<(), String> {
    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);

        sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
        sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Missing inbox_message_id".to_string())
    }
}

pub async fn handle_instagram_dm(tenant_id: String, payload: Value, pool: PgPool) -> Result<(), String> {
    if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
        tracing::info!("Approved instagram dm reply for inbox_id: {}", inbox_id);
        sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Missing inbox_message_id".to_string())
    }
}
