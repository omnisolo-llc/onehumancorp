//! Actual authenticated API, canonical repositories and database queues.
use crate::dynamic_workflow_manager::{
    DynamicWorkflowManager, DynamicWorkflowPlan, WorkflowStatus, WorkflowTask, WorkflowTrigger,
};
use crate::queue::{SqliteTaskQueue, TaskQueue};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Request, StatusCode, header::AUTHORIZATION},
};
use server_auth::user_repository::UserRepository;
use server_common::Claims;
use sqlx::Row;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tower::ServiceExt;

const TENANT_A: &str = "workflow-owned-a";
const TENANT_B: &str = "workflow-owned-b";
const LEGACY_TENANT: &str = "spiffe://onehumancorp.io/web-session";
const PRIVATE_PROMPT: &str = "Create a workflow to audit private tenant-A records";

struct OwnedDirectory(PathBuf);
impl OwnedDirectory {
    fn new() -> Self {
        let path =
            std::env::temp_dir().join(format!("ohc-dynamic-workflow-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for OwnedDirectory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

struct Fixture {
    directory: OwnedDirectory,
    pool: sqlx::SqlitePool,
    store: Arc<server_auth::Store>,
    queue: Arc<ObservedQueue>,
    manager: Arc<DynamicWorkflowManager>,
    identities: Vec<(Claims, HeaderMap)>,
}
// Observe admission calls while delegating every operation to the real queue.
struct ObservedQueue {
    inner: SqliteTaskQueue,
    admissions: AtomicUsize,
}
#[async_trait::async_trait]
impl TaskQueue for ObservedQueue {
    async fn enqueue(&self, job: crate::queue::Job) -> Result<(), String> {
        self.admissions.fetch_add(1, Ordering::SeqCst);
        self.inner.enqueue(job).await
    }
    async fn enqueue_batch(&self, jobs: Vec<crate::queue::Job>) -> Result<(), String> {
        self.admissions.fetch_add(1, Ordering::SeqCst);
        self.inner.enqueue_batch(jobs).await
    }
    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<crate::queue::Job>, String> {
        self.inner.dequeue(roles).await
    }
    async fn complete(&self, id: &str, tenant: &str) -> Result<(), String> {
        self.inner.complete(id, tenant).await
    }
    async fn fail(&self, id: &str, tenant: &str, reason: &str) -> Result<(), String> {
        self.inner.fail(id, tenant, reason).await
    }
    async fn requeue(&self, job: crate::queue::Job) -> Result<(), String> {
        self.inner.requeue(job).await
    }
    async fn cleanup_stale_jobs(&self) -> Result<u64, String> {
        self.inner.cleanup_stale_jobs().await
    }
}
impl Fixture {
    async fn new() -> Self {
        let directory = OwnedDirectory::new();
        let url = format!(
            "sqlite://{}?mode=rwc",
            directory.0.join("canonical.sqlite").display()
        );
        let options = sqlx::sqlite::SqliteConnectOptions::from_str(&url)
            .unwrap()
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(7));
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(options)
            .await
            .unwrap();
        let database = crate::persistence::AppDatabase::from_connection(
            sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone()),
        );
        crate::persistence::migration::migrate(&database)
            .await
            .unwrap();
        let repository = Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(
            database.connection().clone(),
        ));
        let store = Arc::new(server_auth::Store::with_portable_repo(repository.clone()));
        let mut identities = Vec::new();
        for (id, tenant, role) in [
            ("owner-a", TENANT_A, "OWNER"),
            ("owner-b", TENANT_B, "ADMIN"),
            ("member-a", TENANT_A, "MEMBER"),
        ] {
            let now = chrono::Utc::now();
            let user = server_auth::User {
                id: id.into(),
                username: id.into(),
                email: format!("{id}@example.test"),
                password_hash: String::new(),
                roles: vec![role.into()],
                active: true,
                organization_id: Some(tenant.into()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            };
            repository.create_user(user.clone(), tenant).await.unwrap();
            let token = store.issue_token(&user).unwrap();
            let claims = store.validate_token(&token).await.unwrap();
            let mut headers = HeaderMap::new();
            headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
            identities.push((claims, headers));
        }
        let inner = SqliteTaskQueue::new(pool.clone());
        inner.init().await.unwrap();
        let queue = Arc::new(ObservedQueue {
            inner,
            admissions: AtomicUsize::new(0),
        });
        let manager = Arc::new(DynamicWorkflowManager::with_state_dir(
            queue.clone(),
            directory.0.join("plans"),
        ));
        Self {
            directory,
            pool,
            store,
            queue,
            manager,
            identities,
        }
    }
    fn app(&self) -> Router {
        crate::mounted_dynamic_workflow_boundary(self.manager.clone(), self.store.clone())
    }
    fn restart(&mut self) {
        self.manager = Arc::new(DynamicWorkflowManager::with_state_dir(
            self.queue.clone(),
            self.directory.0.join("plans"),
        ));
    }
    fn plan_path(&self, id: &str) -> PathBuf {
        self.directory.0.join("plans").join(format!("{id}.json"))
    }
    fn files(&self) -> BTreeMap<String, Vec<u8>> {
        let path = self.directory.0.join("plans");
        if !path.exists() {
            return BTreeMap::new();
        }
        std::fs::read_dir(path)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                (
                    entry.file_name().into_string().unwrap(),
                    std::fs::read(entry.path()).unwrap(),
                )
            })
            .collect()
    }
    fn seed(&self, tenant: &str, status: WorkflowStatus) -> DynamicWorkflowPlan {
        let id = format!("dwf-{}", uuid::Uuid::new_v4());
        let now = chrono::Utc::now();
        let plan = DynamicWorkflowPlan {
            id: id.clone(),
            tenant_id: tenant.into(),
            parent_task_id: "owned-parent".into(),
            prompt: PRIVATE_PROMPT.into(),
            status,
            trigger: WorkflowTrigger::ExplicitRequest,
            requires_confirmation: status == WorkflowStatus::AwaitingConfirmation,
            estimated_subagents: 1,
            estimated_token_multiplier: 1.0,
            tasks: vec![WorkflowTask {
                id: format!("{id}-task"),
                title: "private title".into(),
                description: "private task".into(),
                role: "workflow-planner".into(),
                phase: "planning".into(),
                dependencies: vec![],
                verification_of: None,
            }],
            created_at: now,
            updated_at: now,
        };
        self.write_plan(&id, &plan);
        plan
    }
    fn write_plan(&self, id: &str, plan: &DynamicWorkflowPlan) {
        std::fs::create_dir_all(self.directory.0.join("plans")).unwrap();
        std::fs::write(self.plan_path(id), serde_json::to_vec(plan).unwrap()).unwrap();
    }
    async fn count(&self) -> i64 {
        sqlx::query_scalar("SELECT COUNT(*) FROM local_queue_jobs")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
    async fn call(
        &self,
        method: &str,
        path: &str,
        identity: usize,
        body: serde_json::Value,
    ) -> (StatusCode, serde_json::Value) {
        send(self.app(), method, path, &self.identities[identity].1, body).await
    }
    async fn reject_without_effects(
        &self,
        method: &str,
        path: &str,
        headers: &HeaderMap,
        status: StatusCode,
    ) {
        let before = self.files();
        let count = self.count().await;
        let admissions = self.queue.admissions.load(Ordering::SeqCst);
        let (actual, body) = send(
            self.app(),
            method,
            path,
            headers,
            start_request(TENANT_B, true),
        )
        .await;
        assert_eq!(actual, status, "{method} {path}: {body}");
        assert_eq!(self.files(), before, "rejected request changed plan bytes");
        assert_eq!(self.count().await, count, "rejected request admitted jobs");
        assert_eq!(
            self.queue.admissions.load(Ordering::SeqCst),
            admissions,
            "rejected request invoked the queue"
        );
        if status == StatusCode::NOT_FOUND {
            assert_eq!(body, serde_json::json!({"error":"workflow not found"}));
        }
    }
}
fn start_request(tenant: &str, confirm: bool) -> serde_json::Value {
    serde_json::json!({"tenant_id":tenant,"parent_task_id":"owned-parent","prompt":PRIVATE_PROMPT,"confirm":confirm,"max_parallel_agents":4,"verifier_agents_per_task":1})
}
async fn send(
    app: Router,
    method: &str,
    path: &str,
    headers: &HeaderMap,
    body: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    // Axum nesting mounts the inner '/' at the prefix itself.
    let path = if path == "/" { "" } else { path };
    let mut request = Request::builder()
        .method(method)
        .uri(format!("/api/v1/dynamic-workflows{path}"))
        .header("content-type", "application/json")
        .header("x-tenant-id", TENANT_B);
    request.headers_mut().unwrap().extend(headers.clone());
    let response = app
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_048_576).await.unwrap();
    (
        status,
        if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap()
        },
    )
}

