//! Mandatory restricted-role PostgreSQL assistant admission and recovery contracts.
use super::*;
use axum::{Router, body::{Body,to_bytes}, http::{Request,StatusCode}};
use tower::ServiceExt;
use std::sync::atomic::{AtomicUsize,Ordering};
struct PgText(Arc<AtomicUsize>);
impl TextInference for PgText {
    fn infer<'a>(&'a self,input:&'a AdmittedAnalysis)->Pin<Box<dyn Future<Output=Result<crate::workflow_execution::InferenceResult,()>>+Send+'a>> {
        Box::pin(async move {self.0.fetch_add(1,Ordering::SeqCst);Ok(crate::workflow_execution::InferenceResult{output:Some(format!("Recorded PostgreSQL text boundary: {}",input.task())),provider_request_id:None,counts:None})})
    }
}
fn execution(f:&Fixture)->(Arc<WorkflowExecution>,Arc<AtomicUsize>){
    let calls=Arc::new(AtomicUsize::new(0));
    (Arc::new(WorkflowExecution::configured(f.auth.clone(),AnalysisPolicy::new("ollama".into(),"owned-receipt-model".into(),256).unwrap(),Arc::new(PgText(calls.clone()))).with_receipts(f.store.database().clone())),calls)
}
fn app(f:&Fixture,execution:Arc<WorkflowExecution>)->Router {
    crate::workflow_execution::assistant::router().layer(axum::Extension(execution))
        .route_layer(axum::middleware::from_fn_with_state(f.auth.clone(),server_auth::strict_bearer_auth_middleware))
}
async fn call(app:Router,headers:&axum::http::HeaderMap,method:&str,path:&str,key:Option<&str>,body:serde_json::Value)->(StatusCode,serde_json::Value){
    let mut request=Request::builder().method(method).uri(path).header("content-type","application/json").header("authorization",headers.get("authorization").unwrap());
    if let Some(key)=key{request=request.header("idempotency-key",key);}
    let response=app.oneshot(request.body(if method=="GET"{Body::empty()}else{Body::from(body.to_string())}).unwrap()).await.unwrap();
    let status=response.status();let body=to_bytes(response.into_body(),2_097_152).await.unwrap();(status,serde_json::from_slice(&body).unwrap_or_default())
}
fn body()->serde_json::Value{serde_json::json!({"prompt":"Actual owner supplied PostgreSQL notes","workspace":"Contract"})}

#[tokio::test]
async fn postgres_assistant_has_forced_rls_scoped_receipts_and_persisted_output(){
    let f=Fixture::open().await;let (execution,calls)=execution(&f);let app=app(&f,execution.clone());let key=Uuid::new_v4().to_string();
    for table in ["assistant_execution_tasks","assistant_execution_attempts"]{
        let forced:bool=sqlx::query_scalar("SELECT relrowsecurity AND relforcerowsecurity FROM pg_class WHERE oid=$1::regclass").bind(table).fetch_one(&f.pool).await.unwrap();assert!(forced);
    }
    let (status,accepted)=call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&key),body()).await;assert_eq!(status,StatusCode::ACCEPTED,"{accepted}");execution.wait_for_workers().await;
    let path=format!("/tasks/{}",accepted["id"].as_str().unwrap());
    let (_,read)=call(app.clone(),&f.identities[0].1,"GET",&path,None,serde_json::Value::Null).await;assert_eq!(read["status"],"completed");assert!(read["execution"]["output"].as_str().unwrap().contains("Actual owner supplied PostgreSQL notes"));
    assert_eq!(call(app.clone(),&f.identities[1].1,"GET",&path,None,serde_json::Value::Null).await.0,StatusCode::NOT_FOUND);
    let unscoped:i64=sqlx::query_scalar("SELECT count(*) FROM assistant_execution_tasks").fetch_one(&f.pool).await.unwrap();assert_eq!(unscoped,0);
    for action in ["archive","unarchive"]{let (status,row)=call(app.clone(),&f.identities[0].1,"PATCH",&path,None,serde_json::json!({"action":action})).await;assert_eq!(status,StatusCode::OK);assert_eq!(row["status"],"completed");assert_eq!(row["archived"],action=="archive");}
    assert_eq!(calls.load(Ordering::SeqCst),1);drop(app);drop(execution);f.close().await;
}

#[tokio::test]
async fn postgres_assistant_concurrent_request_replay_dispatches_once(){
    let f=Fixture::open().await;let (execution,calls)=execution(&f);let app=app(&f,execution.clone());let key=Uuid::new_v4().to_string();
    let (a,b)=tokio::join!(call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&key),body()),call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&key),body()));
    assert_eq!(a.0,StatusCode::ACCEPTED,"{}",a.1);assert_eq!(b.0,StatusCode::ACCEPTED,"{}",b.1);assert_eq!(a.1["id"],b.1["id"]);execution.wait_for_workers().await;assert_eq!(calls.load(Ordering::SeqCst),1);
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM assistant_execution_attempts").fetch_one(&f.admin).await.unwrap();assert_eq!(count,1);
    drop(app);drop(execution);f.close().await;
}

