use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use sea_orm::{ConnectionTrait, Schema};
use serde_json::{Value, json};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;
struct Fixture {
    pool: sqlx::PgPool,
    admin: sqlx::PgPool,
    owner: sqlx::PgPool,
    schema: String,
    role: String,
    auth: Arc<server_auth::Store>,
    token: String,
    member: String,
}
impl Fixture {
    async fn new() -> Self {
        let raw =
            std::env::var("OHC_FEED_TEST_DATABASE_URL").expect("disposable test URL required");
        let url = url::Url::parse(&raw).unwrap();
        assert!(
            url.host_str()
                .unwrap()
                .parse::<std::net::IpAddr>()
                .unwrap()
                .is_loopback()
        );
        assert!(
            url.path().starts_with("/ohc_")
                && url.path().ends_with("_test")
                && url.query().is_none()
        );
        let opts: PgConnectOptions = raw.parse().unwrap();
        let admin = PgPoolOptions::new()
            .connect_with(opts.clone())
            .await
            .unwrap();
        let schema = format!("feed_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let pool = PgPoolOptions::new()
            .connect_with(opts.clone().options([("search_path", schema.as_str())]))
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("schema.sql"))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql("INSERT INTO tenants(id,name) VALUES ('tenant-a','Synthetic A'),('tenant-b','Synthetic B'),('default','Synthetic default');").execute(&pool).await.unwrap();
        let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());
        let backend = sea_orm::DatabaseBackend::Postgres;
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
        sqlx::raw_sql(include_str!("token_fence.sql"))
            .execute(&pool)
            .await
            .unwrap();
        for table in ["users", "identity_user_roles", "auth_revoked_tokens"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&pool).await.unwrap();
        }
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO ohc_bypassrls; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO ohc_bypassrls;")).execute(&pool).await.unwrap();
        let owner = pool;
        let password = format!("synthetic-{}", Uuid::new_v4().simple());
        let role = format!("feed_role_{}", Uuid::new_v4().simple());
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}' NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE;GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                opts.username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(
                sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
            ),
        )));
        let mut tokens = Vec::new();
        for (id, role_name) in [("owner-a", "ADMIN"), ("member-a", "MEMBER")] {
            let user = server_auth::User {
                id: id.into(),
                username: id.into(),
                email: format!("{id}@example.test"),
                password_hash: "unused-fixture".into(),
                roles: vec![role_name.into()],
                active: true,
                organization_id: Some("tenant-a".into()),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                oidc_subject: None,
            };
            sqlx::query(
                "INSERT INTO users(id,username,email,tenant_id)VALUES($1,$1,$2,'tenant-a')",
            )
            .bind(id)
            .bind(&user.email)
            .execute(&owner)
            .await
            .unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)VALUES($1,$2,'tenant-a',0)").bind(id).bind(role_name).execute(&owner).await.unwrap();
            tokens.push(auth.issue_token(&user).unwrap());
        }
        Self {
            pool,
            owner,
            admin,
            schema,
            role,
            auth,
            token: tokens.remove(0),
            member: tokens.remove(0),
        }
    }
    async fn finish(self) {
        self.pool.close().await;
        self.owner.close().await;
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

impl Fixture {
    async fn app(&self) -> Router {
        crate::mounted_decisions(self.pool.clone(), self.auth.clone()).await
    }
    async fn call(&self, token: &str, id: &str, body: Value) -> (StatusCode, Value) {
        let response = self
            .app()
            .await
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/agent-feed/{id}/state"))
                    .method("PUT")
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
    async fn seed(&self, id: &str, tenant: &str) {
        sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state)VALUES($1,$2,'test','{}',$3,'PENDING_APPROVAL')").bind(id).bind(tenant).bind(json!({"feature_type":"create_product","title":"Original","draft_reply":"original draft","recipient":"local-fixture"})).execute(&self.owner).await.unwrap();
    }
    async fn row(&self, id: &str) -> (String, Value) {
        sqlx::query_as("SELECT lifecycle_state,proposed_action FROM agent_feed_items WHERE id=$1")
            .bind(id)
            .fetch_one(&self.owner)
            .await
            .unwrap()
    }
    async fn jobs(&self) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM ohc_job_queue")
            .fetch_one(&self.owner)
            .await
            .unwrap()
    }
    async fn fail_on(&self, table: &str, statement: &str, deferred: bool) {
        let trigger = if deferred {
            format!(
                "CREATE CONSTRAINT TRIGGER reject_fixture AFTER {statement} ON {table} DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_fixture()"
            )
        } else {
            format!(
                "CREATE TRIGGER reject_fixture BEFORE {statement} ON {table} FOR EACH ROW EXECUTE FUNCTION reject_fixture()"
            )
        };
        sqlx::raw_sql(&format!("CREATE FUNCTION reject_fixture() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'controlled fixture failure'; END $$; {trigger};")).execute(&self.owner).await.unwrap();
    }
}
#[tokio::test]
async fn edited_existing_content_keeps_routing_metadata_and_dispatches_that_exact_content() {
    let f = Fixture::new().await;
    f.seed("edit", "tenant-a").await;
    let (status, body) = f
        .call(
            &f.token,
            "edit",
            json!({"state":"APPROVED","edited_payload":"owner edit"}),
        )
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, saved) = f.row("edit").await;
    assert_eq!(saved["feature_type"], "create_product");
    assert_eq!(saved["draft_reply"], "owner edit");
    assert_eq!(saved["recipient"], "local-fixture");
    assert_eq!(f.jobs().await, 1);
    let payload: Value = sqlx::query_scalar("SELECT payload FROM ohc_job_queue")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(payload["payload"], saved);
    f.finish().await;
}
#[tokio::test]
async fn repeated_and_concurrent_approvals_admit_exactly_one_job() {
    let f = Fixture::new().await;
    f.seed("repeat", "tenant-a").await;
    let (a, b) = tokio::join!(
        f.call(&f.token, "repeat", json!({"state":"APPROVED"})),
        f.call(&f.token, "repeat", json!({"state":"APPROVED"}))
    );
    assert_eq!(a.0, StatusCode::OK);
    assert_eq!(b.0, StatusCode::OK);
    let c = f
        .call(&f.token, "repeat", json!({"state":"APPROVED"}))
        .await;
    assert_eq!(c.0, StatusCode::OK);
    assert_eq!(
        f.jobs().await,
        1,
        "Replaying the same decision must not enqueue again"
    );
    f.finish().await;
}
#[tokio::test]
async fn payload_failure_rolls_back_decision_and_job() {
    let f = Fixture::new().await;
    f.seed("payload", "tenant-a").await;
    f.fail_on("agent_feed_items", "UPDATE OF proposed_action", false)
        .await;
    let (s, _) = f
        .call(
            &f.token,
            "payload",
            json!({"state":"APPROVED","modified_content":"edit"}),
        )
        .await;
    assert!(
        !s.is_success(),
        "An ignored payload failure must never return success"
    );
    assert_eq!(f.row("payload").await.0, "PENDING_APPROVAL");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn legacy_sync_failure_rolls_back_canonical_decision() {
    let f = Fixture::new().await;
    f.seed("legacy", "tenant-a").await;
    sqlx::query("INSERT INTO agent_approvals(id,tenant_id,department,description,action_risk,status)VALUES('legacy','tenant-a','ops','fixture','LOW','DRAFT')").execute(&f.owner).await.unwrap();
    f.fail_on("agent_approvals", "UPDATE", false).await;
    let (s, _) = f
        .call(&f.token, "legacy", json!({"state":"APPROVED"}))
        .await;
    assert!(!s.is_success());
    assert_eq!(f.row("legacy").await.0, "PENDING_APPROVAL");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn queue_insertion_failure_rolls_back_decision_and_edited_payload() {
    let f = Fixture::new().await;
    f.seed("queue", "tenant-a").await;
    f.fail_on("ohc_job_queue", "INSERT", false).await;
    let (s,_)=f.call(&f.token,"queue",json!({"state":"APPROVED","proposed_action":{"feature_type":"create_product","title":"Edited"}})).await;
    assert!(!s.is_success());
    let (state, payload) = f.row("queue").await;
    assert_eq!(state, "PENDING_APPROVAL");
    assert_eq!(payload["title"], "Original");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn deferred_commit_failure_rolls_back_all_payload_changes() {
    let f = Fixture::new().await;
    f.seed("commit", "tenant-a").await;
    f.fail_on("agent_feed_items", "UPDATE OF lifecycle_state", true)
        .await;
    let (s,_)=f.call(&f.token,"commit",json!({"state":"APPROVED","proposed_action":{"feature_type":"create_product","title":"Edited"}})).await;
    assert!(!s.is_success());
    let (state, payload) = f.row("commit").await;
    assert_eq!(state, "PENDING_APPROVAL");
    assert_eq!(payload["title"], "Original");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn member_and_demoted_owner_cannot_approve() {
    let f = Fixture::new().await;
    f.seed("authority", "tenant-a").await;
    assert_eq!(
        f.call(&f.member, "authority", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    sqlx::query("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='owner-a'")
        .execute(&f.owner)
        .await
        .unwrap();
    assert_eq!(
        f.call(&f.token, "authority", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(f.row("authority").await.0, "PENDING_APPROVAL");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn foreign_missing_and_invalid_state_never_mutate() {
    let f = Fixture::new().await;
    f.seed("foreign", "tenant-b").await;
    f.seed("invalid", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "foreign", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        f.call(&f.token, "missing", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        f.call(&f.token, "invalid", json!({"state":"DELIVERED"}))
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(f.row("foreign").await.0, "PENDING_APPROVAL");
    assert_eq!(f.row("invalid").await.0, "PENDING_APPROVAL");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn replay_cannot_replace_already_approved_payload() {
    let f = Fixture::new().await;
    f.seed("immutable", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "immutable", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(f.call(&f.token,"immutable",json!({"state":"APPROVED","proposed_action":{"feature_type":"create_product","title":"Changed"}})).await.0,StatusCode::CONFLICT);
    assert_eq!(f.row("immutable").await.1["title"], "Original");
    assert_eq!(f.jobs().await, 1);
    f.finish().await;
}

impl Fixture {
    async fn dequeue(&self) -> crate::omnisolo_job_queue::OmniSoloJob {
        crate::omnisolo_job_queue::OmniSoloJobQueue::new(Arc::new(self.owner.clone()))
            .dequeue(vec!["agent_feed_action"])
            .await
            .unwrap()
            .expect("durable admitted job")
    }
    async fn work(&self, job: crate::omnisolo_job_queue::OmniSoloJob) {
        let redis = std::env::var("OHC_FEED_TEST_REDIS_URL").expect("owned Redis fixture required");
        let worker =
            crate::agent_action_worker::AgentActionWorker::new(self.owner.clone(), redis.clone());
        let lock = crate::redis_lock::RedisLock::new(&redis).unwrap();
        worker
            .process_job(
                job,
                &crate::omnisolo_job_queue::OmniSoloJobQueue::new(Arc::new(self.owner.clone())),
                &lock,
            )
            .await;
    }
    async fn products(&self) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM products")
            .fetch_one(&self.owner)
            .await
            .unwrap()
    }
}
#[tokio::test]
async fn worker_redelivery_does_not_repeat_a_returned_dispatch() {
    let f = Fixture::new().await;
    f.seed("redelivery", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "redelivery", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    f.work(job.clone()).await;
    assert_eq!(f.products().await, 1);
    f.work(job).await;
    assert_eq!(
        f.products().await,
        1,
        "Job redelivery must not repeat the actual product mutation"
    );
    f.finish().await;
}
#[tokio::test]
async fn worker_checks_current_owner_and_cancelled_decision_before_dispatch() {
    let f = Fixture::new().await;
    f.seed("cancelled", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "cancelled", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    assert_eq!(
        f.call(&f.token, "cancelled", json!({"state":"DISMISSED"}))
            .await
            .0,
        StatusCode::OK
    );
    f.work(job).await;
    assert_eq!(f.products().await, 0, "A cancelled approval cannot run");
    f.seed("demoted", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "demoted", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    sqlx::query("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='owner-a'")
        .execute(&f.owner)
        .await
        .unwrap();
    f.work(job).await;
    assert_eq!(
        f.products().await,
        0,
        "A demoted owner cannot dispatch pending work"
    );
    f.finish().await;
}
#[tokio::test]
async fn lost_dispatch_acknowledgement_does_not_allow_worker_retry() {
    let f = Fixture::new().await;
    f.seed("lost-ack", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "lost-ack", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    // Fail only the terminal queue acknowledgement after the real product insert.
    sqlx::raw_sql("CREATE FUNCTION reject_ack() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.status IN ('COMPLETED','DISPATCH_RETURNED') THEN RAISE EXCEPTION 'controlled acknowledgement failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER reject_ack BEFORE UPDATE ON ohc_job_queue FOR EACH ROW EXECUTE FUNCTION reject_ack();").execute(&f.owner).await.unwrap();
    f.work(job.clone()).await;
    assert_eq!(f.products().await, 1);
    sqlx::query("DROP TRIGGER reject_ack ON ohc_job_queue")
        .execute(&f.owner)
        .await
        .unwrap();
    f.work(job).await;
    assert_eq!(
        f.products().await,
        1,
        "Unconfirmed acknowledgement cannot repeat an attempted operation"
    );
    f.finish().await;
}

#[tokio::test]
async fn historical_approved_record_without_receipt_is_held_for_reconciliation() {
    let f = Fixture::new().await;
    f.seed("historical", "tenant-a").await;
    sqlx::query("UPDATE agent_feed_items SET lifecycle_state='APPROVED' WHERE id='historical'")
        .execute(&f.owner)
        .await
        .unwrap();
    let (status, body) = f
        .call(&f.token, "historical", json!({"state":"APPROVED"}))
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["dispatch"]["status"], "RECONCILIATION_REQUIRED");
    assert_eq!(
        f.jobs().await,
        0,
        "Unknown historical execution must not be resubmitted"
    );
    f.finish().await;
}
#[tokio::test]
async fn edited_content_updates_all_delivery_aliases_consistently() {
    let f = Fixture::new().await;
    f.seed("aliases", "tenant-a").await;
    sqlx::query("UPDATE agent_feed_items SET proposed_action=$1 WHERE id='aliases'").bind(json!({"feature_type":"create_product","generated_response":"old generated","draft_reply":"old reply","summary":"old summary","message":"old message","recipient":"retained"})).execute(&f.owner).await.unwrap();
    let (s, _) = f
        .call(
            &f.token,
            "aliases",
            json!({"state":"APPROVED","edited_payload":"owner final text"}),
        )
        .await;
    assert_eq!(s, StatusCode::OK);
    let (_, saved) = f.row("aliases").await;
    for key in ["generated_response", "draft_reply", "summary", "message"] {
        assert_eq!(saved[key], "owner final text", "{key}");
    }
    assert_eq!(saved["recipient"], "retained");
    f.finish().await;
}
#[tokio::test]
async fn legacy_only_approval_is_still_editable_and_acknowledged() {
    let f = Fixture::new().await;
    sqlx::query("INSERT INTO agent_approvals(id,tenant_id,department,description,action_risk,status,payload)VALUES('legacy-only','tenant-a','ops','fixture','LOW','DRAFT',$1)").bind(json!({"feature_type":"create_product","draft_reply":"original","title":"legacy"})).execute(&f.owner).await.unwrap();
    let (s, b) = f
        .call(
            &f.token,
            "legacy-only",
            json!({"state":"APPROVED","modified_content":"legacy edit"}),
        )
        .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(b["proposed_action"]["feature_type"], "create_product");
    assert_eq!(b["proposed_action"]["draft_reply"], "legacy edit");
    assert_eq!(f.jobs().await, 1);
    let row: (String, Value) =
        sqlx::query_as("SELECT status,payload FROM agent_approvals WHERE id='legacy-only'")
            .fetch_one(&f.owner)
            .await
            .unwrap();
    assert_eq!(row.0, "APPROVED");
    assert_eq!(row.1["draft_reply"], "legacy edit");
    f.finish().await;
}
#[tokio::test]
async fn revoked_bearer_and_revoked_accepted_approval_cannot_execute() {
    let f = Fixture::new().await;
    f.seed("revoked", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "revoked", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti.clone(),
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    let status = f
        .call(&f.token, "revoked", json!({"state":"APPROVED"}))
        .await
        .0;
    assert!(matches!(
        status,
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN
    ));
    f.work(job).await;
    assert_eq!(
        f.products().await,
        0,
        "Revoked approval token cannot start accepted work"
    );
    f.finish().await;
}

#[tokio::test]
async fn mirrored_agent_feed_source_keeps_the_approved_edited_payload() {
    let f = Fixture::new().await;
    sqlx::query("INSERT INTO agent_feed(id,tenant_id,source,description,payload,state)VALUES('mirror-source','tenant-a','ops','old context',$1,'PENDING_APPROVAL')").bind(json!({"feature_type":"create_product","draft_reply":"old","title":"Original"})).execute(&f.owner).await.unwrap();
    let (status,_)=f.call(&f.token,"mirror-source",json!({"state":"APPROVED","edited_payload":"edited","context_payload":{"description":"owner context"}})).await;
    assert_eq!(status, StatusCode::OK);
    let source: (String, Value, String) =
        sqlx::query_as("SELECT state,payload,description FROM agent_feed WHERE id='mirror-source'")
            .fetch_one(&f.owner)
            .await
            .unwrap();
    assert_eq!(source.0, "APPROVED");
    assert_eq!(source.1["feature_type"], "create_product");
    assert_eq!(source.1["draft_reply"], "edited");
    assert_eq!(source.2, "owner context");
    f.finish().await;
}

#[tokio::test]
async fn stale_expected_owner_headers_do_not_use_a_new_current_session() {
    let f = Fixture::new().await;
    f.seed("stale-tab", "tenant-a").await;
    let response = f
        .app()
        .await
        .oneshot(
            Request::builder()
                .uri("/api/v1/agent-feed/stale-tab")
                .method("PUT")
                .header("authorization", format!("Bearer {}", f.token))
                .header("content-type", "application/json")
                .header("x-ohc-expected-user", "previous-owner")
                .header("x-ohc-expected-tenant", "tenant-a")
                .body(Body::from(json!({"state":"APPROVED"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
    assert_eq!(f.row("stale-tab").await.0, "PENDING_APPROVAL");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}

#[tokio::test]
async fn revocation_while_queue_write_is_blocked_rolls_back_the_whole_decision() {
    let f = Fixture::new().await;
    f.seed("commit-revoke", "tenant-a").await;
    let key = (Uuid::new_v4().as_u128() & 0x7fff_ffff) as i64;
    let mut barrier = f.owner.acquire().await.unwrap();
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(key)
        .execute(&mut *barrier)
        .await
        .unwrap();
    sqlx::raw_sql(&format!("CREATE FUNCTION block_queue() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM pg_advisory_xact_lock({key}); RETURN NEW; END $$; CREATE TRIGGER block_queue BEFORE INSERT ON ohc_job_queue FOR EACH ROW EXECUTE FUNCTION block_queue();")).execute(&f.owner).await.unwrap();
    let app = f.app().await;
    let request=Request::builder().uri("/api/v1/agent-feed/commit-revoke").method("PUT").header("authorization",format!("Bearer {}",f.token)).header("content-type","application/json").body(Body::from(json!({"state":"APPROVED","proposed_action":{"feature_type":"create_product","title":"Edited"}}).to_string())).unwrap();
    let request = tokio::spawn(async move { app.oneshot(request).await.unwrap() });
    tokio::time::timeout(std::time::Duration::from_secs(5),async{
  loop {
   let blocked:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND classid=0 AND objid=$1::bigint::oid AND NOT granted)").bind(key).fetch_one(&f.owner).await.unwrap();
   if blocked{break;}tokio::time::sleep(std::time::Duration::from_millis(10)).await;
  }
 }).await.expect("actual decision queue insert reached the controlled lock");
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(key)
        .execute(&mut *barrier)
        .await
        .unwrap();
    drop(barrier);
    assert!(
        !request.await.unwrap().status().is_success(),
        "Authority revoked before commit must reject the decision"
    );
    let (state, payload) = f.row("commit-revoke").await;
    assert_eq!(state, "PENDING_APPROVAL");
    assert_eq!(payload["title"], "Original");
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}

impl Fixture {
    async fn read_decision(&self, id: &str) -> (StatusCode, Value) {
        let response = self
            .app()
            .await
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/agent-feed/{id}/decision"))
                    .header("authorization", format!("Bearer {}", self.token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap())
    }
    async fn phase(&self, id: &str) -> String {
        sqlx::query_scalar("SELECT dispatch_status FROM agent_feed_decisions WHERE action_id=$1")
            .bind(id)
            .fetch_one(&self.owner)
            .await
            .unwrap()
    }
}
#[tokio::test]
async fn readback_reports_decision_dispatch_and_return_without_claiming_delivery() {
    let f = Fixture::new().await;
    f.seed("readback", "tenant-a").await;
    assert_eq!(
        f.read_decision("readback").await.1["decision_recorded"],
        false
    );
    assert_eq!(
        f.call(&f.token, "readback", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let (status, before) = f.read_decision("readback").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(before["dispatch"]["status"], "PENDING");
    assert_eq!(before["decision_recorded"], true);
    f.work(f.dequeue().await).await;
    let (status, after) = f.read_decision("readback").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(after["dispatch"]["status"], "DISPATCH_RETURNED");
    assert!(
        after["dispatch"]["detail"]
            .as_str()
            .unwrap()
            .contains("not verified")
    );
    assert!(after.get("token_id").is_none());
    assert!(after["dispatch"].get("token_id").is_none());
    let replay = f
        .call(&f.token, "readback", json!({"state":"APPROVED"}))
        .await;
    assert_eq!(replay.1["dispatch"]["status"], "DISPATCH_RETURNED");
    assert_eq!(f.jobs().await, 1);
    assert_eq!(f.products().await, 1);
    f.finish().await;
}
#[tokio::test]
async fn failed_pre_attempt_claim_retries_safely_without_a_business_mutation() {
    let f = Fixture::new().await;
    f.seed("pre-attempt", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "pre-attempt", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    sqlx::raw_sql("CREATE FUNCTION reject_claim() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.dispatch_status='ATTEMPTING' THEN RAISE EXCEPTION 'controlled claim persistence failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER reject_claim BEFORE UPDATE ON agent_feed_decisions FOR EACH ROW EXECUTE FUNCTION reject_claim();").execute(&f.owner).await.unwrap();
    f.work(job.clone()).await;
    assert_eq!(f.products().await, 0);
    assert_eq!(f.phase("pre-attempt").await, "PENDING");
    let queue: (String, i32) =
        sqlx::query_as("SELECT status,retry_count FROM ohc_job_queue WHERE id=$1")
            .bind(&job.id)
            .fetch_one(&f.owner)
            .await
            .unwrap();
    assert_eq!(queue, ("PENDING".into(), 1));
    sqlx::raw_sql("DROP TRIGGER reject_claim ON agent_feed_decisions; UPDATE ohc_job_queue SET next_retry_at=CURRENT_TIMESTAMP;").execute(&f.owner).await.unwrap();
    f.work(f.dequeue().await).await;
    assert_eq!(f.products().await, 1);
    assert_eq!(f.phase("pre-attempt").await, "DISPATCH_RETURNED");
    assert_eq!(f.jobs().await, 1);
    f.finish().await;
}
#[tokio::test]
async fn post_attempt_error_is_durable_and_not_automatically_retried() {
    let f = Fixture::new().await;
    f.seed("failed-attempt", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "failed-attempt", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    f.fail_on("products", "INSERT", false).await;
    f.work(job.clone()).await;
    assert_eq!(f.phase("failed-attempt").await, "RECONCILIATION_REQUIRED");
    assert_eq!(f.products().await, 0);
    sqlx::query("DROP TRIGGER reject_fixture ON products")
        .execute(&f.owner)
        .await
        .unwrap();
    f.work(job).await;
    assert_eq!(f.products().await, 0);
    assert_eq!(
        f.read_decision("failed-attempt").await.1["dispatch"]["status"],
        "RECONCILIATION_REQUIRED"
    );
    f.finish().await;
}
#[tokio::test]
async fn restart_recovers_an_abandoned_attempt_as_unknown_without_resubmission() {
    let f = Fixture::new().await;
    f.seed("restart", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "restart", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    // Materialize the persisted crash fixture: an old claimed attempt and no return.
    sqlx::query("UPDATE agent_feed_decisions SET dispatch_status='ATTEMPTING',attempted_at=CURRENT_TIMESTAMP-INTERVAL '2 minutes' WHERE action_id='restart'").execute(&f.owner).await.unwrap();
    crate::agent_feed_dispatch::recover_abandoned(&f.owner)
        .await
        .unwrap();
    assert_eq!(f.phase("restart").await, "RECONCILIATION_REQUIRED");
    f.work(job).await;
    assert_eq!(f.products().await, 0);
    let q: Option<crate::omnisolo_job_queue::OmniSoloJob> =
        crate::omnisolo_job_queue::OmniSoloJobQueue::new(Arc::new(f.owner.clone()))
            .dequeue(vec!["agent_feed_action"])
            .await
            .unwrap();
    assert!(q.is_none());
    f.finish().await;
}
#[tokio::test]
async fn cancellation_before_attempt_can_be_reapproved_without_another_job() {
    let f = Fixture::new().await;
    f.seed("reapprove", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "reapprove", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        f.call(&f.token, "reapprove", json!({"state":"DISMISSED"}))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(f.phase("reapprove").await, "CANCELLED");
    assert_eq!(f.call(&f.token,"reapprove",json!({"state":"APPROVED","proposed_action":{"feature_type":"create_product","title":"New review"}})).await.0,StatusCode::OK);
    assert_eq!(f.jobs().await, 1);
    f.work(f.dequeue().await).await;
    assert_eq!(f.products().await, 1);
    let title: String = sqlx::query_scalar("SELECT title FROM products")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(title, "New review");
    f.finish().await;
}
#[tokio::test]
async fn concurrent_worker_delivery_claims_the_business_action_once() {
    let f = Fixture::new().await;
    f.seed("concurrent-worker", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "concurrent-worker", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    tokio::join!(f.work(job.clone()), f.work(job));
    assert_eq!(f.products().await, 1);
    assert_eq!(f.phase("concurrent-worker").await, "DISPATCH_RETURNED");
    f.finish().await;
}
#[tokio::test]
async fn durable_attempt_identity_and_dispatch_snapshot_cannot_be_reset() {
    let f = Fixture::new().await;
    f.seed("immutable-attempt", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "immutable-attempt", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    f.work(f.dequeue().await).await;
    for mutation in [
        "attempted_at=NULL,dispatch_status='PENDING'",
        "job_id='replacement'",
        "actor_id='replacement'",
        "dispatch_payload='{}'",
        "dispatch_returned_at=NULL",
    ] {
        let result = sqlx::query(&format!(
            "UPDATE agent_feed_decisions SET {mutation} WHERE action_id='immutable-attempt'"
        ))
        .execute(&f.owner)
        .await;
        assert!(
            result.is_err(),
            "Persistent dispatch fence must reject {mutation}"
        );
    }
    assert!(
        sqlx::query("DELETE FROM agent_feed_decisions WHERE action_id='immutable-attempt'")
            .execute(&f.owner)
            .await
            .is_err()
    );
    assert_eq!(f.phase("immutable-attempt").await, "DISPATCH_RETURNED");
    f.finish().await;
}

#[tokio::test]
async fn explicit_fresh_authority_can_reapprove_a_revoked_unattempted_job() {
    let f = Fixture::new().await;
    f.seed("reauthorize", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "reauthorize", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    f.work(job).await;
    assert_eq!(f.phase("reauthorize").await, "CANCELLED");
    assert_eq!(f.products().await, 0);
    let owner = f.auth.get_user("owner-a", "tenant-a").await.unwrap();
    let fresh = f.auth.issue_token(&owner).unwrap();
    assert_ne!(fresh, f.token);
    let (status, body) = f
        .call(&fresh, "reauthorize", json!({"state":"APPROVED"}))
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["dispatch"]["status"], "PENDING");
    assert_eq!(f.jobs().await, 1);
    f.work(f.dequeue().await).await;
    assert_eq!(f.products().await, 1);
    f.finish().await;
}
#[tokio::test]
async fn actual_queue_redelivery_restores_terminal_dispatch_receipt_without_reexecution() {
    let f = Fixture::new().await;
    f.seed("queue-redelivery", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "queue-redelivery", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    f.work(job.clone()).await;
    sqlx::query(
        "UPDATE ohc_job_queue SET status='PENDING',next_retry_at=CURRENT_TIMESTAMP WHERE id=$1",
    )
    .bind(&job.id)
    .execute(&f.owner)
    .await
    .unwrap();
    f.work(f.dequeue().await).await;
    assert_eq!(f.products().await, 1);
    let queue: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue WHERE id=$1")
        .bind(&job.id)
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(queue, "DISPATCH_RETURNED");
    f.finish().await;
}
#[tokio::test]
async fn ordinary_worker_pool_does_not_falsely_report_a_downstream_rls_failure_as_success() {
    let f = Fixture::new().await;
    f.seed("rls-boundary", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "rls-boundary", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    // Enable the production product policy for the non-BYPASSRLS worker pool.
    sqlx::raw_sql(include_str!("products_rls.sql"))
        .execute(&f.owner)
        .await
        .unwrap();
    let redis = std::env::var("OHC_FEED_TEST_REDIS_URL").unwrap();
    let worker = crate::agent_action_worker::AgentActionWorker::new(f.pool.clone(), redis.clone());
    worker
        .process_job(
            job,
            &crate::omnisolo_job_queue::OmniSoloJobQueue::new(Arc::new(f.owner.clone())),
            &crate::redis_lock::RedisLock::new(&redis).unwrap(),
        )
        .await;
    assert_eq!(f.products().await, 0);
    assert_eq!(f.phase("rls-boundary").await, "RECONCILIATION_REQUIRED");
    f.finish().await;
}

#[tokio::test]
async fn dispatch_return_acknowledgement_is_itself_idempotent() {
    let f = Fixture::new().await;
    f.seed("finish-replay", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "finish-replay", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    let job = f.dequeue().await;
    let attempt = crate::agent_feed_dispatch::claim(&f.owner, &job)
        .await
        .unwrap()
        .unwrap();
    crate::catalog::handle_create_product(
        &attempt.tenant_id,
        &attempt.payload["payload"],
        &f.owner,
    )
    .await
    .unwrap();
    crate::agent_feed_dispatch::finish(&f.owner, &attempt, true)
        .await
        .unwrap();
    crate::agent_feed_dispatch::finish(&f.owner, &attempt, true)
        .await
        .expect("Repeated acknowledgement preserves original immutable return evidence");
    assert_eq!(f.products().await, 1);
    assert_eq!(f.phase("finish-replay").await, "DISPATCH_RETURNED");
    f.finish().await;
}

#[tokio::test]
async fn terminal_queue_admission_is_held_in_owner_readback_and_approval_replay() {
    let f = Fixture::new().await;
    f.seed("ended-queue", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "ended-queue", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    // Materialize the actual terminal queue boundary reached by backlog cleanup.
    sqlx::query(
        "UPDATE ohc_job_queue SET status='FAILED' WHERE payload->>'action_id'='ended-queue'",
    )
    .execute(&f.owner)
    .await
    .unwrap();
    assert_eq!(
        f.read_decision("ended-queue").await.1["dispatch"]["status"],
        "RECONCILIATION_REQUIRED"
    );
    let (status, body) = f
        .call(&f.token, "ended-queue", json!({"state":"APPROVED"}))
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["dispatch"]["status"], "RECONCILIATION_REQUIRED");
    assert_eq!(f.jobs().await, 1);
    assert_eq!(f.products().await, 0);
    f.finish().await;
}
#[tokio::test]
async fn restart_reconciles_a_missing_queue_admission_without_recreating_it() {
    let f = Fixture::new().await;
    f.seed("missing-queue", "tenant-a").await;
    assert_eq!(
        f.call(&f.token, "missing-queue", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    sqlx::query("DELETE FROM ohc_job_queue WHERE payload->>'action_id'='missing-queue'")
        .execute(&f.owner)
        .await
        .unwrap();
    crate::agent_feed_dispatch::recover_abandoned(&f.owner)
        .await
        .unwrap();
    assert_eq!(f.phase("missing-queue").await, "RECONCILIATION_REQUIRED");
    assert_eq!(
        f.call(&f.token, "missing-queue", json!({"state":"APPROVED"}))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(f.jobs().await, 0);
    f.finish().await;
}
