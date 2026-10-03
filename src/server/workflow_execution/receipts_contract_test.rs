//! Portable receipt and tracked-worker lifecycle contracts.
//! These use real databases and never invoke a live provider.
use super::receipts::{ReceiptStore, RequestMetadata, StoredPhase};
use super::*;
use crate::persistence::{AppDatabase, migration};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

struct ReceiptDirectory(std::path::PathBuf);
impl ReceiptDirectory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("ohc-receipt-{}", Uuid::new_v4()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for ReceiptDirectory {
    fn drop(&mut self) {
        // This exact directory was created uniquely by this test above.
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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
        Box::pin(async { panic!("Storage contracts must not call a provider") })
    }
}

struct ReceiptFixture {
    database: AppDatabase,
    auth: Arc<server_auth::Store>,
    execution: WorkflowExecution,
    identity: (Claims, axum::http::HeaderMap),
    foreign: (Claims, axum::http::HeaderMap),
}
impl ReceiptFixture {
    async fn open(url: &str) -> Self {
        let database = AppDatabase::connect(url).await.unwrap();
        migration::migrate(&database).await.unwrap();
        database
            .connection()
            .execute_unprepared(include_str!(
                "../persistence/tenant_execution_receipts_sqlite.sql"
            ))
            .await
            .unwrap();
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone()),
        )));
        let mut identities = Vec::new();
        for tenant in ["receipt-a", "receipt-b"] {
            let user = auth
                .create_user(
                    format!("owner-{tenant}-{}", Uuid::new_v4()),
                    format!("{}@example.test", Uuid::new_v4()),
                    "public-local-receipt-fixture-password".into(),
                    vec![server_auth::ROLE_ADMIN.into()],
                    tenant.into(),
                )
                .await
                .unwrap();
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
            AnalysisPolicy::new("ollama".into(), "receipt-model".into(), 256).unwrap(),
            Arc::new(NeverInfer),
        );
        Self {
            database,
            auth,
            execution,
            identity: identities.remove(0),
            foreign: identities.remove(0),
        }
    }
    fn store(&self) -> ReceiptStore {
        ReceiptStore::new(self.database.clone())
    }
    async fn admitted(&self) -> AdmittedAnalysis {
        self.execution
            .admit(
                &self.identity.0,
                &self.identity.1,
                "Preserve this real submitted text across a new database connection",
                "Auto",
                "analysis",
            )
            .await
            .unwrap()
    }
    async fn authority(&self) -> Authority {
        WorkflowExecution::unavailable(self.auth.clone())
            .authorize(&self.identity.0, &self.identity.1)
            .await
            .unwrap()
    }
}
fn request() -> RequestMetadata {
    RequestMetadata {
        request_id: Uuid::new_v4(),
        name: "Owned durable text task".into(),
        workflow: "analysis".into(),
        requested_model: "Auto".into(),
        agent_role: None,
    }
}

#[tokio::test]
async fn durable_receipt_reopens_on_the_same_configured_sqlite_database_without_provider_access() {
    let directory = ReceiptDirectory::new();
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.0.join("receipts.sqlite").display()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let reserved = fixture
        .store()
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let receipt = reserved.receipt().clone();
    drop(reserved);
    let reopened = ReceiptStore::new(AppDatabase::connect(&url).await.unwrap());
    let actual = reopened
        .get(&fixture.authority().await, &receipt.id)
        .await
        .unwrap();
    assert_eq!(actual, receipt);
    assert_eq!(actual.phase, StoredPhase::Queued);
    assert!(actual.task.contains("real submitted text"));
}

#[tokio::test]
async fn receipt_reads_cannot_cross_a_real_signed_tenant_boundary() {
    let url = format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let reserved = fixture
        .store()
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let foreign = fixture
        .execution
        .authorize(&fixture.foreign.0, &fixture.foreign.1)
        .await
        .unwrap();
    assert!(
        fixture
            .store()
            .get(&foreign, &reserved.receipt().id)
            .await
            .is_err()
    );
    assert_eq!(
        fixture
            .store()
            .get(&fixture.authority().await, &reserved.receipt().id)
            .await
            .unwrap(),
        *reserved.receipt()
    );
}

