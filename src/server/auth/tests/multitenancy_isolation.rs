// use super::*;
use server_auth::postgres_store::{PgUserRepository, UserRepository};
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

    let repo = server_auth::postgres_store::PgUserRepository::new(pool.clone());

    // In Cloud multi-tenant mode, querying with org_id "system" must be rejected.
    let old_val = std::env::var("OHC_MULTITENANT").ok();
    unsafe { std::env::set_var("OHC_MULTITENANT", "true"); }

    let res: Result<_, _> = repo.get_by_email("dummy_id", "system").await;

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

    let repo = server_auth::postgres_store::PgUserRepository::new(pool.clone());

    let old_val = std::env::var("OHC_MULTITENANT").ok();
    unsafe { std::env::set_var("OHC_MULTITENANT", "false"); }

    let res: Result<_, _> = repo.get_by_email("dummy_id", "system").await;

    if let Some(val) = old_val {
        unsafe { std::env::set_var("OHC_MULTITENANT", val); }
    } else {
        unsafe { std::env::remove_var("OHC_MULTITENANT"); }
    }

    if let Err(e) = res {
        assert_ne!(e, "tenant_id 'system' cannot be queried in multi-tenant mode");
    }
}
