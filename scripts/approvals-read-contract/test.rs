use crate::{
    approvals,
    db::{DB, DbStore},
    orchestration::departments::orchestrator::DepartmentOrchestrator,
};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

struct Fixture {
    pool: sqlx::PgPool,
    tenant: String,
    app: Router,
}
impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_APPROVAL_TEST_DATABASE_URL").expect("owned PostgreSQL required");
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
        sqlx::raw_sql("CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,context_payload JSONB,proposed_action JSONB,lifecycle_state TEXT,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ);").execute(&pool).await.unwrap();
        admin.close().await;
        let tenant = format!("owner-{}", uuid::Uuid::new_v4().simple());
        for (id, owner) in [
            ("a", tenant.as_str()),
            ("b", tenant.as_str()),
            ("c", tenant.as_str()),
            ("foreign", "other-tenant"),
        ] {
            sqlx::query("INSERT INTO agent_feed_items VALUES ($1,$2,'operations','{\"description\":\"Decision\"}','{\"context\":\"original\"}','PENDING_APPROVAL',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(id).bind(owner).execute(&pool).await.unwrap();
        }
        let db = Arc::new(DB {
            pool: pool.clone(),
            store: DbStore::Postgres,
        });
        let app = approvals::router(Arc::new(DepartmentOrchestrator { db }));
        Self { pool, tenant, app }
    }
    async fn get_as(&self, path: &str, tenant: &str) -> (axum::http::HeaderMap, Value) {
        let claims = server_common::Claims {
            sub: "owner".into(),
            exp: i64::MAX,
            iat: 0,
            organization_id: Some(tenant.into()),
            username: String::new(),
            email: String::new(),
            roles: vec!["OWNER".into()],
            session_id: None,
            jti: String::new(),
        };
        let response = self
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .extension(claims)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let headers = response.headers().clone();
        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (headers, serde_json::from_slice(&body).unwrap())
    }
    async fn get(&self, path: &str) -> Value {
        self.get_as(path, &self.tenant).await.1
    }
    async fn reject(&self, id: &str) {
        // Independent committed writer; deliberately no mutation handler stub.
        let count = sqlx::query(
            "UPDATE agent_feed_items SET lifecycle_state='REJECTED' WHERE id=$1 AND tenant_id=$2",
        )
        .bind(id)
        .bind(&self.tenant)
        .execute(&self.pool)
        .await
        .unwrap()
        .rows_affected();
        assert_eq!(count, 1);
        let state: String =
            sqlx::query_scalar("SELECT lifecycle_state FROM agent_feed_items WHERE id=$1")
                .bind(id)
                .fetch_one(&self.pool)
                .await
                .unwrap();
        assert_eq!(state, "REJECTED");
    }
}
fn ids(value: &Value) -> Vec<&str> {
    value["pending_approvals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect()
}
#[tokio::test]
async fn pending_variants_read_committed_rows_after_priming() {
    let f = Fixture::new().await;
    let paths = [
        "/",
        "/?limit=2",
        "/?cursor=a&limit=1",
        "/?mobile_optimized=true",
        "/?cursor=a&limit=1&mobile_optimized=true",
    ];
    for path in paths {
        assert!(ids(&f.get(path).await).contains(&"b"));
    }
    f.reject("b").await;
    for path in paths {
        assert!(
            !ids(&f.get(path).await).contains(&"b"),
            "stale pending row from {path}"
        );
    }
}
#[tokio::test]
async fn activity_variants_show_newly_committed_decisions() {
    let f = Fixture::new().await;
    let paths = [
        "/activity",
        "/activity?limit=1",
        "/activity?cursor=c&limit=2&mobile_optimized=true",
    ];
    for path in paths {
        assert!(ids(&f.get(path).await).is_empty());
    }
    f.reject("b").await;
    for path in paths {
        let body = f.get(path).await;
        assert_eq!(ids(&body), ["b"], "stale history from {path}");
        assert_eq!(body["pending_approvals"][0]["status"], "Rejected");
    }
}
#[tokio::test]
async fn pagination_and_mobile_payload_are_preserved() {
    let f = Fixture::new().await;
    let first = f.get("/?limit=1").await;
    assert_eq!(ids(&first), ["a"]);
    assert_eq!(first["next_cursor"], "a");
    let second = f.get("/?cursor=a&limit=1&mobile_optimized=true").await;
    assert_eq!(ids(&second), ["b"]);
    assert_eq!(second["next_cursor"], "b");
    assert!(second["pending_approvals"][0]["payload"].is_null());
    assert_eq!(
        f.get("/").await["pending_approvals"][0]["payload"]["context"],
        "original"
    );
}
#[tokio::test]
async fn concurrent_reads_cannot_cache_a_precommit_snapshot() {
    let f = Fixture::new().await;
    f.get("/").await;
    let mut tx = f.pool.begin().await.unwrap();
    sqlx::query("UPDATE agent_feed_items SET lifecycle_state='REJECTED' WHERE id='b'")
        .execute(&mut *tx)
        .await
        .unwrap();
    assert!(ids(&f.get("/").await).contains(&"b"));
    let (commit, read) = tokio::join!(tx.commit(), f.get("/"));
    commit.unwrap();
    assert!(ids(&read).contains(&"a"));
    for _ in 0..3 {
        assert!(!ids(&f.get("/").await).contains(&"b"));
    }
}
#[tokio::test]
async fn rolled_back_failed_persistence_remains_pending() {
    let f = Fixture::new().await;
    f.get("/").await;
    sqlx::query("ALTER TABLE agent_feed_items ADD CONSTRAINT reject_write CHECK(lifecycle_state <> 'REJECTED')").execute(&f.pool).await.unwrap();
    let mut tx = f.pool.begin().await.unwrap();
    assert!(
        sqlx::query("UPDATE agent_feed_items SET lifecycle_state='REJECTED' WHERE id='b'")
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    assert_eq!(ids(&f.get("/").await), ["a", "b", "c"]);
    assert!(ids(&f.get("/activity").await).is_empty());
}
#[tokio::test]
async fn primed_owners_keep_separate_pending_and_activity_rows() {
    let f = Fixture::new().await;
    f.get("/").await;
    assert_eq!(ids(&f.get_as("/", "other-tenant").await.1), ["foreign"]);
    f.reject("b").await;
    assert_eq!(ids(&f.get("/").await), ["a", "c"]);
    assert_eq!(ids(&f.get_as("/", "other-tenant").await.1), ["foreign"]);
    assert!(ids(&f.get_as("/activity", "other-tenant").await.1).is_empty());
}
#[tokio::test]
async fn mutable_list_responses_are_private_no_store() {
    let f = Fixture::new().await;
    for path in ["/", "/?cursor=a&limit=1&mobile_optimized=true", "/activity"] {
        let (headers, _) = f.get_as(path, &f.tenant).await;
        assert_eq!(
            headers.get("cache-control").and_then(|v| v.to_str().ok()),
            Some("private, no-store")
        );
    }
}
