//! Actual existing auth writers, with synthetic identities and no provider calls.
use super::*;
use sea_orm::{ActiveModelTrait, Set};
use server_auth::user_repository::UserRepository;
fn registration_user(id: &str, tenant: &str) -> server_auth::User {
    let now = chrono::Utc::now();
    server_auth::User {
        id: id.into(),
        username: id.into(),
        email: format!("{id}@example.test"),
        password_hash: "unused-local-fixture".into(),
        roles: vec!["ADMIN".into()],
        active: true,
        organization_id: Some(tenant.into()),
        created_at: now,
        updated_at: now,
        oidc_subject: None,
    }
}
async fn exercise_registration_writers(
    repo: &server_auth::seaorm_store::SeaOrmAuthRepository,
    store: &DefinitionStore,
    tenant: &str,
) {
    use server_auth::seaorm_store::{RegistrationMode, entities};
    let now = chrono::Utc::now();
    repo.set_registration_mode(RegistrationMode::Open, "fixture-admin", now)
        .await
        .unwrap();
    entities::registration_ticket::ActiveModel {
        id: Set("public-test-ticket".into()),
        email: Set("ticket-user@example.test".into()),
        token_hash: Set("public-synthetic-ticket-only".into()),
        issued_at: Set(now),
        expires_at: Set(now + chrono::Duration::minutes(20)),
        consumed_at: Set(None),
        invitation_id: Set(None),
    }
    .insert(repo.connection())
    .await
    .unwrap();
    let ticket = repo
        .consume_ticket_and_create_user(
            "public-synthetic-ticket-only",
            now,
            registration_user("ticket-user", tenant),
        )
        .await
        .unwrap();
    let oidc = repo
        .create_oidc_user(
            registration_user("oidc-user", tenant),
            "synthetic-offline-provider",
            "https://issuer.example.test",
            "synthetic-subject",
        )
        .await
        .unwrap();
    for user in [ticket, oidc] {
        let current = repo.get_by_id(&user.id, tenant).await.unwrap();
        assert_eq!(current.roles, vec!["ADMIN"]);
        let receipt = store
            .publish(&owner(tenant, &current.id), &typed_publish())
            .await
            .unwrap();
        assert_eq!(receipt.status, "published");
        let mut revoked = current.clone();
        revoked.roles = vec!["STAFF".into()];
        repo.update_user(revoked, tenant).await.unwrap();
        assert!(matches!(
            store
                .publish(&owner(tenant, &current.id), &typed_publish())
                .await,
            Err(crate::agent_definitions::Error::Forbidden)
        ));
        repo.delete_user(&current.id, tenant).await.unwrap();
        assert!(matches!(
            store
                .operation(
                    &owner(tenant, &current.id),
                    Uuid::parse_str(&receipt.request_id).unwrap()
                )
                .await,
            Err(crate::agent_definitions::Error::Forbidden)
        ));
    }
}
#[tokio::test]
async fn sqlite_password_ticket_and_oidc_writers_maintain_current_authority() {
    let f = Fixture::sqlite().await;
    exercise_registration_writers(&f.auth.portable_repo().unwrap(), &f.store, "definition-a").await;
}
#[tokio::test]
async fn postgres_existing_global_registration_writers_maintain_current_authority() {
    let pg = PgFixture::new().await;
    // Existing global registration uses its designated privileged auth boundary.
    // Feature mutations themselves still run on the restricted application pool.
    let repository = server_auth::seaorm_store::SeaOrmAuthRepository::new(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pg.admin.clone()),
    );
    exercise_registration_writers(&repository, &pg.store, "definition-a").await;
    drop(repository);
    pg.close().await;
}
async fn setup_request(db: crate::db::DB) -> StatusCode {
    let app: Router = crate::setup::router(Arc::new(db));
    app.oneshot(Request::builder().method("POST").uri("/admin").header("authorization","Bearer public-local-definition-setup-token-at-least-thirty-two-bytes").header("content-type","application/json").body(Body::from(json!({"username":"setup-owner","email":"setup-owner@example.test","password":"public synthetic strong password","organizationId":"setup-tenant"}).to_string())).unwrap()).await.unwrap().status()
}
#[tokio::test]
async fn actual_sqlite_setup_writer_creates_a_usable_canonical_publisher() {
    let url = format!(
        "sqlite:file:definition_setup_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    );
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    sqlite_schema(&pool, &url).await;
    let status = setup_request(crate::db::DB {
        pool: crate::db::secure_pg_pool_options()
            .connect_lazy("postgres://postgres@127.0.0.1:1/unused")
            .unwrap(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    })
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let id: String = sqlx::query_scalar("SELECT id FROM users WHERE username='setup-owner'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let receipt = DefinitionStore::Sqlite(pool)
        .publish(&owner("setup-tenant", &id), &typed_publish())
        .await
        .unwrap();
    assert_eq!(receipt.status, "published");
}
#[tokio::test]
async fn actual_postgres_setup_writer_creates_a_usable_canonical_publisher() {
    let pg = PgFixture::new().await;
    assert_eq!(
        setup_request(crate::db::DB {
            pool: pg.admin.clone(),
            store: crate::db::DbStore::Postgres
        })
        .await,
        StatusCode::CREATED
    );
    let id: String = sqlx::query_scalar("SELECT id FROM users WHERE username='setup-owner'")
        .fetch_one(&pg.admin)
        .await
        .unwrap();
    assert_eq!(
        pg.store
            .publish(&owner("setup-tenant", &id), &typed_publish())
            .await
            .unwrap()
            .status,
        "published"
    );
    pg.close().await;
}
#[tokio::test]
async fn actual_cli_bootstrap_writer_preserves_explicit_admin_assignment() {
    let path = std::env::temp_dir().join(format!("ohc-authority-cli-{}.sqlite", Uuid::new_v4()));
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    sqlite_schema(&pool, &url).await;
    // The CLI bootstrap contract targets its existing org-1 namespace.
    sqlx::query("INSERT INTO tenants(id,name) VALUES('org-1','Existing organization')")
        .execute(&pool)
        .await
        .unwrap();
    let db = crate::persistence::AppDatabase::connect(&url)
        .await
        .unwrap();
    crate::command_bootstrap::bootstrap_admin(
        &db,
        "cli-owner@example.test",
        "public synthetic strong password",
    )
    .await
    .unwrap();
    let id: String =
        sqlx::query_scalar("SELECT id FROM users WHERE email='cli-owner@example.test'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        DefinitionStore::Sqlite(pool.clone())
            .publish(&owner("org-1", &id), &typed_publish())
            .await
            .unwrap()
            .status,
        "published"
    );
    db.connection().clone().close().await.unwrap();
    pool.close().await;
    drop(pool);
    std::fs::remove_file(path).unwrap();
}
#[tokio::test]
async fn legacy_sqlite_user_writer_cannot_override_canonical_revocation() {
    let f = Fixture::sqlite().await;
    let claims = f.auth.validate_token(&f.owner).await.unwrap();
    let repo = f.auth.portable_repo().unwrap();
    let mut user = repo.get_by_id(&claims.sub, "definition-a").await.unwrap();
    user.roles = vec!["STAFF".into()];
    repo.update_user(user.clone(), "definition-a")
        .await
        .unwrap();
    let legacy = server_auth::sqlite_store::SqliteUserRepository::new(f.sqlite.clone());
    user.roles = vec!["ADMIN".into()];
    legacy.update_user(user, "definition-a").await.unwrap();
    assert!(matches!(
        f.store
            .publish(&owner("definition-a", &claims.sub), &typed_publish())
            .await,
        Err(crate::agent_definitions::Error::Forbidden)
    ));
}
#[tokio::test]
async fn legacy_postgres_user_writer_cannot_override_canonical_revocation() {
    let pg = PgFixture::new().await;
    let canonical = server_auth::seaorm_store::SeaOrmAuthRepository::new(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pg.pool.clone()),
    );
    let mut user = canonical.get_by_id("owner", "pg-a").await.unwrap();
    user.roles = vec!["STAFF".into()];
    canonical.update_user(user.clone(), "pg-a").await.unwrap();
    let legacy = server_auth::postgres_store::PgUserRepository::new(pg.pool.clone());
    user.roles = vec!["ADMIN".into()];
    legacy.update_user(user, "pg-a").await.unwrap();
    assert!(matches!(
        pg.store
            .publish(&owner("pg-a", "owner"), &typed_publish())
            .await,
        Err(crate::agent_definitions::Error::Forbidden)
    ));
    drop(canonical);
    drop(legacy);
    pg.close().await;
}

#[tokio::test]
async fn actual_browser_growth_owner_cte_creates_current_authority_atomically() {
    let pg = PgFixture::new().await;
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/110_trial_extension_claim.sql"
    ))
    .execute(&pg.admin)
    .await
    .unwrap();
    let tenant = format!("e2e-growth-{}", Uuid::new_v4());
    let user = format!("e2e-growth-owner-{}", Uuid::new_v4());
    let email = format!("growth-{}@example.test", Uuid::new_v4());
    // Byte-exact SQL extracted from the maintained browser fixture. Only its
    // parameter values are synthetic; the writable CTE ordering is unchanged.
    let created: (String, String) = sqlx::query_as(include_str!("growth_owner.sql"))
        .bind(&tenant)
        .bind(&user)
        .bind(&email)
        .bind("user-a")
        .bind("pg-a")
        .fetch_one(&pg.admin)
        .await
        .unwrap();
    assert_eq!(created, (user.clone(), tenant.clone()));
    let canonical: String = sqlx::query_scalar(
        "SELECT role_name FROM identity_user_roles WHERE user_id=$1 AND tenant_id=$2",
    )
    .bind(&user)
    .bind(&tenant)
    .fetch_one(&pg.admin)
    .await
    .unwrap();
    assert_eq!(canonical, "ADMIN");
    let eligible: bool = sqlx::query_scalar("SELECT a.eligible FROM users u JOIN agent_definition_authorities a ON a.authority_key=u.marketplace_authority_key WHERE u.id=$1 AND u.tenant_id=$2")
        .bind(&user).bind(&tenant).fetch_one(&pg.admin).await.unwrap();
    assert!(
        eligible,
        "the single browser fixture statement must leave derived authority current"
    );
    let receipt = pg
        .store
        .publish(&owner(&tenant, &user), &typed_publish())
        .await
        .unwrap();
    assert_eq!(receipt.status, "published");
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1 AND tenant_id=$2")
        .bind(&user)
        .bind(&tenant)
        .execute(&pg.admin)
        .await
        .unwrap();
    assert!(matches!(
        pg.store
            .publish(&owner(&tenant, &user), &typed_publish())
            .await,
        Err(crate::agent_definitions::Error::Forbidden)
    ));
    pg.close().await;
}
