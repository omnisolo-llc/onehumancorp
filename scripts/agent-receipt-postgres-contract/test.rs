use crate::persistence::{AppDatabase, migration};
use crate::workflow_execution::{
    AdmittedAnalysis, AnalysisOutcome, AnalysisPolicy, Authority, TextInference, WorkflowExecution,
    receipts::{Error, ReceiptStore, RequestMetadata, StoredPhase},
};
use server_common::Claims;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::{future::Future, pin::Pin, str::FromStr, sync::Arc, time::Duration};
use uuid::Uuid;

#[path = "assistant_test.rs"]
mod assistant_tests;

#[path = "dynamic_queue_test.rs"]
mod dynamic_queue_tests;
#[path = "dynamic_workflow_test.rs"]
mod dynamic_workflow_tests;

struct NeverInfer;
impl TextInference for NeverInfer {
    fn infer<'a>(
        &'a self,
        _: &'a AdmittedAnalysis,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::workflow_execution::InferenceResult, ()>> + Send + 'a,
        >,
    > {
        Box::pin(async { panic!("Receipt storage tests must never invoke a provider") })
    }
}
struct Fixture {
    admin: PgPool,
    pool: PgPool,
    store: ReceiptStore,
    auth: Arc<server_auth::Store>,
    execution: WorkflowExecution,
    identities: Vec<(Claims, axum::http::HeaderMap)>,
    schema: String,
    role: String,
    created_bypass: bool,
}
impl Fixture {
    async fn open() -> Self {
        let url = std::env::var("OHC_AGENT_RECEIPT_TEST_DATABASE_URL")
            .expect("Explicit owned PostgreSQL database is mandatory");
        let parsed = reqwest::Url::parse(&url).unwrap();
        assert!(matches!(parsed.scheme(), "postgres" | "postgresql"));
        assert!(
            parsed
                .host_str()
                .unwrap()
                .parse::<std::net::IpAddr>()
                .unwrap()
                .is_loopback()
        );
        let database_name = parsed.path().trim_start_matches('/');
        assert!(database_name.starts_with("ohc_") && database_name.ends_with("_test"));
        assert!(parsed.query().is_none() && parsed.fragment().is_none());
        let id = Uuid::new_v4().simple().to_string();
        let schema = format!("receipt_{id}");
        let role = format!("receipt_role_{id}");
        let base = PgPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&base)
            .await
            .unwrap();
        let options = PgConnectOptions::from_str(&url)
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let admin = PgPoolOptions::new()
            .max_connections(3)
            .connect_with(options.clone())
            .await
            .unwrap();
        base.close().await;
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
        let (bypass, login): (bool, bool) = sqlx::query_as(
            "SELECT rolbypassrls,rolcanlogin FROM pg_roles WHERE rolname='ohc_bypassrls'",
        )
        .fetch_one(&admin)
        .await
        .unwrap();
        assert!(bypass && !login);
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO ohc_bypassrls; ALTER DEFAULT PRIVILEGES IN SCHEMA {schema} GRANT ALL ON TABLES TO ohc_bypassrls;")).execute(&admin).await.unwrap();
        sqlx::raw_sql(include_str!("core_pg.sql"))
            .execute(&admin)
            .await
            .unwrap();
        let database = AppDatabase::from_connection(
            sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(admin.clone()),
        );
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1018_agent_definition_marketplace.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        migration::migrate(&database).await.unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1022_tenant_workflow_receipts.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        sqlx::raw_sql(include_str!("../../src/server/migrations/1042_assistant_execution.sql"))
            .execute(&admin).await.unwrap();
        use server_auth::user_repository::UserRepository;
        let repository =
            server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone());
        for (user, tenant) in [
            ("owner-a", "receipt-pg-a"),
            ("owner-b", "receipt-pg-b"),
            ("peer-a", "receipt-pg-a"),
        ] {
            sqlx::query("INSERT INTO tenants(id,name) VALUES($1,$1) ON CONFLICT DO NOTHING")
                .bind(tenant)
                .execute(&admin)
                .await
                .unwrap();
            let now = chrono::Utc::now();
            repository
                .create_user(
                    server_auth::User {
                        id: user.into(),
                        username: user.into(),
                        email: format!("{user}@example.test"),
                        password_hash: String::new(),
                        roles: vec!["OWNER".into()],
                        active: true,
                        organization_id: Some(tenant.into()),
                        created_at: now,
                        updated_at: now,
                        oidc_subject: None,
                    },
                    tenant,
                )
                .await
                .unwrap();
        }
        sqlx::raw_sql(&format!("ALTER TABLE users ENABLE ROW LEVEL SECURITY; ALTER TABLE users FORCE ROW LEVEL SECURITY; CREATE POLICY receipt_test_user_tenant ON users USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true)); CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS PASSWORD 'public-local-disposable-receipt-fixture'; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role}; REVOKE UPDATE,DELETE ON agent_definition_authorities FROM {role};")).execute(&admin).await.unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .acquire_timeout(Duration::from_secs(2))
            .connect_with(
                options
                    .username(&role)
                    .password("public-local-disposable-receipt-fixture"),
            )
            .await
            .unwrap();
        let (current,session,superuser,bypass): (String,String,bool,bool) = sqlx::query_as("SELECT current_user::text,session_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();
        assert_eq!(current, role);
        assert_eq!(session, role);
        assert!(!superuser && !bypass);
        let forced: bool = sqlx::query_scalar("SELECT relrowsecurity AND relforcerowsecurity FROM pg_class WHERE oid='tenant_workflow_receipts'::regclass").fetch_one(&pool).await.unwrap();
        assert!(forced);
        let database = AppDatabase::from_connection(
            sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
        );
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone()),
        )));
        let mut identities = Vec::new();
        for (id, tenant) in [
            ("owner-a", "receipt-pg-a"),
            ("owner-b", "receipt-pg-b"),
            ("peer-a", "receipt-pg-a"),
        ] {
            let user = repository.get_by_id(id, tenant).await.unwrap();
            let token = auth.issue_token(&user).unwrap();
            let claims = auth.validate_token(&token).await.unwrap();
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::AUTHORIZATION,
                format!("Bearer {token}").parse().unwrap(),
            );
            identities.push((claims, headers));
        }
        let execution = WorkflowExecution::configured(
            auth.clone(),
            AnalysisPolicy::new("ollama".into(), "owned-receipt-model".into(), 256).unwrap(),
            Arc::new(NeverInfer),
        );
        Self {
            admin,
            pool,
            store: ReceiptStore::new(database),
            auth,
            execution,
            identities,
            schema,
            role,
            created_bypass: !exists,
        }
    }
    async fn admitted(&self, identity: usize) -> AdmittedAnalysis {
        let (claims, headers) = &self.identities[identity];
        self.execution
            .admit(
                claims,
                headers,
                "Real submitted text for the PostgreSQL storage-only contract",
                "Auto",
                "analysis",
            )
            .await
            .unwrap()
    }
    async fn authority(&self, identity: usize) -> Authority {
        let (claims, headers) = &self.identities[identity];
        self.execution.authorize(claims, headers).await.unwrap()
    }
    async fn close(self) {
        drop(self.store);
        drop(self.execution);
        drop(self.auth);
        self.pool.close().await;
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE; DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        if self.created_bypass {
            sqlx::query("DROP ROLE ohc_bypassrls")
                .execute(&self.admin)
                .await
                .unwrap();
        }
        self.admin.close().await;
    }
}
fn request(id: Uuid) -> RequestMetadata {
    RequestMetadata {
        request_id: id,
        name: "Owner submitted analysis".into(),
        workflow: "analysis".into(),
        requested_model: "Auto".into(),
        agent_role: None,
    }
}