#[tokio::test]
async fn sqlite_owned_creation_ignores_every_untrusted_body_tenant_and_persists_real_queue_rows() {
    let f = Fixture::new().await;
    for supplied in [TENANT_B, "", "default", LEGACY_TENANT] {
        let (status, body) = f.call("POST", "/", 0, start_request(supplied, true)).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["plan"]["tenant_id"], TENANT_A);
        let plan: DynamicWorkflowPlan = serde_json::from_value(body["plan"].clone()).unwrap();
        let persisted: DynamicWorkflowPlan =
            serde_json::from_slice(&std::fs::read(f.plan_path(&plan.id)).unwrap()).unwrap();
        assert_eq!(persisted.tenant_id, TENANT_A);
        let rows = sqlx::query("SELECT id,tenant_id,payload FROM local_queue_jobs WHERE task_id=?")
            .bind("owned-parent")
            .fetch_all(&f.pool)
            .await
            .unwrap();
        for task in &plan.tasks {
            let row = rows
                .iter()
                .find(|row| row.get::<String, _>("id") == task.id)
                .unwrap();
            assert_eq!(row.get::<String, _>("tenant_id"), TENANT_A);
            let payload: serde_json::Value =
                serde_json::from_slice(&row.get::<Vec<u8>, _>("payload")).unwrap();
            assert_eq!(payload["workflow_id"], plan.id);
            assert_eq!(payload["task_id"], task.id);
        }
        assert_eq!(body["enqueued_jobs"], plan.tasks.len());
    }
}

