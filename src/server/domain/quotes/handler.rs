use sqlx::PgPool;
use tracing::info;

pub struct QuotesHandler;

impl QuotesHandler {
    pub async fn handle_quote_draft(
        pool: &PgPool,
        tenant_id: &str,
        quote_id: &str,
    ) -> Result<(), String> {
        info!("Approved quote draft: {}", quote_id);
        let _ = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
