use sqlx::PgPool;

pub async fn approve_quote(
    pool: &PgPool,
    tenant_id: &str,
    quote_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
        .bind(uuid::Uuid::parse_str(quote_id).unwrap_or_default())
        .bind(tenant_id)
        .execute(pool)
        .await?;
    Ok(())
}