#[tokio::test]
async fn sqlite_draft_and_confirmation_keep_owner_tenant_after_restart_and_member_can_read() {
    let mut f = Fixture::new().await;
    let (status, body) = f.call("POST", "/", 0, start_request(TENANT_B, false)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["plan"]["tenant_id"], TENANT_A);
    assert_eq!(body["plan"]["status"], "awaiting_confirmation");
    assert_eq!(body["enqueued_jobs"], 0);
    assert_eq!(f.count().await, 0);
    let id = body["plan"]["id"].as_str().unwrap().to_string();
    f.restart();
    let (status, read) = f
        .call("GET", &format!("/{id}"), 2, serde_json::Value::Null)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(read, body["plan"]);
    let (status, confirmed) = f
        .call(
            "POST",
            &format!("/{id}/confirm"),
            0,
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(confirmed["plan"]["tenant_id"], TENANT_A);
    assert_eq!(confirmed["plan"]["status"], "queued");
    assert_eq!(
        confirmed["enqueued_jobs"].as_i64().unwrap(),
        f.count().await
    );
    let tenants: Vec<String> =
        sqlx::query_scalar("SELECT DISTINCT tenant_id FROM local_queue_jobs")
            .fetch_all(&f.pool)
            .await
            .unwrap();
    assert_eq!(tenants, [TENANT_A]);
    let count = f.count().await;
    let (status, repeated) = f
        .call(
            "POST",
            &format!("/{id}/confirm"),
            0,
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(repeated["enqueued_jobs"], 0);
    assert_eq!(f.count().await, count);
}

#[tokio::test]
async fn foreign_tenant_cannot_read_or_confirm_any_status_from_disk_or_cache() {
    let mut f = Fixture::new().await;
    for state in [
        WorkflowStatus::AwaitingConfirmation,
        WorkflowStatus::Queued,
        WorkflowStatus::Running,
        WorkflowStatus::Completed,
        WorkflowStatus::Failed,
    ] {
        let plan = f.seed(TENANT_A, state);
        for cache in [false, true] {
            f.restart();
            if cache {
                assert_eq!(
                    f.call("GET", &format!("/{}", plan.id), 0, serde_json::Value::Null)
                        .await
                        .0,
                    StatusCode::OK
                );
            }
            f.reject_without_effects(
                "GET",
                &format!("/{}", plan.id),
                &f.identities[1].1,
                StatusCode::NOT_FOUND,
            )
            .await;
            f.reject_without_effects(
                "POST",
                &format!("/{}/confirm", plan.id),
                &f.identities[1].1,
                StatusCode::NOT_FOUND,
            )
            .await;
            let (status, own) = f
                .call("GET", &format!("/{}", plan.id), 0, serde_json::Value::Null)
                .await;
            assert_eq!(
                status,
                StatusCode::OK,
                "foreign lookup must not poison or evict owner's plan"
            );
            assert_eq!(own["status"], serde_json::to_value(state).unwrap());
        }
    }
}

#[tokio::test]
async fn foreign_owner_confirmation_cannot_admit_an_existing_draft() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    f.reject_without_effects(
        "POST",
        &format!("/{}/confirm", plan.id),
        &f.identities[1].1,
        StatusCode::NOT_FOUND,
    )
    .await;
}

#[tokio::test]
async fn foreign_disk_read_does_not_insert_an_unauthorized_cache_entry() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    f.reject_without_effects(
        "GET",
        &format!("/{}", plan.id),
        &f.identities[1].1,
        StatusCode::NOT_FOUND,
    )
    .await;
    std::fs::remove_file(f.plan_path(&plan.id)).unwrap();
    f.reject_without_effects(
        "GET",
        &format!("/{}", plan.id),
        &f.identities[0].1,
        StatusCode::NOT_FOUND,
    )
    .await;
}

