//! Optional tax configuration must not abort an otherwise valid quote transaction
//! or use another tenant's host-wide credentials.
use sqlx::{Postgres, Transaction};

fn standalone_key(standalone: bool) -> Option<String> {
    if !standalone {
        return None;
    }
    std::env::var("TAXJAR_API_KEY")
        .ok()
        .filter(|value| !value.trim().is_empty())
}

pub(super) async fn load_quote_taxjar_key(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    standalone: bool,
) -> Result<Option<String>, sqlx::Error> {
    // Catching undefined-table after SELECT would leave PostgreSQL's transaction
    // aborted. Check optional-schema availability before touching the table.
    let exists: bool = sqlx::query_scalar("SELECT to_regclass('integrations') IS NOT NULL")
        .fetch_one(&mut **tx)
        .await?;
    if !exists {
        return Ok(standalone_key(standalone));
    }
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT api_token FROM integrations WHERE tenant_id = $1 AND provider_id = 'taxjar'",
    )
    .bind(tenant)
    .fetch_optional(&mut **tx)
    .await?;
    match row {
        // An explicitly empty/disabled connection must not fall back to env.
        Some((key,)) => Ok(key.filter(|value| !value.trim().is_empty())),
        None => Ok(standalone_key(standalone)),
    }
}

#[cfg(test)]
#[path = "quote_taxjar_test.rs"]
mod tests;
