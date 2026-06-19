use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_booking_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(booking_id) = payload.get("booking_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved booking draft: {}", booking_id);

        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");

        // 1. Update booking status
        sqlx::query("UPDATE bookings SET status = 'pending_payment', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(booking_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        // 2. Also send the reply via inbox
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;
        }
    }
    Ok(())
}