#[tokio::test]
async fn repeated_request_has_one_receipt_and_cannot_obtain_a_second_dispatch() {
    let url = format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let metadata = request();
    let store = fixture.store();
    let first = store
        .reserve(fixture.admitted().await, metadata.clone())
        .await
        .unwrap();
    let id = first.receipt().id.clone();
    let repeat = store
        .reserve(fixture.admitted().await, metadata)
        .await
        .unwrap();
    assert_eq!(repeat.receipt().id, id);
    assert!(repeat.replayed());
    assert!(store.claim(repeat).await.unwrap().is_none());
    assert!(store.claim(first).await.unwrap().is_some());
}

#[tokio::test]
async fn changed_payload_cannot_reuse_an_existing_request_identity() {
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let store = fixture.store();
    let metadata = request();
    let original = store
        .reserve(fixture.admitted().await, metadata.clone())
        .await
        .unwrap();
    let altered = fixture
        .execution
        .admit(
            &fixture.identity.0,
            &fixture.identity.1,
            "A different private task must not replace the acknowledged request",
            "Auto",
            "analysis",
        )
        .await
        .unwrap();
    assert!(matches!(
        store.reserve(altered, metadata).await,
        Err(super::receipts::Error::Conflict)
    ));
    assert_eq!(
        store
            .get(&fixture.authority().await, &original.receipt().id)
            .await
            .unwrap(),
        *original.receipt()
    );
}

#[tokio::test]
async fn competing_reservations_have_one_durable_identity_and_one_claim() {
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let left = fixture.store();
    let right = fixture.store();
    let metadata = request();
    let left_input = fixture.admitted().await;
    let right_input = fixture.admitted().await;
    let (left_result, right_result) = tokio::join!(
        left.reserve(left_input, metadata.clone()),
        right.reserve(right_input, metadata)
    );
    let left_result = left_result.unwrap();
    let right_result = right_result.unwrap();
    assert_eq!(left_result.receipt().id, right_result.receipt().id);
    let (left_claim, right_claim) =
        tokio::join!(left.claim(left_result), right.claim(right_result));
    assert_eq!(
        usize::from(left_claim.unwrap().is_some()) + usize::from(right_claim.unwrap().is_some()),
        1
    );
}

#[tokio::test]
async fn storage_rechecks_role_disabled_user_and_token_after_admission() {
    for loss in ["role", "disabled", "token"] {
        let fixture = ReceiptFixture::open(&format!(
            "sqlite:file:receipts_{}?mode=memory&cache=shared",
            Uuid::new_v4()
        ))
        .await;
        let store = fixture.store();
        let authority = fixture.authority().await;
        let original = store
            .reserve(fixture.admitted().await, request())
            .await
            .unwrap();
        let receipt_id = original.receipt().id.clone();
        let pending = fixture.admitted().await;
        let claims = &fixture.identity.0;
        let tenant = "receipt-a";
        match loss {
            "role" => {
                fixture
                    .auth
                    .update_user(&claims.sub, None, Some(vec!["STAFF".into()]), None, tenant)
                    .await
                    .unwrap();
            }
            "disabled" => {
                fixture
                    .auth
                    .update_user(&claims.sub, None, None, Some(false), tenant)
                    .await
                    .unwrap();
            }
            "token" => {
                fixture
                    .auth
                    .revoke_token(
                        claims.jti.clone(),
                        chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                        tenant,
                    )
                    .await
                    .unwrap();
            }
            _ => unreachable!(),
        }
        assert!(
            matches!(
                store.get(&authority, &receipt_id).await,
                Err(super::receipts::Error::Forbidden)
            ),
            "stale read after {loss}"
        );
        assert!(
            matches!(
                store.reserve(pending, request()).await,
                Err(super::receipts::Error::Forbidden)
            ),
            "stale admission after {loss}"
        );
        assert!(
            matches!(
                store.claim(original).await,
                Err(super::receipts::Error::Forbidden)
            ),
            "stale claim after {loss}"
        );
    }
}

