//! Real HTTP/auth/database regressions for both compatibility triage routes.
//! This module shares only fixture setup with the canonical feed tests.
use super::{Fixture, json};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use sea_orm::{ConnectionTrait, Schema};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

enum Database {
    Postgres(Fixture),
    Sqlite(sqlx::SqlitePool),
}

struct LegacyFixture {
    database: Database,
    data_override: Option<sqlx::PgPool>,
    auth: Arc<server_auth::Store>,
    owner: String,
    member: String,
}

impl LegacyFixture {
    async fn new(postgres: bool) -> Self {
        if postgres {
            let fixture = Fixture::new().await;
            sqlx::raw_sql(include_str!("legacy_schema.sql"))
                .execute(&fixture.owner)
                .await
                .unwrap();
            sqlx::raw_sql(&format!(
                "GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {} TO {},ohc_bypassrls",
                fixture.schema, fixture.role
            ))
            .execute(&fixture.owner)
            .await
            .unwrap();
            return Self {
                data_override: None,
                auth: fixture.auth.clone(),
                owner: fixture.token.clone(),
                member: fixture.member.clone(),
                database: Database::Postgres(fixture),
            };
        }

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .in_memory(true)
                    .foreign_keys(true),
            )
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("legacy_sqlite_schema.sql"))
            .execute(&pool)
            .await
            .unwrap();
        let orm = sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        let backend = sea_orm::DatabaseBackend::Sqlite;
        for statement in [
            Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::identity_user_role::Entity,
            ),
            Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::revoked_token::Entity,
            ),
        ] {
            orm.execute(backend.build(&statement)).await.unwrap();
        }
        sqlx::raw_sql("INSERT INTO tenants(id,name) VALUES('tenant-a','Synthetic A'),('tenant-b','Synthetic B')")
            .execute(&pool).await.unwrap();
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(orm),
        )));
        let mut tokens = Vec::new();
        for (id, role) in [("owner-a", "ADMIN"), ("member-a", "MEMBER")] {
            let user = server_auth::User {
                id: id.into(),
                username: id.into(),
                email: format!("{id}@example.test"),
                password_hash: "unused-fixture".into(),
                roles: vec![role.into()],
                active: true,
                organization_id: Some("tenant-a".into()),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                oidc_subject: None,
            };
            sqlx::query(
                "INSERT INTO users(id,username,email,tenant_id) VALUES(?1,?1,?2,'tenant-a')",
            )
            .bind(id)
            .bind(&user.email)
            .execute(&pool)
            .await
            .unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES(?1,?2,'tenant-a',0)")
                .bind(id).bind(role).execute(&pool).await.unwrap();
            tokens.push(auth.issue_token(&user).unwrap());
        }
        Self {
            data_override: None,
            database: Database::Sqlite(pool),
            auth,
            owner: tokens.remove(0),
            member: tokens.remove(0),
        }
    }

    fn app(&self) -> Router {
        let db = match &self.database {
            Database::Postgres(fixture) => crate::db::DB {
                pool: self
                    .data_override
                    .clone()
                    .unwrap_or_else(|| fixture.pool.clone()),
                store: crate::db::DbStore::Postgres,
            },
            Database::Sqlite(pool) => crate::db::DB {
                // The SQLite handler must never connect to this lazy PG pool.
                pool: sqlx::postgres::PgPoolOptions::new()
                    .connect_lazy("postgres://unused:unused@127.0.0.1:1/ohc_unused_test")
                    .unwrap(),
                store: crate::db::DbStore::Sqlite(pool.clone()),
            },
        };
        crate::legacy_triage::router()
            .with_state(Arc::new(db))
            .layer(axum::extract::Extension(self.auth.clone()))
            .route_layer(axum::middleware::from_fn_with_state(
                self.auth.clone(),
                server_auth::strict_bearer_auth_middleware,
            ))
    }

    async fn call(&self, route: &str, token: &str, body: Value) -> (StatusCode, Value) {
        // Reconstruct the router each time; replay cannot depend on handler memory.
        let response = self
            .app()
            .oneshot(
                Request::builder()
                    .uri(route)
                    .method("POST")
                    .header("authorization", format!("Bearer {token}"))
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    async fn decide(
        &self,
        token: &str,
        id: &str,
        approved: bool,
        edit: Option<&str>,
    ) -> (StatusCode, Value) {
        self.call(
            "/api/v1/triage/action",
            token,
            json!({"triage_item_id":id,"approved":approved,"edited_payload":edit}),
        )
        .await
    }

    async fn execute(&self, statement: &str) {
        match &self.database {
            Database::Postgres(fixture) => {
                sqlx::raw_sql(statement)
                    .execute(&fixture.owner)
                    .await
                    .unwrap();
            }
            Database::Sqlite(pool) => {
                sqlx::raw_sql(statement).execute(pool).await.unwrap();
            }
        }
    }

    async fn count(&self, table: &str) -> i64 {
        let statement = format!("SELECT count(*) FROM {table}");
        match &self.database {
            Database::Postgres(fixture) => sqlx::query_scalar(&statement)
                .fetch_one(&fixture.owner)
                .await
                .unwrap(),
            Database::Sqlite(pool) => sqlx::query_scalar(&statement)
                .fetch_one(pool)
                .await
                .unwrap(),
        }
    }

    async fn text(&self, statement: &str) -> String {
        match &self.database {
            Database::Postgres(fixture) => sqlx::query_scalar(statement)
                .fetch_one(&fixture.owner)
                .await
                .unwrap(),
            Database::Sqlite(pool) => sqlx::query_scalar(statement).fetch_one(pool).await.unwrap(),
        }
    }

    async fn seed_legacy(&self, id: &str, tenant: &str, action_type: &str) {
        // All values are fixed synthetic fixtures, never user/provider input.
        self.execute(&format!("INSERT INTO triage_items(id,tenant_id,source,status) VALUES('{id}','{tenant}','contract','pending');\
            INSERT INTO triage_proposed_actions(id,triage_item_id,tenant_id,action_type,payload) VALUES('action-{id}','{id}','{tenant}','{action_type}','original legacy draft')")).await;
    }

    async fn seed_daily(&self, id: &str, tenant: &str) {
        let actions = json!([
            {"action_type":"Draft Reply","message":"original draft","recipient":"local-fixture","metadata":{"retain":true}},
            {"action_type":"Review Context","message":"untouched second action"}
        ]);
        match &self.database {
            Database::Postgres(fixture) => {
                sqlx::query("INSERT INTO daily_work_items(id,tenant_id,intent,suggested_actions) VALUES($1,$2,'Review draft',$3)")
                    .bind(id).bind(tenant).bind(actions).execute(&fixture.owner).await.unwrap();
            }
            Database::Sqlite(pool) => {
                sqlx::query("INSERT INTO daily_work_items(id,tenant_id,intent,suggested_actions) VALUES(?1,?2,'Review draft',?3)")
                    .bind(id).bind(tenant).bind(actions.to_string()).execute(pool).await.unwrap();
            }
        }
    }

    async fn daily(&self, id: &str) -> (String, Value) {
        let statement =
            "SELECT status,CAST(suggested_actions AS TEXT) FROM daily_work_items WHERE id=$1";
        let (status, actions): (String, String) = match &self.database {
            Database::Postgres(fixture) => sqlx::query_as(statement)
                .bind(id)
                .fetch_one(&fixture.owner)
                .await
                .unwrap(),
            Database::Sqlite(pool) => sqlx::query_as(statement)
                .bind(id)
                .fetch_one(pool)
                .await
                .unwrap(),
        };
        (status, serde_json::from_str(&actions).unwrap())
    }

    async fn receipt(&self, id: &str) -> Value {
        let statement = "SELECT CAST(receipt AS TEXT) FROM legacy_triage_decisions WHERE tenant_id='tenant-a' AND action_id=$1";
        let value: String = match &self.database {
            Database::Postgres(fixture) => sqlx::query_scalar(statement)
                .bind(id)
                .fetch_one(&fixture.owner)
                .await
                .unwrap(),
            Database::Sqlite(pool) => sqlx::query_scalar(statement)
                .bind(id)
                .fetch_one(pool)
                .await
                .unwrap(),
        };
        serde_json::from_str(&value).unwrap()
    }

    async fn finish(self) {
        if let Some(pool) = self.data_override {
            pool.close().await;
        }
        match self.database {
            Database::Postgres(fixture) => fixture.finish().await,
            Database::Sqlite(pool) => pool.close().await,
        }
    }
}

fn recorded(body: &Value, id: &str, state: &str) {
    assert_eq!(body["decision_recorded"], true, "{body}");
    assert_eq!(body["item"]["id"], id, "{body}");
    assert_eq!(body["item"]["tenant_id"], "tenant-a", "{body}");
    assert_eq!(body["item"]["lifecycle_state"], state, "{body}");
    assert_eq!(body["dispatch"]["status"], "NOT_REQUESTED", "{body}");
}

async fn owner_edit_survives_reload_and_replay(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("edited", "tenant-a").await;
    let request = json!({"triage_item_id":"edited","approved":true,"edited_payload":"owner edit"});
    let (status, body) = f
        .call("/api/v1/ui/triage/action", &f.owner, request.clone())
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    recorded(&body, "edited", "APPROVED");
    assert_eq!(body["item"]["edited_payload"], "owner edit");
    let (state, actions) = f.daily("edited").await;
    assert_eq!(state, "APPROVED");
    assert_eq!(actions[0]["message"], "owner edit");
    assert_eq!(actions[0]["recipient"], "local-fixture");
    assert_eq!(actions[0]["metadata"]["retain"], true);
    assert_eq!(actions[1]["message"], "untouched second action");
    assert_eq!(f.receipt("edited").await, body);
    let (replay_status, replay) = f.call("/api/v1/triage/action", &f.owner, request).await;
    assert_eq!(replay_status, StatusCode::OK, "{replay}");
    assert_eq!(replay, body);
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    assert_eq!(f.daily("edited").await, (state, actions));
    if let Database::Postgres(fixture) = &f.database {
        assert_eq!(fixture.jobs().await, 0);
    }
    f.finish().await;
}

async fn current_authority_required_for_both_aliases(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("authority", "tenant-a").await;
    f.seed_legacy("legacy-authority", "tenant-a", "SocialPostDraft")
        .await;
    for route in ["/api/v1/triage/action", "/api/v1/ui/triage/action"] {
        for id in ["authority", "legacy-authority"] {
            let (status, body) = f
                .call(
                    route,
                    &f.member,
                    json!({"triage_item_id":id,"approved":true}),
                )
                .await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
        }
    }
    f.execute("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='owner-a'")
        .await;
    for approved in [true, false] {
        for id in ["authority", "legacy-authority"] {
            assert_eq!(
                f.decide(&f.owner, id, approved, None).await.0,
                StatusCode::FORBIDDEN
            );
        }
    }
    assert_eq!(f.daily("authority").await.0, "PENDING");
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='legacy-authority'")
            .await,
        "pending"
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    f.finish().await;
}

async fn missing_and_foreign_ids_cannot_acknowledge_success(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("foreign", "tenant-b").await;
    for id in ["foreign", "missing"] {
        for approved in [true, false] {
            let (status, body) = f.decide(&f.owner, id, approved, Some("tampered")).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{id}: {body}");
            assert_ne!(body["decision_recorded"], true);
        }
    }
    assert_eq!(f.daily("foreign").await.0, "PENDING");
    assert_eq!(f.daily("foreign").await.1[0]["message"], "original draft");
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    f.finish().await;
}

async fn dismissal_is_durable_without_dispatch(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("dismissed", "tenant-a").await;
    let before = f.daily("dismissed").await.1;
    let (status, body) = f.decide(&f.owner, "dismissed", false, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    recorded(&body, "dismissed", "DISMISSED");
    assert_eq!(f.daily("dismissed").await, ("DISMISSED".into(), before));
    assert_eq!(f.receipt("dismissed").await, body);
    assert_eq!(
        f.decide(&f.owner, "dismissed", false, None).await,
        (StatusCode::OK, body)
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    if let Database::Postgres(fixture) = &f.database {
        assert_eq!(fixture.jobs().await, 0);
    }
    f.finish().await;
}

async fn replay_cannot_replace_saved_decision_or_edit(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("replay", "tenant-a").await;
    let (status, body) = f
        .decide(&f.owner, "replay", true, Some("accepted edit"))
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    for (approved, edit) in [(true, Some("replacement")), (false, None)] {
        let (status, conflict) = f.decide(&f.owner, "replay", approved, edit).await;
        assert_eq!(status, StatusCode::CONFLICT, "{conflict}");
    }
    assert_eq!(f.daily("replay").await.1[0]["message"], "accepted edit");
    assert_eq!(f.receipt("replay").await, body);
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    f.finish().await;
}

async fn failed_commit_rolls_back_edit_and_receipt(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("commit", "tenant-a").await;
    match &f.database {
        Database::Postgres(fixture) => fixture.fail_on("daily_work_items", "UPDATE OF status", true).await,
        Database::Sqlite(_) => f.execute(
            "CREATE TABLE commit_parent(id INTEGER PRIMARY KEY);\
             CREATE TABLE commit_guard(id INTEGER REFERENCES commit_parent(id) DEFERRABLE INITIALLY DEFERRED);\
             CREATE TRIGGER reject_legacy_commit AFTER UPDATE ON daily_work_items \
             BEGIN INSERT INTO commit_guard VALUES(99); END;"
        ).await,
    }
    let (status, body) = f
        .decide(&f.owner, "commit", true, Some("must roll back"))
        .await;
    assert!(
        !status.is_success(),
        "Uncommitted decision acknowledged: {status}: {body}"
    );
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(
        body["decision_recorded"],
        Value::Null,
        "Unconfirmed persistence must not claim a confirmed rejection"
    );
    assert_eq!(f.daily("commit").await.0, "PENDING");
    assert_eq!(f.daily("commit").await.1[0]["message"], "original draft");
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    if let Database::Postgres(fixture) = &f.database {
        assert_eq!(fixture.jobs().await, 0);
    }
    f.finish().await;
}

async fn historical_terminal_item_requires_reconciliation(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_daily("historical", "tenant-a").await;
    f.execute("UPDATE daily_work_items SET status='APPROVED' WHERE id='historical'")
        .await;
    let (status, body) = f.decide(&f.owner, "historical", true, None).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert_ne!(body["decision_recorded"], true);
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    f.finish().await;
}

async fn legacy_fallback_persists_edit_and_exact_receipt(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_legacy("legacy", "tenant-a", "SocialPostDraft").await;
    let (status, body) = f
        .decide(&f.owner, "legacy", true, Some("edited social draft"))
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    recorded(&body, "legacy", "APPROVED");
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='legacy'")
            .await,
        "resolved"
    );
    assert_eq!(
        f.text("SELECT payload FROM triage_proposed_actions WHERE triage_item_id='legacy'")
            .await,
        "edited social draft"
    );
    assert_eq!(f.receipt("legacy").await, body);
    assert_eq!(
        f.decide(&f.owner, "legacy", true, Some("edited social draft"))
            .await,
        (StatusCode::OK, body)
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    if let Database::Postgres(fixture) = &f.database {
        assert_eq!(fixture.jobs().await, 0);
    }
    f.finish().await;
}

async fn legacy_fallback_checks_tenant_and_dismisses_without_effect(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_legacy("owned", "tenant-a", "Draft Reply").await;
    f.seed_legacy("foreign-legacy", "tenant-b", "Draft Reply")
        .await;
    for approved in [true, false] {
        assert_eq!(
            f.decide(&f.owner, "foreign-legacy", approved, None).await.0,
            StatusCode::NOT_FOUND
        );
    }
    let (status, body) = f.decide(&f.owner, "owned", false, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    recorded(&body, "owned", "DISMISSED");
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='owned'")
            .await,
        "dismissed"
    );
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='foreign-legacy'")
            .await,
        "pending"
    );
    assert_eq!(
        f.text("SELECT payload FROM triage_proposed_actions WHERE triage_item_id='owned'")
            .await,
        "original legacy draft"
    );
    assert_eq!(f.receipt("owned").await, body);
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    if let Database::Postgres(fixture) = &f.database {
        assert_eq!(fixture.jobs().await, 0);
    }
    f.finish().await;
}

async fn legacy_update_failure_cannot_acknowledge_or_save_receipt(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_legacy("update-fail", "tenant-a", "SocialPostDraft")
        .await;
    match &f.database {
        Database::Postgres(fixture) => fixture.fail_on("triage_items", "UPDATE", false).await,
        Database::Sqlite(_) => f.execute("CREATE TRIGGER reject_legacy_update BEFORE UPDATE ON triage_items BEGIN SELECT RAISE(ABORT,'controlled fixture failure'); END;").await,
    }
    let (status, body) = f
        .decide(&f.owner, "update-fail", true, Some("unsaved edit"))
        .await;
    assert!(
        !status.is_success(),
        "Failed source UPDATE acknowledged: {status}: {body}"
    );
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(
        body["decision_recorded"],
        Value::Null,
        "Unconfirmed persistence must not claim a confirmed rejection"
    );
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='update-fail'")
            .await,
        "pending"
    );
    assert_eq!(
        f.text("SELECT payload FROM triage_proposed_actions WHERE triage_item_id='update-fail'")
            .await,
        "original legacy draft"
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    f.finish().await;
}

async fn concurrent_legacy_replay_records_once(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_legacy("concurrent", "tenant-a", "SocialPostDraft")
        .await;
    let (first, second) = tokio::join!(
        f.decide(&f.owner, "concurrent", true, Some("one draft")),
        f.decide(&f.owner, "concurrent", true, Some("one draft"))
    );
    assert_eq!(first.0, StatusCode::OK, "{}", first.1);
    assert_eq!(second.0, StatusCode::OK, "{}", second.1);
    assert_eq!(first.1, second.1);
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    assert_eq!(f.receipt("concurrent").await, first.1);
    f.finish().await;
}

async fn legacy_deferred_commit_failure_preserves_pending_card(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.seed_legacy("legacy-commit", "tenant-a", "SocialPostDraft")
        .await;
    match &f.database {
        Database::Postgres(fixture) => fixture.fail_on("triage_items", "UPDATE OF status", true).await,
        Database::Sqlite(_) => f.execute(
            "CREATE TABLE commit_parent(id INTEGER PRIMARY KEY);\
             CREATE TABLE commit_guard(id INTEGER REFERENCES commit_parent(id) DEFERRABLE INITIALLY DEFERRED);\
             CREATE TRIGGER reject_legacy_commit AFTER UPDATE ON triage_items \
             BEGIN INSERT INTO commit_guard VALUES(99); END;"
        ).await,
    }
    let (status, body) = f
        .decide(&f.owner, "legacy-commit", true, Some("must roll back"))
        .await;
    assert!(
        !status.is_success(),
        "Uncommitted legacy decision acknowledged: {status}: {body}"
    );
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(
        body["decision_recorded"],
        Value::Null,
        "Unconfirmed persistence must not claim a confirmed rejection"
    );
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='legacy-commit'")
            .await,
        "pending"
    );
    assert_eq!(
        f.text("SELECT payload FROM triage_proposed_actions WHERE triage_item_id='legacy-commit'")
            .await,
        "original legacy draft"
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    f.finish().await;
}

async fn approved_inbox_draft_does_not_reappear_in_pending_projection(postgres: bool) {
    let f = LegacyFixture::new(postgres).await;
    f.execute("INSERT INTO omni_inbox_messages(id,tenant_id,source,original_content,translated_content,target_language,draft_reply,status)\
        VALUES('inbox-a','tenant-a','Synthetic inbox','Question','Question','en','Original answer','unread'),\
        ('inbox-b','tenant-b','Synthetic inbox','Foreign question','Foreign question','en','Foreign answer','unread')").await;
    let pending = match &f.database {
        Database::Postgres(fixture) => sqlx::query(crate::LEGACY_INBOX_PENDING_PG)
            .bind("tenant-a")
            .fetch_all(&fixture.owner)
            .await
            .unwrap()
            .len(),
        Database::Sqlite(pool) => sqlx::query(crate::LEGACY_INBOX_PENDING_SQLITE)
            .bind("tenant-a")
            .fetch_all(pool)
            .await
            .unwrap()
            .len(),
    };
    assert_eq!(
        pending, 1,
        "The production pending SQL must discover the seeded item"
    );
    let (status, body) = f
        .decide(&f.owner, "inbox-a", true, Some("Owner answer"))
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    recorded(&body, "inbox-a", "APPROVED");
    assert_eq!(
        f.text("SELECT status FROM omni_inbox_messages WHERE id='inbox-a'")
            .await,
        "resolved"
    );
    assert_eq!(
        f.text("SELECT draft_reply FROM omni_inbox_messages WHERE id='inbox-a'")
            .await,
        "Owner answer"
    );
    assert_eq!(
        f.text("SELECT status FROM omni_inbox_messages WHERE id='inbox-b'")
            .await,
        "unread"
    );
    let pending = match &f.database {
        Database::Postgres(fixture) => sqlx::query(crate::LEGACY_INBOX_PENDING_PG)
            .bind("tenant-a")
            .fetch_all(&fixture.owner)
            .await
            .unwrap()
            .len(),
        Database::Sqlite(pool) => sqlx::query(crate::LEGACY_INBOX_PENDING_SQLITE)
            .bind("tenant-a")
            .fetch_all(pool)
            .await
            .unwrap()
            .len(),
    };
    assert_eq!(
        pending, 0,
        "A saved decision must not become a new pending inbox card on reload"
    );
    assert_eq!(f.receipt("inbox-a").await, body);
    f.finish().await;
}

async fn product_booking_alias(postgres: bool, foreign: bool) {
    let payload = json!({"customer_id":"customer-a","product_id":if foreign {"product-b"} else {"product-a"},
        "start_time":"2026-10-10T09:00:00Z","end_time":"2026-10-10T10:00:00Z"});
    let f = if postgres {
        postgres_local_fixture("product-booking", "Draft Booking", payload).await
    } else {
        local_fixture("product-booking", "Draft Booking", payload).await
    };
    f.execute("INSERT INTO products(id,tenant_id,name) VALUES('product-a','tenant-a','Synthetic product'),('product-b','tenant-b','Foreign product')").await;
    let (status, body) = f.decide(&f.owner, "product-booking", true, None).await;
    if foreign {
        assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
        assert_eq!(f.count("bookings").await, 0);
        assert_eq!(f.count("legacy_triage_decisions").await, 0);
        assert_eq!(
            f.text("SELECT status FROM triage_items WHERE id='product-booking'")
                .await,
            "pending"
        );
    } else {
        assert_eq!(status, StatusCode::OK, "{body}");
        local_recorded(&body, "product-booking");
        // Both original legacy handlers accepted product_id as the service_id alias.
        assert_eq!(
            f.text("SELECT service_id FROM bookings WHERE tenant_id='tenant-a'")
                .await,
            "product-a"
        );
        assert_eq!(
            f.decide(&f.owner, "product-booking", true, None).await,
            (StatusCode::OK, body)
        );
        assert_eq!(f.count("bookings").await, 1);
    }
    f.finish().await;
}

#[tokio::test]
async fn sqlite_product_id_booking_alias_preserves_local_reference_and_replay() {
    product_booking_alias(false, false).await;
}
#[tokio::test]
async fn postgres_product_id_booking_alias_preserves_local_reference_and_replay() {
    product_booking_alias(true, false).await;
}
#[tokio::test]
async fn sqlite_product_id_booking_alias_rejects_foreign_reference() {
    product_booking_alias(false, true).await;
}
#[tokio::test]
async fn postgres_product_id_booking_alias_rejects_foreign_reference() {
    product_booking_alias(true, true).await;
}

macro_rules! backend_cases {
    ($module:ident, $postgres:expr) => {
        mod $module {
            #[tokio::test]
            async fn owner_edit_survives_reload_and_replay() {
                super::owner_edit_survives_reload_and_replay($postgres).await;
            }
            #[tokio::test]
            async fn current_authority_required_for_both_aliases() {
                super::current_authority_required_for_both_aliases($postgres).await;
            }
            #[tokio::test]
            async fn missing_and_foreign_ids_cannot_acknowledge_success() {
                super::missing_and_foreign_ids_cannot_acknowledge_success($postgres).await;
            }
            #[tokio::test]
            async fn dismissal_is_durable_without_dispatch() {
                super::dismissal_is_durable_without_dispatch($postgres).await;
            }
            #[tokio::test]
            async fn replay_cannot_replace_saved_decision_or_edit() {
                super::replay_cannot_replace_saved_decision_or_edit($postgres).await;
            }
            #[tokio::test]
            async fn failed_commit_rolls_back_edit_and_receipt() {
                super::failed_commit_rolls_back_edit_and_receipt($postgres).await;
            }
            #[tokio::test]
            async fn historical_terminal_item_requires_reconciliation() {
                super::historical_terminal_item_requires_reconciliation($postgres).await;
            }
            #[tokio::test]
            async fn legacy_fallback_persists_edit_and_exact_receipt() {
                super::legacy_fallback_persists_edit_and_exact_receipt($postgres).await;
            }
            #[tokio::test]
            async fn legacy_fallback_checks_tenant_and_dismisses_without_effect() {
                super::legacy_fallback_checks_tenant_and_dismisses_without_effect($postgres).await;
            }
            #[tokio::test]
            async fn legacy_update_failure_cannot_acknowledge_or_save_receipt() {
                super::legacy_update_failure_cannot_acknowledge_or_save_receipt($postgres).await;
            }
            #[tokio::test]
            async fn concurrent_legacy_replay_records_once() {
                super::concurrent_legacy_replay_records_once($postgres).await;
            }
            #[tokio::test]
            async fn legacy_deferred_commit_failure_preserves_pending_card() {
                super::legacy_deferred_commit_failure_preserves_pending_card($postgres).await;
            }
            #[tokio::test]
            async fn approved_inbox_draft_does_not_reappear_in_pending_projection() {
                super::approved_inbox_draft_does_not_reappear_in_pending_projection($postgres)
                    .await;
            }
        }
    };
}

backend_cases!(postgres, true);
backend_cases!(sqlite, false);

async fn local_fixture(id: &str, kind: &str, payload: Value) -> LegacyFixture {
    let f = LegacyFixture::new(false).await;
    f.execute("INSERT INTO customers(id,tenant_id,name) VALUES('customer-a','tenant-a','Synthetic customer'),('customer-b','tenant-b','Foreign customer');\
        INSERT INTO services(id,tenant_id,title) VALUES('service-a','tenant-a','Synthetic service'),('service-b','tenant-b','Foreign service');\
        INSERT INTO staff_profiles(id,tenant_id,name) VALUES('staff-a','tenant-a','Synthetic staff'),('staff-b','tenant-b','Foreign staff');\
        INSERT INTO shifts(id,tenant_id,staff_id,start_time,end_time,role) VALUES('shift-a','tenant-a','original-staff','2026-10-10T09:00:00Z','2026-10-10T10:00:00Z','Synthetic role')").await;
    f.seed_legacy(id, "tenant-a", kind).await;
    let Database::Sqlite(pool) = &f.database else {
        unreachable!()
    };
    sqlx::query("UPDATE triage_proposed_actions SET payload=?1 WHERE triage_item_id=?2")
        .bind(payload.to_string())
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    f
}

fn quote_payload(customer: &str) -> Value {
    json!({"client_id":customer,"total_amount_cents":2400,"required_deposit_cents":0,
        "line_items":[{"description":"Synthetic item","quantity":2,"unit_price_cents":1200,"is_optional":false}]})
}

fn booking_payload(service: &str) -> Value {
    json!({"customer_id":"customer-a","service_id":service,
        "start_time":"2026-10-10T09:00:00Z","end_time":"2026-10-10T10:00:00Z"})
}

fn local_recorded<'a>(body: &'a Value, id: &str) -> &'a str {
    assert_eq!(body["decision_recorded"], true, "{body}");
    assert_eq!(body["item"]["id"], id, "{body}");
    assert_eq!(body["item"]["lifecycle_state"], "APPROVED", "{body}");
    assert_eq!(body["dispatch"]["status"], "LOCAL_COMMITTED", "{body}");
    body["dispatch"]["receipt_id"]
        .as_str()
        .filter(|id| !id.is_empty())
        .expect("durable local effect identity")
}