#[tokio::test]
async fn member_and_demoted_owner_cannot_create_even_a_draft_or_confirm() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    f.store
        .update_user(
            &f.identities[0].0.sub,
            None,
            Some(vec!["MEMBER".into()]),
            None,
            TENANT_A,
        )
        .await
        .unwrap();
    for identity in [0, 2] {
        for confirm in [false, true] {
            let files = f.files();
            let (status, _) = f
                .call("POST", "/", identity, start_request(TENANT_B, confirm))
                .await;
            assert_eq!(status, StatusCode::FORBIDDEN);
            assert_eq!(f.files(), files);
            assert_eq!(f.count().await, 0);
        }
        f.reject_without_effects(
            "POST",
            &format!("/{}/confirm", plan.id),
            &f.identities[identity].1,
            StatusCode::FORBIDDEN,
        )
        .await;
    }
}

#[tokio::test]
async fn invalid_duplicate_missing_expired_and_revoked_bearers_have_zero_side_effects() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    let mut duplicate = f.identities[1].1.clone();
    duplicate.append(AUTHORIZATION, f.identities[2].1[AUTHORIZATION].clone());
    let mut malformed = HeaderMap::new();
    malformed.insert(AUTHORIZATION, "Bearer invalid.token.value".parse().unwrap());
    let mut expired = f.identities[0].0.clone();
    expired.exp = chrono::Utc::now().timestamp() - 3600;
    expired.jti = uuid::Uuid::new_v4().to_string();
    assert!(!f.store.is_revoked(&expired.jti, TENANT_A).await.unwrap());
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &expired,
        &jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap();
    let mut expired_headers = HeaderMap::new();
    expired_headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    f.store
        .revoke_token(
            f.identities[0].0.jti.clone(),
            chrono::DateTime::from_timestamp(f.identities[0].0.exp, 0).unwrap(),
            TENANT_A,
        )
        .await
        .unwrap();
    // Both duplicate values must still authenticate individually: rejection
    // must be caused by duplicate headers, not the separately revoked owner.
    for value in duplicate.get_all(AUTHORIZATION) {
        let token = value.to_str().unwrap().strip_prefix("Bearer ").unwrap();
        assert!(f.store.validate_token(token).await.is_ok());
    }
    for headers in [
        HeaderMap::new(),
        duplicate,
        malformed,
        expired_headers,
        f.identities[0].1.clone(),
    ] {
        for (method, path) in [
            ("POST", "/".to_string()),
            ("POST", format!("/{}/confirm", plan.id)),
            ("GET", format!("/{}", plan.id)),
        ] {
            f.reject_without_effects(method, &path, &headers, StatusCode::UNAUTHORIZED)
                .await;
        }
    }
}

