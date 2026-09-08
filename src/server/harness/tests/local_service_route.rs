use server_harness::middleware::local_service_gateway::*;
use server_harness::middleware::local_service_route::LocalServiceListener;
use server_harness::middleware::local_services::*;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn child_service_route_uses_issued_scope_and_is_unusable_after_shutdown() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE VIRTUAL TABLE agent_memory USING fts5(content,tags,created_at UNINDEXED)")
        .execute(&pool)
        .await
        .unwrap();
    let registry = LocalServiceRegistry::with_defaults();
    let mut gateway = LocalServiceGateway::new(registry.clone());
    gateway.register(
        LocalServiceKind::Memory,
        Arc::new(SqliteAgentMemoryBackend::new(pool)),
    );
    let gateway = Arc::new(gateway);
    let scope = LocalServiceScopeContext::for_attempt(
        "tenant",
        Some("project"),
        Some("workspace"),
        Uuid::new_v4(),
        Some(Uuid::new_v4()),
        Some(Uuid::new_v4()),
    );
    let bundle = gateway.resolve(scope.clone()).unwrap();
    let listener = LocalServiceListener::start(gateway, bundle.clone(), scope.clone())
        .await
        .unwrap();
    let route = listener.route().clone();
    assert!(!format!("{route:?}").contains(route.token()));
    let client = reqwest::Client::new();
    let url = format!("{}/operations", route.base_url());
    assert_eq!(
        client
            .post(&url)
            .json(&serde_json::json!({"operation":"memory_search","query":"sentinel","limit":10}))
            .send()
            .await
            .unwrap()
            .status(),
        reqwest::StatusCode::UNAUTHORIZED
    );
    client.post(&url).bearer_auth(route.token()).json(&serde_json::json!({"operation":"memory_write","content":"child-sentinel","tenant_id":"foreign"})).send().await.unwrap().error_for_status().unwrap();
    let result: serde_json::Value = client
        .post(&url)
        .bearer_auth(route.token())
        .json(&serde_json::json!({"operation":"memory_search","query":"sentinel","limit":10}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(result, serde_json::json!({"items":["child-sentinel"]}));
    listener.shutdown().await.unwrap();
    assert!(
        registry
            .authorize(&bundle.bindings[0], &scope, "memory.search")
            .is_err()
    );
    assert!(
        client
            .post(&url)
            .bearer_auth(route.token())
            .json(&serde_json::json!({"operation":"memory_search","query":"sentinel","limit":10}))
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn external_process_adapters_write_and_read_the_configured_service_gateway() {
    use server_harness::middleware::harness::{
        HarnessAdapter, HarnessProtocolKind, HarnessSessionRequest, ProcessHarnessAdapter,
        ProcessHarnessSpec,
    };
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE VIRTUAL TABLE agent_memory USING fts5(content,tags,created_at UNINDEXED)")
        .execute(&pool)
        .await
        .unwrap();
    let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
    gateway.register(
        LocalServiceKind::Memory,
        Arc::new(SqliteAgentMemoryBackend::new(pool)),
    );
    let gateway = Arc::new(gateway);
    let scope = LocalServiceScopeContext::for_attempt(
        "tenant",
        Some("project"),
        Some("workspace"),
        Uuid::new_v4(),
        Some(Uuid::new_v4()),
        Some(Uuid::new_v4()),
    );
    let script = r#"
import json, os, sys, urllib.request
for line in sys.stdin:
    request = json.loads(line)
    if os.environ['SERVICE_MODE'] == 'write':
        operation = {'operation':'memory_write','content':'cross-process-sentinel'}
    else:
        operation = {'operation':'memory_search','query':'sentinel','limit':10}
    call = urllib.request.Request(os.environ['OMNISOLO_LOCAL_SERVICE_URL']+'/operations',
        data=json.dumps(operation).encode(), headers={'Authorization':'Bearer '+os.environ['OMNISOLO_LOCAL_SERVICE_TOKEN'],'Content-Type':'application/json'})
    with urllib.request.urlopen(call) as response:
        result = json.load(response)
    print(json.dumps({'request_id':request['request_id'],'ok':True,'payload':{'events':[],'final_text':json.dumps(result)}}), flush=True)
"#;
    for mode in ["write", "read"] {
        let mut scope = scope.clone();
        scope.attempt_id = Some(Uuid::new_v4());
        let bundle = gateway.resolve(scope.clone()).unwrap();
        let listener = LocalServiceListener::start(gateway.clone(), bundle.clone(), scope.clone())
            .await
            .unwrap();
        let request =
            HarnessSessionRequest::new(&scope.tenant_id, scope.session_id, Uuid::new_v4())
                .with_task(scope.task_id.unwrap(), "shared services")
                .with_attempt_id(scope.attempt_id.unwrap())
                .with_local_service_bundle(bundle);
        let mut spec =
            ProcessHarnessSpec::command("python3", ["-u", "-c", script], format!("gateway-{mode}"))
                .with_protocol(HarnessProtocolKind::Custom);
        spec.environment.insert("SERVICE_MODE".into(), mode.into());
        spec.environment.insert(
            "OMNISOLO_LOCAL_SERVICE_URL".into(),
            listener.route().base_url().into(),
        );
        spec.environment.insert(
            "OMNISOLO_LOCAL_SERVICE_TOKEN".into(),
            listener.route().token().into(),
        );
        assert!(!format!("{spec:?}").contains(listener.route().token()));
        let mut adapter = ProcessHarnessAdapter::new(spec);
        let result = adapter
            .execute(
                request.clone(),
                &scope.attempt_id.unwrap().to_string(),
                mode,
                None,
            )
            .await
            .unwrap();
        if mode == "read" {
            assert!(
                result
                    .final_text
                    .unwrap()
                    .contains("cross-process-sentinel")
            );
        }
        listener.shutdown().await.unwrap();
    }
}
