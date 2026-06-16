use sqlx::PgPool;

pub async fn mark_replied(
    pool: &PgPool,
    tenant_id: &str,
    inbox_id: &str,
    draft_reply: Option<&str>,
) -> Result<(), sqlx::Error> {
    if let Some(draft) = draft_reply {
        sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
