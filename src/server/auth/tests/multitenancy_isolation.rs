use crate::postgres_store::UserRepository;
use crate::postgres_store::PgUserRepository;
use crate::User;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
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

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let repo = PgUserRepository::new(pool.clone());

    // In Cloud multi-tenant mode, querying with org_id "system" must be rejected.
    let old_val = std::env::var("OHC_MULTITENANT").ok();
    unsafe { std::env::set_var("OHC_MULTITENANT", "true"); }

    let res: Result<User, String> = repo.get_by_email("dummy_id", "system").await;

    if let Some(val) = old_val {
        unsafe { std::env::set_var("OHC_MULTITENANT", val); }
    } else {
        unsafe { std::env::remove_var("OHC_MULTITENANT"); }
    }

    assert!(res.is_err(), "Must reject system id in multitenant mode");
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

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let repo = PgUserRepository::new(pool.clone());

    let old_val = std::env::var("OHC_MULTITENANT").ok();
    unsafe { std::env::set_var("OHC_MULTITENANT", "false"); }

    let res: Result<User, String> = repo.get_by_email("dummy_id", "system").await;

    if let Some(val) = old_val {
        unsafe { std::env::set_var("OHC_MULTITENANT", val); }
    } else {
        unsafe { std::env::remove_var("OHC_MULTITENANT"); }
    }

    if let Err(e) = res {
        assert_ne!(e, "tenant_id 'system' cannot be queried in multi-tenant mode");
    }
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

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy(&database_url)
        .unwrap();

    let repo = PgUserRepository::new(pool.clone());

    let other_tenant = "tenant_b";
    let current_tenant = "tenant_a";

    // Insert an already expired token for the OTHER tenant using raw query
    let expired_time = chrono::Utc::now() - chrono::Duration::hours(1);
    let _ = sqlx::query("INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, $3) ON CONFLICT (jti) DO NOTHING")
        .bind("other_tenant_expired_token")
        .bind(expired_time)
        .bind(other_tenant)
        .execute(&pool)
        .await;

    // Call revoke token on the CURRENT tenant
    let future_time = chrono::Utc::now() + chrono::Duration::hours(1);
    let res: Result<(), String> = repo.revoke_token("current_tenant_token".to_string(), future_time, current_tenant).await;
    assert!(res.is_ok(), "revoke_token should succeed");

    // Ensure the other tenant's expired token still exists, proving GC only cleared current_tenant
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM revoked_tokens WHERE jti = 'other_tenant_expired_token'")
        .fetch_one(&pool)
        .await
        .unwrap_or((0,));

    assert_eq!(row.0, 1, "The expired token from the other tenant was incorrectly garbage collected, proving cross-tenant deletion vulnerability");
}
