use crate::{polling, storage};
use std::time::Duration;

async fn scan(pool: &sqlx::PgPool, tenant: String) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    polling::bound_postgres_transaction(&mut tx).await?;
    let counts = storage::scan_postgres(&mut tx, &tenant).await?;
    tx.commit().await?;
    let _ = counts;
    Ok(())
}

#[tokio::test]
async fn locked_tenant_cannot_starve_later_tenants_or_erase_owner_decisions() {
    let url = crate::test_database_url();
    let setup = sqlx::PgPool::connect(&url).await.unwrap();
    sqlx::query("CREATE SCHEMA operations_fairness")
        .execute(&setup)
        .await
        .unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("SET search_path TO operations_fairness")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TABLE tenants(id TEXT PRIMARY KEY); INSERT INTO tenants VALUES('a'),('b');
CREATE TABLE raw_materials(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,name TEXT,current_quantity INTEGER,reorder_threshold INTEGER,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ);
CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,event_source TEXT,context_payload JSONB,proposed_action JSONB,lifecycle_state TEXT,created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);
INSERT INTO raw_materials VALUES('b-material','b','Recorded inventory',1,5,'2026-01-01','2026-01-01');
INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,lifecycle_state) VALUES
('a-pending','a','operations','{\"source_table\":\"raw_materials\",\"source_id\":\"removed\"}','PENDING_APPROVAL'),
('a-decided','a','operations','{\"source_table\":\"raw_materials\",\"source_id\":\"removed\"}','APPROVED');")
        .execute(&pool).await.unwrap();
    let mut lock = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM agent_feed_items WHERE id='a-pending' FOR UPDATE")
        .fetch_all(&mut *lock)
        .await
        .unwrap();

    let mut cursor = String::new();
    let page = vec!["a".into(), "b".into()];
    let cancelled = tokio::time::timeout(
        Duration::from_millis(100),
        polling::scan_tenants(&mut cursor, &page, 100, |tenant| scan(&pool, tenant)),
    )
    .await;
    assert!(
        cancelled.is_err(),
        "actual locked row must still be waiting at the outer cancellation"
    );
    assert_eq!(
        cursor, "a",
        "partial-page progress must survive outer cancellation"
    );
    let next: Vec<String> =
        sqlx::query_scalar("SELECT id FROM tenants WHERE id>$1 ORDER BY id LIMIT 100")
            .bind(&cursor)
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(next, ["b"]);
    let results =
        polling::scan_tenants(&mut cursor, &next, 100, |tenant| scan(&pool, tenant)).await;
    assert!(results[0].1.is_ok());
    assert!(
        cursor.is_empty(),
        "completed last page must wrap so failed tenant can be retried"
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_feed_items WHERE tenant_id='b'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        1
    );

    // Keep the competing transaction locked: the DB itself must enforce a
    // timeout before the global batch deadline, allowing b to be inspected.
    let bounded = tokio::time::timeout(
        Duration::from_secs(3),
        polling::scan_tenants(&mut cursor, &page, 100, |tenant| scan(&pool, tenant)),
    )
    .await
    .expect("a database lock must not consume the whole batch");
    assert!(bounded[0].1.is_err());
    assert!(bounded[1].1.is_ok());
    assert!(cursor.is_empty());
    lock.rollback().await.unwrap();

    let retry = polling::scan_tenants(&mut cursor, &page, 100, |tenant| scan(&pool, tenant)).await;
    assert!(retry.iter().all(|(_, result)| result.is_ok()));
    let states: Vec<(String, String)> = sqlx::query_as(
        "SELECT id,lifecycle_state FROM agent_feed_items WHERE tenant_id='a' ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        states,
        [
            ("a-decided".into(), "APPROVED".into()),
            ("a-pending".into(), "PAUSED".into())
        ]
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_feed_items WHERE tenant_id='b'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        1
    );
    pool.close().await;
    setup.close().await;
}

#[tokio::test]
async fn database_lock_timeout_releases_scan_before_the_batch_deadline() {
    let url = crate::test_database_url();
    let setup = sqlx::PgPool::connect(&url).await.unwrap();
    sqlx::query("CREATE SCHEMA operations_lock_budget")
        .execute(&setup)
        .await
        .unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("SET search_path TO operations_lock_budget")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TABLE raw_materials(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,name TEXT,current_quantity INTEGER,reorder_threshold INTEGER,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ);
CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,event_source TEXT,context_payload JSONB,proposed_action JSONB,lifecycle_state TEXT,created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);
INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,lifecycle_state) VALUES('locked','a','operations','{\"source_table\":\"raw_materials\",\"source_id\":\"removed\"}','PENDING_APPROVAL');")
        .execute(&pool).await.unwrap();
    let mut lock = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM agent_feed_items FOR UPDATE")
        .fetch_all(&mut *lock)
        .await
        .unwrap();
    let result = tokio::time::timeout(Duration::from_secs(3), scan(&pool, "a".into()))
        .await
        .expect("database lock timeout must bound a real locked scan");
    let error = result.expect_err("locked row cannot be updated");
    assert_eq!(
        error
            .as_database_error()
            .and_then(|error| error.code())
            .as_deref(),
        Some("55P03")
    );
    lock.rollback().await.unwrap();
    pool.close().await;
    setup.close().await;
}
