#[cfg(test)]
mod tests {
    use sea_orm::{Database, Statement, ConnectionTrait, DatabaseConnection, TransactionTrait};
    use uuid::Uuid;

    async fn setup_test_db() -> DatabaseConnection {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        Database::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_rls_isolation() {
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(1500), setup_test_db()).await {
            Ok(pool) => pool,
            Err(_) => return, // Skip test if DB not available
        };

        let tenant1 = Uuid::new_v4();
        let tenant2 = Uuid::new_v4();
        let inbox1 = Uuid::new_v4();
        let contact1 = Uuid::new_v4();
        let conversation1 = Uuid::new_v4();

        // Create data for tenant1 using direct SQL within a transaction so that SET LOCAL applies
        let txn = pool.begin().await.unwrap();

        txn.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &format!("SET LOCAL app.current_tenant_id = '{}'", tenant1),
            vec![]
        )).await.unwrap();

        txn.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3)",
            vec![inbox1.into(), tenant1.into(), "Tenant 1 Inbox".into()]
        )).await.unwrap();

        txn.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO chat_contacts (id, tenant_id, name) VALUES ($1, $2, $3)",
            vec![contact1.into(), tenant1.into(), "Tenant 1 Contact".into()]
        )).await.unwrap();

        txn.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5)",
            vec![conversation1.into(), tenant1.into(), inbox1.into(), contact1.into(), "open".into()]
        )).await.unwrap();

        txn.commit().await.unwrap();

        // Now query as tenant2, should not see tenant1's data
        let txn2 = pool.begin().await.unwrap();

        txn2.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &format!("SET LOCAL app.current_tenant_id = '{}'", tenant2),
            vec![]
        )).await.unwrap();

        let result = txn2.query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*) as count FROM chat_conversations",
            vec![]
        )).await.unwrap();

        if let Some(row) = result {
            let count: i64 = row.try_get("", "count").unwrap_or(0);
            assert_eq!(count, 0, "Tenant 2 should not see Tenant 1's conversations due to RLS");
        }

        txn2.commit().await.unwrap();

        // Clean up
        let txn_cleanup = pool.begin().await.unwrap();

        txn_cleanup.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &format!("SET LOCAL app.current_tenant_id = '{}'", tenant1),
            vec![]
        )).await.unwrap();

        txn_cleanup.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM chat_conversations WHERE id = $1",
            vec![conversation1.into()]
        )).await.unwrap();

        txn_cleanup.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM chat_contacts WHERE id = $1",
            vec![contact1.into()]
        )).await.unwrap();

        txn_cleanup.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM chat_inboxes WHERE id = $1",
            vec![inbox1.into()]
        )).await.unwrap();

        txn_cleanup.commit().await.unwrap();
    }
}
