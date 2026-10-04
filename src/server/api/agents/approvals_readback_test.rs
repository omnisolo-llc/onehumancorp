use super::*;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::CentrifugeNode;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use omnisolo_builtin_agent::mesh::transport::InProcessTransport;
use serde_json::{Value, json};
use tower::ServiceExt;

struct Fixture {
    db: Arc<DB>,
    app: Router,
    tenant: String,
}

impl Fixture {
    async fn new(postgres: bool) -> Self {
        let tenant = format!("approval-readback-{}", uuid::Uuid::new_v4().simple());
        let db = if postgres {
            let url = std::env::var("OHC_APPROVAL_TEST_DATABASE_URL")
                .expect("owned PostgreSQL fixture is required");
            let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
            assert_eq!(options.get_host(), "127.0.0.1");
            assert_eq!(options.get_database(), Some("ohc_approval_test"));
            let admin = sqlx::PgPool::connect(&url).await.unwrap();
            let schema = format!("approvals_{}", uuid::Uuid::new_v4().simple());
            sqlx::query(&format!("CREATE SCHEMA {schema}"))
                .execute(&admin)
                .await
                .unwrap();
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(4)
                .after_connect(move |connection, _| {
                    let sql = format!("SET search_path TO {schema}");
                    Box::pin(async move {
                        sqlx::query(&sql).execute(connection).await?;
                        Ok(())
                    })
                })
                .connect(&url)
                .await
                .unwrap();
            sqlx::raw_sql("CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, event_source TEXT NOT NULL, context_payload JSONB NOT NULL, proposed_action JSONB, lifecycle_state TEXT NOT NULL, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ);").execute(&pool).await.unwrap();
            admin.close().await;
            Arc::new(DB {
                pool,
                store: DbStore::Postgres,
            })
        } else {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::raw_sql("CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, event_source TEXT NOT NULL, context_payload TEXT NOT NULL, proposed_action TEXT, lifecycle_state TEXT NOT NULL, created_at TEXT, updated_at TEXT);").execute(&pool).await.unwrap();
            Arc::new(DB {
                pool: sqlx::postgres::PgPoolOptions::new()
                    .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
                    .unwrap(),
                store: DbStore::Sqlite(pool),
            })
        };
        let mesh = Arc::new(CentrifugeNode::new(Arc::new(InProcessTransport::new())));
        let app = router(Arc::new(DepartmentOrchestrator::new(db.clone(), mesh)));
        let fixture = Self { db, app, tenant };
        for (id, owner) in [
            ("a", fixture.tenant.as_str()),
            ("b", fixture.tenant.as_str()),
            ("c", fixture.tenant.as_str()),
            ("foreign", "other-tenant"),
        ] {
            let sql = format!(
                "INSERT INTO agent_feed_items VALUES ('{id}', '{owner}', 'operations', '{{\"description\":\"Decision {id}\"}}', '{{\"context\":\"original\"}}', 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            );
            fixture.execute(&sql).await;
        }
        fixture
    }

    async fn execute(&self, sql: &str) {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::raw_sql(sql).execute(&self.db.pool).await.unwrap();
            }
            DbStore::Sqlite(pool) => {
                sqlx::raw_sql(sql).execute(pool).await.unwrap();
            }
        }
    }

    async fn state(&self, id: &str) -> String {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_scalar("SELECT lifecycle_state FROM agent_feed_items WHERE id=$1")
                    .bind(id)
                    .fetch_one(&self.db.pool)
                    .await
                    .unwrap()
            }
            DbStore::Sqlite(pool) => {
                sqlx::query_scalar("SELECT lifecycle_state FROM agent_feed_items WHERE id=?")
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .unwrap()
            }
        }
    }

    async fn request(
        &self,
        method: &str,
        path: &str,
        tenant: &str,
    ) -> (StatusCode, axum::http::HeaderMap, Value) {
        let claims = Claims {
            sub: "fixture-owner".into(),
            exp: i64::MAX,
            iat: 0,
            organization_id: Some(tenant.into()),
            username: String::new(),
            email: String::new(),
            roles: vec!["OWNER".into()],
            session_id: None,
            jti: String::new(),
        };
        let request = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .extension(claims)
            .body(if method == "POST" {
                Body::from(r#"{"approved":false}"#)
            } else {
                Body::empty()
            })
            .unwrap();
        let response = self.app.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (status, headers, serde_json::from_slice(&body).unwrap())
    }

    async fn get(&self, path: &str) -> Value {
        let (status, _, body) = self.request("GET", path, &self.tenant).await;
        assert_eq!(status, StatusCode::OK);
        body
    }

    async fn dismiss(&self, id: &str) {
        let (status, _, body) = self.request("POST", &format!("/{id}"), &self.tenant).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({"success": true}));
        assert_eq!(self.state(id).await, "REJECTED");
    }
}

