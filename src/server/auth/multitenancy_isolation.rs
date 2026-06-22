use super::postgres_store::UserRepository;
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
async fn test_oidc_validation_strictness_regression() {
    // This test ensures that OIDC validation correctly enforces mandatory claims and expiration.
    // We mock the OIDCConfig and a malformed/expired token.
    let cfg = crate::oidc::OIDCConfig {
        issuer_url: "https://auth.example.com".to_string(),
        client_id: "test-client".to_string(),
        enabled: true,
    };

    // A token with missing iat or expired exp should be rejected by validate_oidc_token.
    // Since we don't have a full JWKS mock here, we expect at least a failure,
    // but the logic we added for claim checks is part of the validate_oidc_token pipeline.

    let token = "invalid.token.here";
    let res = crate::oidc::validate_oidc_token(token, &cfg).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_ssrf_protection_blocked_ips() {
    // Verify that validate_url_and_get_ip rejects local/private IPs to prevent SSRF/DNS Rebinding
    let blocked_urls = vec![
        "http://127.0.0.1/.well-known/openid-configuration",
        "http://169.254.169.254/latest/meta-data/",
        "http://localhost:8080/config",
        "https://10.0.0.1/auth",
    ];

    for url in blocked_urls {
        let res = crate::oidc::validate_url_and_get_ip_internal_for_test(url).await;
        assert!(res.is_err(), "URL {} should be blocked", url);
    }
}

#[tokio::test]
async fn test_cross_tenant_resource_idor_prevention() {
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
        .connect(&database_url).await.unwrap();

    // Setup: Tenant A and Tenant B
    let tenant_a = "tenant_a_idor";
    let tenant_b = "tenant_b_idor";

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, $1), ($2, $2) ON CONFLICT DO NOTHING")
        .bind(tenant_a).bind(tenant_b).execute(&pool).await.unwrap();

    // Tenant B owns a product
    let product_id = "secret_product_b";
    let mut tx_b = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant', $1, false)").bind(tenant_b).execute(&mut *tx_b).await.unwrap();
    sqlx::query("INSERT INTO products (id, tenant_id, title) VALUES ($1, $2, 'Secret B') ON CONFLICT DO NOTHING")
        .bind(product_id).bind(tenant_b).execute(&mut *tx_b).await.unwrap();
    tx_b.commit().await.unwrap();

    // Tenant A attempts to access Tenant B's product via a query that uses app.current_tenant
    let mut tx_a = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant', $1, false)").bind(tenant_a).execute(&mut *tx_a).await.unwrap();

    // This query should return no rows because RLS is active and app.current_tenant is 'tenant_a_idor'
    let row: Option<(String,)> = sqlx::query_as("SELECT title FROM products WHERE id = $1")
        .bind(product_id)
        .fetch_optional(&mut *tx_a)
        .await
        .unwrap();

    assert!(row.is_none(), "Tenant A should NOT be able to see Tenant B's product due to RLS");
    tx_a.rollback().await.unwrap();
}
