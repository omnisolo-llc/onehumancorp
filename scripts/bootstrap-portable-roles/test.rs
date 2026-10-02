use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use sea_orm::{ConnectionTrait, Schema};
use server_auth::user_repository::UserRepository;
use std::sync::Arc;
use tower::ServiceExt;

const TOKEN: &str = "public-local-setup-token-at-least-thirty-two-bytes";
const PASSWORD: &str = "correct horse battery staple";

struct Fixture {
    db: Arc<db::DB>,
    reader: sea_orm::DatabaseConnection,
    admin_orm: sea_orm::DatabaseConnection,
    postgres: Option<(String, String, bool)>,
}
async fn identity_tables(connection: &sea_orm::DatabaseConnection) {
    use server_auth::seaorm_store::entities;
    let backend = connection.get_database_backend();
    let schema = Schema::new(backend);
    for table in [
        schema.create_table_from_entity(entities::identity_user_role::Entity),
        schema.create_table_from_entity(entities::identity_email_claim::Entity),
    ] {
        connection.execute(backend.build(&table)).await.unwrap();
    }
}
impl Fixture {
    async fn postgres() -> Self {
        let url = std::env::var("OHC_SETUP_TEST_DATABASE_URL").expect("owned PostgreSQL required");
        let schema = format!("bootstrap_{}", uuid::Uuid::new_v4().simple());
        let role = format!("bootstrap_reader_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        let admin = db::secure_pg_pool_options()
            .max_connections(3)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("postgres-schema.sql"))
            .execute(&admin)
            .await
            .unwrap();
        let admin_orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(admin.clone());
        identity_tables(&admin_orm).await;
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='ohc_bypassrls')",
        )
        .fetch_one(&admin)
        .await
        .unwrap();
        if !exists {
            sqlx::query("CREATE ROLE ohc_bypassrls NOLOGIN BYPASSRLS")
                .execute(&admin)
                .await
                .unwrap();
        }
        let bypass: bool =
            sqlx::query_scalar("SELECT rolbypassrls FROM pg_roles WHERE rolname='ohc_bypassrls'")
                .fetch_one(&admin)
                .await
                .unwrap();
        assert!(
            bypass,
            "Production setup requires the designated migration role"
        );
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'")).execute(&admin).await.unwrap();
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO ohc_bypassrls,{role}; GRANT ALL ON ALL TABLES IN SCHEMA {schema} TO ohc_bypassrls; GRANT SELECT ON users,identity_user_roles,revoked_tokens TO {role};")).execute(&admin).await.unwrap();
        for table in ["users", "identity_user_roles"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY tenant_isolation ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap();
        }
        let scoped = db::secure_pg_pool_options()
            .max_connections(1)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let identity:(String,bool,bool)=sqlx::query_as("SELECT current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(identity, (role.clone(), false, false));
        let reader = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(scoped);
        Self {
            db: Arc::new(db::DB {
                pool: admin,
                store: db::DbStore::Postgres,
            }),
            reader,
            admin_orm,
            postgres: Some((schema, role, !exists)),
        }
    }
    async fn sqlite(normalized: bool) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let orm = sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        let schema = Schema::new(sea_orm::DatabaseBackend::Sqlite);
        orm.execute(sea_orm::DatabaseBackend::Sqlite.build(
            &schema.create_table_from_entity(server_auth::seaorm_store::entities::user::Entity),
        ))
        .await
        .unwrap();
        sqlx::raw_sql("ALTER TABLE users ADD COLUMN roles TEXT NOT NULL DEFAULT '[]'; CREATE TABLE tenants(id TEXT PRIMARY KEY,name TEXT NOT NULL);").execute(&pool).await.unwrap();
        if normalized {
            identity_tables(&orm).await;
        }
        let dummy = db::secure_pg_pool_options()
            .connect_lazy("postgres://localhost/unused")
            .unwrap();
        Self {
            db: Arc::new(db::DB {
                pool: dummy,
                store: db::DbStore::Sqlite(pool),
            }),
            reader: orm.clone(),
            admin_orm: orm,
            postgres: None,
        }
    }
    async fn setup(&self, token: Option<&str>) -> (StatusCode, serde_json::Value) {
        let mut request = Request::builder()
            .method("POST")
            .uri("/admin")
            .header("content-type", "application/json");
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        let body = serde_json::json!({"username":"bootstrap-admin","email":"admin@example.test","password":PASSWORD,"organizationId":"tenant-bootstrap"});
        let response = setup::router::<()>(self.db.clone())
            .oneshot(request.body(Body::from(body.to_string())).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 10_000).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap())
    }
    async fn user(&self) -> server_auth::User {
        server_auth::seaorm_store::SeaOrmAuthRepository::new(self.reader.clone())
            .get_by_username("bootstrap-admin", "tenant-bootstrap")
            .await
            .unwrap()
    }
    async fn count(&self, table: &str) -> i64 {
        let result = self
            .admin_orm
            .query_one(sea_orm::Statement::from_string(
                self.admin_orm.get_database_backend(),
                format!("SELECT COUNT(*) AS n FROM {table}"),
            ))
            .await
            .unwrap()
            .unwrap();
        result.try_get("", "n").unwrap()
    }
    async fn reject_roles(&self) {
        let sql = if self.postgres.is_some() {
            "ALTER TABLE identity_user_roles ADD CONSTRAINT reject_fixture_admin CHECK(role_name <> 'ADMIN')"
        } else {
            "CREATE TRIGGER reject_fixture_admin BEFORE INSERT ON identity_user_roles BEGIN SELECT RAISE(ABORT,'fixture rejects role'); END"
        };
        self.admin_orm
            .execute(sea_orm::Statement::from_string(
                self.admin_orm.get_database_backend(),
                sql,
            ))
            .await
            .unwrap();
    }
    async fn finish(self) {
        self.reader.close().await.unwrap();
        if let Some((schema, role, created_bypass)) = self.postgres {
            sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
                .execute(&self.db.pool)
                .await
                .unwrap();
            sqlx::query(&format!("DROP ROLE {role}"))
                .execute(&self.db.pool)
                .await
                .unwrap();
            if created_bypass {
                sqlx::query("DROP ROLE ohc_bypassrls")
                    .execute(&self.db.pool)
                    .await
                    .unwrap();
            }
        }
        self.admin_orm.close().await.unwrap();
        self.db.pool.close().await;
    }
}

