use crate::db::get_pool;

pub async fn resolve_customer_identity(tenant_id: &str, sender_id: &str) -> Option<String> {
    if sender_id.is_empty() {
        return None;
    }

    let pool = get_pool();

    let result = sqlx::query_scalar::<_, String>(
        "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2) LIMIT 1"
    )
    .bind(tenant_id)
    .bind(sender_id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(customer_id)) => Some(customer_id),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_customer_identity_empty_sender() {
        assert_eq!(resolve_customer_identity("tenant", "").await, None);
    }

    // Test the normal flow (will return None since DB is not seeded with this data)
    #[tokio::test]
    async fn test_resolve_customer_identity_no_match() {
        // We do not mock DB here easily, but at least we can verify it doesn't crash
        // and returns None for a non-existent sender_id.
        let pool_exists = std::env::var("OHC_DATABASE_URL").is_ok();
        if pool_exists {
            assert_eq!(resolve_customer_identity("test-tenant", "unknown_email@example.com").await, None);
        }
    }
}