#[tokio::test]
async fn inactive_canonical_owner_is_rejected_before_both_mutations() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    f.store
        .update_user(&f.identities[0].0.sub, None, None, Some(false), TENANT_A)
        .await
        .unwrap();
    for path in ["/".to_string(), format!("/{}/confirm", plan.id)] {
        f.reject_without_effects("POST", &path, &f.identities[0].1, StatusCode::UNAUTHORIZED)
            .await;
    }
}

#[tokio::test]
async fn missing_or_system_signed_tenant_cannot_read_or_mutate_plans() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    for tenant in [None, Some(String::new()), Some("system".into())] {
        let mut claims = f.identities[0].0.clone();
        claims.organization_id = tenant;
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(
                std::env::var("JWT_SECRET").unwrap().as_bytes(),
            ),
        )
        .unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        for (method, path) in [
            ("POST", "/".to_string()),
            ("POST", format!("/{}/confirm", plan.id)),
            ("GET", format!("/{}", plan.id)),
        ] {
            f.reject_without_effects(method, &path, &headers, StatusCode::UNAUTHORIZED)
                .await;
        }
    }
}

#[tokio::test]
async fn canonical_auth_store_outage_is_fail_closed_before_queue_or_file_use() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    let before = f.files();
    f.pool.close().await;
    for path in ["/".to_string(), format!("/{}/confirm", plan.id)] {
        let (status, _) = f
            .call("POST", &path, 0, start_request(TENANT_A, true))
            .await;
        // Existing strict middleware intentionally reports failed auth lookup as 401.
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(f.files(), before);
        assert_eq!(f.queue.admissions.load(Ordering::SeqCst), 0);
    }
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!(
            "sqlite://{}?mode=ro",
            f.directory.0.join("canonical.sqlite").display()
        ))
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM local_queue_jobs")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    pool.close().await;
}

