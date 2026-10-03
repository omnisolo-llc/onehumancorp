//! Real canonical PostgreSQL schema and verified-registration transaction.
#![allow(dead_code)]
#[path = "../../src/server/persistence/capabilities.rs"]
mod capabilities;
#[path = "../../src/server/persistence/connection.rs"]
mod connection;
#[path = "../../src/server/persistence/entities.rs"]
mod entities;
#[path = "../../src/server/persistence/migration.rs"]
mod migration;
use sea_orm::ConnectionTrait;
use sqlx::{PgPool, Row, postgres::PgConnectOptions};
use std::sync::Arc;

fn owned_database(value: &str) -> PgConnectOptions {
    assert!(
        !value
            .chars()
            .any(|character| matches!(character, '?' | '#' | '%')),
        "owned database URL cannot contain overrides"
    );
    let options: PgConnectOptions = value.parse().expect("explicit PostgreSQL URL required");
    assert!(matches!(
        options.get_host(),
        "127.0.0.1" | "localhost" | "::1"
    ));
    assert_ne!(options.get_port(), 0);
    let database = options
        .get_database()
        .expect("explicit owned database required");
    assert!(
        database.starts_with("ohc_")
            && database.ends_with("_test")
            && database
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    );
    options
}

static ROLE_SETUP: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn fixture() -> (PgPool, connection::AppDatabase, String) {
    let database = std::env::var("OHC_REGISTRATION_TEST_DATABASE_URL")
        .expect("the PostgreSQL registration target requires an explicit owned loopback database");
    let schema = format!("registration_source_{}", uuid::Uuid::new_v4().simple());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect_with(owned_database(&database).options([("search_path", schema.as_str())]))
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    let role_guard = ROLE_SETUP.lock().await;
    let role: Option<(bool, bool, bool)> = sqlx::query_as(
        "SELECT rolcanlogin,rolbypassrls,rolsuper FROM pg_roles WHERE rolname='ohc_bypassrls'",
    )
    .fetch_optional(&pool)
    .await
    .unwrap();
    if let Some(attributes) = role {
        assert_eq!(attributes, (false, true, false));
    } else {
        sqlx::query("CREATE ROLE ohc_bypassrls NOLOGIN BYPASSRLS")
            .execute(&pool)
            .await
            .unwrap();
    }
    drop(role_guard);
    sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO ohc_bypassrls; ALTER DEFAULT PRIVILEGES IN SCHEMA {schema} GRANT SELECT,INSERT,UPDATE,DELETE ON TABLES TO ohc_bypassrls;"))
        .execute(&pool).await.unwrap();
    let initial = include_str!("../../src/server/migrations/001_initial.sql");
    for table in ["tenants", "users"] {
        let start = initial
            .find(&format!("CREATE TABLE IF NOT EXISTS {table} ("))
            .unwrap();
        let end = start + initial[start..].find(");").unwrap() + 2;
        sqlx::raw_sql(&initial[start..end])
            .execute(&pool)
            .await
            .unwrap();
    }
    // The portable migration installs the actual marketplace authority policy.
    // Preserve its canonical target rather than omitting that production step.
    let marketplace =
        include_str!("../../src/server/migrations/1018_agent_definition_marketplace.sql");
    let start = marketplace
        .find("CREATE TABLE agent_definitions (")
        .unwrap();
    let end = start + marketplace[start..].find(");").unwrap() + 2;
    sqlx::raw_sql(&marketplace[start..end])
        .execute(&pool)
        .await
        .unwrap();
    for table in ["tenants", "users"] {
        let start = initial
            .find(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY;"))
            .unwrap();
        let policy = &initial[start..];
        let end = policy.find(";\n\n").unwrap() + 1;
        sqlx::raw_sql(&policy[..end]).execute(&pool).await.unwrap();
        sqlx::query(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
            .execute(&pool)
            .await
            .unwrap();
    }
    let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());
    let db = connection::AppDatabase::from_connection(orm);
    migration::migrate(&db).await.unwrap();
    db.connection()
        .execute_unprepared(
            "UPDATE application_settings SET value='open' WHERE key='registration_mode'",
        )
        .await
        .unwrap();
    sqlx::query("INSERT INTO registration_tickets(id,email,token_hash,issued_at,expires_at) VALUES('pg-source-ticket','pg-source-owner@example.test','pg-source-ticket-hash',NOW(),NOW()+INTERVAL '1 hour')")
        .execute(&pool).await.unwrap();
    (pool, db, schema)
}

#[tokio::test]
async fn verified_registration_creates_its_real_tenant_under_the_canonical_foreign_key() {
    let (pool, db, schema) = fixture().await;
    let repository = Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(
        db.connection().clone(),
    ));
    let store = server_auth::Store::with_portable_repo(repository);
    let result = store
        .create_verified_user(
            "pg-source-owner".into(),
            "pg-source-owner@example.test".into(),
            "public test owned registration password".into(),
            vec![server_auth::ROLE_ADMIN.into()],
            "pg-source-ticket-hash",
        )
        .await;
    let rows = sqlx::query("SELECT u.id, u.tenant_id, t.tier, r.consumed_by_user_id, r.consumed_by_tenant_id FROM users u JOIN tenants t ON t.id=u.tenant_id JOIN registration_tickets r ON r.consumed_by_user_id=u.id WHERE u.email='pg-source-owner@example.test'")
        .fetch_all(&pool).await.unwrap();
    sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
    let created = result.expect("actual registration must create its fresh namespace before the canonical user foreign key is checked");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String, _>("id"), created.id);
    assert_eq!(rows[0].get::<String, _>("consumed_by_user_id"), created.id);
    assert_eq!(
        rows[0].get::<String, _>("consumed_by_tenant_id"),
        created.organization_id.clone().unwrap()
    );
    assert_eq!(
        rows[0].get::<String, _>("tenant_id"),
        created.organization_id.unwrap()
    );
    assert_eq!(rows[0].get::<String, _>("tier").to_lowercase(), "free");
}

