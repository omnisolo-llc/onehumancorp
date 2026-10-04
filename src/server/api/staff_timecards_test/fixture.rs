//! Disposable real-storage fixtures. No external providers or fabricated claims.
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, Schema, Set, Statement};
use serde_json::{Value, json};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{sync::Arc, time::Duration};
use tower::ServiceExt;

pub const ROUTE: &str = "/api/v1/staff/timecard";
pub const OWNER: &str = "clock-owner";
pub const TENANT: &str = "clock-tenant";
const SIGNING_KEY: &str = "public-local-clock-regression-signing-key-only";
pub const STAMP: &str = "2026-10-04T06:00:00.123456Z";

#[derive(Clone, Copy)]
pub enum Backend {
    Sqlite,
    Postgres,
}

enum Storage {
    Sqlite(sqlx::SqlitePool),
    Postgres {
        pool: sqlx::PgPool,
        owner: sqlx::PgPool,
        admin: sqlx::PgPool,
        schema: String,
        role: String,
    },
}

pub struct Fixture {
    pub auth: Arc<server_auth::Store>,
    pub token: String,
    pub db: Arc<crate::db::DB>,
    pub backend: Backend,
    orm: DatabaseConnection,
    storage: Storage,
}

#[derive(Debug)]
pub struct SavedEvent {
    pub id: String,
    pub staff: String,
    pub kind: String,
    pub instant: DateTime<Utc>,
    pub identity: Option<Value>,
}

