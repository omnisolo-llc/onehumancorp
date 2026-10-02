use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    routing::{get, post},
};
use tower::ServiceExt;

static TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

struct PoisonedRegistryFixture;
impl PoisonedRegistryFixture {
    fn new() -> Self {
        assert!(
            std::thread::spawn(|| {
                let _guard = get_workflow_registry().write().unwrap();
                panic!("deliberate local registry-poison fixture");
            })
            .join()
            .is_err()
        );
        assert!(get_workflow_registry().is_poisoned());
        Self
    }
}
impl Drop for PoisonedRegistryFixture {
    fn drop(&mut self) {
        // Test isolation only: restore the deliberately poisoned fixture even
        // when a real production assertion above unwinds. No handler uses this.
        get_workflow_registry().clear_poison();
        get_workflow_registry().write().unwrap().clear();
    }
}
struct RecordingInference;
impl workflow_execution::TextInference for RecordingInference {
    fn infer<'a>(
        &'a self,
        input: &'a workflow_execution::AdmittedAnalysis,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, ()>> + Send + 'a>> {
        Box::pin(async move {
            assert_ne!(input.tenant_id(), "system");
            assert!(!input.actor_id().is_empty());
            assert_eq!(input.policy().model, "configured-model");
            Ok(format!(
                "Recorded text-only result for {}: {}",
                input.tenant_id(),
                input.task()
            ))
        })
    }
}
struct Fixture {
    _guard: tokio::sync::MutexGuard<'static, ()>,
    app: Router,
    store: Arc<server_auth::Store>,
    a: String,
    b: String,
    staff: String,
    admin: String,
}
impl Fixture {
    async fn new() -> Self {
        let guard = TEST_LOCK.lock().await;
        get_workflow_registry().write().unwrap().clear();
        DISPATCHES.write().unwrap().clear();
        let store = Arc::new(server_auth::Store::new());
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
        let execution = Arc::new(workflow_execution::WorkflowExecution::configured(
            store.clone(),
            workflow_execution::AnalysisPolicy::new(
                "ollama".into(),
                "configured-model".into(),
                512,
            )
            .unwrap(),
            Arc::new(RecordingInference),
        ));
        let app = Router::new()
            .route(
                "/api/v1/agents/workflows",
                get(list_workflows_handler).post(create_workflow_handler),
            )
            .route("/api/v1/agents/hire", post(hire_handler))
            .route(
                "/api/v1/agents/execution-policy",
                get(execution_policy_handler),
            )
            .route("/api/v1/agents/", get(list_agents_handler))
            .with_state(hub)
            .layer(axum::Extension(execution))
            .route_layer(axum::middleware::from_fn_with_state(
                store.clone(),
                server_auth::strict_bearer_auth_middleware,
            ));
        Self {
            _guard: guard,
            app,
            store,
            a: tokens.remove(0),
            b: tokens.remove(0),
            staff: tokens.remove(0),
            admin: tokens.remove(0),
        }
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
        // Wait for this fixture's actual queued lifecycle to finish before
        // reusing global registry state in the next serialized HTTP case.
        tokio::time::timeout(std::time::Duration::from_secs(3), async {
            loop {
                let pending = get_workflow_registry()
                    .read()
                    .map(|records| {
                        records
                            .iter()
                            .any(|record| matches!(record.status.as_str(), "queued" | "running"))
                    })
                    .unwrap_or(false);
                if !pending {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
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
    let records = get_workflow_registry().read().unwrap();
    assert_eq!(records.len(), 2);
    assert!(
        records
            .iter()
            .all(|record| record.tenant_id == "workflow-tenant-a" && !record.actor_id.is_empty())
    );
}

async fn assert_registry_failure_creates_no_work(path: &str) {
    let f = Fixture::new().await;
    let _poison = PoisonedRegistryFixture::new();
    let (status, _) = f.request(
        "POST",
        path,
        Some(&f.a),
        if path.ends_with("/hire") { serde_json::json!({"name":"Unavailable registry","role":"Operations","task":"Must not become untracked work"}) } else { serde_json::json!({"name":"Unavailable registry","task":"Must not become untracked work"}) },
    ).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(DISPATCHES.read().unwrap().is_empty());
    assert!(get_workflow_registry().is_poisoned());
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
async fn creating_a_workflow_fails_closed_when_the_registry_is_poisoned() {
    assert_registry_failure_creates_no_work("/api/v1/agents/workflows").await;
}

#[tokio::test]
async fn hiring_an_agent_fails_closed_when_the_registry_is_poisoned() {
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn ordinary_tenant_members_cannot_start_owner_work() {
    let f = Fixture::new().await;
    for path in ["/api/v1/agents/workflows", "/api/v1/agents/hire"] {
        let (status,_)=f.request("POST",path,Some(&f.staff),if path.ends_with("/hire") { serde_json::json!({"name":"unapproved","role":"Operations","task":"must not dispatch"}) } else { serde_json::json!({"name":"unapproved","task":"must not dispatch"}) }).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
    assert!(get_workflow_registry().read().unwrap().is_empty());
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
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
    let (_, receipt) = f
        .request(
            "POST",
            "/api/v1/agents/workflows",
            Some(&f.a),
            serde_json::json!({"name":"Analysis","task":"Text only"}),
        )
        .await;
    let id = receipt["workflow"]["id"].as_str().unwrap();
    get_workflow_registry()
        .write()
        .unwrap()
        .iter_mut()
        .find(|record| record.id == id)
        .unwrap()
        .status = "outcome_unknown".into();
    assert!(!set_workflow_result(
        id,
        "completed",
        Some("late untrusted result".into()),
        None
    ));
    assert_eq!(
        get_workflow_registry()
            .read()
            .unwrap()
            .iter()
            .find(|record| record.id == id)
            .unwrap()
            .status,
        "outcome_unknown"
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
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
    assert!(get_workflow_registry().read().unwrap().is_empty());
    assert!(DISPATCHES.read().unwrap().is_empty());
}