async fn register(db: &connection::AppDatabase) -> Result<server_auth::User, String> {
    let repository = Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(
        db.connection().clone(),
    ));
    server_auth::Store::with_portable_repo(repository)
        .create_verified_user(
            "pg-source-owner".into(),
            "pg-source-owner@example.test".into(),
            "public test owned registration password".into(),
            vec![server_auth::ROLE_ADMIN.into()],
            "pg-source-ticket-hash",
        )
        .await
}
async fn cleanup(pool: PgPool, schema: &str) {
    sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
}

#[tokio::test]
async fn verified_oidc_registration_creates_its_actual_free_namespace() {
    let (pool, db, schema) = fixture().await;
    let repository = Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(
        db.connection().clone(),
    ));
    let store = server_auth::Store::with_portable_repo(repository);
    // This is the already-verified identity boundary; no provider or token verifier is replaced.
    let identity = server_common::Claims {
        sub: "owned-idp-subject".into(),
        exp: chrono::Utc::now().timestamp() + 3600,
        iat: chrono::Utc::now().timestamp(),
        organization_id: Some("untrusted-provider-tenant".into()),
        username: "pg-oidc-owner".into(),
        email: "pg-oidc-owner@example.test".into(),
        roles: vec![],
        session_id: None,
        jti: "owned-idp-token".into(),
    };
    let result = store
        .authenticate_oidc_identity("owned-idp", "https://idp.example.test", &identity)
        .await;
    let rows = sqlx::query("SELECT u.id,u.tenant_id,t.name,t.tier,e.subject FROM users u JOIN tenants t ON t.id=u.tenant_id JOIN external_identities e ON e.user_id=u.id").fetch_all(&pool).await.unwrap();
    cleanup(pool, &schema).await;
    let created =
        result.expect("verified OIDC registration must persist its server-created namespace");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String, _>("id"), created.id);
    assert_eq!(
        rows[0].get::<String, _>("tenant_id"),
        created.organization_id.unwrap()
    );
    assert_ne!(
        rows[0].get::<String, _>("tenant_id"),
        "untrusted-provider-tenant"
    );
    assert_eq!(rows[0].get::<String, _>("name"), "pg-oidc-owner");
    assert_eq!(rows[0].get::<String, _>("tier"), "free");
    assert_eq!(rows[0].get::<String, _>("subject"), "owned-idp-subject");
}

#[tokio::test]
async fn failed_source_receipt_rolls_back_the_namespace_and_all_registration_effects() {
    let (pool, db, schema) = fixture().await;
    sqlx::raw_sql("CREATE FUNCTION deny_registration_source() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'test-owned source write denied'; END $$; CREATE TRIGGER deny_registration_source BEFORE UPDATE OF consumed_by_user_id ON registration_tickets FOR EACH ROW EXECUTE FUNCTION deny_registration_source();").execute(&pool).await.unwrap();
    let result = register(&db).await;
    let mut counts = Vec::new();
    for table in [
        "tenants",
        "users",
        "identity_user_roles",
        "identity_email_claims",
        "agent_definition_authorities",
    ] {
        counts.push((
            table,
            sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) FROM {table}"))
                .fetch_one(&pool)
                .await
                .unwrap(),
        ));
    }
    let ticket: (Option<chrono::DateTime<chrono::Utc>>,Option<String>,Option<String>) = sqlx::query_as("SELECT consumed_at,consumed_by_user_id,consumed_by_tenant_id FROM registration_tickets WHERE id='pg-source-ticket'").fetch_one(&pool).await.unwrap();
    cleanup(pool, &schema).await;
    assert!(result.is_err());
    assert_eq!(ticket, (None, None, None));
    for (table, count) in counts {
        assert_eq!(count, 0, "{table} must roll back");
    }
}

