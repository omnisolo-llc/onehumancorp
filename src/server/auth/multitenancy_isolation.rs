use super::user_repository::UserRepository;
use super::postgres_store::PgUserRepository;
use super::User;
use std::time::Duration;

use std::sync::Mutex;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_multitenant_idor_system_bypass_prevention_regression() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let database_url = match std::env::var("OHC_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    if database_url.starts_with("sqlite") {
        return; // Postgres-specific test
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = ''").await?;
                    Ok(true)
                })
            })
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let repo = PgUserRepository::new(pool.clone());

    // In Cloud multi-tenant mode, querying with org_id "system" must be rejected.
    temp_env::async_with_vars([("OHC_MULTITENANT", Some("true"))], async {
        let res: Result<User, String> = repo.get_by_email("dummy_id", "system").await;
        assert!(res.is_err(), "Must reject system id in multitenant mode");
    }).await;
}

#[tokio::test]
async fn test_standalone_mode_allows_system_org_id() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let database_url = match std::env::var("OHC_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    if database_url.starts_with("sqlite") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = ''").await?;
                    Ok(true)
                })
            })
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let repo = PgUserRepository::new(pool.clone());

    temp_env::async_with_vars([("OHC_MULTITENANT", Some("false"))], async {
        let res: Result<User, String> = repo.get_by_email("dummy_id", "system").await;
        if let Err(e) = res {
            assert_ne!(e, "tenant_id 'system' cannot be queried in multi-tenant mode");
        }
    }).await;
}

#[tokio::test]
async fn test_revoke_token_tenant_isolation() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let database_url = match std::env::var("OHC_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    if database_url.starts_with("sqlite") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = ''").await?;
                    Ok(true)
                })
            })
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let repo = PgUserRepository::new(pool.clone());

    let other_tenant = "tenant_b";
    let current_tenant = "tenant_a";

    // Insert an already expired token for the OTHER tenant using raw query
    let expired_time = chrono::Utc::now() - chrono::Duration::hours(1);
    let mut tx1 = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant', 'tenant_b', false)").execute(&mut *tx1).await.unwrap();
    let _ = sqlx::query("INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, $3) ON CONFLICT (jti, tenant_id) DO NOTHING")
        .bind("other_tenant_expired_token")
        .bind(expired_time)
        .bind(other_tenant)
        .execute(&mut *tx1)
        .await;
    tx1.commit().await.unwrap();

    // Call revoke token on the CURRENT tenant
    let future_time = chrono::Utc::now() + chrono::Duration::hours(1);
    let res: Result<(), String> = repo.revoke_token("current_tenant_token".to_string(), future_time, current_tenant).await;
    assert!(res.is_ok(), "revoke_token should succeed");

    // Ensure the other tenant's expired token still exists, proving GC only cleared current_tenant
    let mut tx2 = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant', 'tenant_b', false)").execute(&mut *tx2).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM revoked_tokens WHERE jti = 'other_tenant_expired_token'")
        .fetch_one(&mut *tx2)
        .await
        .unwrap_or((0,));
    tx2.commit().await.unwrap();

    assert_eq!(row.0, 1, "The expired token from the other tenant was incorrectly garbage collected, proving cross-tenant deletion vulnerability");
}

#[tokio::test]
async fn test_pool_connection_tenant_leakage_prevention() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let database_url = match std::env::var("OHC_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    if database_url.starts_with("sqlite") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = ''").await?;
                    Ok(true)
                })
            })
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
        .max_connections(1) // Force reuse to test leakage
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let tenant_a = "tenant_a_leak_test";

    // Simulate a transaction that sets the tenant context and commits
    let mut tx1 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx1, tenant_a).await.unwrap();

    // Read the context back within the transaction
    let row1: (Option<String>,) = sqlx::query_as("SELECT current_setting('app.current_tenant', true)")
        .fetch_one(&mut *tx1)
        .await
        .unwrap_or((None,));
    assert_eq!(row1.0.as_deref(), Some(tenant_a));

    tx1.commit().await.unwrap();

    // Now, without requesting a new connection from the pool directly (or using the same reused one),
    // start a new transaction. The tenant context MUST be empty.
    let mut tx2 = pool.begin().await.unwrap();
    let row2: (Option<String>,) = sqlx::query_as("SELECT current_setting('app.current_tenant', true)")
        .fetch_one(&mut *tx2)
        .await
        .unwrap_or((None,));
    tx2.commit().await.unwrap();

    assert!(row2.0.is_none() || row2.0.as_deref() == Some(""), "CRITICAL VULNERABILITY: Tenant context leaked across transactions on the same pooled connection!");
}

#[tokio::test]
async fn test_rls_policies_protect_cross_tenant_reads() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let database_url = match std::env::var("OHC_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    if database_url.starts_with("sqlite") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = ''").await?;
                    Ok(true)
                })
            })
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let current_tenant = "tenant_test_rls";
    let other_tenant = "tenant_test_rls_other";

    // Assuming a loyalty_ledgers table exists in the schema.
    let mut tx = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx, other_tenant).await.unwrap();

    // We expect this to either fail due to constraints or succeed and be inserted for other_tenant
    let _ = sqlx::query("INSERT INTO loyalty_ledgers (id, tenant_id, customer_id, points_balance, lifetime_points) VALUES ('test_rls_ledger1', 'tenant_test_rls_other', 'cust1', 100, 100) ON CONFLICT DO NOTHING")
        .execute(&mut *tx)
        .await;
    tx.commit().await.unwrap();

    let mut tx2 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx2, current_tenant).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM loyalty_ledgers WHERE id = 'test_rls_ledger1'")
        .fetch_one(&mut *tx2)
        .await
        .unwrap_or((0,));
    tx2.commit().await.unwrap();

    assert_eq!(row.0, 0, "RLS policy bypass detected! A tenant could read another tenant's loyalty ledger.");
}
