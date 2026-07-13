use super::User;
use super::postgres_store::PgUserRepository;
use super::postgres_test_support::postgres_security_pool;
use super::user_repository::UserRepository;

use std::sync::Mutex;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_multitenant_idor_system_bypass_prevention_regression() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let Some(pool) = postgres_security_pool(5).await else {
        return;
    };

    let repo = PgUserRepository::new(pool.clone());

    // In Cloud multi-tenant mode, querying with org_id "system" must be rejected.
    temp_env::async_with_vars([("OHC_MULTITENANT", Some("true"))], async {
        let res: Result<User, String> = repo.get_by_email("dummy_id", "system").await;
        assert_eq!(
            res.unwrap_err(),
            "tenant_id 'system' cannot be queried in multi-tenant mode"
        );
    })
    .await;
}

#[tokio::test]
async fn test_standalone_mode_allows_system_org_id() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let Some(pool) = postgres_security_pool(5).await else {
        return;
    };

    let repo = PgUserRepository::new(pool.clone());

    temp_env::async_with_vars([("OHC_MULTITENANT", Some("false"))], async {
        let mut role_tx = pool.begin().await.unwrap();
        ::server_common::auth_utils::set_org_context(&mut *role_tx, "system")
            .await
            .unwrap();
        let (session_user, current_user, bypasses_rls): (String, String, bool) = sqlx::query_as(
            "SELECT session_user::text, current_user::text, rolbypassrls FROM pg_roles WHERE rolname = current_user",
        )
        .fetch_one(&mut *role_tx)
        .await
        .unwrap();
        assert_eq!(session_user, "ohc_security_test");
        assert_eq!(current_user, "ohc_bypassrls");
        assert!(bypasses_rls, "system context must explicitly enter BYPASSRLS role");
        tracing::error!(
            "postgres security system context: session_user={session_user} current_user={current_user} rolbypassrls={bypasses_rls}"
        );
        role_tx.rollback().await.unwrap();

        let res: Result<User, String> = repo.get_by_email("dummy_id", "system").await;
        assert_eq!(res.unwrap_err(), "user not found");
    })
    .await;
}

#[tokio::test]
async fn test_revoke_token_tenant_isolation() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let Some(pool) = postgres_security_pool(5).await else {
        return;
    };

    let repo = PgUserRepository::new(pool.clone());

    let other_tenant = "tenant_b";
    let current_tenant = "tenant_a";

    let other_jti = "f10_revoke_other_tenant_expired";
    let current_jti = "f10_revoke_current_tenant";

    // Insert an already expired token for the OTHER tenant using raw query.
    let expired_time = chrono::Utc::now() - chrono::Duration::hours(1);
    let mut tx1 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx1, other_tenant)
        .await
        .unwrap();
    sqlx::query("DELETE FROM revoked_tokens WHERE jti = $1")
        .bind(other_jti)
        .execute(&mut *tx1)
        .await
        .unwrap();
    sqlx::query("INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, $3)")
        .bind(other_jti)
        .bind(expired_time)
        .bind(other_tenant)
        .execute(&mut *tx1)
        .await
        .unwrap();
    tx1.commit().await.unwrap();

    // Call revoke token on the CURRENT tenant
    let future_time = chrono::Utc::now() + chrono::Duration::hours(1);
    repo.revoke_token(current_jti.to_string(), future_time, current_tenant)
        .await
        .unwrap();

    // Ensure the other tenant's expired token still exists, proving GC only cleared current_tenant
    let mut tx2 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx2, other_tenant)
        .await
        .unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1")
        .bind(other_jti)
        .fetch_one(&mut *tx2)
        .await
        .unwrap();

    assert_eq!(
        row.0, 1,
        "The expired token from the other tenant was incorrectly garbage collected, proving cross-tenant deletion vulnerability"
    );

    sqlx::query("DELETE FROM revoked_tokens WHERE jti = $1")
        .bind(other_jti)
        .execute(&mut *tx2)
        .await
        .unwrap();
    tx2.commit().await.unwrap();

    let mut cleanup = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *cleanup, current_tenant)
        .await
        .unwrap();
    sqlx::query("DELETE FROM revoked_tokens WHERE jti = $1")
        .bind(current_jti)
        .execute(&mut *cleanup)
        .await
        .unwrap();
    cleanup.commit().await.unwrap();
}