fn user(id: &str, tenant: &str, role: &str) -> server_auth::User {
    server_auth::User {
        id: id.into(),
        username: id.into(),
        email: format!("{id}@example.test"),
        password_hash: "synthetic-unused".into(),
        roles: vec![role.into()],
        active: true,
        organization_id: Some(tenant.into()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        oidc_subject: None,
    }
}

async fn install_identity(orm: &DatabaseConnection) {
    use server_auth::seaorm_store::entities::{
        identity_user_role, revoked_token, user as user_entity,
    };
    let backend = orm.get_database_backend();
    for statement in [
        Schema::new(backend).create_table_from_entity(user_entity::Entity),
        Schema::new(backend).create_table_from_entity(identity_user_role::Entity),
        Schema::new(backend).create_table_from_entity(revoked_token::Entity),
    ] {
        orm.execute(backend.build(&statement)).await.unwrap();
    }
    for (id, tenant, role) in [
        (OWNER, TENANT, "OWNER"),
        ("other-owner", TENANT, "ADMIN"),
        ("member", TENANT, "STAFF"),
        ("foreign-owner", "foreign-tenant", "OWNER"),
    ] {
        let value = user(id, tenant, role);
        user_entity::ActiveModel {
            id: Set(value.id),
            username: Set(value.username),
            email: Set(value.email),
            password_hash: Set(value.password_hash),
            active: Set(true),
            tenant_id: Set(tenant.into()),
            oidc_subject: Set(None),
            created_at: Set(value.created_at),
            updated_at: Set(value.updated_at),
        }
        .insert(orm)
        .await
        .unwrap();
        identity_user_role::ActiveModel {
            user_id: Set(id.into()),
            role_name: Set(role.into()),
            tenant_id: Set(tenant.into()),
            position: Set(0),
        }
        .insert(orm)
        .await
        .unwrap();
    }
}

// Compile fixtures against the mounted SQLite schema, not a parallel hand-written
// approximation. Missing or moved boundaries must fail test setup explicitly.
fn sqlite_business_schema() -> String {
    let source = include_str!("../../db.rs");
    ["ohc_staff_member", "ohc_timecard_event"]
        .into_iter()
        .map(|table| {
            let marker = format!("CREATE TABLE IF NOT EXISTS {table} (");
            assert_eq!(
                source.matches(&marker).count(),
                1,
                "ambiguous production schema"
            );
            let start = source.find(&marker).unwrap();
            let tail = &source[start..];
            let end = tail
                .find("\n                    );")
                .expect("production SQLite schema boundary")
                + "\n                    );".len();
            tail[..end].to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

impl Fixture {
    pub async fn new(backend: Backend) -> Self {
        let (storage, orm, db, canonical) = match backend {
            Backend::Sqlite => {
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(1)
                    .connect_with(
                        sqlx::sqlite::SqliteConnectOptions::new()
                            .in_memory(true)
                            .foreign_keys(true)
                            .busy_timeout(Duration::from_secs(3)),
                    )
                    .await
                    .unwrap();
                sqlx::raw_sql(&sqlite_business_schema())
                    .execute(&pool)
                    .await
                    .unwrap();
                let orm = sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
                install_identity(&orm).await;
                let db = Arc::new(crate::db::DB {
                    pool: crate::db::secure_pg_pool_options()
                        .connect_lazy("postgres://unused@127.0.0.1:1/unused")
                        .unwrap(),
                    store: crate::db::DbStore::Sqlite(pool.clone()),
                });
                (Storage::Sqlite(pool), orm.clone(), db, orm)
            }
            Backend::Postgres => {
                let raw = std::env::var("OHC_CLOCK_TEST_DATABASE_URL")
                    .or_else(|_| std::env::var("OHC_SYNC_TEST_DATABASE_URL"))
                    .or_else(|_| std::env::var("OMNISOLO_POSTGRES_ADMIN_URL"))
                    .expect("clock PostgreSQL regressions require a disposable loopback /ohc_*_test database URL; missing prerequisites are not a passing test");
                let url = url::Url::parse(&raw).unwrap();
                assert!(
                    matches!(
                        url.host_str(),
                        Some("localhost" | "127.0.0.1" | "[::1]" | "::1")
                    ),
                    "clock tests require loopback PostgreSQL"
                );
                assert!(
                    url.path().starts_with("/ohc_")
                        && url.path().ends_with("_test")
                        && url.query().is_none(),
                    "clock tests require a dedicated /ohc_*_test database"
                );
                let options: PgConnectOptions = raw.parse().unwrap();
                let admin = PgPoolOptions::new()
                    .max_connections(1)
                    .connect_with(options.clone())
                    .await
                    .unwrap();
                let suffix = uuid::Uuid::new_v4().simple().to_string();
                let schema = format!("clock_{suffix}");
                let role = format!("clock_role_{suffix}");
                sqlx::query(&format!("CREATE SCHEMA {schema}"))
                    .execute(&admin)
                    .await
                    .unwrap();
                let owner = PgPoolOptions::new()
                    .max_connections(2)
                    .connect_with(options.clone().options([("search_path", schema.as_str())]))
                    .await
                    .unwrap();
                sqlx::raw_sql(include_str!("../../migrations/1027_staff_mesh_native.sql"))
                    .execute(&owner)
                    .await
                    .unwrap();
                sqlx::raw_sql(include_str!(
                    "../../migrations/1035_staff_timecard_receipts.sql"
                ))
                .execute(&owner)
                .await
                .unwrap();
                let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(owner.clone());
                install_identity(&orm).await;
                for table in ["users", "identity_user_roles", "auth_revoked_tokens"] {
                    sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&owner).await.unwrap();
                }
                let password = format!("synthetic-{suffix}");
                sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}' NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
                let pool = crate::db::secure_pg_pool_options()
                    .max_connections(4)
                    .connect_with(
                        options
                            .username(&role)
                            .password(&password)
                            .options([("search_path", schema.as_str())]),
                    )
                    .await
                    .unwrap();
                let canonical =
                    sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());
                let db = Arc::new(crate::db::DB {
                    pool: pool.clone(),
                    store: crate::db::DbStore::Postgres,
                });
                (
                    Storage::Postgres {
                        pool,
                        owner,
                        admin,
                        schema,
                        role,
                    },
                    orm,
                    db,
                    canonical,
                )
            }
        };
        // A fixed test-only secret avoids loading or creating the developer's
        // persisted credentials when these native tests run outside the gate.
        let auth = temp_env::with_vars(
            [
                ("JWT_SECRET", Some(SIGNING_KEY)),
                ("JWT_SECRET_FILE", None),
                ("OMNISOLO_REDIS_URL", None),
                ("REDIS_URL", None),
                ("REDIS_URL_FILE", None),
            ],
            || {
                Arc::new(server_auth::Store::with_portable_repo(Arc::new(
                    server_auth::seaorm_store::SeaOrmAuthRepository::new(canonical),
                )))
            },
        );
        let token = auth.issue_token(&user(OWNER, TENANT, "OWNER")).unwrap();
        let fixture = Self {
            auth,
            token,
            db,
            backend,
            orm,
            storage,
        };
        fixture.execute("INSERT INTO ohc_staff_member(id,tenant_id,name,phone_number,role) VALUES ('owned-staff','clock-tenant','Synthetic staff','','STAFF'),('foreign-staff','foreign-tenant','Foreign staff','','STAFF')").await;
        fixture
    }

    pub async fn app(&self) -> Router {
        let access =
            crate::api::staff_timecards::TimecardAccess::configured(&self.db, self.auth.clone())
                .await;
        Router::new()
            .nest(
                "/api/v1/staff",
                crate::api::staff_mesh::router(self.db.clone()).layer(axum::Extension(access)),
            )
            .route_layer(axum::middleware::from_fn_with_state(
                self.auth.clone(),
                server_auth::strict_bearer_auth_middleware,
            ))
    }

    pub async fn receipt(&self, id: &str) -> (StatusCode, Value) {
        self.receipt_token(id, &self.token).await
    }
    pub async fn receipt_token(&self, id: &str, token: &str) -> (StatusCode, Value) {
        let response = self
            .app()
            .await
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("{ROUTE}/receipts/{id}"))
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        if status.is_success() {
            assert_eq!(
                response.headers().get("cache-control").unwrap(),
                "private, no-store"
            );
        }
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
    }
    pub async fn post(&self, events: Vec<Value>) -> (StatusCode, Value) {
        self.post_token(events, &self.token).await
    }
    pub async fn post_token(&self, events: Vec<Value>, token: &str) -> (StatusCode, Value) {
        let response = self
            .app()
            .await
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(ROUTE)
                    .header("authorization", format!("Bearer {token}"))
                    .header("content-type", "application/json")
                    .body(Body::from(json!({"events": events}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
    }
    /// Deliberately bypass middleware to test the handler's own exact-token
    /// binding. Claims are not authority, even inside the process.
    pub async fn post_injected(
        &self,
        claims: server_common::Claims,
        token: Option<&str>,
        db: Arc<crate::db::DB>,
    ) -> (StatusCode, Value) {
        let access =
            crate::api::staff_timecards::TimecardAccess::configured(&db, self.auth.clone()).await;
        let app = Router::new().nest(
            "/api/v1/staff",
            crate::api::staff_mesh::router(db).layer(axum::Extension(access)),
        );
        let mut builder = Request::builder()
            .method("POST")
            .uri(ROUTE)
            .header("content-type", "application/json");
        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }
        let mut request = builder
            .body(Body::from(
                json!({"events":[clock("injected-clock")]}).to_string(),
            ))
            .unwrap();
        request.extensions_mut().insert(claims);
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
    }
    pub async fn expired_token(&self) -> String {
        let mut claims = self.auth.validate_token(&self.token).await.unwrap();
        claims.exp = Utc::now().timestamp() - 120;
        claims.iat = claims.exp - 300;
        jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(SIGNING_KEY.as_bytes()),
        )
        .unwrap()
    }
    pub fn token_for(&self, id: &str, tenant: &str, role: &str) -> String {
        self.auth.issue_token(&user(id, tenant, role)).unwrap()
    }
    pub async fn execute(&self, sql: &str) {
        self.orm.execute_unprepared(sql).await.unwrap();
    }
    pub async fn count(&self) -> i64 {
        self.orm
            .query_one(Statement::from_string(
                self.orm.get_database_backend(),
                "SELECT count(*) AS n FROM ohc_timecard_event",
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get("", "n")
            .unwrap()
    }
    pub async fn saved(&self) -> Vec<SavedEvent> {
        let time = match self.backend {
            Backend::Sqlite => "CAST(event_time AS TEXT)",
            Backend::Postgres => {
                "to_char(event_time AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"')"
            }
        };
        self.orm.query_all(Statement::from_string(self.orm.get_database_backend(), format!("SELECT id,staff_id,event_type,{time} AS event_time,request_identity FROM ohc_timecard_event ORDER BY id")))
            .await.unwrap().into_iter().map(|row| SavedEvent {
                id: row.try_get("", "id").unwrap(), staff: row.try_get("", "staff_id").unwrap(), kind: row.try_get("", "event_type").unwrap(),
                instant: DateTime::parse_from_rfc3339(&row.try_get::<String>("", "event_time").unwrap()).unwrap().with_timezone(&Utc),
                identity: row.try_get::<Option<String>>("", "request_identity").unwrap().map(|raw| serde_json::from_str(&raw).unwrap()),
            }).collect()
    }
    pub async fn fail_second_write(&self) {
        self.execute(match self.backend {
            Backend::Sqlite => "CREATE TRIGGER fail_clock BEFORE INSERT ON ohc_timecard_event WHEN NEW.id='zz-fail' BEGIN SELECT RAISE(ABORT,'synthetic clock write failure'); END",
            Backend::Postgres => "CREATE FUNCTION fail_clock() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.id='zz-fail' THEN RAISE EXCEPTION 'synthetic clock write failure' USING ERRCODE='23514'; END IF; RETURN NEW; END $$; CREATE TRIGGER fail_clock BEFORE INSERT ON ohc_timecard_event FOR EACH ROW EXECUTE FUNCTION fail_clock()",
        }).await;
    }
    pub async fn finish(self) {
        match self.storage {
            Storage::Sqlite(pool) => pool.close().await,
            Storage::Postgres {
                pool,
                owner,
                admin,
                schema,
                role,
            } => {
                pool.close().await;
                owner.close().await;
                sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
                    .execute(&admin)
                    .await
                    .unwrap();
                sqlx::query(&format!("DROP ROLE {role}"))
                    .execute(&admin)
                    .await
                    .unwrap();
                admin.close().await;
            }
        }
    }
}

pub fn event(id: &str, staff: &str, kind: &str, timestamp: &str) -> Value {
    json!({"id": id,"staff_id": staff,"event_type": kind,"offline_timestamp": timestamp})
}
pub fn clock(id: &str) -> Value {
    event(id, OWNER, "CLOCK_IN", STAMP)
}
pub fn identity(value: &Value, actor: &str) -> Value {
    json!({"version":1,"actor_id":actor,"id":value["id"],"staff_id":value["staff_id"],"event_type":value["event_type"],"offline_timestamp":value["offline_timestamp"]})
}
pub fn assert_ack(response: &(StatusCode, Value), ids: &[&str]) {
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    assert_eq!(response.1["success"], true);
    let outcomes = response.1["outcomes"]
        .as_array()
        .expect("a committed timecard needs per-ID receipts, not aggregate success");
    assert_eq!(outcomes.len(), ids.len());
    for id in ids {
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| outcome["id"] == *id
                    && outcome["route"] == ROUTE
                    && outcome["status"] == "acknowledged")
                .count(),
            1,
            "{response:?}"
        );
    }
}