#[tokio::test]
async fn postgres_receipt_reopens_with_exact_content_and_forced_rls() {
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let saved = reserved.receipt().clone();
    drop(reserved);
    let reopened = ReceiptStore::new(AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(f.pool.clone()),
    ));
    assert_eq!(
        reopened
            .get(&f.authority(0).await, &saved.id)
            .await
            .unwrap(),
        saved
    );
    assert!(matches!(
        reopened.get(&f.authority(1).await, &saved.id).await,
        Err(Error::NotFound)
    ));
    let unscoped: i64 = sqlx::query_scalar("SELECT count(*) FROM tenant_workflow_receipts")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(unscoped, 0);
    let mut other = f.pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant','receipt-pg-b',true)")
        .execute(&mut *other)
        .await
        .unwrap();
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM tenant_workflow_receipts")
        .fetch_one(&mut *other)
        .await
        .unwrap();
    assert_eq!(rows, 0);
    other.rollback().await.unwrap();
    drop(reopened);
    f.close().await;
}
#[tokio::test]
async fn postgres_concurrent_same_request_has_one_admission_and_one_claim() {
    let f = Fixture::open().await;
    let id = Uuid::new_v4();
    let a = f.admitted(0).await;
    let b = f.admitted(0).await;
    let (a, b) = tokio::join!(
        f.store.reserve(a, request(id)),
        f.store.reserve(b, request(id))
    );
    let a = a.unwrap();
    let b = b.unwrap();
    assert_eq!(a.receipt(), b.receipt());
    assert_ne!(a.replayed(), b.replayed());
    let (a, b) = tokio::join!(f.store.claim(a), f.store.claim(b));
    assert_eq!(
        usize::from(a.unwrap().is_some()) + usize::from(b.unwrap().is_some()),
        1
    );
    f.close().await;
}
#[tokio::test]
async fn postgres_terminal_result_is_durable_and_identical_replay_is_read_only() {
    let f = Fixture::open().await;
    let reservation = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let lease = f.store.claim(reservation).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    let result = AnalysisOutcome::Completed(
        "Clearly labelled storage test artifact, not a provider result".into(),
    );
    let completed = f.store.finish(&proof, &result).await.unwrap();
    assert_eq!(completed.phase, StoredPhase::Completed);
    sqlx::raw_sql("CREATE FUNCTION reject_any_receipt_update() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'replay_must_be_read_only'; END $$; CREATE TRIGGER reject_any_receipt_update BEFORE UPDATE ON tenant_workflow_receipts FOR EACH ROW EXECUTE FUNCTION reject_any_receipt_update();").execute(&f.admin).await.unwrap();
    assert_eq!(f.store.finish(&proof, &result).await.unwrap(), completed);
    assert!(matches!(
        f.store
            .finish(&proof, &AnalysisOutcome::Completed("changed result".into()))
            .await,
        Err(Error::Conflict)
    ));
    assert_eq!(
        f.store
            .get(&f.authority(0).await, &completed.id)
            .await
            .unwrap(),
        completed
    );
    drop(lease);
    f.close().await;
}
#[tokio::test]
async fn postgres_revoked_completion_stores_uncertainty_without_private_output() {
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let id = reserved.receipt().id.clone();
    let lease = f.store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    let claims = &f.identities[0].0;
    f.auth
        .revoke_token(
            claims.jti.clone(),
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "receipt-pg-a",
        )
        .await
        .unwrap();
    assert!(matches!(
        f.store
            .finish(
                &proof,
                &AnalysisOutcome::Completed("Must never be persisted or disclosed".into())
            )
            .await,
        Err(Error::Forbidden)
    ));
    let (phase, output, error): (String, Option<String>, Option<String>) =
        sqlx::query_as("SELECT phase,output,error FROM tenant_workflow_receipts WHERE id=$1")
            .bind(&id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(phase, "outcome_unknown");
    assert_eq!(output, None);
    assert_eq!(error.as_deref(), Some("authority_lost"));
    assert_eq!(
        f.store.get(&f.authority(2).await, &id).await.unwrap().phase,
        StoredPhase::OutcomeUnknown
    );
    drop(lease);
    f.close().await;
}

#[tokio::test]
async fn postgres_failed_terminal_commit_preserves_claim_for_storage_acknowledgement_only() {
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let id = reserved.receipt().id.clone();
    let lease = f.store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    sqlx::raw_sql("CREATE FUNCTION reject_receipt_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'owned_deferred_commit_failure'; END $$; CREATE CONSTRAINT TRIGGER reject_receipt_commit AFTER UPDATE ON tenant_workflow_receipts DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_receipt_commit();").execute(&f.admin).await.unwrap();
    let result = AnalysisOutcome::Completed("Storage-only acknowledgement fixture".into());
    assert!(matches!(
        f.store.finish(&proof, &result).await,
        Err(Error::Database(_))
    ));
    let phase: String =
        sqlx::query_scalar("SELECT phase FROM tenant_workflow_receipts WHERE id=$1")
            .bind(&id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(phase, "dispatching");
    sqlx::raw_sql("DROP TRIGGER reject_receipt_commit ON tenant_workflow_receipts; DROP FUNCTION reject_receipt_commit();").execute(&f.admin).await.unwrap();
    assert_eq!(
        f.store.finish(&proof, &result).await.unwrap().phase,
        StoredPhase::Completed
    );
    drop(lease);
    f.close().await;
}
async fn wait_for_owned_lock_wait(f: &Fixture) {
    tokio::time::timeout(Duration::from_secs(3),async {
        loop {
            let blocked:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND wait_event_type='Lock')").bind(&f.role).fetch_one(&f.admin).await.unwrap();
            if blocked { break; }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("The actual restricted receipt transaction must reach the held authority lock");
}
#[tokio::test]
async fn postgres_revocation_during_authority_lock_wait_prevents_admission() {
    let f = Fixture::open().await;
    let admitted = f.admitted(0).await;
    let mut lock = f.admin.begin().await.unwrap();
    sqlx::query("SELECT id FROM users WHERE id='owner-a' FOR UPDATE")
        .fetch_one(&mut *lock)
        .await
        .unwrap();
    let store = f.store.clone();
    let work = tokio::spawn(async move { store.reserve(admitted, request(Uuid::new_v4())).await });
    wait_for_owned_lock_wait(&f).await;
    sqlx::query("UPDATE users SET active=false WHERE id='owner-a'")
        .execute(&mut *lock)
        .await
        .unwrap();
    lock.commit().await.unwrap();
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(3), work)
            .await
            .unwrap()
            .unwrap(),
        Err(Error::Forbidden)
    ));
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM tenant_workflow_receipts")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    assert_eq!(count, 0);
    f.close().await;
}
#[tokio::test]
async fn postgres_finish_lock_contention_is_bounded_and_does_not_report_success() {
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let id = reserved.receipt().id.clone();
    let lease = f.store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    let mut lock = f.admin.begin().await.unwrap();
    sqlx::query("SELECT id FROM tenant_workflow_receipts WHERE id=$1 FOR UPDATE")
        .bind(&id)
        .fetch_one(&mut *lock)
        .await
        .unwrap();
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        f.store.finish(
            &proof,
            &AnalysisOutcome::Completed("Must not claim success while locked".into()),
        ),
    )
    .await
    .expect("Receipt lock acquisition must be bounded");
    assert!(matches!(
        result,
        Err(Error::Database(_)) | Err(Error::Unavailable)
    ));
    lock.rollback().await.unwrap();
    let phase: String =
        sqlx::query_scalar("SELECT phase FROM tenant_workflow_receipts WHERE id=$1")
            .bind(&id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(phase, "dispatching");
    drop(lease);
    f.close().await;
}

#[tokio::test]
async fn postgres_changed_request_and_oversized_output_are_refused_without_mutation() {
    let f = Fixture::open().await;
    let id = Uuid::new_v4();
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(id))
        .await
        .unwrap();
    let saved = reserved.receipt().clone();
    let (claims, headers) = &f.identities[0];
    let different = f
        .execution
        .admit(
            claims,
            headers,
            "Changed task cannot replace an existing operation",
            "Auto",
            "analysis",
        )
        .await
        .unwrap();
    assert!(matches!(
        f.store.reserve(different, request(id)).await,
        Err(Error::Conflict)
    ));
    let lease = f.store.claim(reserved).await.unwrap().unwrap();
    assert!(matches!(
        f.store
            .finish(
                &lease.completion_proof(),
                &AnalysisOutcome::Completed("x".repeat(64001))
            )
            .await,
        Err(Error::Invalid)
    ));
    let phase: String =
        sqlx::query_scalar("SELECT phase FROM tenant_workflow_receipts WHERE id=$1")
            .bind(&saved.id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(phase, "dispatching");
    let mut tx = f.pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.current_tenant','receipt-pg-a',true)")
        .execute(&mut *tx)
        .await
        .unwrap();
    assert!(
        sqlx::query("UPDATE tenant_workflow_receipts SET payload='{}' WHERE id=$1")
            .bind(&saved.id)
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    drop(lease);
    f.close().await;
}
#[tokio::test]
async fn postgres_reopened_expired_claim_becomes_unknown_and_never_reclaims() {
    use sha2::{Digest, Sha256};
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let (payload,token,expiry,session):(String,String,i64,Option<String>)=sqlx::query_as("SELECT payload,token_id,token_expires_at,session_id FROM tenant_workflow_receipts WHERE id=$1").bind(&reserved.receipt().id).fetch_one(&f.admin).await.unwrap();
    drop(reserved);
    let id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4();
    let now: i64 =
        sqlx::query_scalar("SELECT floor(extract(epoch FROM clock_timestamp()))::bigint")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    // Build a historical storage fixture through the real immutable admission and
    // claim transitions. No production clock is replaced and no trigger disabled.
    // Preserve the exact production payload bytes inside the fingerprint tuple.
    let mut identity = serde_json::to_string(&(
        "ohc-tenant-text-admission-v1",
        "receipt-pg-a",
        "owner-a",
        request_id.to_string(),
        &token,
        expiry,
        &session,
    ))
    .unwrap();
    assert_eq!(identity.pop(), Some(']'));
    identity.push(',');
    identity.push_str(&payload);
    identity.push(']');
    let fingerprint = format!("{:x}", Sha256::digest(identity.as_bytes()));
    sqlx::query("INSERT INTO tenant_workflow_receipts(id,tenant_id,actor_id,request_id,fingerprint,payload,token_id,token_expires_at,session_id,phase,generation,created_at,updated_at) VALUES($1,'receipt-pg-a','owner-a',$2,$3,$4,$5,$6,$7,'queued',0,$8,$8)").bind(&id).bind(request_id.to_string()).bind(fingerprint).bind(payload).bind(token).bind(expiry).bind(session).bind(now-121).execute(&f.admin).await.unwrap();
    sqlx::query("UPDATE tenant_workflow_receipts SET phase='dispatching',generation=1,lease=$1,lease_expires_at=$2,updated_at=$3 WHERE id=$4").bind(Uuid::new_v4().to_string()).bind(now-1).bind(now-120).bind(&id).execute(&f.admin).await.unwrap();
    let reopened = ReceiptStore::new(AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(f.pool.clone()),
    ));
    let recovered = reopened.get(&f.authority(0).await, &id).await.unwrap();
    assert_eq!(recovered.phase, StoredPhase::OutcomeUnknown);
    assert_eq!(recovered.output, None);
    assert_eq!(recovered.error.as_deref(), Some("lease_expired"));
    let replay = reopened
        .reserve(f.admitted(0).await, request(request_id))
        .await
        .unwrap();
    assert!(replay.replayed());
    assert_eq!(replay.receipt(), &recovered);
    assert!(reopened.claim(replay).await.unwrap().is_none());
    drop(reopened);
    f.close().await;
}

#[tokio::test]
async fn postgres_token_revocation_committed_while_receipt_is_locked_discards_result() {
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let id = reserved.receipt().id.clone();
    let lease = f.store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    let mut lock = f.admin.begin().await.unwrap();
    sqlx::query("SELECT id FROM tenant_workflow_receipts WHERE id=$1 FOR UPDATE")
        .bind(&id)
        .fetch_one(&mut *lock)
        .await
        .unwrap();
    let store = f.store.clone();
    let work = tokio::spawn(async move {
        store
            .finish(
                &proof,
                &AnalysisOutcome::Completed(
                    "Private output from an already revoked attempt".into(),
                ),
            )
            .await
    });
    wait_for_owned_lock_wait(&f).await;
    let claims = &f.identities[0].0;
    f.auth
        .revoke_token(
            claims.jti.clone(),
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "receipt-pg-a",
        )
        .await
        .unwrap();
    lock.commit().await.unwrap();
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(3), work)
            .await
            .unwrap()
            .unwrap(),
        Err(Error::Forbidden)
    ));
    let (phase, output, error): (String, Option<String>, Option<String>) =
        sqlx::query_as("SELECT phase,output,error FROM tenant_workflow_receipts WHERE id=$1")
            .bind(&id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(phase, "outcome_unknown");
    assert_eq!(output, None);
    assert_eq!(error.as_deref(), Some("authority_lost"));
    drop(lease);
    f.close().await;
}