#[tokio::test]
async fn postgres_bootstrapped_admin_retains_role_through_portable_reader_under_rls() {
    let f = Fixture::postgres().await;
    let status = f.setup(Some(TOKEN)).await.0;
    let user = f.user().await;
    let count = f.count("identity_user_roles").await;
    f.finish().await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(user.roles, vec![server_auth::ROLE_ADMIN.to_string()]);
    assert_eq!(count, 1);
    assert!(bcrypt::verify(PASSWORD, &user.password_hash).unwrap());
    assert_eq!(user.organization_id.as_deref(), Some("tenant-bootstrap"));
}
#[tokio::test]
async fn sqlite_bootstrapped_admin_retains_role_through_portable_reader() {
    let f = Fixture::sqlite(true).await;
    let status = f.setup(Some(TOKEN)).await.0;
    let user = f.user().await;
    let count = f.count("identity_user_roles").await;
    f.finish().await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(user.roles, vec![server_auth::ROLE_ADMIN.to_string()]);
    assert_eq!(count, 1);
}
#[tokio::test]
async fn postgres_role_write_failure_rolls_back_the_complete_bootstrap() {
    let f = Fixture::postgres().await;
    f.reject_roles().await;
    let result = f.setup(Some(TOKEN)).await;
    let counts = [
        f.count("users").await,
        f.count("tenants").await,
        f.count("identity_email_claims").await,
    ];
    f.finish().await;
    assert_eq!(result.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(counts, [0, 0, 0]);
}
#[tokio::test]
async fn sqlite_role_write_failure_rolls_back_the_complete_bootstrap() {
    let f = Fixture::sqlite(true).await;
    f.reject_roles().await;
    let result = f.setup(Some(TOKEN)).await;
    let counts = [
        f.count("users").await,
        f.count("tenants").await,
        f.count("identity_email_claims").await,
    ];
    f.finish().await;
    assert_eq!(result.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(counts, [0, 0, 0]);
}
#[tokio::test]
async fn concurrent_postgres_bootstrap_creates_exactly_one_portable_admin() {
    let f = Fixture::postgres().await;
    let (a, b) = tokio::join!(f.setup(Some(TOKEN)), f.setup(Some(TOKEN)));
    let user = f.user().await;
    let counts = [f.count("users").await, f.count("identity_user_roles").await];
    f.finish().await;
    let mut statuses = [a.0.as_u16(), b.0.as_u16()];
    statuses.sort();
    assert_eq!(statuses, [201, 409]);
    assert_eq!(counts, [1, 1]);
    assert_eq!(user.roles, vec![server_auth::ROLE_ADMIN.to_string()]);
}
#[tokio::test]
async fn legacy_sqlite_schema_remains_supported_without_normalized_role_table() {
    let f = Fixture::sqlite(false).await;
    let result = f.setup(Some(TOKEN)).await;
    let db::DbStore::Sqlite(pool) = &f.db.store else {
        unreachable!()
    };
    let roles: String = sqlx::query_scalar("SELECT roles FROM users")
        .fetch_one(pool)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(result.0, StatusCode::CREATED);
    assert_eq!(
        serde_json::from_str::<Vec<String>>(&roles).unwrap(),
        vec![server_auth::ROLE_ADMIN.to_string()]
    );
}
#[tokio::test]
async fn missing_setup_authority_never_creates_identity_rows() {
    let f = Fixture::sqlite(true).await;
    let result = f.setup(None).await;
    let counts = [
        f.count("users").await,
        f.count("identity_user_roles").await,
        f.count("tenants").await,
    ];
    f.finish().await;
    assert_eq!(result.0, StatusCode::UNAUTHORIZED);
    assert_eq!(counts, [0, 0, 0]);
}

async fn normalized_admin_blocks_repeat(f: Fixture) {
    f.admin_orm
        .execute(sea_orm::Statement::from_string(
            f.admin_orm.get_database_backend(),
            "INSERT INTO tenants(id,name)VALUES('existing-tenant','Existing')",
        ))
        .await
        .unwrap();
    let user = server_auth::User {
        id: "existing-admin".into(),
        username: "existing-admin".into(),
        email: "existing@example.test".into(),
        password_hash: String::new(),
        roles: vec![server_auth::ROLE_ADMIN.into()],
        active: true,
        organization_id: Some("existing-tenant".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    };
    server_auth::seaorm_store::SeaOrmAuthRepository::new(f.admin_orm.clone())
        .create_user(user, "existing-tenant")
        .await
        .unwrap();
    let sql = if f.postgres.is_some() {
        "UPDATE users SET roles=ARRAY[]::TEXT[] WHERE id='existing-admin'"
    } else {
        "UPDATE users SET roles='[]' WHERE id='existing-admin'"
    };
    f.admin_orm
        .execute(sea_orm::Statement::from_string(
            f.admin_orm.get_database_backend(),
            sql,
        ))
        .await
        .unwrap();
    let result = f.setup(Some(TOKEN)).await;
    let counts = [f.count("users").await, f.count("identity_user_roles").await];
    f.finish().await;
    assert_eq!(result.0, StatusCode::CONFLICT);
    assert_eq!(counts, [1, 1]);
}
#[tokio::test]
async fn postgres_normalized_admin_prevents_second_bootstrap_with_stale_legacy_roles() {
    normalized_admin_blocks_repeat(Fixture::postgres().await).await;
}
#[tokio::test]
async fn sqlite_normalized_admin_prevents_second_bootstrap_with_stale_legacy_roles() {
    normalized_admin_blocks_repeat(Fixture::sqlite(true).await).await;
}
