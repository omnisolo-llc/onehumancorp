//! Real authenticated assistant handlers and canonical durable execution.
use super::*;

async fn call(f: &Fixture, token: Option<&str>, method: &str, path: &str, key: Option<&str>, body: serde_json::Value) -> (StatusCode,serde_json::Value) {
    let app = workflow_execution::assistant::router()
        .layer(axum::Extension(f.execution.clone()))
        .route_layer(axum::middleware::from_fn_with_state(f.store.clone(),server_auth::strict_bearer_auth_middleware));
    let mut request = Request::builder().method(method).uri(path).header("content-type","application/json");
    if let Some(token)=token { request=request.header("authorization",format!("Bearer {token}")); }
    if let Some(key)=key { request=request.header("idempotency-key",key); }
    let response=app.oneshot(request.body(if method=="GET" {Body::empty()} else {Body::from(body.to_string())}).unwrap()).await.unwrap();
    let status=response.status();
    let bytes=to_bytes(response.into_body(),2_097_152).await.unwrap();
    (status,serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null))
}
fn task() -> serde_json::Value { serde_json::json!({"prompt":"Compare the supplied numbers: 14 and 28","workspace":"Local contract","mode":"Ask","model":"Auto","provider":"Auto","outputFormat":"Text","permissionProfile":"Guarded","constraints":"State the difference","workDirectory":""}) }

#[tokio::test]
async fn assistant_dispatches_actual_text_and_reads_persisted_output_once() {
    let f=Fixture::new().await; let key=uuid::Uuid::new_v4().to_string();
    let (status,first)=call(&f,Some(&f.a),"POST","/tasks",Some(&key),task()).await;
    assert_eq!(status,StatusCode::ACCEPTED,"{first}");
    assert!(first["execution"]["id"].is_string());
    f.execution.wait_for_workers().await;
    let path=format!("/tasks/{}",first["id"].as_str().unwrap());
    let (status,read)=call(&f,Some(&f.a),"GET",&path,None,serde_json::Value::Null).await;
    assert_eq!(status,StatusCode::OK); assert_eq!(read["status"],"completed");
    assert!(read["execution"]["output"].as_str().unwrap().contains("14 and 28"));
    assert_eq!(DISPATCHES.read().unwrap().len(),1);
    let (status,replay)=call(&f,Some(&f.a),"POST","/tasks",Some(&key),task()).await;
    assert_eq!(status,StatusCode::ACCEPTED); assert_eq!(replay["id"],first["id"]);
    f.execution.wait_for_workers().await; assert_eq!(DISPATCHES.read().unwrap().len(),1);
    let (status,by_key)=call(&f,Some(&f.a),"GET",&format!("/tasks/by-request/{key}"),None,serde_json::Value::Null).await;
    assert_eq!(status,StatusCode::OK); assert_eq!(by_key["id"],first["id"]);
}