#[tokio::test]
async fn public_receipt_never_contains_bearer_or_session_authority_fields() {
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let reserved = fixture
        .store()
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let receipt = serde_json::to_value(reserved.receipt()).unwrap();
    let encoded = serde_json::to_string(&receipt).unwrap();
    let bearer = fixture
        .identity
        .1
        .get(axum::http::header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(!encoded.contains(bearer));
    assert!(!encoded.contains(&fixture.identity.0.jti));
    for field in ["token_id", "expires_at", "session_id", "lease", "authority"] {
        assert!(
            receipt.get(field).is_none(),
            "private field {field} escaped into receipt"
        );
    }
}

#[tokio::test]
async fn completed_storage_result_is_committed_and_identical_terminal_replay_is_idempotent() {
    let directory = ReceiptDirectory::new();
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.0.join("terminal.sqlite").display()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let store = fixture.store();
    let reserved = store
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let lease = store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    // This is a storage artifact supplied by the test. NeverInfer proves this
    // checkpoint does not simulate, invoke or certify an external provider.
    let artifact = AnalysisOutcome::Completed("Explicit storage-only test artifact 雪".into());
    let completed = store.finish(&proof, &artifact).await.unwrap();
    assert_eq!(completed.phase, StoredPhase::Completed);
    assert_eq!(
        completed.output.as_deref(),
        Some("Explicit storage-only test artifact 雪")
    );
    assert_eq!(store.finish(&proof, &artifact).await.unwrap(), completed);
    assert!(matches!(
        store
            .finish(
                &proof,
                &AnalysisOutcome::Completed("Different artifact".into())
            )
            .await,
        Err(super::receipts::Error::Conflict)
    ));
    let reopened = ReceiptStore::new(AppDatabase::connect(&url).await.unwrap());
    assert_eq!(
        reopened
            .get(&fixture.authority().await, &proof.receipt.id)
            .await
            .unwrap(),
        completed
    );
}

#[tokio::test]
async fn terminal_storage_rejects_changed_nonce_generation_and_admitted_payload() {
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let store = fixture.store();
    let reserved = store
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let lease = store.claim(reserved).await.unwrap().unwrap();
    let original = lease.completion_proof();
    for field in ["nonce", "generation", "payload", "fingerprint", "tenant"] {
        let mut altered = original.clone();
        match field {
            "nonce" => altered.nonce = Uuid::new_v4().to_string(),
            "generation" => altered.generation += 1,
            "payload" => altered.receipt.task = "Unadmitted replacement".into(),
            "fingerprint" => altered.fingerprint = "0".repeat(64),
            "tenant" => altered.authority.tenant_id = "receipt-b".into(),
            _ => unreachable!(),
        }
        assert!(
            matches!(
                store
                    .finish(
                        &altered,
                        &AnalysisOutcome::Completed("Storage-only artifact".into())
                    )
                    .await,
                Err(super::receipts::Error::Conflict
                    | super::receipts::Error::NotFound
                    | super::receipts::Error::Forbidden)
            ),
            "accepted altered {field}"
        );
        assert_eq!(
            store
                .get(&fixture.authority().await, &original.receipt.id)
                .await
                .unwrap()
                .phase,
            StoredPhase::Dispatching
        );
    }
    assert_eq!(
        store
            .finish(&original, &AnalysisOutcome::Cancelled)
            .await
            .unwrap()
            .phase,
        StoredPhase::Cancelled
    );
}

#[tokio::test]
async fn empty_or_oversized_terminal_output_never_replaces_the_pending_dispatch() {
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let store = fixture.store();
    let reserved = store
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let lease = store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    for output in [" \n\t".into(), "x".repeat(64_001), "🧪".repeat(16_001)] {
        assert!(matches!(
            store
                .finish(&proof, &AnalysisOutcome::Completed(output))
                .await,
            Err(super::receipts::Error::Invalid)
        ));
        assert_eq!(
            store
                .get(&fixture.authority().await, &proof.receipt.id)
                .await
                .unwrap()
                .phase,
            StoredPhase::Dispatching
        );
    }
    let maximum = "x".repeat(64_000);
    assert_eq!(
        store
            .finish(&proof, &AnalysisOutcome::Completed(maximum.clone()))
            .await
            .unwrap()
            .output,
        Some(maximum)
    );
}

#[tokio::test]
async fn recorded_unknown_outcome_cannot_be_reclaimed_or_overwritten_by_a_late_result() {
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:receipts_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let store = fixture.store();
    let metadata = request();
    let reserved = store
        .reserve(fixture.admitted().await, metadata.clone())
        .await
        .unwrap();
    let lease = store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    let unknown = store
        .finish(&proof, &AnalysisOutcome::OutcomeUnknown)
        .await
        .unwrap();
    assert_eq!(unknown.phase, StoredPhase::OutcomeUnknown);
    assert!(unknown.output.is_none());
    assert_eq!(unknown.error.as_deref(), Some("execution_uncertain"));
    assert_eq!(
        store
            .finish(&proof, &AnalysisOutcome::OutcomeUnknown)
            .await
            .unwrap(),
        unknown
    );
    let replay = store
        .reserve(fixture.admitted().await, metadata)
        .await
        .unwrap();
    assert_eq!(replay.receipt(), &unknown);
    assert!(store.claim(replay).await.unwrap().is_none());
    assert!(matches!(
        store
            .finish(&proof, &AnalysisOutcome::Completed("Late artifact".into()))
            .await,
        Err(super::receipts::Error::Conflict)
    ));
    assert_eq!(
        store
            .get(&fixture.authority().await, &proof.receipt.id)
            .await
            .unwrap(),
        unknown
    );
}

#[tokio::test]
async fn revoked_terminal_authority_discards_output_and_records_uncertainty() {
    for loss in ["role", "disabled", "token"] {
        let fixture = ReceiptFixture::open(&format!(
            "sqlite:file:receipts_{}?mode=memory&cache=shared",
            Uuid::new_v4()
        ))
        .await;
        let store = fixture.store();
        let reserved = store
            .reserve(fixture.admitted().await, request())
            .await
            .unwrap();
        let lease = store.claim(reserved).await.unwrap().unwrap();
        let proof = lease.completion_proof();
        let claims = &fixture.identity.0;
        match loss {
            "role" => {
                fixture
                    .auth
                    .update_user(
                        &claims.sub,
                        None,
                        Some(vec!["STAFF".into()]),
                        None,
                        "receipt-a",
                    )
                    .await
                    .unwrap();
            }
            "disabled" => {
                fixture
                    .auth
                    .update_user(&claims.sub, None, None, Some(false), "receipt-a")
                    .await
                    .unwrap();
            }
            "token" => {
                fixture
                    .auth
                    .revoke_token(
                        claims.jti.clone(),
                        chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                        "receipt-a",
                    )
                    .await
                    .unwrap();
            }
            _ => unreachable!(),
        }
        assert!(matches!(
            store
                .finish(
                    &proof,
                    &AnalysisOutcome::Completed("Must not persist after revoked authority".into())
                )
                .await,
            Err(super::receipts::Error::Forbidden)
        ));
        let row = fixture
            .database
            .connection()
            .query_one(sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                "SELECT phase,output,error FROM tenant_workflow_receipts WHERE id=$1",
                [proof.receipt.id.clone().into()],
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            row.try_get::<String>("", "phase").unwrap(),
            "outcome_unknown"
        );
        assert_eq!(row.try_get::<Option<String>>("", "output").unwrap(), None);
        assert_eq!(
            row.try_get::<String>("", "error").unwrap(),
            "authority_lost"
        );
    }
}

#[tokio::test]
async fn deferred_terminal_commit_failure_preserves_dispatch_and_safe_storage_retry() {
    let directory = ReceiptDirectory::new();
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.0.join("failed-commit.sqlite").display()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let store = fixture.store();
    let reserved = store
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let lease = store.claim(reserved).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    fixture.database.connection().execute_unprepared("CREATE TABLE terminal_parent (id TEXT PRIMARY KEY); CREATE TABLE terminal_child (id TEXT REFERENCES terminal_parent(id) DEFERRABLE INITIALLY DEFERRED); CREATE TRIGGER reject_terminal_commit AFTER UPDATE OF phase ON tenant_workflow_receipts WHEN NEW.phase='completed' BEGIN INSERT INTO terminal_child VALUES('missing-parent'); END;").await.unwrap();
    let artifact = AnalysisOutcome::Completed("Storage artifact for deferred-commit test".into());
    assert!(matches!(
        store.finish(&proof, &artifact).await,
        Err(super::receipts::Error::Database(_))
    ));
    let reopened = ReceiptStore::new(AppDatabase::connect(&url).await.unwrap());
    let unchanged = reopened
        .get(&fixture.authority().await, &proof.receipt.id)
        .await
        .unwrap();
    assert_eq!(unchanged.phase, StoredPhase::Dispatching);
    assert!(unchanged.output.is_none());
    fixture
        .database
        .connection()
        .execute_unprepared("DROP TRIGGER reject_terminal_commit")
        .await
        .unwrap();
    let completed = reopened.finish(&proof, &artifact).await.unwrap();
    assert_eq!(completed.phase, StoredPhase::Completed);
    assert_eq!(
        store
            .get(&fixture.authority().await, &proof.receipt.id)
            .await
            .unwrap(),
        completed
    );
}

#[tokio::test]
async fn reopening_an_expired_dispatch_records_unknown_without_reexecution_or_late_completion() {
    use sha2::{Digest, Sha256};
    let directory = ReceiptDirectory::new();
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.0.join("expired.sqlite").display()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let store = fixture.store();
    let reserved = store
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let lease = store.claim(reserved).await.unwrap().unwrap();
    let mut expired = lease.completion_proof();
    let backend = sea_orm::DatabaseBackend::Sqlite;
    let connection = fixture.database.connection();
    let actual = connection.query_one(sea_orm::Statement::from_sql_and_values(backend, "SELECT payload,CAST(strftime('%s','now') AS INTEGER) AS now FROM tenant_workflow_receipts WHERE id=$1", [expired.receipt.id.clone().into()])).await.unwrap().unwrap();
    let payload: String = actual.try_get("", "payload").unwrap();
    let now: i64 = actual.try_get("", "now").unwrap();
    // Build an explicitly backdated restart fixture through the canonical
    // admission/claim transitions. No schema trigger is dropped, no clock is
    // mocked and no assertion claims that a provider performed this fixture.
    expired.receipt.id = Uuid::new_v4().to_string();
    expired.receipt.request_id = Uuid::new_v4().to_string();
    expired.receipt.created_at = now - 300;
    expired.receipt.updated_at = now - 240;
    expired.nonce = Uuid::new_v4().to_string();
    let prefix = serde_json::to_string(&(
        "ohc-tenant-text-admission-v1",
        &expired.authority.tenant_id,
        &expired.authority.actor_id,
        &expired.receipt.request_id,
        &expired.authority.token_id,
        expired.authority.expires_at,
        expired.authority.session_id.as_deref(),
    ))
    .unwrap();
    let encoded = format!("{},{}]", prefix.strip_suffix(']').unwrap(), payload);
    expired.fingerprint = format!("{:x}", Sha256::digest(encoded.as_bytes()));
    connection.execute(sea_orm::Statement::from_sql_and_values(backend,
        "INSERT INTO tenant_workflow_receipts(id,tenant_id,actor_id,request_id,fingerprint,payload,token_id,token_expires_at,session_id,phase,generation,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,'queued',0,$10,$10)",
        vec![expired.receipt.id.clone().into(), expired.authority.tenant_id.clone().into(), expired.authority.actor_id.clone().into(), expired.receipt.request_id.clone().into(), expired.fingerprint.clone().into(), payload.into(), expired.authority.token_id.clone().into(), expired.authority.expires_at.into(), expired.authority.session_id.clone().into(), (now-300).into()]
    )).await.unwrap();
    connection.execute(sea_orm::Statement::from_sql_and_values(backend,
        "UPDATE tenant_workflow_receipts SET phase='dispatching',generation=1,lease=$1,lease_expires_at=$2,updated_at=$3 WHERE id=$4",
        vec![expired.nonce.clone().into(), (now-120).into(), (now-240).into(), expired.receipt.id.clone().into()]
    )).await.unwrap();
    let reopened = ReceiptStore::new(AppDatabase::connect(&url).await.unwrap());
    let recovered = reopened
        .get(&fixture.authority().await, &expired.receipt.id)
        .await
        .unwrap();
    assert_eq!(recovered.phase, StoredPhase::OutcomeUnknown);
    assert_eq!(recovered.error.as_deref(), Some("lease_expired"));
    assert!(recovered.output.is_none());
    assert!(matches!(
        reopened
            .finish(
                &expired,
                &AnalysisOutcome::Completed("Late storage artifact".into())
            )
            .await,
        Err(super::receipts::Error::Conflict)
    ));
    assert_eq!(
        reopened
            .get(&fixture.authority().await, &expired.receipt.id)
            .await
            .unwrap(),
        recovered
    );
    let metadata = RequestMetadata {
        request_id: Uuid::parse_str(&expired.receipt.request_id).unwrap(),
        name: expired.receipt.name.clone(),
        workflow: expired.receipt.workflow.clone(),
        requested_model: "Auto".into(),
        agent_role: None,
    };
    let replay = reopened
        .reserve(fixture.admitted().await, metadata)
        .await
        .unwrap();
    assert_eq!(replay.receipt(), &recovered);
    assert!(reopened.claim(replay).await.unwrap().is_none());
}

#[tokio::test]
async fn an_unclaimed_restart_receipt_expires_on_both_direct_read_and_request_replay() {
    use sha2::{Digest, Sha256};
    let fixture = ReceiptFixture::open(&format!(
        "sqlite:file:queued_restart_{}?mode=memory&cache=shared",
        Uuid::new_v4()
    ))
    .await;
    let store = fixture.store();
    let authority = fixture.authority().await;
    let original = store
        .reserve(fixture.admitted().await, request())
        .await
        .unwrap();
    let connection = fixture.database.connection();
    let backend = sea_orm::DatabaseBackend::Sqlite;
    let row=connection.query_one(sea_orm::Statement::from_sql_and_values(backend,"SELECT payload,CAST(strftime('%s','now') AS INTEGER) AS now FROM tenant_workflow_receipts WHERE id=$1",[original.receipt().id.clone().into()])).await.unwrap().unwrap();
    let payload: String = row.try_get("", "payload").unwrap();
    let now: i64 = row.try_get("", "now").unwrap();
    for by_request in [false, true] {
        let id = Uuid::new_v4().to_string();
        let request_id = Uuid::new_v4();
        let prefix = serde_json::to_string(&(
            "ohc-tenant-text-admission-v1",
            &authority.tenant_id,
            &authority.actor_id,
            request_id.to_string(),
            &authority.token_id,
            authority.expires_at,
            authority.session_id.as_deref(),
        ))
        .unwrap();
        let encoded = format!("{},{}]", prefix.strip_suffix(']').unwrap(), payload);
        let fingerprint = format!("{:x}", Sha256::digest(encoded.as_bytes()));
        // A backdated, never-claimed restart fixture follows the actual INSERT
        // contract. No trigger or database clock is replaced.
        connection.execute(sea_orm::Statement::from_sql_and_values(backend,"INSERT INTO tenant_workflow_receipts(id,tenant_id,actor_id,request_id,fingerprint,payload,token_id,token_expires_at,session_id,phase,generation,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,'queued',0,$10,$10)",vec![id.clone().into(),authority.tenant_id.clone().into(),authority.actor_id.clone().into(),request_id.to_string().into(),fingerprint.into(),payload.clone().into(),authority.token_id.clone().into(),authority.expires_at.into(),authority.session_id.clone().into(),(now-300).into()])).await.unwrap();
        let actual = if by_request {
            let metadata = RequestMetadata {
                request_id,
                name: original.receipt().name.clone(),
                workflow: original.receipt().workflow.clone(),
                requested_model: "Auto".into(),
                agent_role: None,
            };
            store
                .find_request(&authority, &original.receipt().task, &metadata)
                .await
                .unwrap()
                .unwrap()
                .receipt()
                .clone()
        } else {
            store.get(&authority, &id).await.unwrap()
        };
        assert_eq!(
            actual.phase,
            StoredPhase::Cancelled,
            "unclaimed expired request is terminal on read/replay: {by_request}"
        );
        assert!(actual.output.is_none());
    }
}

// This test-only inference future exposes scheduling and cancellation, and only
// returns a failure. It never fabricates provider output, usage or success.
#[derive(Default)]
struct WorkerLifecycleInference {
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
    calls: std::sync::atomic::AtomicUsize,
    returned: std::sync::atomic::AtomicUsize,
    dropped: std::sync::atomic::AtomicUsize,
}
impl TextInference for WorkerLifecycleInference {
    fn infer<'a>(
        &'a self,
        _: &'a AdmittedAnalysis,
    ) -> Pin<Box<dyn Future<Output = Result<InferenceResult, ()>> + Send + 'a>> {
        Box::pin(async move {
            struct ObserveDrop<'a>(&'a std::sync::atomic::AtomicUsize);
            impl Drop for ObserveDrop<'_> {
                fn drop(&mut self) {
                    self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
            let _drop = ObserveDrop(&self.dropped);
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.entered.notify_one();
            self.release.notified().await;
            self.returned
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Err(())
        })
    }
}
fn lifecycle_execution(
    fixture: &ReceiptFixture,
    inference: Arc<WorkerLifecycleInference>,
) -> Arc<WorkflowExecution> {
    Arc::new(
        WorkflowExecution::configured(
            fixture.auth.clone(),
            AnalysisPolicy::new("ollama".into(), "lifecycle-fixture".into(), 256).unwrap(),
            inference,
        )
        .with_receipts(fixture.database.clone()),
    )
}

