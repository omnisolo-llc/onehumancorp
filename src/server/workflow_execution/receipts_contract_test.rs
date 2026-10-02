//! Storage contract cases for the next durable-admission checkpoint.
//! These require the configured portable database and never invoke a provider.
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
    ) -> Pin<Box<dyn Future<Output = Result<String, ()>> + Send + 'a>> {
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
