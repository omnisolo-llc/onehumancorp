#[cfg(test)]
mod tests {
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Executor;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_booking_logic() {
        if std::env::var("DATABASE_URL").is_err() || std::env::var("DATABASE_URL").unwrap_or_default().contains("localhost") { return; }

        let pool = PgPoolOptions::new()
            .after_release(|conn, _| {
                Box::pin(async move {
                    let _ = conn.execute("RESET app.current_tenant").await;
                    let _ = conn.execute("RESET ROLE").await;
                    Ok(true)
                })
            })
            .connect("postgres://postgres:postgres@localhost/postgres").await.unwrap();

        let db = std::sync::Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let service = crate::services::booking::service::BookingService::new(db);
        let org_id = Uuid::new_v4().to_string();

        let mut tx = pool.begin().await.unwrap();
        tx.execute(format!("SET LOCAL app.current_tenant = '{}'", org_id).as_str()).await.unwrap();
        let _ = sqlx::query("INSERT INTO tenants (tenant_id, owner_id) VALUES ($1, 'test_owner') ON CONFLICT DO NOTHING").bind(Uuid::parse_str(&org_id).unwrap()).execute(&mut *tx).await;
        tx.commit().await.unwrap();

        let mut tx = pool.begin().await.unwrap();
        tx.execute(format!("SET LOCAL app.current_tenant = '{}'", org_id).as_str()).await.unwrap();

        // Create draft
        let quote = service.draft_quote(&org_id, "cust-1", 15000, "Fix sink").await.unwrap();
        assert_eq!(quote.status, "DRAFT");

        // Approve
        let approved = service.approve_quote(&org_id, &quote.id).await.unwrap();
        assert_eq!(approved.status, "APPROVED");

        // Book
        let booking = service.create_booking(&org_id, "cust-1", Some(approved.id.clone()), 1000, 2000).await.unwrap();
        assert_eq!(booking.status, "PENDING");
        assert!(booking.payment_link.is_some());

        // Double book error
        let err = service.create_booking(&org_id, "cust-2", None, 1500, 2500).await;
        assert!(err.is_err());
    }
}