#[tokio::test]
async fn postgres_every_canonical_revocation_writer_waits_for_the_final_receipt_commit_fence() {
    let f = Fixture::open().await;
    let mut observed = Vec::new();
    for direct in [false, true] {
        let claims = f.identities[if direct { 2 } else { 0 }].0.clone();
        let mut finishing = f.admin.begin().await.unwrap();
        // This is the last-step shared fence held by a receipt commit. It must
        // also cover direct canonical-table inserts, not just an HTTP endpoint.
        sqlx::query("SELECT pg_advisory_xact_lock_shared(hashtextextended(jsonb_build_array('ohc-token-fence-v1',$1::text,$2::text)::text,0))")
            .bind("receipt-pg-a").bind(&claims.jti).execute(&mut *finishing).await.unwrap();
        let auth = f.auth.clone();
        let pool = f.admin.clone();
        let (sent, received) = tokio::sync::oneshot::channel();
        let writer = tokio::spawn(async move {
            let result = if direct {
                sqlx::query("INSERT INTO auth_revoked_tokens(jti,tenant_id,expires_at) VALUES($1,'receipt-pg-a',$2)")
                    .bind(claims.jti)
                    .bind(chrono::DateTime::from_timestamp(claims.exp, 0).unwrap())
                    .execute(&pool).await.map(|_| ()).map_err(|e| e.to_string())
            } else {
                auth.revoke_token(
                    claims.jti,
                    chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                    "receipt-pg-a",
                )
                .await
            };
            let _ = sent.send(());
            result
        });
        let completed_before_receipt_commit =
            tokio::time::timeout(Duration::from_millis(150), received)
                .await
                .is_ok();
        finishing.commit().await.unwrap();
        writer.await.unwrap().unwrap();
        observed.push(completed_before_receipt_commit);
    }
    f.close().await;
    assert_eq!(
        observed,
        [false, false],
        "canonical repository and direct revocation writers must serialize with receipt commit"
    );
}

