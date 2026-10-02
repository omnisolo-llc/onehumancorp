use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    routing::{get, post},
};
use tower::ServiceExt;

async fn raw_rpc_response(
    fixture: &Fixture,
    route: &str,
    token: Option<&str>,
) -> (StatusCode, serde_json::Value) {
    let mut request = Request::builder()
        .method("POST")
        .uri(route)
        .header("content-type", "application/json")
        .header("x-tenant-id", "workflow-tenant-a")
        .header("x-user-id", "forged-owner");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = mounted_rpc_boundary(fixture.store.clone()).oneshot(request.body(Body::from(
        serde_json::json!({"jsonrpc":"2.0","id":"owned-local-probe","method":"ap_list_tasks","params":{"tenant_id":"workflow-tenant-a"}}).to_string()
    )).unwrap()).await.unwrap();
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
async fn actual_rpc_mounts_reject_anonymous_dispatch() {
    let fixture = Fixture::new().await;
    RPC_DISPATCHES.store(0, std::sync::atomic::Ordering::SeqCst);
    for route in ["/api/v1/rpc", "/rpc"] {
        let (status, _) = raw_rpc_response(&fixture, route, None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "actual mount {route}");
        assert_eq!(RPC_DISPATCHES.load(std::sync::atomic::Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn actual_raw_rpc_mount_rejects_staff_before_dispatch() {
    let fixture = Fixture::new().await;
    RPC_DISPATCHES.store(0, std::sync::atomic::Ordering::SeqCst);
    let (status, _) = raw_rpc_response(&fixture, "/rpc", Some(&fixture.staff)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(RPC_DISPATCHES.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[tokio::test]
async fn actual_raw_rpc_mount_never_shares_a_global_dispatcher_between_signed_tenants() {
    let fixture = Fixture::new().await;
    RPC_DISPATCHES.store(0, std::sync::atomic::Ordering::SeqCst);
    let mut responses = Vec::new();
    for token in [&fixture.a, &fixture.b] {
        responses.push(raw_rpc_response(&fixture, "/rpc", Some(token)).await);
    }
    assert_eq!(responses.len(), 2);
    for (status, body) in responses {
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert!(body.get("result").is_none());
        assert_eq!(RPC_DISPATCHES.load(std::sync::atomic::Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn actual_authenticated_rpc_does_not_switch_to_a_global_fallback_after_transport_loss() {
    use tokio::io::AsyncReadExt;
    let fixture = Fixture::new().await;
    assert!(
        agent_rpc_available(server_config::get().multitenant),
        "This fixture must exercise the actual allowed standalone proxy branch"
    );
    RPC_DISPATCHES.store(0, std::sync::atomic::Ordering::SeqCst);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let accepted = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = accepted.clone();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = [0_u8; 4096];
        let count = stream.read(&mut buffer).await.unwrap();
        assert!(count > 0);
        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // Actual local transport failure: no fabricated application response.
        drop(stream);
        drop(listener);
    });
    struct RestoreFixtureUrl;
    impl Drop for RestoreFixtureUrl {
        fn drop(&mut self) {
            // run.sh fixes this public local fixture value before the serial run.
            unsafe { std::env::set_var("OMNISOLO_AGENT_URL", "http://127.0.0.1:1") };
        }
    }
    let _restore = RestoreFixtureUrl;
    // The harness always runs with --test-threads=1 and an empty agent token.
    unsafe { std::env::set_var("OMNISOLO_AGENT_URL", format!("http://{address}")) };
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(8),
        raw_rpc_response(&fixture, "/api/v1/rpc", Some(&fixture.a)),
    )
    .await;
    server.abort();
    let _ = server.await;
    let (status, body) = response.unwrap();
    assert_eq!(accepted.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(body.get("result").is_none());
    assert_eq!(RPC_DISPATCHES.load(std::sync::atomic::Ordering::SeqCst), 0);
}

struct RestoreRpcEnvironment {
    url: Option<String>,
    port: Option<String>,
}
impl RestoreRpcEnvironment {
    fn capture() -> Self {
        Self {
            url: std::env::var("OMNISOLO_AGENT_URL").ok(),
            port: std::env::var("OMNISOLO_PORT").ok(),
        }
    }
}
impl Drop for RestoreRpcEnvironment {
    fn drop(&mut self) {
        // This harness is strictly serial and changes only its own process env.
        for (key, value) in [
            ("OMNISOLO_AGENT_URL", &self.url),
            ("OMNISOLO_PORT", &self.port),
        ] {
            unsafe {
                if let Some(value) = value {
                    std::env::set_var(key, value);
                } else {
                    std::env::remove_var(key);
                }
            }
        }
    }
}

async fn assert_internal_rpc_auth_failure_is_not_a_user_session_failure(status: StatusCode) {
    let fixture = Fixture::new().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let requests = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = requests.clone();
    let app = Router::new().route(
        "/rpc",
        post(move |headers: axum::http::HeaderMap| {
            let observed = observed.clone();
            async move {
                assert_eq!(headers["x-tenant-id"], "workflow-tenant-a");
                assert_ne!(headers["x-user-id"], "forged-owner");
                observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                status
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let _restore = RestoreRpcEnvironment::capture();
    unsafe {
        std::env::set_var("OMNISOLO_AGENT_URL", format!("http://{address}"));
    }
    let (received, body) = raw_rpc_response(&fixture, "/api/v1/rpc", Some(&fixture.a)).await;
    server.abort();
    let _ = server.await;
    assert_eq!(
        received,
        StatusCode::BAD_GATEWAY,
        "internal status {status}"
    );
    assert_eq!(
        body["error"],
        "Agent runtime authentication failed; check the server-side runtime configuration"
    );
    assert!(body.get("result").is_none());
    assert_eq!(requests.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(fixture.store.validate_token(&fixture.a).await.is_ok());
}

#[tokio::test]
async fn upstream_rpc_unauthorized_does_not_expire_an_authenticated_browser_session() {
    assert_internal_rpc_auth_failure_is_not_a_user_session_failure(StatusCode::UNAUTHORIZED).await;
}
#[tokio::test]
async fn upstream_rpc_forbidden_does_not_change_the_authenticated_callers_permissions() {
    assert_internal_rpc_auth_failure_is_not_a_user_session_failure(StatusCode::FORBIDDEN).await;
}
#[tokio::test]
async fn absent_explicit_agent_runtime_never_calls_the_servers_retired_raw_endpoint() {
    let fixture = Fixture::new().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let requests = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = requests.clone();
    let app = Router::new().route(
        "/rpc",
        post(move || {
            let observed = observed.clone();
            async move {
                observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                StatusCode::UNAUTHORIZED
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let _restore = RestoreRpcEnvironment::capture();
    unsafe {
        std::env::remove_var("OMNISOLO_AGENT_URL");
        std::env::set_var("OMNISOLO_PORT", port.to_string());
    }
    let (status, body) = raw_rpc_response(&fixture, "/api/v1/rpc", Some(&fixture.a)).await;
    server.abort();
    let _ = server.await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        body["error"],
        "Agent runtime is not configured; no work was dispatched"
    );
    assert_eq!(requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    assert!(fixture.store.validate_token(&fixture.a).await.is_ok());
}

static TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

struct RecordingInference;
impl workflow_execution::TextInference for RecordingInference {
    fn infer<'a>(
        &'a self,
        input: &'a workflow_execution::AdmittedAnalysis,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<workflow_execution::InferenceResult, ()>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            DISPATCHES.write().unwrap().push(ObservedDispatch {
                task: input.task().to_owned(),
            });
            assert_ne!(input.tenant_id(), "system");
            assert!(!input.actor_id().is_empty());
            assert_eq!(input.policy().model, "configured-model");
            Ok(workflow_execution::InferenceResult {
                output: Some(format!(
                    "Recorded text-only result for {}: {}",
                    input.tenant_id(),
                    input.task()
                )),
                provider_request_id: None,
                counts: None,
            })
        })
    }
}
struct Fixture {
    _guard: tokio::sync::MutexGuard<'static, ()>,
    app: Router,
    store: Arc<server_auth::Store>,
    database: persistence::AppDatabase,
    execution: Arc<workflow_execution::WorkflowExecution>,
    a: String,
    b: String,
    staff: String,
    admin: String,
}
impl Fixture {
    async fn new() -> Self {
        Self::new_at(&format!(
            "sqlite:file:workflow_{}?mode=memory&cache=shared",
            uuid::Uuid::new_v4()
        ))
        .await
    }
    async fn new_at(url: &str) -> Self {
        let guard = TEST_LOCK.lock().await;
        DISPATCHES.write().unwrap().clear();
        let database = persistence::AppDatabase::connect(url).await.unwrap();
        persistence::migration::migrate(&database).await.unwrap();
        let store = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone()),
        )));
        let mut tokens = Vec::new();
        for (tenant, role) in [
            ("workflow-tenant-a", "owner"),
            ("workflow-tenant-b", "owner"),
            ("workflow-tenant-a", "staff"),
            ("workflow-tenant-a", server_auth::ROLE_ADMIN),
        ] {
            let owner = store
                .create_user(
                    format!("{role}-{tenant}"),
                    format!("{role}-{tenant}@example.test"),
                    "public-local-fixture-password".into(),
                    vec![role.into()],
                    tenant.into(),
                )
                .await
                .unwrap();
            tokens.push(store.issue_token(&owner).unwrap());
        }
        let hub = Arc::new(Hub::default());
        let execution = Arc::new(
            workflow_execution::WorkflowExecution::configured(
                store.clone(),
                workflow_execution::AnalysisPolicy::new(
                    "ollama".into(),
                    "configured-model".into(),
                    512,
                )
                .unwrap(),
                Arc::new(RecordingInference),
            )
            .with_receipts(database.clone()),
        );
        let app = Router::new()
            .route(
                "/api/v1/agents/workflows",
                get(list_workflows_handler).post(create_workflow_handler),
            )
            .route("/api/v1/agents/hire", post(hire_handler))
            .route(
                "/api/v1/agents/workflows/by-request/{id}",
                get(workflow_request_receipt_handler),
            )
            .route(
                "/api/v1/agents/workflows/{id}",
                get(workflow_receipt_handler),
            )
            .route(
                "/api/v1/agents/workflows/{id}/cancel",
                post(cancel_workflow_handler),
            )
            .route(
                "/api/v1/agents/execution-policy",
                get(execution_policy_handler),
            )
            .route("/api/v1/agents/", get(list_agents_handler))
            .with_state(hub)
            .layer(axum::Extension(execution.clone()))
            .route_layer(axum::middleware::from_fn_with_state(
                store.clone(),
                server_auth::strict_bearer_auth_middleware,
            ));
        Self {
            _guard: guard,
            app,
            store,
            database,
            execution,
            a: tokens.remove(0),
            b: tokens.remove(0),
            staff: tokens.remove(0),
            admin: tokens.remove(0),
        }
    }
    async fn records(&self) -> Vec<WorkflowRecord> {
        let mut result = Vec::new();
        for token in [&self.admin, &self.b] {
            let claims = self.store.validate_token(token).await.unwrap();
            let mut headers = axum::http::HeaderMap::new();
            headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
            result.extend(
                self.execution
                    .list_receipts(&claims, &headers, &Default::default())
                    .await
                    .unwrap()
                    .into_iter()
                    .map(WorkflowRecord::from),
            );
        }
        result
    }
    async fn request(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: serde_json::Value,
    ) -> (StatusCode, serde_json::Value) {
        let mut request = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .header("x-tenant-id", "workflow-tenant-b")
            .header(
                "x-spiffe-id",
                "spiffe://ohc/org/workflow-tenant-b/agent/forged",
            );
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        if method == "POST"
            && (path == "/api/v1/agents/workflows"
                || (path == "/api/v1/agents/hire" && body.get("task").is_some()))
        {
            request = request.header("idempotency-key", uuid::Uuid::new_v4().to_string());
        }
        let response = self
            .app
            .clone()
            .oneshot(
                request
                    .body(if method == "GET" {
                        Body::empty()
                    } else {
                        Body::from(body.to_string())
                    })
                    .unwrap(),
            )
            .await
            .unwrap();
        tokio::time::timeout(
            std::time::Duration::from_secs(3),
            self.execution.wait_for_workers(),
        )
        .await
        .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
        )
    }
}

#[tokio::test]
async fn the_production_admin_role_can_start_work_for_its_signed_tenant() {
    let f = Fixture::new().await;
    let mut statuses = Vec::new();
    for path in ["/api/v1/agents/workflows", "/api/v1/agents/hire"] {
        let (status, _) = f.request(
            "POST",
            path,
            Some(&f.admin),
            if path.ends_with("/hire") { serde_json::json!({"name":"Administrator work","role":"Operations","task":"Prepare an owner report"}) } else { serde_json::json!({"name":"Administrator work","task":"Prepare an owner report"}) },
        ).await;
        statuses.push(status);
    }
    assert_eq!(
        statuses,
        [StatusCode::ACCEPTED, StatusCode::CREATED],
        "the actual signed ADMIN role must remain authorized on both routes"
    );
    let records = f.records().await;
    assert_eq!(records.len(), 2);
    assert!(
        records
            .iter()
            .all(|record| record.tenant_id == "workflow-tenant-a" && !record.actor_id.is_empty())
    );
}

async fn assert_registry_failure_creates_no_work(path: &str) {
    let f = Fixture::new().await;
    use sea_orm::ConnectionTrait;
    f.database
        .connection()
        .execute_unprepared("DROP TABLE tenant_workflow_receipts")
        .await
        .unwrap();
    let (status, _) = f.request(
        "POST",
        path,
        Some(&f.a),
        if path.ends_with("/hire") { serde_json::json!({"name":"Unavailable registry","role":"Operations","task":"Must not become untracked work"}) } else { serde_json::json!({"name":"Unavailable registry","task":"Must not become untracked work"}) },
    ).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(DISPATCHES.read().unwrap().is_empty());
    use sea_orm::Statement;
    assert!(
        f.database
            .connection()
            .query_one(Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                "SELECT name FROM sqlite_master WHERE name='tenant_workflow_receipts'".to_string()
            ))
            .await
            .unwrap()
            .is_none()
    );
    let (status, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        agents,
        serde_json::json!([]),
        "failed workflow admission must not leave an agent registration"
    );
}

#[tokio::test]
async fn creating_a_workflow_fails_closed_when_durable_storage_is_unavailable() {
    assert_registry_failure_creates_no_work("/api/v1/agents/workflows").await;
}

#[tokio::test]
async fn hiring_an_agent_fails_closed_when_durable_storage_is_unavailable() {
    assert_registry_failure_creates_no_work("/api/v1/agents/hire").await;
}
#[tokio::test]
async fn signed_workflow_reads_stay_with_the_creating_tenant() {
    let f = Fixture::new().await;
    for (token, name) in [
        (&f.a, "owner-a-private-work"),
        (&f.b, "owner-b-private-work"),
    ] {
        let (status,_)=f.request("POST","/api/v1/agents/workflows",Some(token),serde_json::json!({"name":name,"task":format!("Prepare {name}"),"workflow":"expert_task"})).await;
        assert_eq!(status, StatusCode::ACCEPTED);
    }
    let (_, a) = f
        .request(
            "GET",
            "/api/v1/agents/workflows",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    let (_, b) = f
        .request(
            "GET",
            "/api/v1/agents/workflows",
            Some(&f.b),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(
        a["workflows"].as_array().unwrap().len(),
        1,
        "tenant A must receive only its own actual created record"
    );
    assert_eq!(a["workflows"][0]["name"], "owner-a-private-work");
    assert_eq!(b["workflows"].as_array().unwrap().len(), 1);
    assert_eq!(b["workflows"][0]["name"], "owner-b-private-work");
}
#[tokio::test]
async fn hire_preserves_the_actual_requested_task_at_the_dispatch_boundary() {
    let f = Fixture::new().await;
    let task = "Prepare a bounded inventory reconciliation report for the owner.";
    let (status,_)=f.request("POST","/api/v1/agents/hire",Some(&f.a),serde_json::json!({"name":"Inventory Specialist","role":"Operations","providerType":"builtin","model":"Auto","task":task})).await;
    assert_eq!(status, StatusCode::CREATED);
    let effects = DISPATCHES.read().unwrap();
    assert_eq!(effects.len(), 1);
    assert!(
        effects[0].task.contains(task),
        "the queued work must retain the caller's actual task instead of a generic business swarm"
    );
}
#[tokio::test]
async fn signed_agent_lists_do_not_return_other_tenants_registrations() {
    let f = Fixture::new().await;
    for (token, name) in [(&f.a, "Owner A agent"), (&f.b, "Owner B agent")] {
        let (status,_)=f.request("POST","/api/v1/agents/hire",Some(token),serde_json::json!({"name":name,"role":"Operations","providerType":"builtin","model":"Auto"})).await;
        assert_eq!(status, StatusCode::CREATED);
    }
    let (_, a) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(
        a.as_array().unwrap().len(),
        1,
        "tenant A must not read tenant B's actual registration"
    );
    assert_eq!(a[0]["name"], "Owner A agent");
}
#[tokio::test]
async fn forged_headers_without_a_valid_bearer_create_no_work_or_agents() {
    let f = Fixture::new().await;
    for (method, path) in [
        ("GET", "/api/v1/agents/workflows"),
        ("POST", "/api/v1/agents/workflows"),
        ("POST", "/api/v1/agents/hire"),
        ("GET", "/api/v1/agents/"),
    ] {
        let (status,_)=f.request(method,path,None,serde_json::json!({"name":"forged","task":"must never dispatch","role":"Operations"})).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn ordinary_tenant_members_cannot_start_owner_work() {
    let f = Fixture::new().await;
    for path in ["/api/v1/agents/workflows", "/api/v1/agents/hire"] {
        let (status,_)=f.request("POST",path,Some(&f.staff),if path.ends_with("/hire") { serde_json::json!({"name":"unapproved","role":"Operations","task":"must not dispatch"}) } else { serde_json::json!({"name":"unapproved","task":"must not dispatch"}) }).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}
#[tokio::test]
async fn an_explicit_empty_task_does_not_fall_back_to_paid_generic_work() {
    let f = Fixture::new().await;
    let (status, _) = f
        .request(
            "POST",
            "/api/v1/agents/hire",
            Some(&f.a),
            serde_json::json!({"name":"Empty","role":"Operations","task":"   "}),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
    let (_, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(agents, serde_json::json!([]));
}

#[tokio::test]
async fn registration_without_a_task_stays_idle_and_creates_no_workflow() {
    let f = Fixture::new().await;
    let (status, receipt) = f
        .request(
            "POST",
            "/api/v1/agents/hire",
            Some(&f.a),
            serde_json::json!({"name":"Idle specialist","role":"Operations"}),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(receipt["status"], "idle");
    assert_eq!(receipt["workflow_id"], "");
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
    let (_, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(agents.as_array().unwrap().len(), 1);
    assert_eq!(agents[0]["id"], receipt["id"]);
    assert_eq!(agents[0]["status"], "IDLE");
    let (_, other) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.b),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(other, serde_json::json!([]));
}

#[tokio::test]
async fn unsupported_task_capabilities_are_rejected_before_any_registration() {
    let f = Fixture::new().await;
    let (status, _) = f.request("POST", "/api/v1/agents/hire", Some(&f.a), serde_json::json!({"name":"Unsupported specialist","role":"Operations","task":"Inspect private files","workspace":"other-business","workDirectory":"/private","connectors":["Stripe"],"skills":["Shell"],"customProvider":"https://unapproved.invalid/v1"})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
    let (_, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(agents, serde_json::json!([]));
}

#[tokio::test]
async fn completed_analysis_updates_only_its_existing_owned_agent() {
    let f = Fixture::new().await;
    let (status, receipt) = f.request("POST", "/api/v1/agents/hire", Some(&f.a), serde_json::json!({"name":"Owned analysis","role":"Operations","task":"Analyze this supplied text","model":"Auto"})).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(receipt["status"], "queued");
    let (_, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(agents[0]["id"], receipt["agent_id"]);
    assert_eq!(agents[0]["status"], "COMPLETED");
    let (_, workflows) = f
        .request(
            "GET",
            "/api/v1/agents/workflows",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(workflows["workflows"][0]["status"], "completed");
    assert_eq!(workflows["workflows"][0]["model"], "configured-model");
    assert_eq!(workflows["workflows"][0]["provider"], "ollama");
}

#[tokio::test]
async fn status_updates_do_not_cross_tenants_or_resurrect_removed_agents() {
    let hub = Hub::default();
    hub.register_agent(server_omnisolo::orchestration::Agent {
        id: "owned".into(),
        name: "Owned".into(),
        role: "Operations".into(),
        organization_id: "tenant-a".into(),
        status: "QUEUED".into(),
        provider_type: "builtin".into(),
    })
    .await;
    assert!(
        !hub.update_agent_status("owned", "tenant-b", "COMPLETED")
            .await
    );
    assert_eq!(hub.get_agent("owned").await.unwrap().status, "QUEUED");
    assert!(
        hub.update_agent_status("owned", "tenant-a", "COMPLETED")
            .await
    );
    hub.fire_agent("owned").await;
    assert!(
        !hub.update_agent_status("owned", "tenant-a", "COMPLETED")
            .await
    );
    assert!(hub.get_agent("owned").await.is_none());
}

#[tokio::test]
async fn late_completion_cannot_overwrite_a_terminal_unknown_result() {
    let f = Fixture::new().await;
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    let reservation = f
        .execution
        .prepare(
            &claims,
            &headers,
            "Text only",
            workflow_execution::receipts::RequestMetadata {
                request_id: uuid::Uuid::new_v4(),
                name: "Analysis".into(),
                workflow: "analysis".into(),
                requested_model: "Auto".into(),
                agent_role: None,
            },
        )
        .await
        .unwrap();
    let id = reservation.receipt().id.clone();
    let store = f.execution.receipt_store().unwrap();
    let lease = store.claim(reservation).await.unwrap().unwrap();
    let proof = lease.completion_proof();
    assert_eq!(
        f.execution
            .cancel_receipt(&claims, &headers, &id)
            .await
            .unwrap()
            .phase,
        workflow_execution::receipts::StoredPhase::OutcomeUnknown
    );
    assert!(matches!(
        store
            .finish(
                &proof,
                &workflow_execution::AnalysisOutcome::Completed("late untrusted result".into())
            )
            .await,
        Err(workflow_execution::receipts::Error::Conflict)
    ));
    assert_eq!(
        f.execution
            .get_receipt(&claims, &headers, &id)
            .await
            .unwrap()
            .phase,
        workflow_execution::receipts::StoredPhase::OutcomeUnknown
    );
}

#[tokio::test]
async fn execution_policy_reports_only_configured_text_capabilities_to_an_owner() {
    let f = Fixture::new().await;
    let (status, policy) = f
        .request(
            "GET",
            "/api/v1/agents/execution-policy",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(policy["available"], true);
    assert_eq!(policy["policy"]["model"], "configured-model");
    assert_eq!(policy["policy"]["provider"], "ollama");
    assert_eq!(policy["workspace_access"], false);
    assert_eq!(policy["tools"], serde_json::json!([]));
    assert!(DISPATCHES.read().unwrap().is_empty());
    assert!(f.records().await.is_empty());
    for (token, expected) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(f.staff.as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            f.request(
                "GET",
                "/api/v1/agents/execution-policy",
                token,
                serde_json::Value::Null
            )
            .await
            .0,
            expected
        );
    }
}

#[tokio::test]
async fn an_owner_role_removed_after_token_issue_cannot_register_idle_metadata() {
    let f = Fixture::new().await;
    let signed = f.store.validate_token(&f.a).await.unwrap();
    assert!(signed.roles.iter().any(|role| role == "owner"));
    f.store
        .update_user(
            &signed.sub,
            None,
            Some(vec!["staff".into()]),
            None,
            "workflow-tenant-a",
        )
        .await
        .unwrap();
    let (status, _) = f
        .request(
            "POST",
            "/api/v1/agents/hire",
            Some(&f.a),
            serde_json::json!({"name":"Idle agent","role":"Operations"}),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.admin),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(agents, serde_json::json!([]));
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn a_disabled_user_with_an_unexpired_token_cannot_register_idle_metadata() {
    let f = Fixture::new().await;
    let signed = f.store.validate_token(&f.a).await.unwrap();
    assert!(signed.exp > chrono::Utc::now().timestamp());
    f.store
        .update_user(&signed.sub, None, None, Some(false), "workflow-tenant-a")
        .await
        .unwrap();
    let (status, _) = f
        .request(
            "POST",
            "/api/v1/agents/hire",
            Some(&f.a),
            serde_json::json!({"name":"Idle agent","role":"Operations"}),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.admin),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(agents, serde_json::json!([]));
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}

async fn keyed_work_request(
    f: &Fixture,
    path: &str,
    key: &str,
    body: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    let response = f
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("authorization", format!("Bearer {}", f.a))
                .header("content-type", "application/json")
                .header("idempotency-key", key)
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_048_576).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn authenticated_workflow_request_id_replay_returns_one_durable_effect() {
    let f = Fixture::new().await;
    let key = uuid::Uuid::new_v4().to_string();
    let payload = serde_json::json!({"name":"Replay-safe work","task":"Preserve this exact submitted text","model":"Auto"});
    let (first_status, first) =
        keyed_work_request(&f, "/api/v1/agents/workflows", &key, payload.clone()).await;
    let (second_status, second) =
        keyed_work_request(&f, "/api/v1/agents/workflows", &key, payload).await;
    assert_eq!(first_status, StatusCode::ACCEPTED);
    assert!(second_status.is_success());
    f.request(
        "GET",
        "/api/v1/agents/workflows",
        Some(&f.a),
        serde_json::Value::Null,
    )
    .await;
    assert_eq!(
        first["workflow"]["id"], second["workflow"]["id"],
        "the same authenticated request must not create another provider effect"
    );
    assert_eq!(DISPATCHES.read().unwrap().len(), 1);
}

#[tokio::test]
async fn authenticated_hire_request_id_replay_preserves_one_agent_and_workflow() {
    let f = Fixture::new().await;
    let key = uuid::Uuid::new_v4().to_string();
    let payload = serde_json::json!({"name":"Replay-safe specialist","role":"Operations","task":"Analyze the submitted text","model":"Auto"});
    let (first_status, first) =
        keyed_work_request(&f, "/api/v1/agents/hire", &key, payload.clone()).await;
    let (second_status, second) =
        keyed_work_request(&f, "/api/v1/agents/hire", &key, payload).await;
    assert_eq!(first_status, StatusCode::CREATED);
    assert!(second_status.is_success());
    let (_, agents) = f
        .request(
            "GET",
            "/api/v1/agents/",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(
        first["workflow_id"], second["workflow_id"],
        "a retried hire cannot dispatch a second task"
    );
    assert_eq!(first["agent_id"], second["agent_id"]);
    assert_eq!(agents.as_array().unwrap().len(), 1);
    assert_eq!(DISPATCHES.read().unwrap().len(), 1);
}

#[tokio::test]
async fn mounted_workflow_receipt_survives_a_new_database_connection_and_unavailable_provider() {
    let directory =
        std::env::temp_dir().join(format!("ohc-mounted-receipt-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.join("workflow.sqlite").display()
    );
    let f = Fixture::new_at(&url).await;
    let key = uuid::Uuid::new_v4().to_string();
    let body = serde_json::json!({"name":"Durable private work","task":"Preserve the actual owner text across restart","model":"Auto"});
    let (_, accepted) =
        keyed_work_request(&f, "/api/v1/agents/workflows", &key, body.clone()).await;
    f.execution.wait_for_workers().await;
    f.execution.stop_workers().await;
    let reopened = persistence::AppDatabase::connect(&url).await.unwrap();
    let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(reopened.connection().clone()),
    )));
    let execution = Arc::new(
        workflow_execution::WorkflowExecution::unavailable(auth.clone()).with_receipts(reopened),
    );
    let app = Router::new()
        .route(
            "/api/v1/agents/workflows",
            get(list_workflows_handler).post(create_workflow_handler),
        )
        .layer(axum::Extension(execution))
        .route_layer(axum::middleware::from_fn_with_state(
            auth,
            server_auth::strict_bearer_auth_middleware,
        ));
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/agents/workflows")
                .header("authorization", format!("Bearer {}", f.a))
                .header("idempotency-key", &key)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let saved: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1_048_576).await.unwrap()).unwrap();
    assert_eq!(saved["workflow"]["id"], accepted["workflow"]["id"]);
    assert_eq!(saved["workflow"]["request_id"], key);
    assert_eq!(saved["workflow"]["status"], "completed");
    assert!(
        saved["workflow"]["output"]
            .as_str()
            .unwrap()
            .contains("actual owner text across restart")
    );
    assert_eq!(DISPATCHES.read().unwrap().len(), 1);
    drop(f);
    std::fs::remove_dir_all(directory).unwrap();
}

#[tokio::test]
async fn request_id_reuse_with_changed_text_or_hire_role_is_a_conflict() {
    let f = Fixture::new().await;
    for path in ["/api/v1/agents/workflows", "/api/v1/agents/hire"] {
        let key = uuid::Uuid::new_v4().to_string();
        let mut body =
            serde_json::json!({"name":"Immutable work","task":"Original text","model":"Auto"});
        if path.ends_with("hire") {
            body["role"] = "Operations".into();
        }
        assert!(
            keyed_work_request(&f, path, &key, body.clone())
                .await
                .0
                .is_success()
        );
        f.execution.wait_for_workers().await;
        if path.ends_with("hire") {
            body["role"] = "Different role".into();
        } else {
            body["task"] = "Different submitted text".into();
        }
        assert_eq!(
            keyed_work_request(&f, path, &key, body).await.0,
            StatusCode::CONFLICT
        );
    }
    assert_eq!(DISPATCHES.read().unwrap().len(), 2);
}

#[tokio::test]
async fn malformed_or_nil_request_ids_do_not_create_work() {
    let f = Fixture::new().await;
    for key in [
        "",
        "not-a-request-id",
        "00000000-0000-0000-0000-000000000000",
    ] {
        assert_eq!(
            keyed_work_request(
                &f,
                "/api/v1/agents/workflows",
                key,
                serde_json::json!({"name":"Invalid","task":"No effect"})
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    assert!(f.records().await.is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn authenticated_cancel_before_claim_prevents_dispatch_and_cross_tenant_cancellation() {
    let f = Fixture::new().await;
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    let reservation = f
        .execution
        .prepare(
            &claims,
            &headers,
            "No provider effect",
            workflow_execution::receipts::RequestMetadata {
                request_id: uuid::Uuid::new_v4(),
                name: "Cancel queued work".into(),
                workflow: "analysis".into(),
                requested_model: "Auto".into(),
                agent_role: None,
            },
        )
        .await
        .unwrap();
    let id = reservation.receipt().id.clone();
    let path = format!("/api/v1/agents/workflows/{id}/cancel");
    assert_eq!(
        f.request("POST", &path, Some(&f.b), serde_json::Value::Null)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    let (status, cancelled) = f
        .request("POST", &path, Some(&f.a), serde_json::Value::Null)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(cancelled["workflow"]["status"], "cancelled");
    assert!(f.execution.dispatch(reservation, None).await.is_err());
    assert!(DISPATCHES.read().unwrap().is_empty());
}

struct OwnedHttpProvider {
    execution: Arc<workflow_execution::WorkflowExecution>,
    requests: Arc<std::sync::Mutex<Vec<serde_json::Value>>>,
    response: Arc<std::sync::Mutex<serde_json::Value>>,
    hold: Arc<std::sync::atomic::AtomicBool>,
    started: Arc<tokio::sync::Notify>,
    release: Arc<tokio::sync::Notify>,
    usage_at_effect:
        Arc<std::sync::Mutex<Vec<server_harness::middleware::usage_ledger::UsageRecord>>>,
    server: tokio::task::JoinHandle<()>,
}
impl Drop for OwnedHttpProvider {
    fn drop(&mut self) {
        self.release.notify_waiters();
        self.server.abort();
    }
}
impl OwnedHttpProvider {
    async fn open(f: &Fixture) -> Self {
        Self::open_with_payer(f, "managed_api").await
    }
    async fn open_with_payer(f: &Fixture, payer: &str) -> Self {
        Self::open_with_ceiling(f, payer, 20000).await
    }
    async fn open_with_ceiling(f: &Fixture, payer: &str, ceiling: i64) -> Self {
        let requests = Arc::new(std::sync::Mutex::new(Vec::new()));
        let captured = requests.clone();
        let hold = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let hold_request = hold.clone();
        let started = Arc::new(tokio::sync::Notify::new());
        let signal_started = started.clone();
        let release = Arc::new(tokio::sync::Notify::new());
        let await_release = release.clone();
        let response = Arc::new(std::sync::Mutex::new(
            serde_json::json!({"id":"owned-http-receipt-1","choices":[{"message":{"role":"assistant","content":"Observed local provider result"},"finish_reason":"stop"}],"usage":{"prompt_tokens":100,"completion_tokens":30,"total_tokens":130}}),
        ));
        let returned = response.clone();
        let usage_at_effect = Arc::new(std::sync::Mutex::new(Vec::new()));
        let observed = usage_at_effect.clone();
        let ledger = server_harness::middleware::usage_ledger::UsageLedger::Sqlite(
            f.database.connection().get_sqlite_connection_pool().clone(),
        );
        let app = Router::new().route(
            "/v1/chat/completions",
            post(move |Json(body): Json<serde_json::Value>| {
                let captured = captured.clone();
                let returned = returned.clone();
                let ledger = ledger.clone();
                let observed = observed.clone();
                let hold = hold_request.clone();
                let started = signal_started.clone();
                let release = await_release.clone();
                async move {
                    captured.lock().unwrap().push(body);
                    *observed.lock().unwrap() =
                        ledger.records("workflow-tenant-a", "").await.unwrap();
                    started.notify_one();
                    if hold.load(std::sync::atomic::Ordering::SeqCst) {
                        release.notified().await;
                    }
                    Json(returned.lock().unwrap().clone())
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/v1", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let settings=[("OMNISOLO_LLM_PROVIDER","openai-compatible".to_owned()),("OMNISOLO_LLM_MODEL","configured-model".into()),("OMNISOLO_LLM_ENDPOINT",endpoint),("OMNISOLO_LLM_API_KEY","public-local-provider-fixture".into()),("OMNISOLO_MAX_TOKENS","128".into()),("OMNISOLO_LLM_TENANT_ID","workflow-tenant-a".into()),("OMNISOLO_USAGE_PAYER",payer.into()),("OMNISOLO_USAGE_MAX_REQUEST_MICROS",ceiling.to_string()),("OMNISOLO_USAGE_RATE_CARDS",r#"{"openai-compatible/configured-model":{"revision":"owned-fixture-rate-v1","input_micros_per_million":1000000,"output_micros_per_million":2000000,"cached_input_micros_per_million":1000000}}"#.into())];
        let old = settings
            .iter()
            .map(|(key, _)| (*key, std::env::var(key).ok()))
            .collect::<Vec<_>>();
        for (key, value) in &settings {
            unsafe {
                std::env::set_var(key, value);
            }
        }
        let execution = configured_workflow_execution(f.store.clone(), f.database.clone());
        for (key, value) in old {
            unsafe {
                if let Some(value) = value {
                    std::env::set_var(key, value);
                } else {
                    std::env::remove_var(key);
                }
            }
        }
        Self {
            execution,
            requests,
            response,
            hold,
            started,
            release,
            usage_at_effect,
            server,
        }
    }
    async fn prepare(
        &self,
        f: &Fixture,
        token: &str,
    ) -> Result<workflow_execution::receipts::Reservation, workflow_execution::receipts::Error>
    {
        let claims = f.store.validate_token(token).await.unwrap();
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
        self.execution
            .prepare(
                &claims,
                &headers,
                "Analyze only this actual owner-supplied text",
                workflow_execution::receipts::RequestMetadata {
                    request_id: uuid::Uuid::new_v4(),
                    name: "Budgeted provider call".into(),
                    workflow: "analysis".into(),
                    requested_model: "Auto".into(),
                    agent_role: None,
                },
            )
            .await
    }
}

#[tokio::test]
async fn production_provider_admission_denies_a_different_signed_operator_tenant() {
    let f = Fixture::new().await;
    let provider = OwnedHttpProvider::open(&f).await;
    let result = provider.prepare(&f, &f.b).await;
    assert!(
        matches!(result, Err(workflow_execution::receipts::Error::Forbidden)),
        "a signed owner is not authority over another tenant's provider credential"
    );
    assert!(provider.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn production_provider_admission_requires_a_durable_budget_reservation() {
    let f = Fixture::new().await;
    let ledger = server_harness::middleware::usage_ledger::UsageLedger::Sqlite(
        f.database.connection().get_sqlite_connection_pool().clone(),
    );
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 1).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    assert!(
        provider.prepare(&f, &f.a).await.is_err(),
        "exhausted hard budget must reject before queue acceptance or any provider effect"
    );
    assert!(provider.requests.lock().unwrap().is_empty());
    assert_eq!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros,
        0
    );
}

#[tokio::test]
async fn production_provider_dispatch_commits_reservation_then_settles_only_observed_usage_once() {
    use server_harness::middleware::usage_ledger::{PayerMode, UsageLedger};
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    let reserved = provider.prepare(&f, &f.a).await.unwrap();
    let saved = reserved.receipt().clone();
    assert!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros
            > 0
    );
    provider.execution.dispatch(reserved, None).await.unwrap();
    provider.execution.wait_for_workers().await;
    let at_effect = provider.usage_at_effect.lock().unwrap().clone();
    assert_eq!(at_effect.len(), 1);
    assert_eq!(
        at_effect[0].state, "in_flight",
        "provider must observe a committed in-flight reservation before receiving a request"
    );
    assert_eq!(at_effect[0].scope.payer, PayerMode::ManagedApi);
    assert_eq!(at_effect[0].event_id, saved.id);
    assert!(at_effect[0].reserved_micros > 160);
    drop(at_effect);
    let entries = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].state, "settled");
    assert_eq!(entries[0].charged_micros, Some(160));
    assert_eq!(
        entries[0].receipt.as_ref().unwrap().provider_request_id,
        "owned-http-receipt-1"
    );
    assert_eq!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros,
        0
    );
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
    assert_eq!(
        saved.funding.as_ref().unwrap().operator_tenant,
        "workflow-tenant-a"
    );
}

#[tokio::test]
async fn missing_provider_usage_remains_unknown_and_retains_the_hard_budget_hold() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    provider
        .response
        .lock()
        .unwrap()
        .as_object_mut()
        .unwrap()
        .remove("usage");
    let reserved = provider.prepare(&f, &f.a).await.unwrap();
    provider.execution.dispatch(reserved, None).await.unwrap();
    provider.execution.wait_for_workers().await;
    let entries = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(entries[0].state, "reconciliation_required");
    assert_eq!(entries[0].charged_micros, None);
    assert_eq!(entries[0].receipt.as_ref().unwrap().counts, None);
    assert!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros
            > 0
    );
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn cancellation_of_a_durably_queued_unstarted_request_releases_only_its_reserved_budget() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    let reservation = provider.prepare(&f, &f.a).await.unwrap();
    let id = reservation.receipt().id.clone();
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    assert_eq!(
        provider
            .execution
            .cancel_receipt(&claims, &headers, &id)
            .await
            .unwrap()
            .phase,
        workflow_execution::receipts::StoredPhase::Cancelled
    );
    assert_eq!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros,
        0
    );
    assert!(
        provider
            .execution
            .dispatch(reservation, None)
            .await
            .is_err()
    );
    assert!(provider.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn restart_after_durable_claim_retains_unknown_usage_exposure_without_reexecution() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    let reserved = provider.prepare(&f, &f.a).await.unwrap();
    let id = reserved.receipt().id.clone();
    let lease = provider
        .execution
        .receipt_store()
        .unwrap()
        .claim(reserved)
        .await
        .unwrap()
        .unwrap();
    drop(lease);
    // New process/controller has no provider configuration. Cancellation of an
    // old durable claim is uncertain; it cannot free a possibly consumed hold.
    let reopened = workflow_execution::WorkflowExecution::unavailable(f.store.clone())
        .with_receipts(f.database.clone());
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    assert_eq!(
        reopened
            .cancel_receipt(&claims, &headers, &id)
            .await
            .unwrap()
            .phase,
        workflow_execution::receipts::StoredPhase::OutcomeUnknown
    );
    let entries = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(entries[0].state, "reconciliation_required");
    assert!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros
            > 0
    );
    assert_eq!(entries[0].charged_micros, None);
    assert!(provider.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn production_execution_policy_does_not_offer_another_tenants_provider() {
    let f = Fixture::new().await;
    let provider = OwnedHttpProvider::open(&f).await;
    let app = Router::new()
        .route("/policy", get(execution_policy_handler))
        .layer(axum::Extension(provider.execution.clone()))
        .route_layer(axum::middleware::from_fn_with_state(
            f.store.clone(),
            server_auth::strict_bearer_auth_middleware,
        ));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/policy")
                .header("authorization", format!("Bearer {}", f.b))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1_048_576).await.unwrap()).unwrap();
    assert!(body.get("policy").is_none());
    assert!(provider.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn duplicate_provider_receipt_preserves_evidence_without_a_second_charge_or_refund() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    for _ in 0..2 {
        let reservation = provider.prepare(&f, &f.a).await.unwrap();
        provider
            .execution
            .dispatch(reservation, None)
            .await
            .unwrap();
        provider.execution.wait_for_workers().await;
    }
    let entries = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(entries.len(), 2);
    let uncharged = entries
        .iter()
        .find(|entry| entry.charged_micros.is_none())
        .unwrap();
    assert_eq!(
        uncharged
            .receipt
            .as_ref()
            .map(|receipt| receipt.provider_request_id.as_str()),
        Some("owned-http-receipt-1"),
        "conflicting observed receipts remain durable reconciliation evidence"
    );
    assert_eq!(uncharged.state, "reconciliation_required");
    let account = ledger.summary("workflow-tenant-a").await.unwrap();
    assert_eq!(account.spent_micros, 160);
    assert!(account.reserved_micros > 0);
}

#[tokio::test]
async fn missing_request_keys_cannot_mutate_budget_register_agents_or_reach_a_provider() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    let hub = Arc::new(Hub::default());
    let app = Router::new()
        .route("/api/v1/agents/workflows", post(create_workflow_handler))
        .route("/api/v1/agents/hire", post(hire_handler))
        .with_state(hub.clone())
        .layer(axum::Extension(provider.execution.clone()))
        .route_layer(axum::middleware::from_fn_with_state(
            f.store.clone(),
            server_auth::strict_bearer_auth_middleware,
        ));
    let mut statuses = Vec::new();
    for path in ["/api/v1/agents/workflows", "/api/v1/agents/hire"] {
        let mut body =
            serde_json::json!({"name":"Missing key","task":"No untracked effect","model":"Auto"});
        if path.ends_with("hire") {
            body["role"] = "Operations".into();
        }
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(path)
                    .header("authorization", format!("Bearer {}", f.a))
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        statuses.push(response.status());
        provider.execution.wait_for_workers().await;
    }
    assert_eq!(statuses, [StatusCode::BAD_REQUEST, StatusCode::BAD_REQUEST]);
    assert!(provider.requests.lock().unwrap().is_empty());
    assert!(
        ledger
            .records("workflow-tenant-a", "")
            .await
            .unwrap()
            .is_empty()
    );
    let account = ledger.summary("workflow-tenant-a").await.unwrap();
    assert_eq!(account.spent_micros, 0);
    assert_eq!(account.reserved_micros, 0);
    assert!(hub.get_agents_by_org("workflow-tenant-a").await.is_empty());
}

#[tokio::test]
async fn actual_authenticated_http_route_worker_and_provider_settle_one_durable_hire() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    let hub = Arc::new(Hub::default());
    let app = Router::new()
        .route("/api/v1/agents/hire", post(hire_handler))
        .route("/api/v1/agents/workflows", get(list_workflows_handler))
        .with_state(hub)
        .layer(axum::Extension(provider.execution.clone()))
        .route_layer(axum::middleware::from_fn_with_state(
            f.store.clone(),
            server_auth::strict_bearer_auth_middleware,
        ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let api = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    let key = uuid::Uuid::new_v4().to_string();
    let body = serde_json::json!({"name":"Owned HTTP specialist","role":"Operations","task":"Real serialized owner text","model":"Auto"});
    let mut replies = Vec::new();
    for _ in 0..2 {
        let response = client
            .post(format!("http://{address}/api/v1/agents/hire"))
            .bearer_auth(&f.a)
            .header("Idempotency-Key", &key)
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::CREATED);
        replies.push(response.json::<serde_json::Value>().await.unwrap());
        provider.execution.wait_for_workers().await;
    }
    assert_eq!(replies[0]["workflow_id"], replies[1]["workflow_id"]);
    assert_eq!(replies[0]["agent_id"], replies[1]["agent_id"]);
    let records = client
        .get(format!("http://{address}/api/v1/agents/workflows"))
        .bearer_auth(&f.a)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    api.abort();
    let _ = api.await;
    assert_eq!(records["workflows"][0]["request_id"], key);
    assert_eq!(records["workflows"][0]["agent_id"], replies[0]["agent_id"]);
    assert_eq!(records["workflows"][0]["status"], "completed");
    assert_eq!(
        records["workflows"][0]["output"],
        "Observed local provider result"
    );
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
    assert_eq!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .spent_micros,
        160
    );
}

#[tokio::test]
async fn cancellation_after_provider_dispatch_keeps_an_unknown_receipt_and_all_budget_exposure() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    provider
        .hold
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let reservation = provider.prepare(&f, &f.a).await.unwrap();
    let id = reservation.receipt().id.clone();
    provider
        .execution
        .dispatch(reservation, None)
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.started.notified(),
    )
    .await
    .unwrap();
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    let cancelled = provider
        .execution
        .cancel_receipt(&claims, &headers, &id)
        .await
        .unwrap();
    assert_eq!(
        cancelled.phase,
        workflow_execution::receipts::StoredPhase::OutcomeUnknown
    );
    assert_eq!(cancelled.output, None);
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.execution.wait_for_workers(),
    )
    .await
    .unwrap();
    let record = &ledger.records("workflow-tenant-a", "").await.unwrap()[0];
    assert_eq!(record.state, "reconciliation_required");
    assert_eq!(record.charged_micros, None);
    let account = ledger.summary("workflow-tenant-a").await.unwrap();
    assert!(account.reserved_micros > 0);
    assert_eq!(account.spent_micros, 0);
    provider.release.notify_waiters();
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn concurrent_admission_cannot_spend_a_budget_already_reserved_by_an_inflight_provider() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 5000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    provider
        .hold
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let reservation = provider.prepare(&f, &f.a).await.unwrap();
    provider
        .execution
        .dispatch(reservation, None)
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.started.notified(),
    )
    .await
    .unwrap();
    assert!(matches!(
        provider.prepare(&f, &f.a).await,
        Err(workflow_execution::receipts::Error::Budget)
    ));
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
    provider.release.notify_one();
    provider.execution.wait_for_workers().await;
    let account = ledger.summary("workflow-tenant-a").await.unwrap();
    assert_eq!(account.spent_micros, 160);
    assert_eq!(account.reserved_micros, 0);
}

#[tokio::test]
async fn canonical_token_revocation_during_real_provider_io_discards_output_and_retains_unknown_usage()
 {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    provider
        .hold
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let reservation = provider.prepare(&f, &f.a).await.unwrap();
    let id = reservation.receipt().id.clone();
    provider
        .execution
        .dispatch(reservation, None)
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.started.notified(),
    )
    .await
    .unwrap();
    let claims = f.store.validate_token(&f.a).await.unwrap();
    f.store
        .revoke_token(
            claims.jti,
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "workflow-tenant-a",
        )
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.execution.wait_for_workers(),
    )
    .await
    .unwrap();
    let admin = f.store.validate_token(&f.admin).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "authorization",
        format!("Bearer {}", f.admin).parse().unwrap(),
    );
    let receipt = provider
        .execution
        .get_receipt(&admin, &headers, &id)
        .await
        .unwrap();
    assert_eq!(
        receipt.phase,
        workflow_execution::receipts::StoredPhase::OutcomeUnknown
    );
    assert!(receipt.output.is_none());
    let record = &ledger.records("workflow-tenant-a", "").await.unwrap()[0];
    assert_eq!(record.state, "reconciliation_required");
    assert_eq!(record.charged_micros, None);
    assert!(record.reserved_micros > 0);
    provider.release.notify_waiters();
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn customer_direct_byok_usage_is_observed_without_a_second_inference_charge() {
    use server_harness::middleware::usage_ledger::{PayerMode, UsageLedger};
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 0).await.unwrap();
    let provider = OwnedHttpProvider::open_with_payer(&f, "byok_api").await;
    let reserved = provider.prepare(&f, &f.a).await.unwrap();
    provider.execution.dispatch(reserved, None).await.unwrap();
    provider.execution.wait_for_workers().await;
    let records = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].scope.payer, PayerMode::ByokApi);
    assert_eq!(records[0].state, "settled");
    assert_eq!(records[0].charged_micros, Some(0));
    assert_eq!(records[0].provider_cost_micros, Some(160));
    assert_eq!(
        records[0].receipt.as_ref().unwrap().provider_request_id,
        "owned-http-receipt-1"
    );
    assert_eq!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .spent_micros,
        0
    );
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn managed_reservation_covers_the_actual_normalized_provider_text_and_fixed_system() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 50000).await.unwrap();
    let provider = OwnedHttpProvider::open_with_ceiling(&f, "managed_api", 50000).await;
    let task = format!("[{}]", vec!["1e2"; 3000].join(","));
    assert!(task.chars().count() <= 16000);
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    let reservation = provider
        .execution
        .prepare(
            &claims,
            &headers,
            &task,
            workflow_execution::receipts::RequestMetadata {
                request_id: uuid::Uuid::new_v4(),
                name: "Normalized input bound".into(),
                workflow: "analysis".into(),
                requested_model: "Auto".into(),
                agent_role: None,
            },
        )
        .await
        .unwrap();
    provider
        .execution
        .dispatch(reservation, None)
        .await
        .unwrap();
    provider.execution.wait_for_workers().await;
    let requests = provider.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let messages = requests[0]["messages"].as_array().unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0]["role"], "system");
    assert_eq!(messages[1]["role"], "user");
    assert!(
        requests[0].get("tools").is_none() || requests[0]["tools"].as_array().unwrap().is_empty()
    );
    assert!(requests[0].get("previous_response_id").is_none());
    let actual_text_bytes: i64 = messages
        .iter()
        .map(|message| message["content"].as_str().unwrap().len() as i64)
        .sum();
    assert!(
        actual_text_bytes > task.len() as i64 + 4096,
        "fixture must expose JSON number normalization growth beyond the old allowance"
    );
    let reserved = provider.usage_at_effect.lock().unwrap()[0].reserved_micros;
    assert!(
        reserved >= actual_text_bytes + 2 * 128,
        "the actual serialized provider text cannot exceed the reserved input/output token bound"
    );
}

#[tokio::test]
async fn missing_external_provider_id_is_not_replaced_by_billable_sdk_evidence() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    provider
        .response
        .lock()
        .unwrap()
        .as_object_mut()
        .unwrap()
        .remove("id");
    let reserved = provider.prepare(&f, &f.a).await.unwrap();
    let id = reserved.receipt().id.clone();
    provider.execution.dispatch(reserved, None).await.unwrap();
    provider.execution.wait_for_workers().await;
    let records = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(records[0].event_id, id);
    assert_eq!(records[0].state, "reconciliation_required");
    assert_eq!(records[0].charged_micros, None);
    assert_eq!(
        records[0].receipt.as_ref().unwrap().provider_request_id,
        format!("unknown:{id}")
    );
    assert_eq!(
        records[0]
            .receipt
            .as_ref()
            .unwrap()
            .counts
            .as_ref()
            .unwrap()
            .input,
        100
    );
    assert_eq!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .spent_micros,
        0
    );
    assert!(
        ledger
            .summary("workflow-tenant-a")
            .await
            .unwrap()
            .reserved_micros
            > 0
    );
}

#[tokio::test]
async fn oversized_provider_json_is_not_parsed_into_a_completion_or_settlement_receipt() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let f = Fixture::new().await;
    let ledger = UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.initialize().await.unwrap();
    ledger.set_limit("workflow-tenant-a", 20000).await.unwrap();
    let provider = OwnedHttpProvider::open(&f).await;
    provider.response.lock().unwrap()["choices"][0]["message"]["content"] = "x"
        .repeat(omnisolo_builtin_agent_llm::MAX_PROVIDER_RESPONSE_BYTES + 1)
        .into();
    let reserved = provider.prepare(&f, &f.a).await.unwrap();
    let id = reserved.receipt().id.clone();
    provider.execution.dispatch(reserved, None).await.unwrap();
    provider.execution.wait_for_workers().await;
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    let receipt = provider
        .execution
        .get_receipt(&claims, &headers, &id)
        .await
        .unwrap();
    assert_eq!(
        receipt.phase,
        workflow_execution::receipts::StoredPhase::OutcomeUnknown
    );
    assert!(receipt.output.is_none());
    let records = ledger.records("workflow-tenant-a", "").await.unwrap();
    assert_eq!(records[0].state, "reconciliation_required");
    assert_eq!(records[0].charged_micros, None);
    assert!(records[0].receipt.is_none());
    assert_eq!(provider.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn exact_provider_json_reader_bounds_announced_and_chunked_bodies_before_parsing() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    for chunked in [false, true] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (release, wait) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut data = [0u8; 4096];
            let read = stream.read(&mut data).await.unwrap();
            assert!(read > 0);
            if chunked {
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n6\r\n{\"x\":\"\r\n").await.unwrap();
                let block = vec![b'x'; 65536];
                for _ in 0..33 {
                    if stream.write_all(b"10000\r\n").await.is_err()
                        || stream.write_all(&block).await.is_err()
                        || stream.write_all(b"\r\n").await.is_err()
                    {
                        break;
                    }
                }
                let _ = stream.write_all(b"2\r\n\"}\r\n0\r\n\r\n").await;
            } else {
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2147483647\r\n\r\n").await.unwrap();
                let _ = wait.await;
            }
        });
        let response = reqwest::Client::builder()
            .no_proxy()
            .build()
            .unwrap()
            .get(format!("http://{address}/"))
            .send()
            .await
            .unwrap();
        let outcome = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            llm_wire_contract::read_provider_json::<serde_json::Value>(response),
        )
        .await;
        let _ = release.send(());
        let _ = server.await;
        let error = outcome
            .expect("byte bounds must reject before waiting for an oversized body")
            .unwrap_err();
        assert!(error.to_string().contains("byte limit"), "{error}");
    }
}

async fn seed_readback_receipts(
    f: &Fixture,
    count: usize,
    large: bool,
) -> Vec<workflow_execution::receipts::Receipt> {
    let claims = f.store.validate_token(&f.a).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {}", f.a).parse().unwrap());
    let mut receipts = Vec::new();
    for index in 0..count {
        let task = if large {
            "\u{0002}".repeat(16000)
        } else {
            "Readback storage fixture".into()
        };
        let reserved = f
            .execution
            .prepare(
                &claims,
                &headers,
                &task,
                workflow_execution::receipts::RequestMetadata {
                    request_id: uuid::Uuid::new_v4(),
                    name: format!("Owned receipt {index}"),
                    workflow: "analysis".into(),
                    requested_model: "Auto".into(),
                    agent_role: Some("Text analyst".into()),
                },
            )
            .await
            .unwrap();
        let lease = f
            .execution
            .receipt_store()
            .unwrap()
            .claim(reserved)
            .await
            .unwrap()
            .unwrap();
        receipts.push(
            f.execution
                .receipt_store()
                .unwrap()
                .finish(
                    &lease.completion_proof(),
                    &workflow_execution::AnalysisOutcome::Completed(if large {
                        "\u{0001}".repeat(64000)
                    } else {
                        format!("Stored output {index}")
                    }),
                )
                .await
                .unwrap(),
        );
    }
    receipts
}

#[tokio::test]
async fn readback_by_exact_request_id_finds_older_than_one_hundred_and_fences_actor_tenant() {
    let f = Fixture::new().await;
    let receipts = seed_readback_receipts(&f, 105, false).await;
    let oldest = receipts
        .iter()
        .min_by(|a, b| (a.created_at, &a.id).cmp(&(b.created_at, &b.id)))
        .unwrap();
    let path = format!("/api/v1/agents/workflows/by-request/{}", oldest.request_id);
    let (status, body) = f
        .request("GET", &path, Some(&f.a), serde_json::Value::Null)
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "direct request read must not depend on newest100 history"
    );
    assert_eq!(body["workflow"]["id"], oldest.id);
    for token in [&f.b, &f.admin] {
        assert_eq!(
            f.request("GET", &path, Some(token), serde_json::Value::Null)
                .await
                .0,
            StatusCode::NOT_FOUND
        );
    }
    for key in [
        "AAAAAAAA-AAAA-4AAA-8AAA-AAAAAAAAAAAA",
        "00000000-0000-0000-0000-000000000000",
        "not-a-uuid",
    ] {
        assert_eq!(
            f.request(
                "GET",
                &format!("/api/v1/agents/workflows/by-request/{key}"),
                Some(&f.a),
                serde_json::Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
}

#[tokio::test]
async fn readback_pages_preserve_tied_timestamps_empty_last_page_and_stale_cursors() {
    let f = Fixture::new().await;
    let records = seed_readback_receipts(&f, 105, false).await;
    assert!(
        records
            .windows(2)
            .any(|r| r[0].created_at == r[1].created_at)
    );
    let mut path = "/api/v1/agents/workflows?limit=7".to_string();
    let mut ids = std::collections::HashSet::new();
    loop {
        let (status, body) = f
            .request("GET", &path, Some(&f.a), serde_json::Value::Null)
            .await;
        assert_eq!(status, StatusCode::OK);
        let rows = body["workflows"].as_array().unwrap();
        assert!(rows.len() <= 7, "client page limit must be enforced");
        for row in rows {
            assert!(ids.insert(row["id"].as_str().unwrap().to_owned()));
        }
        match body["next_cursor"].as_str() {
            Some(cursor) => {
                assert!(!rows.is_empty());
                path = format!("/api/v1/agents/workflows?limit=7&before={cursor}");
            }
            None => break,
        }
    }
    assert_eq!(ids.len(), 105);
    let (status, empty) = f
        .request(
            "GET",
            "/api/v1/agents/workflows?before=0:aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty["workflows"], serde_json::json!([]));
    assert!(empty["next_cursor"].is_null());
    for query in [
        "limit=0",
        "limit=21",
        "limit=7&limit=8",
        "before=bad",
        "before=0:00000000-0000-0000-0000-000000000000",
    ] {
        assert_eq!(
            f.request(
                "GET",
                &format!("/api/v1/agents/workflows?{query}"),
                Some(&f.a),
                serde_json::Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
}

#[tokio::test]
async fn readback_large_escaped_complete_receipts_pass_the_real_authenticated_proxy_limit() {
    use tokio::io::AsyncWriteExt;
    let f = Fixture::new().await;
    let records = seed_readback_receipts(&f, 8, true).await;
    let mut path = "/api/v1/agents/workflows".to_string();
    let mut ids = std::collections::HashSet::new();
    loop {
        let (status, body) = f
            .request("GET", &path, Some(&f.a), serde_json::Value::Null)
            .await;
        assert_eq!(status, StatusCode::OK);
        assert!(serde_json::to_vec(&body).unwrap().len() <= 1_048_576);
        let mut child = tokio::process::Command::new("node")
            .kill_on_drop(true)
            .arg("scripts/agent-workflow-contract/proxy-readback-proof.cjs")
            .current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(&body).unwrap())
            .await
            .unwrap();
        let result =
            tokio::time::timeout(std::time::Duration::from_secs(15), child.wait_with_output())
                .await
                .unwrap()
                .unwrap();
        assert!(
            result.status.success(),
            "actual proxy proof: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        for row in body["workflows"].as_array().unwrap() {
            assert_eq!(row["task"], "\u{0002}".repeat(16000));
            assert_eq!(row["output"], "\u{0001}".repeat(64000));
            assert!(ids.insert(row["id"].as_str().unwrap().to_owned()));
        }
        match body["next_cursor"].as_str() {
            Some(cursor) => path = format!("/api/v1/agents/workflows?before={cursor}"),
            None => break,
        }
    }
    assert_eq!(ids.len(), records.len());
    let (status, detail) = f
        .request(
            "GET",
            &format!("/api/v1/agents/workflows/{}", records[0].id),
            Some(&f.a),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(detail["workflow"]["output"], "\u{0001}".repeat(64000));
}
