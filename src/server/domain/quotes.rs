pub async fn handle_quote_draft(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    payload: &serde_json::Value,
) -> Result<(), String> {
    if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved quote draft: {}", quote_id);
        let res = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
            .bind(tenant_id)
            .execute(pool)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to update quote draft: {}", e);
            return Err(e.to_string());
        }
    }
    Ok(())
}