fn ids(body: &Value) -> Vec<&str> {
    body["pending_approvals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect()
}

async fn committed_decision_is_immediately_visible_in_every_primed_variant(postgres: bool) {
    let fixture = Fixture::new(postgres).await;
    let variants = [
        "/",
        "/?limit=1",
        "/?cursor=a&limit=1",
        "/?mobile_optimized=true",
        "/?cursor=a&limit=2&mobile_optimized=true",
    ];
    for path in variants {
        assert!(
            ids(&fixture.get(path).await)
                .iter()
                .all(|id| *id != "foreign")
        );
    }
    for path in [
        "/activity",
        "/activity?limit=1",
        "/activity?cursor=c&limit=2&mobile_optimized=true",
    ] {
        assert!(ids(&fixture.get(path).await).is_empty());
    }
    fixture.dismiss("b").await;
    for path in variants {
        let body = fixture.get(path).await;
        assert!(
            !ids(&body).contains(&"b"),
            "dismissed row resurrected in {path}"
        );
    }
    assert_eq!(ids(&fixture.get("/?limit=1").await), ["a"]);
    assert_eq!(fixture.get("/?limit=1").await["next_cursor"], "a");
    let mobile = fixture
        .get("/?cursor=a&limit=1&mobile_optimized=true")
        .await;
    assert_eq!(ids(&mobile), ["c"]);
    assert!(mobile["pending_approvals"][0]["payload"].is_null());
    assert_eq!(
        fixture.get("/").await["pending_approvals"][0]["payload"],
        json!({"context":"original"})
    );
    for path in [
        "/activity",
        "/activity?limit=1",
        "/activity?cursor=c&limit=2&mobile_optimized=true",
    ] {
        let body = fixture.get(path).await;
        assert_eq!(ids(&body), ["b"], "committed decision missing from {path}");
        assert_eq!(body["pending_approvals"][0]["status"], "Rejected");
    }
}

async fn independent_writer_and_concurrent_reader_cannot_restore_pending_state(postgres: bool) {
    let fixture = Fixture::new(postgres).await;
    assert_eq!(ids(&fixture.get("/").await), ["a", "b", "c"]);
    let (read, ()) = tokio::join!(fixture.get("/"), fixture.dismiss("b"));
    assert!(ids(&read).contains(&"a"));
    assert!(!ids(&fixture.get("/").await).contains(&"b"));
    fixture
        .execute("UPDATE agent_feed_items SET lifecycle_state='REJECTED' WHERE id='c'")
        .await;
    assert_eq!(ids(&fixture.get("/").await), ["a"]);
}

async fn rejected_persistence_stays_pending_and_reports_failure(postgres: bool) {
    let fixture = Fixture::new(postgres).await;
    fixture.get("/").await;
    if postgres {
        fixture.execute("ALTER TABLE agent_feed_items ADD CONSTRAINT reject_decision CHECK(lifecycle_state <> 'REJECTED')").await;
    } else {
        fixture.execute("CREATE TRIGGER reject_decision BEFORE UPDATE ON agent_feed_items BEGIN SELECT RAISE(ABORT, 'fixture rejects writes'); END").await;
    }
    let (status, _, body) = fixture.request("POST", "/b", &fixture.tenant).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body, json!({"success":false}));
    assert_eq!(fixture.state("b").await, "PENDING_APPROVAL");
    assert_eq!(ids(&fixture.get("/").await), ["a", "b", "c"]);
}

async fn decisions_and_reload_remain_tenant_scoped(postgres: bool) {
    let fixture = Fixture::new(postgres).await;
    let (status, _, other) = fixture.request("GET", "/", "other-tenant").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(ids(&other), ["foreign"]);
    fixture.get("/").await;
    let (status, _, body) = fixture.request("POST", "/foreign", &fixture.tenant).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body, json!({"success":false}));
    assert_eq!(fixture.state("foreign").await, "PENDING_APPROVAL");
    fixture.dismiss("b").await;
    let (_, _, other) = fixture.request("GET", "/", "other-tenant").await;
    assert_eq!(ids(&other), ["foreign"]);
    assert_eq!(ids(&fixture.get("/").await), ["a", "c"]);
}

async fn mutable_approval_lists_are_private_no_store(postgres: bool) {
    let fixture = Fixture::new(postgres).await;
    for path in [
        "/",
        "/?limit=1&mobile_optimized=true",
        "/activity",
        "/activity?cursor=c&limit=1",
    ] {
        let (status, headers, _) = fixture.request("GET", path, &fixture.tenant).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            headers.get("cache-control").and_then(|v| v.to_str().ok()),
            Some("private, no-store")
        );
    }
}

macro_rules! readback_cases {
    ($module:ident, $postgres:expr) => {
        mod $module {
            use super::*;
            #[tokio::test]
            async fn primed_variants() {
                committed_decision_is_immediately_visible_in_every_primed_variant($postgres).await;
            }
            #[tokio::test]
            async fn concurrent_and_independent_writers() {
                independent_writer_and_concurrent_reader_cannot_restore_pending_state($postgres)
                    .await;
            }
            #[tokio::test]
            async fn failed_persistence() {
                rejected_persistence_stays_pending_and_reports_failure($postgres).await;
            }
            #[tokio::test]
            async fn tenant_isolation() {
                decisions_and_reload_remain_tenant_scoped($postgres).await;
            }
            #[tokio::test]
            async fn private_reads() {
                mutable_approval_lists_are_private_no_store($postgres).await;
            }
        }
    };
}
readback_cases!(sqlite, false);
readback_cases!(postgres, true);