#[tokio::test]
async fn test_pool_connection_tenant_leakage_prevention() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let Some(pool) = postgres_security_pool(1).await else {
        return;
    }; // Force reuse to test leakage.

    let tenant_a = "tenant_a_leak_test";

    // Simulate a transaction that sets the tenant context and commits
    let mut tx1 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx1, tenant_a)
        .await
        .unwrap();

    // Read the context back within the transaction
    let row1: (Option<String>,) =
        sqlx::query_as("SELECT current_setting('app.current_tenant', true)")
            .fetch_one(&mut *tx1)
            .await
            .unwrap();
    assert_eq!(row1.0.as_deref(), Some(tenant_a));

    tx1.commit().await.unwrap();

    // Now, without requesting a new connection from the pool directly (or using the same reused one),
    // start a new transaction. The tenant context MUST be empty.
    let mut tx2 = pool.begin().await.unwrap();
    let row2: (Option<String>,) =
        sqlx::query_as("SELECT current_setting('app.current_tenant', true)")
            .fetch_one(&mut *tx2)
            .await
            .unwrap();
    tx2.commit().await.unwrap();

    assert!(
        row2.0.is_none() || row2.0.as_deref() == Some(""),
        "CRITICAL VULNERABILITY: Tenant context leaked across transactions on the same pooled connection!"
    );
}

#[tokio::test]
async fn test_rls_policies_protect_cross_tenant_reads() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let Some(pool) = postgres_security_pool(5).await else {
        return;
    };

    let current_tenant = "tenant_test_rls";
    let other_tenant = "tenant_test_rls_other";

    let fixture_jti = "f10_rls_read_other_tenant_token";
    let mut tx = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx, other_tenant)
        .await
        .unwrap();
    sqlx::query("DELETE FROM revoked_tokens WHERE jti = $1")
        .bind(fixture_jti)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("INSERT INTO revoked_tokens (jti, tenant_id, expires_at) VALUES ($1, $2, $3)")
        .bind(fixture_jti)
        .bind(other_tenant)
        .bind(chrono::Utc::now() + chrono::Duration::hours(1))
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let mut tx2 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx2, current_tenant)
        .await
        .unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1")
        .bind(fixture_jti)
        .fetch_one(&mut *tx2)
        .await
        .unwrap();
    tx2.commit().await.unwrap();

    assert_eq!(
        row.0, 0,
        "RLS policy bypass detected! A tenant could read another tenant's revoked token."
    );

    let mut cleanup = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *cleanup, other_tenant)
        .await
        .unwrap();
    sqlx::query("DELETE FROM revoked_tokens WHERE jti = $1")
        .bind(fixture_jti)
        .execute(&mut *cleanup)
        .await
        .unwrap();
    cleanup.commit().await.unwrap();
}

#[tokio::test]
async fn test_rls_policies_protect_cross_tenant_writes() {
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let Some(pool) = postgres_security_pool(5).await else {
        return;
    };

    let tenant_1 = "f10_rls_write_tenant_1";
    let tenant_2 = "f10_rls_write_tenant_2";

    // Set context to tenant 1
    let mut tx1 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx1, tenant_1)
        .await
        .unwrap();

    // Try to insert a row for tenant 2 while context is tenant 1
    let insert_result = sqlx::query("INSERT INTO revoked_tokens (jti, tenant_id, expires_at) VALUES ('f10_rls_forbidden_write', $1, $2)")
        .bind(tenant_2)
        .bind(chrono::Utc::now() + chrono::Duration::hours(1))
        .execute(&mut *tx1)
        .await;

    match insert_result {
        Err(sqlx::Error::Database(db_error)) if db_error.code().as_deref() == Some("42501") => {}
        Err(error) => panic!("expected PostgreSQL RLS denial 42501, got {error:?}"),
        Ok(result) => panic!(
            "CRITICAL VULNERABILITY: RLS bypass inserted {} row(s) for another tenant",
            result.rows_affected()
        ),
    }

    tx1.rollback().await.unwrap();
}