#[tokio::test]
async fn postgres_receipt_final_commit_fence_closes_revocation_after_the_last_authority_read() {
    let f = Fixture::open().await;
    let reserved = f
        .store
        .reserve(f.admitted(0).await, request(Uuid::new_v4()))
        .await
        .unwrap();
    let lease = f.store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    sqlx::raw_sql("CREATE FUNCTION block_receipt_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM pg_advisory_xact_lock(720031921); RETURN NEW; END $$; CREATE CONSTRAINT TRIGGER block_receipt_commit AFTER UPDATE ON tenant_workflow_receipts DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION block_receipt_commit();").execute(&f.admin).await.unwrap();
    let mut barrier = f.admin.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock(720031921)")
        .execute(&mut *barrier)
        .await
        .unwrap();
    let store = f.store.clone();
    let finishing = tokio::spawn(async move {
        store
            .finish(
                &proof,
                &AnalysisOutcome::Completed(
                    "Confirmed provider result awaiting durable commit".into(),
                ),
            )
            .await
    });
    wait_for_owned_lock_wait(&f).await;
    let auth = f.auth.clone();
    let claims = f.identities[0].0.clone();
    let (sent, received) = tokio::sync::oneshot::channel();
    let revoking = tokio::spawn(async move {
        let result = auth
            .revoke_token(
                claims.jti,
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                "receipt-pg-a",
            )
            .await;
        let _ = sent.send(());
        result
    });
    let revocation_overtook_commit = tokio::time::timeout(Duration::from_millis(150), received)
        .await
        .is_ok();
    barrier.commit().await.unwrap();
    let finished = finishing.await.unwrap();
    revoking.await.unwrap().unwrap();
    drop(lease);
    f.close().await;
    assert!(
        !revocation_overtook_commit,
        "revocation cannot commit between the last authority check and receipt COMMIT"
    );
    assert_eq!(finished.unwrap().phase, StoredPhase::Completed);
}

