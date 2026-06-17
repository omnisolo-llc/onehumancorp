use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_quote_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved quote draft: {}", quote_id);
        sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
            .bind(tenant_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