#[tokio::test]
async fn postgres_assistant_failed_association_commit_cannot_dispatch_or_redispatch(){
    let f=Fixture::open().await;let (execution,calls)=execution(&f);let app=app(&f,execution.clone());let key=Uuid::new_v4().to_string();
    sqlx::raw_sql("CREATE FUNCTION test_reject_assistant_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'owned-association-commit-rejected'; END $$; CREATE CONSTRAINT TRIGGER test_reject_assistant_commit AFTER INSERT ON assistant_execution_tasks DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION test_reject_assistant_commit();").execute(&f.admin).await.unwrap();
    let (status,_)=call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&key),body()).await;assert_eq!(status,StatusCode::SERVICE_UNAVAILABLE);execution.wait_for_workers().await;assert_eq!(calls.load(Ordering::SeqCst),0);
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM assistant_execution_tasks").fetch_one(&f.admin).await.unwrap();assert_eq!(count,0);
    sqlx::raw_sql("DROP TRIGGER test_reject_assistant_commit ON assistant_execution_tasks; DROP FUNCTION test_reject_assistant_commit();").execute(&f.admin).await.unwrap();
    let (status,recovered)=call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&key),body()).await;assert_eq!(status,StatusCode::ACCEPTED,"{recovered}");assert_eq!(recovered["status"],"queued");execution.wait_for_workers().await;assert_eq!(calls.load(Ordering::SeqCst),0);
    drop(app);drop(execution);f.close().await;
}

#[tokio::test]
async fn postgres_assistant_cancelled_orphan_resume_reconciles_without_dispatch(){
    let f=Fixture::open().await;let (execution,calls)=execution(&f);let app=app(&f,execution.clone());let root_key=Uuid::new_v4().to_string();
    sqlx::raw_sql("CREATE FUNCTION test_assistant_claim_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.phase='dispatching' THEN RAISE EXCEPTION 'owned-claim-failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER test_assistant_claim_failure BEFORE UPDATE ON tenant_workflow_receipts FOR EACH ROW EXECUTE FUNCTION test_assistant_claim_failure();").execute(&f.admin).await.unwrap();
    assert_eq!(call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&root_key),body()).await.0,StatusCode::SERVICE_UNAVAILABLE);
    let (_,root)=call(app.clone(),&f.identities[0].1,"GET",&format!("/tasks/by-request/{root_key}"),None,serde_json::Value::Null).await;let id=root["id"].as_str().unwrap();let path=format!("/tasks/{id}");
    let (_,cancelled)=call(app.clone(),&f.identities[0].1,"PATCH",&path,None,serde_json::json!({"action":"stop"})).await;assert_eq!(cancelled["status"],"cancelled");
    let (claims,headers)=&f.identities[0];let source=execution.get_receipt(claims,headers,id).await.unwrap();let key=Uuid::new_v4();
    let reservation=execution.prepare(claims,headers,&source.task,RequestMetadata{request_id:key,name:source.name.clone(),workflow:"analysis".into(),requested_model:String::new(),agent_role:Some(format!("assistant-resume:{id}:{id}"))}).await.unwrap();let target=reservation.receipt().id.clone();drop(reservation);
    execution.cancel_receipt(claims,headers,&target).await.unwrap();
    let (status,reconciled)=call(app.clone(),headers,"PATCH",&path,Some(&key.to_string()),serde_json::json!({"action":"resume","sourceReceiptId":id})).await;
    assert_eq!(status,StatusCode::OK,"{reconciled}");assert_eq!(reconciled["status"],"cancelled");assert_eq!(reconciled["execution"]["id"],target);assert_eq!(calls.load(Ordering::SeqCst),0);
    drop(app);drop(execution);f.close().await;
}

#[tokio::test]
async fn postgres_same_key_resume_race_cannot_cancel_the_winning_attempt(){
    let f=Fixture::open().await;let (execution,calls)=execution(&f);let app=app(&f,execution.clone());let root_key=Uuid::new_v4().to_string();
    sqlx::raw_sql("CREATE FUNCTION test_assistant_claim_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.phase='dispatching' THEN RAISE EXCEPTION 'owned-claim-failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER test_assistant_claim_failure BEFORE UPDATE ON tenant_workflow_receipts FOR EACH ROW EXECUTE FUNCTION test_assistant_claim_failure();").execute(&f.admin).await.unwrap();
    assert_eq!(call(app.clone(),&f.identities[0].1,"POST","/tasks",Some(&root_key),body()).await.0,StatusCode::SERVICE_UNAVAILABLE);
    let (_,root)=call(app.clone(),&f.identities[0].1,"GET",&format!("/tasks/by-request/{root_key}"),None,serde_json::Value::Null).await;let id=root["id"].as_str().unwrap();let path=format!("/tasks/{id}");
    assert_eq!(call(app.clone(),&f.identities[0].1,"PATCH",&path,None,serde_json::json!({"action":"stop"})).await.1["status"],"cancelled");
    sqlx::raw_sql("DROP TRIGGER test_assistant_claim_failure ON tenant_workflow_receipts; DROP FUNCTION test_assistant_claim_failure();").execute(&f.admin).await.unwrap();
    let key=Uuid::new_v4().to_string();let resume=serde_json::json!({"action":"resume","sourceReceiptId":id});
    let (first,second)=tokio::join!(call(app.clone(),&f.identities[0].1,"PATCH",&path,Some(&key),resume.clone()),call(app.clone(),&f.identities[0].1,"PATCH",&path,Some(&key),resume));
    assert_eq!(first.0,StatusCode::OK,"{}",first.1);assert_eq!(second.0,StatusCode::OK,"{}",second.1);
    execution.wait_for_workers().await;
    let (_,read)=call(app.clone(),&f.identities[0].1,"GET",&path,None,serde_json::Value::Null).await;
    assert_eq!(read["status"],"completed");assert_eq!(calls.load(Ordering::SeqCst),1);assert!(read["execution"]["output"].is_string());
    drop(app);drop(execution);f.close().await;
}