#[tokio::test]
async fn invite_registration_preserves_the_existing_paid_namespace() {
    let (pool, db, schema) = fixture().await;
    sqlx::raw_sql("INSERT INTO tenants(id,name,tier) VALUES('paid-inviter-tenant','Existing business','business'); INSERT INTO users(id,username,email,tenant_id) VALUES('inviter','inviter','inviter@example.test','paid-inviter-tenant'); INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES('inviter','ADMIN','paid-inviter-tenant',0); UPDATE application_settings SET value='invite_only' WHERE key='registration_mode'; INSERT INTO registration_invitations(id,email,token_hash,created_at,expires_at,created_by) VALUES('paid-invitation','pg-source-owner@example.test','paid-invitation-hash',NOW(),NOW()+INTERVAL '1 hour','inviter'); UPDATE registration_tickets SET invitation_id='paid-invitation' WHERE id='pg-source-ticket';").execute(&pool).await.unwrap();
    let result = register(&db).await;
    let tenants: Vec<(String, String, String)> = sqlx::query_as("SELECT id,name,tier FROM tenants")
        .fetch_all(&pool)
        .await
        .unwrap();
    let ticket: (Option<String>,Option<String>) = sqlx::query_as("SELECT consumed_by_user_id,consumed_by_tenant_id FROM registration_tickets WHERE id='pg-source-ticket'").fetch_one(&pool).await.unwrap();
    cleanup(pool, &schema).await;
    let created = result.unwrap();
    assert_eq!(
        created.organization_id.as_deref(),
        Some("paid-inviter-tenant")
    );
    assert_eq!(
        tenants,
        vec![(
            "paid-inviter-tenant".into(),
            "Existing business".into(),
            "business".into()
        )]
    );
    assert_eq!(ticket, (Some(created.id), created.organization_id));
}

#[tokio::test]
async fn open_registration_cannot_adopt_or_overwrite_an_existing_namespace() {
    let (pool, db, schema) = fixture().await;
    sqlx::query(
        "INSERT INTO tenants(id,name,tier) VALUES('existing-paid','Existing business','pro')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let repository = server_auth::seaorm_store::SeaOrmAuthRepository::new(db.connection().clone());
    let now = chrono::Utc::now();
    let result = repository
        .consume_ticket_and_create_user(
            "pg-source-ticket-hash",
            now,
            server_auth::User {
                id: "collision-user".into(),
                username: "pg-source-owner".into(),
                email: "pg-source-owner@example.test".into(),
                password_hash: "unused-owned-fixture".into(),
                roles: vec![server_auth::ROLE_ADMIN.into()],
                active: true,
                organization_id: Some("existing-paid".into()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            },
        )
        .await;
    let tenants: Vec<(String, String, String)> = sqlx::query_as("SELECT id,name,tier FROM tenants")
        .fetch_all(&pool)
        .await
        .unwrap();
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    let consumed: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar(
        "SELECT consumed_at FROM registration_tickets WHERE id='pg-source-ticket'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    cleanup(pool, &schema).await;
    assert!(
        result.is_err(),
        "open registration must insert a fresh namespace, never join an existing one"
    );
    assert_eq!(
        tenants,
        vec![(
            "existing-paid".into(),
            "Existing business".into(),
            "pro".into()
        )]
    );
    assert_eq!(users, 0);
    assert_eq!(consumed, None);
}

#[tokio::test]
async fn consumed_ticket_retry_cannot_create_an_extra_namespace() {
    let (pool, db, schema) = fixture().await;
    let first = register(&db).await;
    let retry = register(&db).await;
    let counts:(i64,i64,i64)=sqlx::query_as("SELECT (SELECT COUNT(*) FROM tenants),(SELECT COUNT(*) FROM users),(SELECT COUNT(*) FROM registration_tickets WHERE consumed_by_user_id IS NOT NULL)").fetch_one(&pool).await.unwrap();
    cleanup(pool, &schema).await;
    assert!(first.is_ok());
    assert!(retry.is_err());
    assert_eq!(counts, (1, 1, 1));
}

#[tokio::test]
async fn failed_oidc_identity_receipt_rolls_back_the_new_namespace() {
    let (pool, db, schema) = fixture().await;
    sqlx::raw_sql("CREATE FUNCTION deny_external_identity() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'test-owned identity receipt denied'; END $$; CREATE TRIGGER deny_external_identity BEFORE INSERT ON external_identities FOR EACH ROW EXECUTE FUNCTION deny_external_identity();").execute(&pool).await.unwrap();
    let repository = server_auth::seaorm_store::SeaOrmAuthRepository::new(db.connection().clone());
    let now = chrono::Utc::now();
    let result = repository
        .create_oidc_user(
            server_auth::User {
                id: "oidc-rollback".into(),
                username: "oidc-rollback".into(),
                email: "oidc-rollback@example.test".into(),
                password_hash: "unused-owned-fixture".into(),
                roles: vec![server_auth::ROLE_ADMIN.into()],
                active: true,
                organization_id: Some("new-oidc-namespace".into()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            },
            "owned-idp",
            "https://idp.example.test",
            "owned-subject",
        )
        .await;
    let mut counts = Vec::new();
    for table in [
        "tenants",
        "users",
        "identity_email_claims",
        "identity_user_roles",
        "external_identities",
        "agent_definition_authorities",
    ] {
        counts.push((
            table,
            sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) FROM {table}"))
                .fetch_one(&pool)
                .await
                .unwrap(),
        ));
    }
    cleanup(pool, &schema).await;
    assert!(result.is_err());
    for (table, count) in counts {
        assert_eq!(count, 0, "{table} must roll back");
    }
}