#[tokio::test]
async fn draining_a_failed_worker_waits_for_its_durable_unknown_receipt() {
    use std::sync::atomic::Ordering::SeqCst;
    let fixture = ReceiptFixture::open("sqlite::memory:").await;
    let inference = Arc::new(WorkerLifecycleInference::default());
    let execution = lifecycle_execution(&fixture, inference.clone());
    let metadata = request();
    let task = "Track this explicitly failing inference through its durable receipt";
    let (claims, headers) = &fixture.identity;
    let reserved = execution
        .prepare(claims, headers, task, metadata.clone())
        .await
        .unwrap();
    let id = reserved.receipt().id.clone();
    execution.dispatch(reserved, None).await.unwrap();
    tokio::time::timeout(Duration::from_secs(3), inference.entered.notified())
        .await
        .unwrap();
    assert_eq!(
        execution
            .get_receipt(claims, headers, &id)
            .await
            .unwrap()
            .phase,
        StoredPhase::Dispatching
    );
    let mut draining = Box::pin(execution.wait_for_workers());
    assert!(
        tokio::time::timeout(Duration::from_millis(20), &mut draining)
            .await
            .is_err(),
        "draining must stay pending while the tracked inference is in flight"
    );
    assert_eq!(inference.returned.load(SeqCst), 0);
    inference.release.notify_one();
    tokio::time::timeout(Duration::from_secs(3), draining)
        .await
        .unwrap();
    let saved = execution.get_receipt(claims, headers, &id).await.unwrap();
    assert_eq!(saved.phase, StoredPhase::OutcomeUnknown);
    assert!(saved.output.is_none());
    assert_eq!(inference.returned.load(SeqCst), 1);
    assert_eq!(inference.dropped.load(SeqCst), 1);
    assert!(execution.workers.lock().await.is_empty());
    let replay = execution
        .prepare(claims, headers, task, metadata)
        .await
        .unwrap();
    assert!(replay.replayed());
    execution.dispatch(replay, None).await.unwrap();
    execution.wait_for_workers().await;
    assert_eq!(inference.calls.load(SeqCst), 1);
}

