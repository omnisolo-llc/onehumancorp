#[cfg(test)]
mod tests {
    use super::super::multi_currency_ledger::MultiCurrencyLedger;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    async fn setup_test_db() -> Arc<PgPool> {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        let pool = PgPool::connect(&db_url).await.unwrap();

        // Seed FX rates for testing
        sqlx::query("INSERT INTO ohc_fx_rates (id, from_currency, to_currency, rate) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
            .bind(Uuid::new_v4().to_string())
            .bind("USD")
            .bind("EUR")
            .bind(0.92)
            .execute(&pool)
            .await
            .unwrap();

        Arc::new(pool)
    }

    #[tokio::test]
    async fn test_record_transaction_online() {
        let pool = setup_test_db().await;
        let ledger = MultiCurrencyLedger::new(pool.clone());
        let tenant_id = "test_tenant";

        // Set context for RLS
        sqlx::query("SET app.current_tenant = 'test_tenant'").execute(&*pool).await.unwrap();

        let res = ledger.record_transaction(tenant_id, 10000, "USD", "EUR", None).await;
        assert!(res.is_ok());

        let entry_id = res.unwrap();
        let row: (i64, f64) = sqlx::query_as("SELECT settlement_amount, exchange_rate FROM ohc_multi_currency_ledger WHERE id = $1")
            .bind(entry_id)
            .fetch_one(&*pool)
            .await
            .unwrap();

        assert_eq!(row.0, 9200);
        assert_eq!(row.1, 0.92);
    }

    #[tokio::test]
    async fn test_record_transaction_offline_sync_with_safe_margin() {
        let pool = setup_test_db().await;
        let ledger = MultiCurrencyLedger::new(pool.clone());
        let tenant_id = "test_tenant";

        // Set context for RLS
        sqlx::query("SET app.current_tenant = 'test_tenant'").execute(&*pool).await.unwrap();

        // Offline rate was 0.91, current rate is 0.92
        let res = ledger.record_transaction(tenant_id, 10000, "USD", "EUR", Some(0.91)).await;
        assert!(res.is_ok());

        let entry_id = res.unwrap();
        let row: (i64, i64, bool) = sqlx::query_as("SELECT settlement_amount, safe_margin_absorbed, is_offline_sync FROM ohc_multi_currency_ledger WHERE id = $1")
            .bind(entry_id)
            .fetch_one(&*pool)
            .await
            .unwrap();

        assert_eq!(row.0, 9100);
        assert_eq!(row.1, 100); // 9200 - 9100 = 100 cents absorbed
        assert!(row.2);
    }

    #[tokio::test]
    async fn test_process_offline_batch() {
        let pool = setup_test_db().await;
        let ledger = MultiCurrencyLedger::new(pool.clone());
        let tenant_id = "test_tenant";

        sqlx::query("SET app.current_tenant = 'test_tenant'").execute(&*pool).await.unwrap();

        let batch = vec![
            (10000, "USD".to_string(), "EUR".to_string(), 0.91),
            (5000, "USD".to_string(), "EUR".to_string(), 0.90)
        ];

        let res = ledger.process_offline_batch(tenant_id, batch).await;
        assert!(res.is_ok());

        let ids = res.unwrap();
        assert_eq!(ids.len(), 2);
    }
}