#[tokio::test]
async fn sqlite_quote_effect_and_lines_commit_once_with_receipt() {
    let f = local_fixture("quote", "Draft Quote", quote_payload("customer-a")).await;
    let (status, body) = f.decide(&f.owner, "quote", true, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let id = local_recorded(&body, "quote");
    let Database::Sqlite(pool) = &f.database else {
        unreachable!()
    };
    let row: (String, String, i64, String) = sqlx::query_as(
        "SELECT tenant_id,customer_id,total_amount_cents,status FROM quotes WHERE id=?1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        row,
        ("tenant-a".into(), "customer-a".into(), 2400, "DRAFT".into())
    );
    let line: (String, i64, i64, bool) = sqlx::query_as("SELECT description,quantity,unit_price_cents,is_optional FROM quote_line_items WHERE quote_id=?1")
        .bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(line, ("Synthetic item".into(), 2, 1200, false));
    assert_eq!(f.receipt("quote").await, body);
    assert_eq!(
        f.decide(&f.owner, "quote", true, None).await,
        (StatusCode::OK, body)
    );
    assert_eq!(f.count("quotes").await, 1);
    assert_eq!(f.count("quote_line_items").await, 1);
    f.finish().await;
}

#[tokio::test]
async fn sqlite_quote_cannot_reference_foreign_customer() {
    let f = local_fixture("quote-foreign", "Draft Quote", quote_payload("customer-b")).await;
    let (status, body) = f.decide(&f.owner, "quote-foreign", true, None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(f.count("quotes").await, 0);
    assert_eq!(f.count("quote_line_items").await, 0);
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='quote-foreign'")
            .await,
        "pending"
    );
    f.finish().await;
}

#[tokio::test]
async fn sqlite_quote_effect_rolls_back_when_receipt_commit_fails() {
    let f = local_fixture("quote-failure", "Draft Quote", quote_payload("customer-a")).await;
    f.execute("CREATE TABLE commit_parent(id INTEGER PRIMARY KEY);\
        CREATE TABLE commit_guard(id INTEGER REFERENCES commit_parent(id) DEFERRABLE INITIALLY DEFERRED);\
        CREATE TRIGGER reject_effect_commit AFTER INSERT ON legacy_triage_decisions BEGIN INSERT INTO commit_guard VALUES(99); END;").await;
    let (status, body) = f.decide(&f.owner, "quote-failure", true, None).await;
    assert!(!status.is_success(), "{status}: {body}");
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(
        body["decision_recorded"],
        Value::Null,
        "Unconfirmed persistence must not claim a confirmed rejection"
    );
    assert_eq!(f.count("quotes").await, 0);
    assert_eq!(f.count("quote_line_items").await, 0);
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='quote-failure'")
            .await,
        "pending"
    );
    f.finish().await;
}

