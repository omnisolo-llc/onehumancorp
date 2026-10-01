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
struct Fixture {
    _guard: tokio::sync::MutexGuard<'static, ()>,
    app: Router,
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
        let app = Router::new()
            .route(
                "/api/v1/agents/workflows",
                get(list_workflows_handler).post(create_workflow_handler),
            )
            .route("/api/v1/agents/hire", post(hire_handler))
            .route("/api/v1/agents/", get(list_agents_handler))
            .with_state(hub)
            .route_layer(axum::middleware::from_fn_with_state(
                store,
                server_auth::strict_bearer_auth_middleware,
            ));
        Self {
            _guard: guard,
            app,
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
            serde_json::json!({"name":"Administrator work","role":"Operations","task":"Prepare an owner report"}),
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
        serde_json::json!({"name":"Unavailable registry","role":"Operations","task":"Must not become untracked work"}),
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
        let (status,_)=f.request("POST","/api/v1/agents/workflows",Some(token),serde_json::json!({"name":name,"task":format!("Prepare {name}"),"workflow":"ohc_review_branch"})).await;
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
        let (status,_)=f.request("POST",path,Some(&f.staff),serde_json::json!({"name":"unapproved","role":"Operations","task":"must not dispatch"})).await;
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