#[tokio::test]
async fn forced_rls_postgres_worker_uses_the_configured_pool_and_settles_actual_provider_usage() {
    use axum::{Json, Router, routing::post};
    use server_harness::middleware::usage_ledger::UsageLedger;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let f = Fixture::open().await;
    let ledger = UsageLedger::Postgres(f.pool.clone());
    ledger.set_limit("receipt-pg-a", 5000).await.unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let observed = calls.clone();
    let provider_ledger = ledger.clone();
    let app=Router::new().route("/v1/chat/completions",post(move |Json(body):Json<serde_json::Value>| {let observed=observed.clone();let ledger=provider_ledger.clone();async move {
        assert_eq!(body["model"],"owned-receipt-model");assert_eq!(body["max_tokens"],256);
        let records=ledger.records("receipt-pg-a","").await.unwrap();assert!(records.iter().any(|record|record.state=="in_flight" && record.reserved_micros>160));
        observed.fetch_add(1,Ordering::SeqCst);
        Json(serde_json::json!({"id":"owned-pg-provider-receipt","choices":[{"message":{"role":"assistant","content":"Observed PostgreSQL provider result"},"finish_reason":"stop"}],"usage":{"prompt_tokens":100,"completion_tokens":30}}))
    }}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/v1", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let values=[("OMNISOLO_LLM_PROVIDER","openai-compatible".to_owned()),("OMNISOLO_LLM_MODEL","owned-receipt-model".into()),("OMNISOLO_LLM_ENDPOINT",endpoint),("OMNISOLO_LLM_API_KEY","public-local-pg-fixture".into()),("OMNISOLO_MAX_TOKENS","256".into()),("OMNISOLO_LLM_TENANT_ID","receipt-pg-a".into()),("OMNISOLO_USAGE_PAYER","managed_api".into()),("OMNISOLO_USAGE_MAX_REQUEST_MICROS","20000".into()),("OMNISOLO_USAGE_RATE_CARDS",r#"{"openai-compatible/owned-receipt-model":{"revision":"owned-pg-tariff","input_micros_per_million":1000000,"output_micros_per_million":2000000,"cached_input_micros_per_million":1000000}}"#.into())];
    let old = values
        .iter()
        .map(|(key, _)| (*key, std::env::var(key).ok()))
        .collect::<Vec<_>>();
    for (key, value) in &values {
        unsafe {
            std::env::set_var(key, value);
        }
    }
    let database = AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(f.pool.clone()),
    );
    let execution = crate::configured_workflow_execution(f.auth.clone(), database);
    for (key, value) in old {
        unsafe {
            if let Some(value) = value {
                std::env::set_var(key, value);
            } else {
                std::env::remove_var(key);
            }
        }
    }
    let (claims, headers) = &f.identities[0];
    let metadata = request(Uuid::new_v4());
    let task = "The actual owner supplied this bounded text";
    let reserved = execution
        .prepare(claims, headers, task, metadata.clone())
        .await
        .unwrap();
    let saved = reserved.receipt().clone();
    assert_eq!(saved.tenant_id, "receipt-pg-a");
    assert_eq!(saved.actor_id, claims.sub);
    assert_eq!(
        saved.funding.as_ref().unwrap().operator_tenant,
        "receipt-pg-a"
    );
    assert!(matches!(
        execution
            .prepare(
                &f.identities[1].0,
                &f.identities[1].1,
                task,
                request(Uuid::new_v4())
            )
            .await,
        Err(Error::Forbidden)
    ));
    assert!(matches!(
        execution
            .prepare(claims, headers, task, request(Uuid::new_v4()))
            .await,
        Err(Error::Budget)
    ));
    execution.dispatch(reserved, None).await.unwrap();
    execution.wait_for_workers().await;
    let replay = execution
        .prepare(claims, headers, task, metadata)
        .await
        .unwrap();
    assert!(replay.replayed());
    execution.dispatch(replay, None).await.unwrap();
    execution.wait_for_workers().await;
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let account = ledger.summary("receipt-pg-a").await.unwrap();
    assert_eq!(account.spent_micros, 160);
    assert_eq!(account.reserved_micros, 0);
    let receipt = execution
        .get_receipt(claims, headers, &saved.id)
        .await
        .unwrap();
    assert_eq!(receipt.phase, StoredPhase::Completed);
    assert_eq!(
        receipt.output.as_deref(),
        Some("Observed PostgreSQL provider result")
    );
    let rows = ledger.records("receipt-pg-a", "").await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].receipt.as_ref().unwrap().provider_request_id,
        "owned-pg-provider-receipt"
    );
    let naked: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ohc_usage_records")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(
        naked, 0,
        "forced RLS forbids usage reads without current tenant context"
    );
    // Reapplying deployment DDL cannot reset an observed, settled charge.
    let before_summary = ledger.summary("receipt-pg-a").await.unwrap();
    let before_rows = ledger.records("receipt-pg-a", "").await.unwrap();
    let migration_db = AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(f.admin.clone()),
    );
    migration::migrate(&migration_db).await.unwrap();
    assert_eq!(
        ledger.summary("receipt-pg-a").await.unwrap(),
        before_summary
    );
    assert_eq!(
        ledger.records("receipt-pg-a", "").await.unwrap(),
        before_rows
    );
    server.abort();
    let _ = server.await;
    drop(execution);
    drop(ledger);
    f.close().await;
}

