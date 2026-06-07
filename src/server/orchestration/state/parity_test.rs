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
        if let Ok(url) = std::env::var("OHC_DATABASE_URL") {
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
            sqlite_db.execute_with_retry("insert_customer", || async { sqlx::query("INSERT INTO customers (id, tenant_id, email, name) VALUES (?, ?, ?, ?)")
                .bind(&customer_id)
                .bind(org_id)
                .bind("test@example.com")
                .bind("Test User")
                .execute(pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

            let email: String = sqlx::query_scalar("SELECT email FROM customers WHERE id = ?")
                .bind(&customer_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(email, "test@example.com");
        }

        if let Some(ref db) = pg_db {
            db.execute_with_retry("insert_customer", || async { sqlx::query("INSERT INTO customers (id, tenant_id, email, name) VALUES ($1, $2, $3, $4)")
                .bind(&customer_id)
                .bind(org_id)
                .bind("test@example.com")
                .bind("Test User")
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

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
            sqlite_db.execute_with_retry("insert_task", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, tenant_id) VALUES (?, ?, NULL, ?, 'default_tenant')")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .execute(pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

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
            db.execute_with_retry("insert_task", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, tenant_id) VALUES ($1, $2, NULL, $3, 'default_tenant')")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

            let parent_plan_id: Option<String> = sqlx::query_scalar("SELECT parent_plan_id FROM swarm_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(parent_plan_id, None);
        }
    }

    #[tokio::test]
    async fn test_parity_null_equivalence() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id1 = uuid::Uuid::new_v4().to_string();
        let task_id2 = uuid::Uuid::new_v4().to_string();
        let mission_id = "mission_12345";
        let title = "Test Null Title modified";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, tenant_id) VALUES (?, ?, NULL, ?, 'default_tenant')")
                .bind(&task_id1)
                .bind(mission_id)
                .bind(title)
                .execute(pool)
                .await
                .unwrap();

            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, tenant_id) VALUES (?, ?, 'plan_1', ?, 'default_tenant')")
                .bind(&task_id2)
                .bind(mission_id)
                .bind(title)
                .execute(pool)
                .await
                .unwrap();

            let null_tasks: Vec<String> = sqlx::query_scalar("SELECT id FROM swarm_tasks WHERE parent_plan_id IS NULL AND mission_id = ?")
                .bind(mission_id)
                .fetch_all(pool)
                .await
                .unwrap();
            assert_eq!(null_tasks.len(), 1);
            assert_eq!(null_tasks[0], task_id1);

            let non_null_tasks: Vec<String> = sqlx::query_scalar("SELECT id FROM swarm_tasks WHERE parent_plan_id = 'plan_1' AND mission_id = ?")
                .bind(mission_id)
                .fetch_all(pool)
                .await
                .unwrap();
            assert_eq!(non_null_tasks.len(), 1);
            assert_eq!(non_null_tasks[0], task_id2);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id1 = uuid::Uuid::parse_str(&task_id1).unwrap();
            let parsed_id2 = uuid::Uuid::parse_str(&task_id2).unwrap();

            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, tenant_id) VALUES ($1, $2, NULL, $3, 'default_tenant')")
                .bind(parsed_id1)
                .bind(mission_id)
                .bind(title)
                .execute(&db.pool)
                .await
                .unwrap();

            sqlx::query("INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, tenant_id) VALUES ($1, $2, 'plan_1', $3, 'default_tenant')")
                .bind(parsed_id2)
                .bind(mission_id)
                .bind(title)
                .execute(&db.pool)
                .await
                .unwrap();

            let null_tasks: Vec<uuid::Uuid> = sqlx::query_scalar("SELECT id FROM swarm_tasks WHERE parent_plan_id IS NULL AND mission_id = $1")
                .bind(mission_id)
                .fetch_all(&db.pool)
                .await
                .unwrap();
            assert_eq!(null_tasks.len(), 1);
            assert_eq!(null_tasks[0], parsed_id1);

            let non_null_tasks: Vec<uuid::Uuid> = sqlx::query_scalar("SELECT id FROM swarm_tasks WHERE parent_plan_id = 'plan_1' AND mission_id = $1")
                .bind(mission_id)
                .fetch_all(&db.pool)
                .await
                .unwrap();
            assert_eq!(non_null_tasks.len(), 1);
            assert_eq!(non_null_tasks[0], parsed_id2);
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
            sqlite_db.execute_with_retry("insert_task_tx", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, tenant_id) VALUES (?, ?, ?, \'PENDING\', 'default_tenant')")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .execute(pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

            let mut tx1 = pool.begin().await.unwrap();
            // In SQLite, an immediate transaction acquires a lock, we simulate updating.
            sqlx::query("UPDATE swarm_tasks SET status = 'IN_PROGRESS' WHERE id = ? AND status = 'PENDING'")
                .bind(&task_id)
                .execute(&mut *tx1)
                .await
                .unwrap();

            let pool_clone = pool.clone();
            let task_id_clone = task_id.clone();

            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            let tx_clone = tx.clone();

            // Spawn a task that tries to execute tx2. If it is blocked by tx1, it will hang.
            // If it fails immediately with a busy error, that is also valid.
            tokio::spawn(async move {
                let mut tx2 = pool_clone.begin().await.unwrap();
                let res = sqlx::query("UPDATE swarm_tasks SET status = 'COMPLETED' WHERE id = ? AND status = 'PENDING'")
                    .bind(&task_id_clone)
                    .execute(&mut *tx2)
                    .await;
                let _ = tx_clone.send(res).await;
            });

            // Use a deterministic try_recv loop to see if the second transaction can immediately complete.
            // If it's isolated properly, it will either block (rx empty) or return an Error (DatabaseBusy).
            // We give it a short bound (using task yields rather than time) to ensure we don't accidentally wait forever on success.
            let mut completed = false;
            for _ in 0..5 {
                tokio::task::yield_now().await;
                match rx.try_recv() {
                    Ok(Ok(_)) => {
                        completed = true;
                        break;
                    }
                    Ok(Err(_)) => {
                        // Received an error (like Database(Sqlite(Busy))), which proves isolation.
                        completed = false;
                        break;
                    }
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                        // Still blocked
                        continue;
                    }
                    Err(_) => break,
                }
            }

            assert!(!completed, "Second transaction should be blocked by isolation and not complete successfully while tx1 holds the lock");

            tx1.commit().await.unwrap();

            // Now tx2 should be able to finish (or it already failed, which is also fine).
            let _ = rx.recv().await;
        }

        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();
            db.execute_with_retry("insert_task_tx", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, tenant_id) VALUES ($1, $2, $3, \'PENDING\', 'default_tenant')")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

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

            let pool_clone = db.pool.clone();

            // For Postgres, we can try to acquire the lock with NOWAIT. If it errors out
            // rather than waiting, we know it's isolated properly. We don't need channel loops,
            // we can just await the task since NOWAIT guarantees it returns immediately with an error if locked.
            let join_handle = tokio::spawn(async move {
                let mut tx2 = pool_clone.begin().await.unwrap();
                let res = sqlx::query("SELECT status FROM swarm_tasks WHERE id = $1 FOR UPDATE NOWAIT")
                    .bind(parsed_id)
                    .fetch_optional(&mut *tx2)
                    .await;
                res
            });

            let res = join_handle.await.unwrap();
            assert!(res.is_err(), "Second transaction should be blocked by isolation in Postgres and fail with NOWAIT");

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
            sqlite_db.execute_with_retry("insert_task_tz", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, created_at, tenant_id) VALUES (?, ?, ?, \'PENDING\', ?, 'default_tenant')")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .bind(dt)
                .execute(pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

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
            db.execute_with_retry("insert_task_tz", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, created_at, tenant_id) VALUES ($1, $2, $3, \'PENDING\', $4, 'default_tenant')")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .bind(dt)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

            let row = sqlx::query("SELECT created_at FROM swarm_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            assert_eq!(created_at.timestamp(), dt.timestamp());
        }
    }

    #[tokio::test]
    async fn test_parity_timezones_edge_cases() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let mission_id = "mission_1234";
        let title = "Test Timezone Edge Case Title";

        use chrono::TimeZone;
        let dt = chrono::Utc.with_ymd_and_hms(2025, 2, 28, 15, 30, 45).unwrap();

        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlite_db.execute_with_retry("insert_task_tz", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, created_at, tenant_id) VALUES (?, ?, ?, \'PENDING\', ?, 'default_tenant')")
                .bind(&task_id)
                .bind(mission_id)
                .bind(title)
                .bind(dt)
                .execute(pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

            let row = sqlx::query("SELECT created_at FROM swarm_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            assert_eq!(created_at.timestamp(), dt.timestamp());
        }

        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();
            db.execute_with_retry("insert_task_tz", || async { sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, created_at, tenant_id) VALUES ($1, $2, $3, \'PENDING\', $4, 'default_tenant')")
                .bind(parsed_id)
                .bind(mission_id)
                .bind(title)
                .bind(dt)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string()) }).await.unwrap();

            let row = sqlx::query("SELECT created_at FROM swarm_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            assert_eq!(created_at.timestamp(), dt.timestamp());
        }
    }

    #[tokio::test]
    async fn test_department_tasks_parity() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = "tenant_parity";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload) VALUES (?, ?, 'ops', 'test', '{}')")
                .bind(&task_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .unwrap();

            let dept: String = sqlx::query_scalar("SELECT department FROM department_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(dept, "ops");
        }

        // Postgres
        if let Some(ref db) = pg_db {
            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload) VALUES ($1, $2, 'ops', 'test', '{}')")
                .bind(&task_id)
                .bind(tenant_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let dept: String = sqlx::query_scalar("SELECT department FROM department_tasks WHERE id = $1")
                .bind(&task_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(dept, "ops");
        }
    }

    #[tokio::test]
    async fn test_shared_tasks_parity() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let org_id = "org_parity";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO shared_tasks (id, organization_id, title) VALUES (?, ?, 'test_task')")
                .bind(&task_id)
                .bind(org_id)
                .execute(pool)
                .await
                .unwrap();

            let title: String = sqlx::query_scalar("SELECT title FROM shared_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(title, "test_task");
        }

        // Postgres
        if let Some(ref db) = pg_db {
            sqlx::query("INSERT INTO shared_tasks (id, organization_id, title) VALUES ($1, $2, 'test_task')")
                .bind(&task_id)
                .bind(org_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let title: String = sqlx::query_scalar("SELECT title FROM shared_tasks WHERE id = $1")
                .bind(&task_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(title, "test_task");
        }
    }


    #[tokio::test]
    async fn test_parity_jsonb_merging() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let org_id = "org_parity_jsonb";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload) VALUES (?, ?, 'ops', 'jsonb_test', '{\"original\": true}')")
                .bind(&task_id)
                .bind(org_id)
                .execute(pool)
                .await
                .unwrap();

            let reason = "Task failed due to error XYZ";
            let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap();

            sqlx::query("UPDATE department_tasks SET payload = json_patch(COALESCE(payload, '{}'), ?) WHERE id = ?")
                .bind(&payload_update)
                .bind(&task_id)
                .execute(pool)
                .await
                .unwrap();

            let payload_str: String = sqlx::query_scalar("SELECT payload FROM department_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();

            let parsed: serde_json::Value = serde_json::from_str(&payload_str).unwrap();
            assert_eq!(parsed["original"], true);
            assert_eq!(parsed["error"], reason);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();

            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload) VALUES ($1, $2, 'ops', 'jsonb_test', '{\"original\": true}'::jsonb)")
                .bind(parsed_id)
                .bind(org_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let reason = "Task failed due to error XYZ";
            let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap();

            sqlx::query("UPDATE department_tasks SET payload = COALESCE(payload::jsonb, '{}'::jsonb) || $2::jsonb WHERE id = $1")
                .bind(parsed_id)
                .bind(&payload_update)
                .execute(&db.pool)
                .await
                .unwrap();

            let payload_value: serde_json::Value = sqlx::query_scalar("SELECT payload FROM department_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();

            assert_eq!(payload_value["original"], true);
            assert_eq!(payload_value["error"], reason);
        }
    }

    #[tokio::test]
    async fn test_parity_delete_stale_sessions() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let now = chrono::Utc::now();
        let threshold = now - chrono::Duration::days(1);

        // SQLite
        let res = sqlite_db.delete_stale_sessions(threshold).await;
        assert!(res.is_ok());

        if let Some(pg) = pg_db {
            let res = pg.delete_stale_sessions(threshold).await;
            assert!(res.is_ok());
        }
    }



    #[tokio::test]
    async fn test_parity_pos_offline_transactions() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = "tenant_parity";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status) VALUES (?, ?, 'client_1', 1000, 'USD', '{}', 'PENDING')")
                .bind(&task_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .unwrap();

            let amount: i64 = sqlx::query_scalar("SELECT amount_cents FROM pos_offline_transactions WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(amount, 1000);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            sqlx::query("INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status) VALUES ($1, $2, 'client_1', 1000, 'USD', '{}'::jsonb, 'PENDING')")
                .bind(&task_id)
                .bind(tenant_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let amount: i64 = sqlx::query_scalar("SELECT amount_cents FROM pos_offline_transactions WHERE id = $1")
                .bind(&task_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(amount, 1000);
        }
    }

    #[tokio::test]
    async fn test_parity_task_dependencies() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let depends_on_task_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = "tenant_parity";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO task_dependencies (task_id, depends_on_task_id, tenant_id) VALUES (?, ?, ?)")
                .bind(&task_id)
                .bind(&depends_on_task_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .unwrap();

            let result_tenant_id: String = sqlx::query_scalar("SELECT tenant_id FROM task_dependencies WHERE task_id = ? AND depends_on_task_id = ?")
                .bind(&task_id)
                .bind(&depends_on_task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(result_tenant_id, tenant_id);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_task_id = uuid::Uuid::parse_str(&task_id).unwrap();
            let parsed_depends_on_task_id = uuid::Uuid::parse_str(&depends_on_task_id).unwrap();

            sqlx::query("INSERT INTO task_dependencies (task_id, depends_on_task_id, tenant_id) VALUES ($1, $2, $3)")
                .bind(parsed_task_id)
                .bind(parsed_depends_on_task_id)
                .bind(tenant_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let result_tenant_id: String = sqlx::query_scalar("SELECT tenant_id FROM task_dependencies WHERE task_id = $1 AND depends_on_task_id = $2")
                .bind(parsed_task_id)
                .bind(parsed_depends_on_task_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(result_tenant_id, tenant_id);
        }
    }

    #[tokio::test]
    async fn test_parity_delivery_zones() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let zone_id = uuid::Uuid::new_v4().to_string();
        let org_id = "org_parity";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO delivery_zones (id, organization_id, flat_fee_cents, min_order_value_cents) VALUES (?, ?, 500, 1500)")
                .bind(&zone_id)
                .bind(org_id)
                .execute(pool)
                .await
                .unwrap();

            let flat_fee: i64 = sqlx::query_scalar("SELECT flat_fee_cents FROM delivery_zones WHERE id = ?")
                .bind(&zone_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(flat_fee, 500);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&zone_id).unwrap();

            sqlx::query("INSERT INTO delivery_zones (id, organization_id, flat_fee_cents, min_order_value_cents) VALUES ($1, $2, 500, 1500)")
                .bind(parsed_id)
                .bind(org_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let flat_fee: i64 = sqlx::query_scalar("SELECT flat_fee_cents FROM delivery_zones WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(flat_fee, 500);
        }
    }

    #[tokio::test]
    async fn test_parity_route_plans() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let plan_id = uuid::Uuid::new_v4().to_string();
        let org_id = "org_parity";

        let date_str = "2024-01-01";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO route_plans (id, organization_id, delivery_date) VALUES (?, ?, ?)")
                .bind(&plan_id)
                .bind(org_id)
                .bind(date_str)
                .execute(pool)
                .await
                .unwrap();

            let result_org: String = sqlx::query_scalar("SELECT organization_id FROM route_plans WHERE id = ?")
                .bind(&plan_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(result_org, org_id);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&plan_id).unwrap();
            let parsed_date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();

            sqlx::query("INSERT INTO route_plans (id, organization_id, delivery_date) VALUES ($1, $2, $3)")
                .bind(parsed_id)
                .bind(org_id)
                .bind(parsed_date)
                .execute(&db.pool)
                .await
                .unwrap();

            let result_org: String = sqlx::query_scalar("SELECT organization_id FROM route_plans WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(result_org, org_id);
        }
    }

    #[tokio::test]
    async fn test_parity_delivery_tasks() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let task_id = uuid::Uuid::new_v4().to_string();
        let org_id = "org_parity";
        let order_id = "order_123";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO delivery_tasks (id, organization_id, order_id) VALUES (?, ?, ?)")
                .bind(&task_id)
                .bind(org_id)
                .bind(order_id)
                .execute(pool)
                .await
                .unwrap();

            let status: String = sqlx::query_scalar("SELECT status FROM delivery_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(status, "PENDING");
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&task_id).unwrap();

            sqlx::query("INSERT INTO delivery_tasks (id, organization_id, order_id) VALUES ($1, $2, $3)")
                .bind(parsed_id)
                .bind(org_id)
                .bind(order_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let status: String = sqlx::query_scalar("SELECT status FROM delivery_tasks WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(status, "PENDING");
        }
    }

    #[tokio::test]
    async fn test_parity_ohc_job_queue() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let job_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = "tenant_parity";
        let job_type = "test_job";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES (?, ?, ?, '{}')")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(job_type)
                .execute(pool)
                .await
                .unwrap();

            let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = ?")
                .bind(&job_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(status, "PENDING");
        }

        // Postgres
        if let Some(ref db) = pg_db {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, $3, '{}'::jsonb)")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(job_type)
                .execute(&db.pool)
                .await
                .unwrap();

            let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id = $1")
                .bind(&job_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(status, "PENDING");
        }
    }

    #[tokio::test]
    async fn test_parity_ohc_universal_ledger() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let ledger_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = "tenant_parity";
        let department = "finance";
        let action_type = "payment";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES (?, ?, ?, ?, '{}')")
                .bind(&ledger_id)
                .bind(tenant_id)
                .bind(department)
                .bind(action_type)
                .execute(pool)
                .await
                .unwrap();

            let dept: String = sqlx::query_scalar("SELECT department FROM ohc_universal_ledger WHERE id = ?")
                .bind(&ledger_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(dept, department);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ($1, $2, $3, $4, '{}'::jsonb)")
                .bind(&ledger_id)
                .bind(tenant_id)
                .bind(department)
                .bind(action_type)
                .execute(&db.pool)
                .await
                .unwrap();

            let dept: String = sqlx::query_scalar("SELECT department FROM ohc_universal_ledger WHERE id = $1")
                .bind(&ledger_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(dept, department);
        }
    }

    #[tokio::test]
    async fn test_parity_smart_pricing_policies() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let policy_id = uuid::Uuid::new_v4().to_string();
        let product_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = uuid::Uuid::new_v4().to_string();

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO smart_pricing_policies (id, tenant_id, product_id, min_margin_percent, auto_discount_trigger_days_stagnant, max_discount_percent) VALUES (?, ?, ?, 10.5, 30, 25.0)")
                .bind(&policy_id)
                .bind(&tenant_id)
                .bind(&product_id)
                .execute(pool)
                .await
                .unwrap();

            let margin: f64 = sqlx::query_scalar("SELECT min_margin_percent FROM smart_pricing_policies WHERE id = ?")
                .bind(&policy_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(margin, 10.5);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&policy_id).unwrap();
            let parsed_tenant_id = uuid::Uuid::parse_str(&tenant_id).unwrap();
            let parsed_product_id = uuid::Uuid::parse_str(&product_id).unwrap();

            sqlx::query("INSERT INTO smart_pricing_policies (id, tenant_id, product_id, min_margin_percent, auto_discount_trigger_days_stagnant, max_discount_percent) VALUES ($1, $2, $3, 10.5, 30, 25.0)")
                .bind(parsed_id)
                .bind(parsed_tenant_id)
                .bind(parsed_product_id)
                .execute(&db.pool)
                .await
                .unwrap();

            let margin: f64 = sqlx::query_scalar("SELECT min_margin_percent FROM smart_pricing_policies WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(margin, 10.5);
        }
    }

    #[tokio::test]
    async fn test_parity_active_discounts() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let discount_id = uuid::Uuid::new_v4().to_string();
        let product_id = uuid::Uuid::new_v4().to_string();
        let tenant_id = uuid::Uuid::new_v4().to_string();
        let date_str = "2024-01-01 00:00:00+00";

        // SQLite
        if let DbStore::Sqlite(pool) = &sqlite_db.store {
            sqlx::query("INSERT INTO active_discounts (id, tenant_id, product_id, discount_amount, expires_at) VALUES (?, ?, ?, 5.5, ?)")
                .bind(&discount_id)
                .bind(&tenant_id)
                .bind(&product_id)
                .bind(date_str)
                .execute(pool)
                .await
                .unwrap();

            let amount: f64 = sqlx::query_scalar("SELECT discount_amount FROM active_discounts WHERE id = ?")
                .bind(&discount_id)
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(amount, 5.5);
        }

        // Postgres
        if let Some(ref db) = pg_db {
            let parsed_id = uuid::Uuid::parse_str(&discount_id).unwrap();
            let parsed_tenant_id = uuid::Uuid::parse_str(&tenant_id).unwrap();
            let parsed_product_id = uuid::Uuid::parse_str(&product_id).unwrap();

            sqlx::query("INSERT INTO active_discounts (id, tenant_id, product_id, discount_amount, expires_at) VALUES ($1, $2, $3, 5.5, $4)")
                .bind(parsed_id)
                .bind(parsed_tenant_id)
                .bind(parsed_product_id)
                .bind(chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00+00:00").unwrap())
                .execute(&db.pool)
                .await
                .unwrap();

            let amount: f64 = sqlx::query_scalar("SELECT discount_amount FROM active_discounts WHERE id = $1")
                .bind(parsed_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            assert_eq!(amount, 5.5);
        }
    }


}