#[tokio::test]
async fn assistant_rejects_forged_lifecycle_and_unsupported_capabilities() {
    let f=Fixture::new().await;
    for (field,value) in [("status","completed"),("mode","Coding"),("outputFormat","PDF"),("workDirectory","/private"),("provider","custom")] {
        let mut body=task(); body[field]=value.into();
        let (status,_)=call(&f,Some(&f.a),"POST","/tasks",Some(&uuid::Uuid::new_v4().to_string()),body).await;
        assert!(status.is_client_error(),"accepted {field}");
    }
    assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn assistant_requires_request_identity_and_current_owner_authority() {
    let f=Fixture::new().await;
    for (token,key,expected) in [(None,None,StatusCode::UNAUTHORIZED),(Some(f.staff.as_str()),Some("2d56e46b-62b5-426f-a673-a9fb2e0a60b6"),StatusCode::FORBIDDEN),(Some(f.a.as_str()),None,StatusCode::BAD_REQUEST)] {
        assert_eq!(call(&f,token,"POST","/tasks",key,task()).await.0,expected);
    }
    assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn assistant_reused_identity_cannot_change_prompt_or_workspace() {
    let f=Fixture::new().await; let key=uuid::Uuid::new_v4().to_string();
    assert_eq!(call(&f,Some(&f.a),"POST","/tasks",Some(&key),task()).await.0,StatusCode::ACCEPTED);
    for field in ["prompt","workspace","constraints"] { let mut body=task(); body[field]="changed".into(); assert_eq!(call(&f,Some(&f.a),"POST","/tasks",Some(&key),body).await.0,StatusCode::CONFLICT); }
    f.execution.wait_for_workers().await; assert_eq!(DISPATCHES.read().unwrap().len(),1);
}

#[tokio::test]
async fn assistant_archive_and_unarchive_preserve_completed_output() {
    let f=Fixture::new().await;
    let (_,first)=call(&f,Some(&f.a),"POST","/tasks",Some(&uuid::Uuid::new_v4().to_string()),task()).await;
    f.execution.wait_for_workers().await;
    for (action,archived) in [("archive",true),("unarchive",false)] {
        let (status,row)=call(&f,Some(&f.a),"PATCH",&format!("/tasks/{}",first["id"].as_str().unwrap()),None,serde_json::json!({"action":action})).await;
        assert_eq!(status,StatusCode::OK); assert_eq!(row["archived"],archived); assert_eq!(row["status"],"completed"); assert!(row["execution"]["output"].is_string());
    }
    assert_eq!(DISPATCHES.read().unwrap().len(),1);
}

#[tokio::test]
async fn assistant_other_tenant_cannot_read_mutate_or_lookup_request() {
    let f=Fixture::new().await; let key=uuid::Uuid::new_v4().to_string();
    let (_,first)=call(&f,Some(&f.a),"POST","/tasks",Some(&key),task()).await;
    for (method,path,body) in [("GET",format!("/tasks/{}",first["id"].as_str().unwrap()),serde_json::Value::Null),("GET",format!("/tasks/by-request/{key}"),serde_json::Value::Null),("PATCH",format!("/tasks/{}",first["id"].as_str().unwrap()),serde_json::json!({"action":"stop"}))] { assert_eq!(call(&f,Some(&f.b),method,&path,None,body).await.0,StatusCode::NOT_FOUND); }
    assert_eq!(call(&f,Some(&f.b),"GET","/tasks",None,serde_json::Value::Null).await.1,serde_json::json!([]));
}

#[tokio::test]
async fn assistant_missing_association_schema_never_dispatches() {
    use sea_orm::ConnectionTrait;
    let f=Fixture::new().await;
    f.database.connection().execute_unprepared("DROP TABLE assistant_execution_attempts; DROP TABLE assistant_execution_tasks").await.unwrap();
    let (status,body)=call(&f,Some(&f.a),"POST","/tasks",Some(&uuid::Uuid::new_v4().to_string()),task()).await;
    assert_eq!(status,StatusCode::SERVICE_UNAVAILABLE,"{body}");
    f.execution.wait_for_workers().await; assert!(DISPATCHES.read().unwrap().is_empty());
}

#[tokio::test]
async fn assistant_cancelled_undispatched_attempt_can_resume_once_with_new_authority() {
    use sea_orm::ConnectionTrait;
    let f=Fixture::new().await; let key=uuid::Uuid::new_v4().to_string();
    f.database.connection().execute_unprepared("CREATE TRIGGER test_assistant_claim_failure BEFORE UPDATE ON tenant_workflow_receipts WHEN NEW.phase='dispatching' BEGIN SELECT RAISE(ABORT,'owned-test-claim-failure'); END;").await.unwrap();
    assert_eq!(call(&f,Some(&f.a),"POST","/tasks",Some(&key),task()).await.0,StatusCode::SERVICE_UNAVAILABLE);
    let (_,pending)=call(&f,Some(&f.a),"GET",&format!("/tasks/by-request/{key}"),None,serde_json::Value::Null).await;
    assert_eq!(pending["status"],"queued");
    let path=format!("/tasks/{}",pending["id"].as_str().unwrap());
    let (_,cancelled)=call(&f,Some(&f.a),"PATCH",&path,None,serde_json::json!({"action":"stop"})).await;
    assert_eq!(cancelled["status"],"cancelled");assert!(DISPATCHES.read().unwrap().is_empty());
    f.database.connection().execute_unprepared("DROP TRIGGER test_assistant_claim_failure").await.unwrap();
    let retry=uuid::Uuid::new_v4().to_string(); let body=serde_json::json!({"action":"resume","sourceReceiptId":pending["execution"]["id"]});
    let (status,resumed)=call(&f,Some(&f.a),"PATCH",&path,Some(&retry),body.clone()).await;
    assert_eq!(status,StatusCode::OK,"{resumed}");assert_ne!(resumed["execution"]["id"],pending["execution"]["id"]);assert_eq!(resumed["id"],pending["id"]);
    f.execution.wait_for_workers().await;
    let (status,replay)=call(&f,Some(&f.a),"PATCH",&path,Some(&retry),body).await;
    assert_eq!(status,StatusCode::OK);assert_eq!(replay["execution"]["id"],resumed["execution"]["id"]);assert_eq!(DISPATCHES.read().unwrap().len(),1);
}

#[tokio::test]
async fn assistant_real_configured_http_adapter_settles_and_persists_output() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let mut f=Fixture::new().await;
    let ledger=UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());
    ledger.set_limit("workflow-tenant-a",20000).await.unwrap();
    let provider=OwnedHttpProvider::open(&f).await; f.execution=provider.execution.clone();
    let (status,accepted)=call(&f,Some(&f.a),"POST","/tasks",Some(&uuid::Uuid::new_v4().to_string()),task()).await;
    assert_eq!(status,StatusCode::ACCEPTED,"{accepted}");f.execution.wait_for_workers().await;
    let (_,read)=call(&f,Some(&f.a),"GET",&format!("/tasks/{}",accepted["id"].as_str().unwrap()),None,serde_json::Value::Null).await;
    assert_eq!(read["status"],"completed");assert_eq!(read["execution"]["output"],"Observed local provider result");
    let requests=provider.requests.lock().unwrap();assert_eq!(requests.len(),1);
    assert!(requests[0]["messages"].to_string().contains("14 and 28"));assert!(requests[0]["messages"].to_string().contains("State the difference"));
    assert!(requests[0].get("tools").is_none_or(|tools|tools.as_array().is_some_and(Vec::is_empty)));drop(requests);
    assert_eq!(ledger.summary("workflow-tenant-a").await.unwrap().spent_micros,160);
}

