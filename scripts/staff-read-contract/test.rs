use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::{Row, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
use tower::ServiceExt;
use uuid::Uuid;

const PATHS: [(&str, &str); 3] = [
    ("/staff", "staff"),
    ("/tasks", "tasks"),
    ("/summaries", "summaries"),
];
fn claims(tenant: &str) -> server_common::Claims {
    server_common::Claims {
        sub: "test-owner".into(),
        exp: 4_000_000_000,
        iat: 0,
        organization_id: Some(tenant.into()),
        username: "owner".into(),
        email: "owner@example.test".into(),
        roles: vec!["ADMIN".into()],
        session_id: None,
        jti: "explicit-test-session".into(),
    }
}
async fn read(db: Arc<DB>, path: &str, tenant: Option<&str>) -> (StatusCode, Value) {
    let app = if let Some(tenant) = tenant {
        router(db).layer(Extension(claims(tenant)))
    } else {
        router(db)
    };
    let response = app
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), 1_048_576).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap())
}
async fn sqlite(schema: bool) -> Arc<DB> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    if schema {
        sqlx::raw_sql("CREATE TABLE ohc_staff_member(id TEXT PRIMARY KEY,tenant_id TEXT,name TEXT,phone_number TEXT,role TEXT);CREATE TABLE staff_tasks(id TEXT PRIMARY KEY,tenant_id TEXT,staff_id TEXT,title TEXT,description TEXT,status TEXT,priority TEXT,created_at TEXT);CREATE TABLE shift_summaries(id TEXT PRIMARY KEY,tenant_id TEXT,summary_text TEXT,escalations TEXT,created_at TEXT);").execute(&pool).await.unwrap();
    }
    Arc::new(DB {
        pool: PgPoolOptions::new()
            .connect_lazy("postgres://unused@127.0.0.1:1/unused")
            .unwrap(),
        store: db::DbStore::Sqlite(pool),
    })
}
async fn seed_sqlite(db: &DB) {
    let db::DbStore::Sqlite(pool) = &db.store else {
        unreachable!()
    };
    for tenant in ["a", "b"] {
        sqlx::query("INSERT INTO ohc_staff_member VALUES(?,?,?,?,?)")
            .bind(format!("staff-{tenant}"))
            .bind(tenant)
            .bind(format!("Recorded {tenant}"))
            .bind("+10000000000")
            .bind("Cashier")
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO staff_tasks VALUES(?,?,?,?,?,?,?,?)")
            .bind(format!("task-{tenant}"))
            .bind(tenant)
            .bind(format!("staff-{tenant}"))
            .bind("Real task title")
            .bind("Recorded task description")
            .bind("pending")
            .bind("medium")
            .bind("2026-10-02 10:00:00")
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO shift_summaries VALUES(?,?,?,?,?)")
            .bind(format!("summary-{tenant}"))
            .bind(tenant)
            .bind("Recorded shift summary")
            .bind(Option::<String>::None)
            .bind("2026-10-02 10:00:00")
            .execute(pool)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn unauthenticated_and_invalid_tenants_do_not_read_storage() {
    let db = sqlite(false).await;
    for (path, _) in PATHS {
        for tenant in [None, Some(""), Some("system"), Some(" system ")] {
            assert_eq!(
                read(db.clone(), path, tenant).await.0,
                StatusCode::UNAUTHORIZED
            );
        }
    }
}
#[tokio::test]
async fn sqlite_missing_storage_is_an_error_not_empty_business_state() {
    let db = sqlite(false).await;
    for (path, key) in PATHS {
        let (status, body) = read(db.clone(), path, Some("a")).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR, "{path}: {body}");
        assert_eq!(body, json!({"error":"db_error"}));
        assert!(body.get(key).is_none());
    }
}
#[tokio::test]
async fn sqlite_genuinely_empty_reads_remain_empty() {
    let db = sqlite(true).await;
    for (path, key) in PATHS {
        let (status, body) = read(db.clone(), path, Some("a")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body[key], json!([]));
    }
}
#[tokio::test]
async fn sqlite_returns_only_recorded_current_tenant_rows() {
    let db = sqlite(true).await;
    seed_sqlite(&db).await;
    for tenant in ["a", "b", "a"] {
        for (path, key) in PATHS {
            let (status, body) = read(db.clone(), path, Some(tenant)).await;
            assert_eq!(status, StatusCode::OK);
            let rows = body[key].as_array().unwrap();
            assert_eq!(rows.len(), 1);
            assert!(
                rows[0]["id"]
                    .as_str()
                    .unwrap()
                    .ends_with(&format!("-{tenant}"))
            );
        }
    }
    assert_eq!(
        read(db, "/summaries", Some("a")).await.1["summaries"][0]["summary_text"],
        "Recorded shift summary"
    );
}
struct PgFixture {
    admin: sqlx::PgPool,
    db: Arc<DB>,
    schema: String,
    role: String,
}
impl PgFixture {
    async fn new(schema_ready: bool) -> Self {
        let url = std::env::var("OHC_STAFF_TEST_DATABASE_URL")
            .expect("Owned loopback PostgreSQL URL required; run the guarded runner");
        let suffix = Uuid::new_v4().simple().to_string();
        let schema = format!("staff_read_{suffix}");
        let role = format!("staff_owner_{suffix}");
        let password = Uuid::new_v4().simple().to_string();
        let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
        let admin = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options.clone().options([("search_path", schema.as_str())]))
            .await
            .unwrap();
        sqlx::query(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}' NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS")).execute(&admin).await.unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema} AUTHORIZATION {role}"))
            .execute(&admin)
            .await
            .unwrap();
        if schema_ready {
            sqlx::raw_sql(
                "CREATE TABLE tenants(id TEXT PRIMARY KEY);INSERT INTO tenants VALUES('a'),('b');",
            )
            .execute(&admin)
            .await
            .unwrap();
            sqlx::raw_sql(include_str!(
                "../../src/server/migrations/206_staff_management.sql"
            ))
            .execute(&admin)
            .await
            .unwrap();
            sqlx::raw_sql("ALTER TABLE shift_summaries ADD COLUMN escalations TEXT")
                .execute(&admin)
                .await
                .unwrap();
            sqlx::raw_sql(include_str!(
                "../../src/server/migrations/1027_staff_mesh_native.sql"
            ))
            .execute(&admin)
            .await
            .unwrap();
            for table in [
                "ohc_staff_member",
                "ohc_timecard_event",
                "staff_tasks",
                "shift_summaries",
            ] {
                sqlx::raw_sql(&format!("ALTER TABLE {table} OWNER TO {role};ALTER TABLE {table} FORCE ROW LEVEL SECURITY")).execute(&admin).await.unwrap();
            }
            sqlx::query(&format!("GRANT SELECT ON tenants TO {role}"))
                .execute(&admin)
                .await
                .unwrap();
        }
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_with(
                options
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let actual =
            sqlx::query("SELECT rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(!actual.get::<bool, _>("rolsuper"));
        assert!(!actual.get::<bool, _>("rolbypassrls"));
        Self {
            admin,
            db: Arc::new(DB {
                pool,
                store: db::DbStore::Postgres,
            }),
            schema,
            role,
        }
    }
    async fn seed(&self) {
        for tenant in ["a", "b"] {
            sqlx::query("INSERT INTO ohc_staff_member(id,tenant_id,name,phone_number,role) VALUES($1,$2,$3,'+10000000000','Cashier')").bind(format!("staff-{tenant}")).bind(tenant).bind(format!("Recorded {tenant}")).execute(&self.admin).await.unwrap();
            sqlx::query("INSERT INTO staff_tasks(id,tenant_id,staff_id,title,description) VALUES($1,$2,$3,'Real task title','Recorded task description')").bind(format!("task-{tenant}")).bind(tenant).bind(format!("staff-{tenant}")).execute(&self.admin).await.unwrap();
            sqlx::query("INSERT INTO shift_summaries(id,tenant_id,shift_date,summary_text) VALUES($1,$2,CURRENT_DATE,'Recorded shift summary')").bind(format!("summary-{tenant}")).bind(tenant).execute(&self.admin).await.unwrap();
        }
    }
    async fn close(self) {
        self.db.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        sqlx::query(&format!("DROP ROLE {}", self.role))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}
#[tokio::test]
async fn postgres_missing_storage_is_an_error_not_empty_business_state() {
    let f = PgFixture::new(false).await;
    for (path, key) in PATHS {
        let (status, body) = read(f.db.clone(), path, Some("a")).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR, "{path}: {body}");
        assert!(body.get(key).is_none());
    }
    f.close().await;
}
#[tokio::test]
async fn postgres_empty_reads_use_actual_migrated_tables() {
    let f = PgFixture::new(true).await;
    for (path, key) in PATHS {
        let (status, body) = read(f.db.clone(), path, Some("a")).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body[key], json!([]));
    }
    f.close().await;
}
#[tokio::test]
async fn postgres_reused_connection_never_returns_another_tenants_rows() {
    let f = PgFixture::new(true).await;
    f.seed().await;
    for tenant in ["a", "b", "a"] {
        for (path, key) in PATHS {
            let (status, body) = read(f.db.clone(), path, Some(tenant)).await;
            assert_eq!(status, StatusCode::OK, "{body}");
            let rows = body[key].as_array().unwrap();
            assert_eq!(rows.len(), 1);
            assert!(
                rows[0]["id"]
                    .as_str()
                    .unwrap()
                    .ends_with(&format!("-{tenant}"))
            );
        }
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM ohc_staff_member")
        .fetch_one(&f.db.pool)
        .await
        .unwrap();
    assert_eq!(
        count, 0,
        "transaction-scoped tenant context must reset after each read"
    );
    f.close().await;
}
#[tokio::test]
async fn additive_staff_migration_preserves_existing_rows_and_forces_owner_rls() {
    let f = PgFixture::new(true).await;
    f.seed().await;
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/1027_staff_mesh_native.sql"
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM ohc_staff_member")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    assert_eq!(count, 2);
    for table in ["ohc_staff_member", "ohc_timecard_event"] {
        let forced: bool =
            sqlx::query_scalar("SELECT relforcerowsecurity FROM pg_class WHERE oid=$1::regclass")
                .bind(table)
                .fetch_one(&f.db.pool)
                .await
                .unwrap();
        assert!(forced);
    }
    let mut tx = f.db.pool.begin().await.unwrap();
    server_common::auth_utils::set_org_context(&mut *tx, "a")
        .await
        .unwrap();
    let result=sqlx::query("INSERT INTO ohc_staff_member(id,tenant_id,name,phone_number,role) VALUES('foreign','b','Wrong','x','Cashier')").execute(&mut *tx).await;
    assert!(result.is_err());
    tx.rollback().await.unwrap();
    f.close().await;
}
