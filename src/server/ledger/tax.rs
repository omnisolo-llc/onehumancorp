use uuid::Uuid;
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub channel: String,
    pub amount: f64,
    pub tax_amount: f64,
    pub tax_region: String,
}

pub async fn record_transaction(pool: &PgPool, entry: &LedgerEntry) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO ledger_entries (id, tenant_id, channel, amount, tax_amount, tax_region)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(entry.id)
    .bind(entry.tenant_id)
    .bind(&entry.channel)
    .bind(entry.amount)
    .bind(entry.tax_amount)
    .bind(&entry.tax_region)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_tax_reserve(pool: &PgPool, tenant_id: Uuid, tax_amount: f64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO tax_reserves (tenant_id, total_reserved, last_updated)
        VALUES ($1, $2, NOW())
        ON CONFLICT (tenant_id) DO UPDATE
        SET total_reserved = tax_reserves.total_reserved + EXCLUDED.total_reserved,
            last_updated = NOW()
        "#,
    )
    .bind(tenant_id)
    .bind(tax_amount)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_tax_reserve(pool: &PgPool, tenant_id: Uuid) -> Result<f64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT total_reserved FROM tax_reserves WHERE tenant_id = $1
        "#,
    )
    .bind(tenant_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(r.get::<rust_decimal::Decimal, _>("total_reserved").try_into().unwrap_or(0.0)),
        None => Ok(0.0),
    }
}
