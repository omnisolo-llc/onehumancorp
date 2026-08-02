#[cfg(test)]
mod tests {
    use sqlx::{PgPool, Executor, Row};
    use uuid::Uuid;
    use chrono::{DateTime, Utc};

    #[sqlx::test]
    async fn test_tenant_isolation_rls(pool: PgPool) -> sqlx::Result<()> {
        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();
        let inbox_id_a = Uuid::new_v4();

        // 1. Insert an inbox for tenant A. We must bypass RLS for setup, or set the context to tenant A.
        let mut tx = pool.begin().await?;

        tx.execute(format!("SET LOCAL app.current_tenant_id = '{}'", tenant_a).as_str()).await?;

        sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3)")
            .bind(inbox_id_a)
            .bind(tenant_a)
            .bind("Tenant A Inbox")
            .execute(&mut *tx)
            .await?;

        // 2. Verify Tenant A can see their own inbox
        let row = sqlx::query("SELECT COUNT(*) FROM chat_inboxes WHERE id = $1")
            .bind(inbox_id_a)
            .fetch_one(&mut *tx)
            .await?;

        let count: i64 = row.get(0);
        assert_eq!(count, 1, "Tenant A should see their own inbox");

        // 3. Switch context to Tenant B
        tx.execute(format!("SET LOCAL app.current_tenant_id = '{}'", tenant_b).as_str()).await?;

        // 4. Verify Tenant B cannot see Tenant A's inbox
        let row_b = sqlx::query("SELECT COUNT(*) FROM chat_inboxes WHERE id = $1")
            .bind(inbox_id_a)
            .fetch_one(&mut *tx)
            .await?;

        let count_b: i64 = row_b.get(0);
        assert_eq!(count_b, 0, "Tenant B MUST NOT see Tenant A's inbox due to RLS");

        tx.rollback().await?;
        Ok(())
    }
}