#[tokio::test]
async fn stopping_an_inflight_worker_preserves_its_claim_for_uncertain_restart_recovery() {
    use std::sync::atomic::Ordering::SeqCst;
    let directory = ReceiptDirectory::new();
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.0.join("stopped-worker.sqlite").display()
    );
    let fixture = ReceiptFixture::open(&url).await;
    let inference = Arc::new(WorkerLifecycleInference::default());
    let execution = lifecycle_execution(&fixture, inference.clone());
    let metadata = request();
    let task = "Stopping local work cannot prove an external effect never happened";
    let (claims, headers) = &fixture.identity;
    let reserved = execution
        .prepare(claims, headers, task, metadata.clone())
        .await
        .unwrap();
    let id = reserved.receipt().id.clone();
    execution.dispatch(reserved, None).await.unwrap();
    tokio::time::timeout(Duration::from_secs(3), inference.entered.notified())
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(3), execution.stop_workers())
        .await
        .unwrap();
    assert_eq!(inference.calls.load(SeqCst), 1);
    assert_eq!(inference.returned.load(SeqCst), 0);
    assert_eq!(inference.dropped.load(SeqCst), 1);
    assert!(execution.workers.lock().await.is_empty());
    drop(execution);
    let reopened_database = AppDatabase::connect(&url).await.unwrap();
    let reopened_auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(
            reopened_database.connection().clone(),
        ),
    )));
    let reopened =
        Arc::new(WorkflowExecution::unavailable(reopened_auth).with_receipts(reopened_database));
    let saved = reopened.get_receipt(claims, headers, &id).await.unwrap();
    assert_eq!(saved.phase, StoredPhase::Dispatching);
    assert!(saved.output.is_none());
    let uncertain = reopened.cancel_receipt(claims, headers, &id).await.unwrap();
    assert_eq!(uncertain.phase, StoredPhase::OutcomeUnknown);
    assert!(uncertain.output.is_none());
    let replay = reopened
        .prepare(claims, headers, task, metadata)
        .await
        .unwrap();
    assert!(replay.replayed());
    assert_eq!(replay.receipt().phase, StoredPhase::OutcomeUnknown);
    reopened.dispatch(replay, None).await.unwrap();
    reopened.wait_for_workers().await;
    assert_eq!(inference.calls.load(SeqCst), 1);
}