#[tokio::test]
async fn postgres_readback_pagination_and_exact_actor_request_lookup_preserve_forced_rls() {
    use crate::workflow_execution::receipts::ReceiptQuery;
    let f = Fixture::open().await;
    let authority = f.authority(0).await;
    let mut saved = Vec::new();
    for _ in 0..25 {
        let reserved = f
            .store
            .reserve(f.admitted(0).await, request(Uuid::new_v4()))
            .await
            .unwrap();
        saved.push(reserved.receipt().clone());
    }
    let target = &saved[0];
    assert_eq!(
        f.store
            .by_request_id(&authority, &target.request_id)
            .await
            .unwrap()
            .id,
        target.id
    );
    for identity in [1, 2] {
        assert!(matches!(
            f.store
                .by_request_id(&f.authority(identity).await, &target.request_id)
                .await,
            Err(Error::NotFound)
        ));
    }
    let mut query = ReceiptQuery {
        limit: Some(7),
        before: None,
    };
    let mut ids = std::collections::HashSet::new();
    loop {
        let rows = f.store.list(&authority, &query).await.unwrap();
        assert!(rows.len() <= 8);
        for row in rows.iter().take(7) {
            assert!(ids.insert(row.id.clone()));
        }
        if rows.len() <= 7 {
            break;
        }
        query.before = Some(format!("{}:{}", rows[6].created_at, rows[6].id));
    }
    assert_eq!(ids.len(), 25);
    assert!(
        f.store
            .list(&f.authority(1).await, &ReceiptQuery::default())
            .await
            .unwrap()
            .is_empty()
    );
    query.before = Some("0:aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".into());
    assert!(f.store.list(&authority, &query).await.unwrap().is_empty());
    f.close().await;
}

