use sqlx::sqlite::SqlitePoolOptions;
use server_auth::sqlite_store::SqliteUserRepository;
use server_auth::User;
use server_auth::postgres_store::UserRepository;
use chrono::Utc;

#[tokio::test]
async fn test_sqlite_tenant_isolation_system_bypass_integration() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL DEFAULT '',
            roles TEXT DEFAULT '[]',
            active BOOLEAN DEFAULT TRUE,
            tenant_id TEXT,
            oidc_subject TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS revoked_tokens (
            jti TEXT PRIMARY KEY,
            tenant_id TEXT DEFAULT 'system',
            expires_at TIMESTAMP NOT NULL
        );
        "#
    ).execute(&pool).await.unwrap();

    let repo = SqliteUserRepository::new(pool.clone());

    // Create a user in tenant "tenant_a"
    let user_a = User {
        id: "user_a_id".to_string(),
        username: "user_a".to_string(),
        email: "user_a@example.com".to_string(),
        password_hash: "hash".to_string(),
        roles: vec![],
        active: true,
        organization_id: Some("tenant_a".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        oidc_subject: None,
    };
    repo.create_user(user_a, "tenant_a").await.unwrap();

    // Create a user in tenant "tenant_b"
    let user_b = User {
        id: "user_b_id".to_string(),
        username: "user_b".to_string(),
        email: "user_b@example.com".to_string(),
        password_hash: "hash".to_string(),
        roles: vec![],
        active: true,
        organization_id: Some("tenant_b".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        oidc_subject: None,
    };
    repo.create_user(user_b, "tenant_b").await.unwrap();

    // In a multi-tenant environment, querying with "system" should be rejected.
    let is_multitenant = ::server_config::get().multitenant;

    if is_multitenant {
        let res = repo.list_users("system").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("cannot be queried in multi-tenant mode"));

        let res = repo.get_by_id("user_a_id", "system").await;
        assert!(res.is_err());

        let res = repo.get_by_username("user_b", "system").await;
        assert!(res.is_err());
    } else {
        // If not running in multitenant mode, system queries pass
        let users = repo.list_users("system").await.unwrap();
        assert_eq!(users.len(), 2);
    }
}