#[tokio::test]
async fn rejected_trigger_and_empty_prompt_preserve_existing_request_errors_without_effects() {
    let f = Fixture::new().await;
    for (prompt, message) in [
        (" ", "prompt is required"),
        ("Fix typo", "task does not require a dynamic workflow"),
    ] {
        let mut body = start_request(TENANT_A, false);
        body["prompt"] = prompt.into();
        let (status, body) = f.call("POST", "/", 0, body).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body, serde_json::json!({"error":message}));
        assert!(f.files().is_empty());
        assert_eq!(f.count().await, 0);
        assert_eq!(f.queue.admissions.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn storage_path_failure_has_a_generic_http_error_without_exposing_the_path() {
    let f = Fixture::new().await;
    let path = f.directory.0.join("private-state-directory-marker");
    std::fs::write(&path, b"owned sentinel").unwrap();
    let manager = Arc::new(DynamicWorkflowManager::with_state_dir(
        f.queue.clone(),
        path.clone(),
    ));
    let app = crate::mounted_dynamic_workflow_boundary(manager, f.store.clone());
    let (status, body) = send(
        app,
        "POST",
        "/",
        &f.identities[0].1,
        start_request(TENANT_A, false),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        body,
        serde_json::json!({"error":"workflow operation failed"})
    );
    assert_eq!(std::fs::read(path).unwrap(), b"owned sentinel");
    assert_eq!(f.queue.admissions.load(Ordering::SeqCst), 0);
    assert_eq!(f.count().await, 0);
}

#[tokio::test]
async fn real_owner_verification_rejects_claims_that_disagree_with_genuine_bearer() {
    let f = Fixture::new().await;
    let before = f.files();
    let mut claims = f.identities[0].0.clone();
    claims.organization_id = Some(TENANT_B.into());
    assert!(matches!(
        server_auth::commit_authority::verify_owner(&f.store, &claims, &f.identities[0].1).await,
        Err(server_auth::commit_authority::AuthorityError::Forbidden)
    ));
    assert_eq!(f.files(), before);
    assert_eq!(f.count().await, 0);
}

#[tokio::test]
async fn legacy_and_embedded_id_mismatches_are_never_adopted_or_cached() {
    let f = Fixture::new().await;
    for tenant in [LEGACY_TENANT, "", "default", TENANT_B] {
        let plan = f.seed(tenant, WorkflowStatus::AwaitingConfirmation);
        for path in [format!("/{}", plan.id), format!("/{}/confirm", plan.id)] {
            let method = if path.ends_with("/confirm") {
                "POST"
            } else {
                "GET"
            };
            f.reject_without_effects(method, &path, &f.identities[0].1, StatusCode::NOT_FOUND)
                .await;
        }
    }
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    let alias = format!("dwf-{}", uuid::Uuid::new_v4());
    f.write_plan(&alias, &plan);
    f.reject_without_effects(
        "GET",
        &format!("/{alias}"),
        &f.identities[0].1,
        StatusCode::NOT_FOUND,
    )
    .await;
    f.reject_without_effects(
        "POST",
        &format!("/{alias}/confirm"),
        &f.identities[0].1,
        StatusCode::NOT_FOUND,
    )
    .await;
    std::fs::remove_file(f.plan_path(&alias)).unwrap();
    f.reject_without_effects(
        "GET",
        &format!("/{alias}"),
        &f.identities[0].1,
        StatusCode::NOT_FOUND,
    )
    .await;
}

#[tokio::test]
async fn invalid_filename_spellings_are_rejected_before_reading_invalid_json() {
    let f = Fixture::new().await;
    let id = uuid::Uuid::new_v4();
    std::fs::create_dir_all(f.directory.0.join("plans")).unwrap();
    for invalid in [
        format!("dwf-{}", id.simple()),
        format!("dwf-{}", id.to_string().to_uppercase()),
        "dwf-invalid".into(),
        "unprefixed".into(),
    ] {
        std::fs::write(
            f.plan_path(&invalid),
            b"private invalid JSON must not be read",
        )
        .unwrap();
        f.reject_without_effects(
            "GET",
            &format!("/{invalid}"),
            &f.identities[0].1,
            StatusCode::NOT_FOUND,
        )
        .await;
        f.reject_without_effects(
            "POST",
            &format!("/{invalid}/confirm"),
            &f.identities[0].1,
            StatusCode::NOT_FOUND,
        )
        .await;
    }
    let sentinel = f.directory.0.join("outside.json");
    std::fs::write(&sentinel, b"outside sentinel").unwrap();
    f.reject_without_effects(
        "GET",
        "/..%2Foutside",
        &f.identities[0].1,
        StatusCode::NOT_FOUND,
    )
    .await;
    assert_eq!(std::fs::read(sentinel).unwrap(), b"outside sentinel");
}

#[tokio::test]
async fn corrupt_persisted_plan_errors_are_generic_on_read_and_confirmation() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    std::fs::write(
        f.plan_path(&plan.id),
        br#"{"status":"private-corruption-marker"}"#,
    )
    .unwrap();
    for (method, path) in [
        ("GET", format!("/{}", plan.id)),
        ("POST", format!("/{}/confirm", plan.id)),
    ] {
        let before = f.files();
        let (status, body) = f.call(method, &path, 0, serde_json::Value::Null).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body,
            serde_json::json!({"error":"workflow operation failed"})
        );
        assert_eq!(f.files(), before);
        assert_eq!(f.count().await, 0);
    }
}