#[tokio::test]
async fn assistant_inflight_stop_keeps_unknown_output_budget_hold_and_blocks_resume() {
    use server_harness::middleware::usage_ledger::UsageLedger;
    let mut f=Fixture::new().await;
    let ledger=UsageLedger::Sqlite(f.database.connection().get_sqlite_connection_pool().clone());ledger.set_limit("workflow-tenant-a",20000).await.unwrap();
    let provider=OwnedHttpProvider::open(&f).await;provider.hold.store(true,std::sync::atomic::Ordering::SeqCst);f.execution=provider.execution.clone();
    let (_,accepted)=call(&f,Some(&f.a),"POST","/tasks",Some(&uuid::Uuid::new_v4().to_string()),task()).await;
    tokio::time::timeout(std::time::Duration::from_secs(5),provider.started.notified()).await.unwrap();
    let path=format!("/tasks/{}",accepted["id"].as_str().unwrap());
    let (status,stopped)=call(&f,Some(&f.a),"PATCH",&path,None,serde_json::json!({"action":"stop","sourceReceiptId":accepted["execution"]["id"]})).await;
    assert_eq!(status,StatusCode::OK);assert_eq!(stopped["status"],"outcome_unknown");assert!(stopped["execution"]["output"].is_null());
    let (status,_)=call(&f,Some(&f.a),"PATCH",&path,Some(&uuid::Uuid::new_v4().to_string()),serde_json::json!({"action":"resume","sourceReceiptId":accepted["execution"]["id"]})).await;
    assert_eq!(status,StatusCode::CONFLICT);assert_eq!(provider.requests.lock().unwrap().len(),1);
    provider.release.notify_waiters();f.execution.wait_for_workers().await;
    assert!(ledger.summary("workflow-tenant-a").await.unwrap().reserved_micros>0);
}

