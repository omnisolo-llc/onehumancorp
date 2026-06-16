use sqlx::PgPool;

pub struct SalesHandler;

impl SalesHandler {
    pub async fn handle_quote_draft(pool: &PgPool, tenant_id: &str, quote_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Approved quote draft: {}", quote_id);
        let _ = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
            .bind(tenant_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