async fn usage_api_fixture(f: &Fixture) -> (tokio::task::JoinHandle<()>, String) {
    use axum::Router;
    let hub = Arc::new(crate::hub::Hub {
        task_manager: crate::hub::TaskManager {
            db: std::sync::RwLock::new(Some(Arc::new(crate::db::DB {
                store: crate::db::DbStore::Postgres,
                pool: f.pool.clone(),
            }))),
        },
    });
    let app = Router::new()
        .nest(
            "/api/v1/billing",
            crate::usage_api::router().with_state(hub),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            f.auth.clone(),
            server_auth::strict_bearer_auth_middleware,
        ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    (
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        }),
        url,
    )
}
async fn usage_proxy(f: &Fixture, base_url: &str, operation: &str) -> std::process::Output {
    use tokio::io::AsyncWriteExt;
    let (claims, headers) = &f.identities[0];
    let token = headers["authorization"]
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
        .unwrap();
    let mut child = tokio::process::Command::new("node")
        .kill_on_drop(true)
        .arg("scripts/agent-workflow-contract/usage-api-proxy-proof.cjs")
        .current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"base_url":base_url,"operation":operation,"tenant":claims.organization_id,"actor":claims.sub,"token":token}).to_string().as_bytes()).await.unwrap();
    tokio::time::timeout(Duration::from_secs(15), child.wait_with_output())
        .await
        .unwrap()
        .unwrap()
}
#[tokio::test]
async fn usage_owner_set_limit_works_without_schema_privileges_through_real_proxy() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::open().await;
    let ledger = UsageLedger::Postgres(f.pool.clone());
    ledger.set_limit("receipt-pg-a", 1000).await.unwrap();
    ledger.set_limit("receipt-pg-b", 777).await.unwrap();
    let (server, url) = usage_api_fixture(&f).await;
    let output = usage_proxy(&f, &url, "set_limit").await;
    server.abort();
    let _ = server.await;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        ledger.summary("receipt-pg-a").await.unwrap().limit_micros,
        3000
    );
    assert_eq!(
        ledger.summary("receipt-pg-b").await.unwrap().limit_micros,
        777
    );
    assert_eq!(
        ledger.summary("receipt-pg-a").await.unwrap().spent_micros,
        0
    );
    f.close().await;
}
#[tokio::test]
async fn usage_owner_records_accept_verified_proxy_identity_without_cross_tenant_override() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::open().await;
    let ledger = UsageLedger::Postgres(f.pool.clone());
    for (tenant, event) in [
        ("receipt-pg-a", "owned-record-a"),
        ("receipt-pg-b", "private-record-b"),
    ] {
        ledger.set_limit(tenant, 1000).await.unwrap();
        ledger
            .reserve(
                &server_harness::middleware::usage_ledger::UsageScope {
                    tenant_id: tenant.into(),
                    task_id: event.into(),
                    attempt_id: "1".into(),
                    provider: "ollama".into(),
                    model: "owned-ledger-storage-fixture".into(),
                    payer: server_harness::middleware::usage_ledger::PayerMode::Local,
                    rate_card: None,
                },
                event,
                &"a".repeat(64),
                0,
            )
            .await
            .unwrap();
    }
    let (server, url) = usage_api_fixture(&f).await;
    let output = usage_proxy(&f, &url, "records").await;
    let client = reqwest::Client::new();
    for query in [
        "tenant_id=receipt-pg-b",
        "user_id=peer-a",
        "extra=unsafe",
        "after=x&after=y",
    ] {
        let response = client
            .get(format!("{url}/api/v1/billing/usage/records?{query}"))
            .header("authorization", f.identities[0].1["authorization"].clone())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    }
    server.abort();
    let _ = server.await;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    f.close().await;
}