#[tokio::test]
async fn sqlite_booking_effect_commits_once_with_receipt() {
    let f = local_fixture("booking", "Draft Booking", booking_payload("service-a")).await;
    let (status, body) = f.decide(&f.owner, "booking", true, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let id = local_recorded(&body, "booking");
    let Database::Sqlite(pool) = &f.database else {
        unreachable!()
    };
    let row: (String, String, String, String) =
        sqlx::query_as("SELECT tenant_id,customer_id,service_id,status FROM bookings WHERE id=?1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        row,
        (
            "tenant-a".into(),
            "customer-a".into(),
            "service-a".into(),
            "scheduled".into()
        )
    );
    assert_eq!(f.receipt("booking").await, body);
    assert_eq!(
        f.decide(&f.owner, "booking", true, None).await,
        (StatusCode::OK, body)
    );
    assert_eq!(f.count("bookings").await, 1);
    f.finish().await;
}

#[tokio::test]
async fn sqlite_booking_cannot_reference_foreign_service() {
    let f = local_fixture(
        "booking-foreign",
        "Draft Booking",
        booking_payload("service-b"),
    )
    .await;
    let (status, body) = f.decide(&f.owner, "booking-foreign", true, None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(f.count("bookings").await, 0);
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='booking-foreign'")
            .await,
        "pending"
    );
    f.finish().await;
}

#[tokio::test]
async fn sqlite_shift_reassignment_commits_once_with_receipt() {
    let f = local_fixture(
        "shift",
        "Reassign Shift",
        json!({"shift_id":"shift-a","new_staff_id":"staff-a"}),
    )
    .await;
    let (status, body) = f.decide(&f.owner, "shift", true, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(local_recorded(&body, "shift"), "shift-a");
    assert_eq!(
        f.text("SELECT staff_id FROM shifts WHERE id='shift-a'")
            .await,
        "staff-a"
    );
    assert_eq!(f.receipt("shift").await, body);
    assert_eq!(
        f.decide(&f.owner, "shift", true, None).await,
        (StatusCode::OK, body)
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    f.finish().await;
}

#[tokio::test]
async fn sqlite_shift_cannot_reference_foreign_staff() {
    let f = local_fixture(
        "shift-foreign",
        "Reassign Shift",
        json!({"shift_id":"shift-a","new_staff_id":"staff-b"}),
    )
    .await;
    let (status, body) = f.decide(&f.owner, "shift-foreign", true, None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(
        f.text("SELECT staff_id FROM shifts WHERE id='shift-a'")
            .await,
        "original-staff"
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='shift-foreign'")
            .await,
        "pending"
    );
    f.finish().await;
}

#[test]
fn every_pending_task_projection_excludes_recorded_terminal_decisions() {
    assert_eq!(crate::LEGACY_TASK_READ_QUERIES.len(), 8);
    for query in crate::LEGACY_TASK_READ_QUERIES {
        assert!(
            query.contains("status NOT IN ('COMPLETED', 'APPROVED', 'DISMISSED')"),
            "Recorded decisions must remain absent on pending reload: {query}"
        );
        assert!(query.contains("tenant_id = "), "{query}");
    }
}

async fn postgres_local_fixture(id: &str, kind: &str, payload: Value) -> LegacyFixture {
    let f = LegacyFixture::new(true).await;
    f.execute("INSERT INTO customers(id,tenant_id,name) VALUES('customer-a','tenant-a','Synthetic customer'),('customer-b','tenant-b','Foreign customer');\
        INSERT INTO services(id,tenant_id,name) VALUES('service-a','tenant-a','Synthetic service'),('service-b','tenant-b','Foreign service');\
        INSERT INTO ohc_staff_member(id,tenant_id,name,phone_number,role) VALUES('staff-a','tenant-a','Synthetic staff','fixture-a','member'),('staff-b','tenant-b','Foreign staff','fixture-b','member');\
        INSERT INTO shifts(id,tenant_id,staff_id,start_time,end_time,role) VALUES('shift-a','tenant-a','original-staff','2026-10-10T09:00:00Z','2026-10-10T10:00:00Z','Synthetic role')").await;
    f.seed_legacy(id, "tenant-a", kind).await;
    let Database::Postgres(fixture) = &f.database else {
        unreachable!()
    };
    sqlx::query("UPDATE triage_proposed_actions SET payload=$1 WHERE triage_item_id=$2")
        .bind(payload.to_string())
        .bind(id)
        .execute(&fixture.owner)
        .await
        .unwrap();
    f
}

async fn postgres_local_effect(kind: &str, payload: Value, table: &str) {
    let f = postgres_local_fixture("local-effect", kind, payload).await;
    let (status, body) = f.decide(&f.owner, "local-effect", true, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let id = local_recorded(&body, "local-effect");
    let Database::Postgres(fixture) = &f.database else {
        unreachable!()
    };
    match table {
        "quotes" => {
            let row: (String, String, i64, String) = sqlx::query_as(
                "SELECT tenant_id,customer_id,total_amount_cents,status FROM quotes WHERE id=$1",
            )
            .bind(id)
            .fetch_one(&fixture.owner)
            .await
            .unwrap();
            assert_eq!(
                row,
                ("tenant-a".into(), "customer-a".into(), 2400, "DRAFT".into())
            );
            let line: (String, i64, i64, bool, String) = sqlx::query_as("SELECT description,CAST(quantity AS BIGINT),unit_price_cents,is_optional,tenant_id FROM quote_line_items WHERE quote_id=$1")
                .bind(id).fetch_one(&fixture.owner).await.unwrap();
            assert_eq!(
                line,
                ("Synthetic item".into(), 2, 1200, false, "tenant-a".into())
            );
        }
        "bookings" => {
            let row: (String, String, String, String) = sqlx::query_as(
                "SELECT tenant_id,customer_id,service_id,status FROM bookings WHERE id=$1",
            )
            .bind(id)
            .fetch_one(&fixture.owner)
            .await
            .unwrap();
            assert_eq!(
                row,
                (
                    "tenant-a".into(),
                    "customer-a".into(),
                    "service-a".into(),
                    "scheduled".into()
                )
            );
        }
        "shifts" => {
            assert_eq!(id, "shift-a");
            assert_eq!(
                f.text("SELECT staff_id FROM shifts WHERE id='shift-a'")
                    .await,
                "staff-a"
            );
        }
        _ => panic!("Unsupported fixture effect"),
    }
    assert_eq!(f.receipt("local-effect").await, body);
    assert_eq!(
        f.decide(&f.owner, "local-effect", true, None).await,
        (StatusCode::OK, body)
    );
    assert_eq!(f.count(table).await, 1);
    assert_eq!(f.count("legacy_triage_decisions").await, 1);
    assert_eq!(fixture.jobs().await, 0);
    f.finish().await;
}

#[tokio::test]
async fn postgres_quote_effect_and_lines_commit_once_with_receipt() {
    postgres_local_effect("Draft Quote", quote_payload("customer-a"), "quotes").await;
}

#[tokio::test]
async fn postgres_booking_effect_commits_once_with_receipt() {
    postgres_local_effect("Draft Booking", booking_payload("service-a"), "bookings").await;
}

#[tokio::test]
async fn postgres_shift_reassignment_commits_once_with_receipt() {
    postgres_local_effect(
        "Reassign Shift",
        json!({"shift_id":"shift-a","new_staff_id":"staff-a"}),
        "shifts",
    )
    .await;
}

async fn postgres_foreign_reference(kind: &str, payload: Value) {
    let f = postgres_local_fixture("foreign-ref", kind, payload).await;
    let (status, body) = f.decide(&f.owner, "foreign-ref", true, None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(f.count("quotes").await, 0);
    assert_eq!(f.count("quote_line_items").await, 0);
    assert_eq!(f.count("bookings").await, 0);
    assert_eq!(
        f.text("SELECT staff_id FROM shifts WHERE id='shift-a'")
            .await,
        "original-staff"
    );
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='foreign-ref'")
            .await,
        "pending"
    );
    f.finish().await;
}

#[tokio::test]
async fn postgres_quote_cannot_reference_foreign_customer() {
    postgres_foreign_reference("Draft Quote", quote_payload("customer-b")).await;
}

#[tokio::test]
async fn postgres_booking_cannot_reference_foreign_service() {
    postgres_foreign_reference("Draft Booking", booking_payload("service-b")).await;
}

#[tokio::test]
async fn postgres_shift_cannot_reference_foreign_staff() {
    postgres_foreign_reference(
        "Reassign Shift",
        json!({"shift_id":"shift-a","new_staff_id":"staff-b"}),
    )
    .await;
}

#[tokio::test]
async fn postgres_quote_effect_rolls_back_when_receipt_commit_fails() {
    let f =
        postgres_local_fixture("quote-failure", "Draft Quote", quote_payload("customer-a")).await;
    let Database::Postgres(fixture) = &f.database else {
        unreachable!()
    };
    fixture
        .fail_on("legacy_triage_decisions", "INSERT", true)
        .await;
    let (status, body) = f.decide(&f.owner, "quote-failure", true, None).await;
    assert!(!status.is_success(), "{status}: {body}");
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(
        body["decision_recorded"],
        Value::Null,
        "Unconfirmed persistence must not claim a confirmed rejection"
    );
    assert_eq!(f.count("quotes").await, 0);
    assert_eq!(f.count("quote_line_items").await, 0);
    assert_eq!(f.count("legacy_triage_decisions").await, 0);
    assert_eq!(
        f.text("SELECT status FROM triage_items WHERE id='quote-failure'")
            .await,
        "pending"
    );
    f.finish().await;
}

#[tokio::test]
async fn postgres_distinct_auth_and_data_pools_require_real_relation_parity() {
    let mut f = LegacyFixture::new(true).await;
    let Database::Postgres(fixture) = &f.database else {
        unreachable!()
    };
    assert!(
        sqlx::query_scalar::<_, bool>("SELECT to_regclass('staff_profiles') IS NULL")
            .fetch_one(&fixture.owner)
            .await
            .unwrap(),
        "Do not invent an optional PostgreSQL staff_profiles table to satisfy relation proof"
    );
    let independent = crate::db::secure_pg_pool_options()
        .connect_with(fixture.pool.connect_options().as_ref().clone())
        .await
        .unwrap();
    assert!(!std::ptr::eq(independent.options(), fixture.pool.options()));
    f.data_override = Some(independent);
    f.seed_daily("relation-proof", "tenant-a").await;
    let (status, body) = f.decide(&f.owner, "relation-proof", true, None).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "Actual production relation proof failed: {body}"
    );
    recorded(&body, "relation-proof", "APPROVED");
    assert_eq!(f.receipt("relation-proof").await, body);
    f.finish().await;
}
