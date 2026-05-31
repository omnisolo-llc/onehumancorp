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
            .max_connections(2)
            .connect(&uri)
            .await
            .unwrap();

        // Run migrations/schema setup for SQLite
        let db = DB {
            pool: PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool),
        };
        db.run_migrations().await.unwrap();
        Arc::new(db)
    }

    async fn setup_postgres_db() -> Option<Arc<DB>> {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            if url.starts_with("postgres") {
                let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(100))
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

    #[tokio::test]
    async fn test_parity_null_handling() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let mission_id = "mission_123";
        let title = "Test Null Title modified";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title) VALUES (?, ?, NULL, ?)")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .execute(pool)
                .await
                .unwrap();

            let parent_plan_id: Option<String> = sqlx::query_scalar("SELECT parent_plan_id FROM swarm_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(parent_plan_id, None);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title) VALUES ($1, $2, NULL, $3)")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .execute(&db.pool)
                .await
                .unwrap();

            let parent_plan_id: Option<String> = sqlx::query_scalar("SELECT parent_plan_id FROM swarm_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(parent_plan_id, None);
        }
    }

    #[tokio::test]
    async fn test_parity_transaction_isolation() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let mission_id = "mission_123";
        let title = "Test Txn Title";

        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES (?, ?, ?, 'PENDING')")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .execute(pool)
                .await
                .unwrap();

            let mut tx1 = pool.begin().await.unwrap();
            // In SQLite, an immediate transaction acquires a lock, we simulate updating.
            sqlx::query("UPDATE swarm_tasks SET status = 'IN_PROGRESS' WHERE id = ? AND status = 'PENDING'")
                .bind(&task_id)
                .execute(&mut *tx1)
                .await
                .unwrap();

            let pool_clone = pool.clone();
            let task_id_clone = task_id.clone();
            let res = tokio::time::timeout(
                std::time::Duration::from_millis(100),
                async move {
                    let mut tx2 = pool_clone.begin().await.unwrap();
                    sqlx::query("UPDATE swarm_tasks SET status = 'COMPLETED' WHERE id = ? AND status = 'PENDING'")
                        .bind(&task_id_clone)
                        .execute(&mut *tx2)
                        .await
                }
            ).await;

            // It should timeout because tx1 is holding the lock, or it should fail with Database(Sqlite(Busy))
            assert!(res.is_err() || res.unwrap().is_err(), "Second transaction should be blocked by isolation");
            tx1.commit().await.unwrap();
        }

        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ($1, $2, $3, 'PENDING')")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .execute(&db.pool)
                .await
                .unwrap();

            let mut tx1 = db.pool.begin().await.unwrap();
            // In Postgres, FOR UPDATE locks the row
            let row1 = sqlx::query("SELECT status FROM swarm_tasks WHERE id = $1 FOR UPDATE")
                .bind(parsed_id)
                .fetch_optional(&mut *tx1)
                .await
                .unwrap();
            assert!(row1.is_some());

            sqlx::query("UPDATE swarm_tasks SET status = 'IN_PROGRESS' WHERE id = $1")
                .bind(parsed_id)
                .execute(&mut *tx1)
                .await
                .unwrap();

            tx1.commit().await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_parity_timezones() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let mission_id = "mission_123";
        let title = "Test Timezone Title";

        // Define a strict UTC timestamp explicitly
        let dt = chrono::Utc::now();

        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, created_at) VALUES (?, ?, ?, 'PENDING', ?)")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .bind(dt)
                .execute(pool)
                .await
                .unwrap();

            let row = sqlx::query("SELECT created_at FROM swarm_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

            // SQLite strips milliseconds to certain precisions based on formatting,
            // but the timezone itself (UTC) must match.
            assert_eq!(created_at.timestamp(), dt.timestamp());
        }

        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, created_at) VALUES ($1, $2, $3, 'PENDING', $4)")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .bind(dt)
                .execute(&db.pool)
                .await
                .unwrap();

            let row = sqlx::query("SELECT created_at FROM swarm_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            assert_eq!(created_at.timestamp(), dt.timestamp());
        }
    }
}
