pub async fn handle_ambassador_reply(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    feature_type: &str,
    payload: &serde_json::Value,
) -> Result<(), String> {
    if feature_type == "ambassador_reply" {
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);
            let res = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;
            if let Err(e) = res {
                tracing::error!("Failed to update inbox message reply status: {}", e);
            }
        }
    }

    if feature_type == "instagram_dm" || feature_type == "ambassador_reply" {
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
            let res = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;
            if let Err(e) = res {
                tracing::error!("Failed to update omni inbox message reply status: {}", e);
                return Err(e.to_string());
            }
        }
    }

    Ok(())
}
