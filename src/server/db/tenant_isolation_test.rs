#[cfg(test)]
mod tenant_isolation_tests {
    use crate::db::DB;
    use sqlx::Row;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_postgresql_tenant_isolation_rls() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db = DB::new().await.unwrap();
        if db.is_sqlite() {
            return;
        }

        let tenant_a = Uuid::new_v4().to_string();
        let tenant_b = Uuid::new_v4().to_string();

        // 1. Setup tenants
        sqlx::query("INSERT INTO tenants (id, business_name, owner_email) VALUES ($1, $2, $3)")
            .bind(&tenant_a)
            .bind("Tenant A")
            .bind("a@example.com")
            .execute(&db.pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO tenants (id, business_name, owner_email) VALUES ($1, $2, $3)")
            .bind(&tenant_b)
            .bind("Tenant B")
            .bind("b@example.com")
            .execute(&db.pool)
            .await
            .unwrap();

        // 2. Insert data for Tenant A
        let mut tx = db.pool.begin().await.unwrap();
        sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_a))
            .execute(&mut *tx)
            .await
            .unwrap();

        sqlx::query("INSERT INTO products (tenant_id, title, type, price_cents) VALUES ($1, $2, $3, $4)")
            .bind(&tenant_a)
            .bind("Product A")
            .bind("physical")
            .bind(1000)
            .execute(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        // 3. Verify Tenant B cannot see Tenant A's data
        let mut tx = db.pool.begin().await.unwrap();
        sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_b))
            .execute(&mut *tx)
            .await
            .unwrap();

        let row = sqlx::query("SELECT count(*) FROM products")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        let count: i64 = row.get(0);
        assert_eq!(count, 0, "Tenant B should not see Tenant A's data");
        tx.commit().await.unwrap();

        // 4. Verify Tenant A can see their own data
        let mut tx = db.pool.begin().await.unwrap();
        sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_a))
            .execute(&mut *tx)
            .await
            .unwrap();

        let row = sqlx::query("SELECT count(*) FROM products")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        let count: i64 = row.get(0);
        assert_eq!(count, 1, "Tenant A should see their own data");
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_sqlite_composite_keys() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE test_composite (
                tenant_id TEXT NOT NULL,
                id TEXT NOT NULL,
                data TEXT,
                PRIMARY KEY (tenant_id, id)
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        let tid1 = "t1";
        let tid2 = "t2";
        let id = "shared_id";

        // Should allow same ID for different tenants
        sqlx::query("INSERT INTO test_composite (tenant_id, id, data) VALUES (?, ?, ?)")
            .bind(tid1)
            .bind(id)
            .bind("data1")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO test_composite (tenant_id, id, data) VALUES (?, ?, ?)")
            .bind(tid2)
            .bind(id)
            .bind("data2")
            .execute(&pool)
            .await
            .unwrap();

        // Should fail on same ID for same tenant
        let result = sqlx::query("INSERT INTO test_composite (tenant_id, id, data) VALUES (?, ?, ?)")
            .bind(tid1)
            .bind(id)
            .bind("data3")
            .execute(&pool)
            .await;

        assert!(result.is_err(), "Duplicate composite key should fail");
    }
}