#[tokio::test]
async fn assistant_readback_survives_reopened_database_without_a_provider() {
    let path=std::env::temp_dir().join(format!("ohc-assistant-{}.sqlite",uuid::Uuid::new_v4()));
    let url=format!("sqlite://{}?mode=rwc",path.display());let mut f=Fixture::new_at(&url).await;
    let (_,accepted)=call(&f,Some(&f.a),"POST","/tasks",Some(&uuid::Uuid::new_v4().to_string()),task()).await;f.execution.wait_for_workers().await;
    let reopened=persistence::AppDatabase::connect(&url).await.unwrap();
    f.execution=Arc::new(workflow_execution::WorkflowExecution::unavailable(f.store.clone()).with_receipts(reopened));
    let (status,read)=call(&f,Some(&f.a),"GET",&format!("/tasks/{}",accepted["id"].as_str().unwrap()),None,serde_json::Value::Null).await;
    assert_eq!(status,StatusCode::OK);assert_eq!(read["status"],"completed");assert!(read["execution"]["output"].as_str().unwrap().contains("14 and 28"));
    drop(f);std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn assistant_expired_unassociated_resume_reconciles_without_replaying_provider(){
    use sea_orm::ConnectionTrait;
    let f=Fixture::new().await;let key=uuid::Uuid::new_v4().to_string();
    f.database.connection().execute_unprepared("CREATE TRIGGER test_assistant_claim_failure BEFORE UPDATE ON tenant_workflow_receipts WHEN NEW.phase='dispatching' BEGIN SELECT RAISE(ABORT,'owned-test-claim-failure'); END;").await.unwrap();
    assert_eq!(call(&f,Some(&f.a),"POST","/tasks",Some(&key),task()).await.0,StatusCode::SERVICE_UNAVAILABLE);
    let (_,root)=call(&f,Some(&f.a),"GET",&format!("/tasks/by-request/{key}"),None,serde_json::Value::Null).await;let id=root["id"].as_str().unwrap();let path=format!("/tasks/{id}");
    assert_eq!(call(&f,Some(&f.a),"PATCH",&path,None,serde_json::json!({"action":"stop"})).await.1["status"],"cancelled");
    let claims=f.store.validate_token(&f.a).await.unwrap();let mut headers=axum::http::HeaderMap::new();headers.insert("authorization",format!("Bearer {}",f.a).parse().unwrap());
    let previous=f.execution.get_receipt(&claims,&headers,id).await.unwrap();let retry=uuid::Uuid::new_v4();
    let reservation=f.execution.prepare(&claims,&headers,&previous.task,workflow_execution::receipts::RequestMetadata{request_id:retry,name:previous.name.clone(),workflow:"analysis".into(),requested_model:String::new(),agent_role:Some(format!("assistant-resume:{id}:{id}"))}).await.unwrap();let target=reservation.receipt().id.clone();drop(reservation);
    // Owned test clock fixture only: age the unclaimed receipt, then reinstall
    // the exact production immutable/transition triggers before exercising readback.
    f.database.connection().execute_unprepared("DROP TRIGGER tenant_receipt_immutable_identity; DROP TRIGGER tenant_receipt_transition;").await.unwrap();
    sqlx::query("UPDATE tenant_workflow_receipts SET created_at=created_at-121,updated_at=updated_at-121 WHERE id=?").bind(&target).execute(f.database.connection().get_sqlite_connection_pool()).await.unwrap();
    f.database.connection().execute_unprepared(include_str!("../../src/server/persistence/tenant_execution_receipts_sqlite.sql")).await.unwrap();
    let (status,reconciled)=call(&f,Some(&f.a),"PATCH",&path,Some(&retry.to_string()),serde_json::json!({"action":"resume","sourceReceiptId":id})).await;
    assert_eq!(status,StatusCode::OK,"{reconciled}");assert_eq!(reconciled["status"],"cancelled");assert_eq!(reconciled["execution"]["id"],target);f.execution.wait_for_workers().await;assert!(DISPATCHES.read().unwrap().is_empty());
}