#[tokio::test]
async fn actual_queue_failures_are_generic_for_start_and_confirmation() {
    let f = Fixture::new().await;
    let plan = f.seed(TENANT_A, WorkflowStatus::AwaitingConfirmation);
    sqlx::query("DROP TABLE local_queue_jobs")
        .execute(&f.pool)
        .await
        .unwrap();
    for path in ["/".to_string(), format!("/{}/confirm", plan.id)] {
        let before = f.files();
        let (status, body) = f
            .call("POST", &path, 0, start_request(TENANT_B, true))
            .await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body,
            serde_json::json!({"error":"workflow operation failed"})
        );
        assert_eq!(f.files(), before);
    }
}

#[tokio::test]
async fn postgres_real_queue_rows_use_the_canonical_owner_and_foreign_confirmation_has_no_effect() {
    let f = super::Fixture::open().await;
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/104_sub_agent_queue.sql"
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    // The actual legacy queue uses a separate pool from canonical auth. This
    // table-owner fixture proves stored row identity, not forced queue RLS.
    let queue = Arc::new(crate::queue::PostgresTaskQueue::new(f.admin.clone()));
    let directory = OwnedDirectory::new();
    let manager = Arc::new(DynamicWorkflowManager::with_state_dir(
        queue,
        directory.0.join("plans"),
    ));
    let app = crate::mounted_dynamic_workflow_boundary(manager, f.auth.clone());
    let (status, body) = send(
        app.clone(),
        "POST",
        "/",
        &f.identities[0].1,
        start_request("receipt-pg-b", false),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["plan"]["tenant_id"], "receipt-pg-a");
    let id = body["plan"]["id"].as_str().unwrap();
    let before = std::fs::read(directory.0.join("plans").join(format!("{id}.json"))).unwrap();
    let (status, denied) = send(
        app.clone(),
        "POST",
        &format!("/{id}/confirm"),
        &f.identities[1].1,
        serde_json::Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(denied, serde_json::json!({"error":"workflow not found"}));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sub_agent_queue")
            .fetch_one(&f.admin)
            .await
            .unwrap(),
        0
    );
    assert_eq!(
        std::fs::read(directory.0.join("plans").join(format!("{id}.json"))).unwrap(),
        before
    );
    let (status, accepted) = send(
        app.clone(),
        "POST",
        &format!("/{id}/confirm"),
        &f.identities[0].1,
        serde_json::Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{accepted}");
    let plan: DynamicWorkflowPlan = serde_json::from_value(accepted["plan"].clone()).unwrap();
    let rows = sqlx::query("SELECT id,tenant_id,payload FROM sub_agent_queue")
        .fetch_all(&f.admin)
        .await
        .unwrap();
    assert_eq!(rows.len(), plan.tasks.len());
    assert_eq!(accepted["enqueued_jobs"], rows.len());
    for task in &plan.tasks {
        let row = rows
            .iter()
            .find(|row| row.get::<String, _>("id") == task.id)
            .unwrap();
        assert_eq!(row.get::<String, _>("tenant_id"), "receipt-pg-a");
        let payload: serde_json::Value = row.get("payload");
        assert_eq!(payload["workflow_id"], id);
        assert_eq!(payload["task_id"], task.id);
    }
    let (status, _) = send(
        app.clone(),
        "POST",
        &format!("/{id}/confirm"),
        &f.identities[1].1,
        serde_json::Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sub_agent_queue")
            .fetch_one(&f.admin)
            .await
            .unwrap() as usize,
        rows.len()
    );
    drop(app);
    f.close().await;
}
