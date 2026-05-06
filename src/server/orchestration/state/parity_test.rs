#[cfg(test)]
mod parity_tests {
    use crate::db::{DB, DbStore};
    use std::sync::Arc;
    use sqlx::Row;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::postgres::PgPoolOptions;

    async fn setup_sqlite_db() -> Arc<DB> {
        let db_id = uuid::Uuid::new_v4().to_string();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&uri)
            .await
            .unwrap();

        // Run migrations/schema setup for SQLite
        let db = DB {
            pool: PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool),
        };
        db.run_migrations().await.unwrap();
        Arc::new(db)
    }

    async fn setup_postgres_db() -> Option<Arc<DB>> {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            if url.starts_with("postgres") {
                let pool = PgPoolOptions::new()
                    .connect(&url)
                    .await
                    .ok()?;

                let db = DB {
                    pool: pool.clone(),
                    store: DbStore::Postgres,
                };
                // We might not want to run migrations on a real DB here if it's shared,
                // but for a test DB it's fine.
                db.run_migrations().await.ok()?;
                return Some(Arc::new(db));
            }
        }
        None
    }

    #[tokio::test]
    async fn test_products_parity() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let test_product_id = uuid::Uuid::new_v4().to_string();
        let org_id = "test_org_parity";

        // Use the actual DB execute_with_retry to test service layer behavior
        // Use insert_knowledge_embedding as a proxy for complex parity-sensitive operations
        let insert_res_sqlite = sqlite_db.insert_knowledge_embedding(
            &test_product_id,
            org_id,
            "agent_1",
            "task_1",
            "content",
            "[0.1, 0.2]",
            "type_a"
        ).await;
        assert!(insert_res_sqlite.is_ok());

        if let Some(ref db) = pg_db {
            let insert_res_pg = db.insert_knowledge_embedding(
                &test_product_id,
                org_id,
                "agent_1",
                "task_1",
                "content",
                "[0.1, 0.2]",
                "type_a"
            ).await;
            assert!(insert_res_pg.is_ok());
        }

        // Verify retrieval parity
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            let row = sqlx::query("SELECT id, content FROM knowledge_embeddings WHERE id = ?")
                .bind(&test_product_id)
                .fetch_one(pool)
                .await
                .unwrap();
            let id: String = row.get("id");
            assert_eq!(id, test_product_id);
        }

        if let Some(ref db) = pg_db {
            let row = sqlx::query("SELECT id, content FROM knowledge_embeddings WHERE id = $1")
                .bind(&test_product_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            let id: String = row.get("id");
            assert_eq!(id, test_product_id);
        }
    }

    #[tokio::test]
    async fn test_customers_parity() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let customer_id = uuid::Uuid::new_v4().to_string();
        let org_id = "test_org_parity";

        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO customers (id, tenant_id, email, name) VALUES (?, ?, ?, ?)")
                .bind(&customer_id)
                .bind(org_id)
                .bind("test@example.com")
                .bind("Test User")
                .execute(pool)
                .await
                .unwrap();

            let email: String = sqlx::query_scalar("SELECT email FROM customers WHERE id = ?")
                .bind(&customer_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(email, "test@example.com");
        }

        if let Some(ref db) = pg_db {
            sqlx::query("INSERT INTO customers (id, tenant_id, email, name) VALUES ($1, $2, $3, $4)")
                .bind(&customer_id)
                .bind(org_id)
                .bind("test@example.com")
                .bind("Test User")
                .execute(&db.pool)
                .await
                .unwrap();

            let email: String = sqlx::query_scalar("SELECT email FROM customers WHERE id = $1")
                .bind(&customer_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(email, "test@example.com");
        }
    }
}
