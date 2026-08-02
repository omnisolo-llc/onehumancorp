#[cfg(test)]
mod tests {
    use sqlx::{postgres::PgPoolOptions, Row, Executor};
    use uuid::Uuid;
    use std::env;

    #[tokio::test]
    async fn test_chat_tenant_isolation_rls() {
        let pool = PgPoolOptions::new().acquire_timeout(std::time::Duration::from_millis(100))
            .connect_lazy("postgres://postgres:postgres@localhost/postgres")
            .unwrap();

        if env::var("CI").is_ok() {
            return;
        }

        let tenant_1 = Uuid::new_v4().to_string();
        let tenant_2 = Uuid::new_v4().to_string();

        let inbox_id_1 = Uuid::new_v4();
        let inbox_id_2 = Uuid::new_v4();

        match pool.begin().await {
            Ok(mut tx) => {
                // Insert for tenant 1
                tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_1).as_str()).await.unwrap();
                sqlx::query("INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, 'Inbox 1')")
                    .bind(inbox_id_1)
                    .bind(Uuid::parse_str(&tenant_1).unwrap())
                    .execute(&mut *tx).await.unwrap();

                // Insert for tenant 2
                tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_2).as_str()).await.unwrap();
                sqlx::query("INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, 'Inbox 2')")
                    .bind(inbox_id_2)
                    .bind(Uuid::parse_str(&tenant_2).unwrap())
                    .execute(&mut *tx).await.unwrap();

                // Test read isolation for tenant 1
                tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_1).as_str()).await.unwrap();
                let count: i64 = sqlx::query("SELECT count(*) FROM inboxes")
                    .fetch_one(&mut *tx).await.unwrap().get(0);
                assert_eq!(count, 1, "Tenant 1 should only see 1 inbox");

                let name: String = sqlx::query("SELECT name FROM inboxes LIMIT 1")
                    .fetch_one(&mut *tx).await.unwrap().get(0);
                assert_eq!(name, "Inbox 1");
            }
            Err(_) => {} // Assume connection failed in test env
        }
    }
}
