use super::*;
use serde_json::json;
use std::sync::atomic::{AtomicUsize,Ordering};

struct Provider { calls: AtomicUsize, result: Result<String,SendFailure> }
#[async_trait::async_trait]
impl DeliveryProvider for Provider {
    async fn send(&self, binding:&Binding, recipient:&str, body:&str, _credential:&Credential)->Result<String,SendFailure> {
        self.calls.fetch_add(1,Ordering::SeqCst);
        assert_eq!(binding.from,"14155550000");
        assert_eq!(recipient,"14155550123");
        assert_eq!(body,"Exact approved reply");
        self.result.clone()
    }
}
async fn fixture()->(Store,sqlx::SqlitePool,serde_json::Value) {
    let pool=sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
    sqlx::raw_sql("CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,proposed_action TEXT,lifecycle_state TEXT);CREATE TABLE agent_action_requests(id TEXT,tenant_id TEXT,action_type TEXT);CREATE TABLE inbox_messages(id TEXT,tenant_id TEXT,source TEXT,sender_id TEXT,status TEXT,draft_reply TEXT);CREATE TABLE omni_inbox_messages(id TEXT,tenant_id TEXT,source TEXT,sender_id TEXT,status TEXT,draft_reply TEXT);CREATE TABLE integration_credentials(id TEXT,tenant_id TEXT,integration_id TEXT,bot_token TEXT,api_token TEXT,from_phone TEXT);").execute(&pool).await.unwrap();
    sqlx::raw_sql(include_str!("../../src/server/persistence/department_message_delivery_sqlite.sql")).execute(&pool).await.unwrap();
    sqlx::raw_sql(include_str!("../../src/server/persistence/manual_inbox_requests_sqlite.sql")).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO inbox_messages VALUES('inbox-a','tenant-a','whatsapp','14155550123','pending_approval',NULL)").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO integration_credentials VALUES('credential-a','tenant-a','whatsapp_cloud_api','','fixture-token','14155550000')").execute(&pool).await.unwrap();
    let store=Store::Sqlite(pool.clone());
    let payload=prepare(&store,"tenant-a",json!({"feature_type":"ambassador_reply","inbox_message_id":"inbox-a","generated_response":"Exact approved reply","source":"attacker","sender_id":"attacker"})).await.unwrap();
    sqlx::query("INSERT INTO agent_feed_items VALUES('action-a','tenant-a','customer_success',$1,'APPROVED')").bind(payload.to_string()).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO agent_action_requests VALUES('action-a','tenant-a','HIGH')").execute(&pool).await.unwrap();
    (store,pool,payload)
}
fn provider(result:Result<String,SendFailure>)->Provider { Provider{calls:AtomicUsize::new(0),result} }
#[tokio::test]
async fn accepted_receipt_is_durable_and_repeat_never_resends() {
    let (store,pool,payload)=fixture().await;
    assert_eq!(payload["source"],"whatsapp");assert_eq!(payload["sender_id"],"14155550123");
    let p=provider(Ok("wamid.fixture".into()));
    let first=dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap();
    assert_eq!(first.state,"accepted");assert_eq!(first.provider_message_id.as_deref(),Some("wamid.fixture"));
    assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap(),first);
    assert_eq!(p.calls.load(Ordering::SeqCst),1);
    let status:String=sqlx::query_scalar("SELECT status FROM inbox_messages").fetch_one(&pool).await.unwrap();assert_eq!(status,"provider_accepted");
}
#[tokio::test]
async fn ambiguous_transport_is_persisted_unknown_and_never_retried() {
    let (store,pool,_)=fixture().await;let p=provider(Err(SendFailure::Unknown));
    assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"unknown");
    assert_eq!(dispatch_with(&Store::Sqlite(pool.clone()),"tenant-a","action-a",&p).await.unwrap().state,"unknown");
    assert_eq!(p.calls.load(Ordering::SeqCst),1);
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM inbox_messages").fetch_one(&pool).await.unwrap(),"delivery_unknown");
}
#[tokio::test]
async fn missing_credentials_and_definite_rejection_never_claim_success() {
    let (store,pool,_)=fixture().await;sqlx::query("DELETE FROM integration_credentials").execute(&pool).await.unwrap();
    let p=provider(Ok("wamid.fixture".into()));let receipt=dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap();
    assert_eq!(receipt.state,"blocked");assert_eq!(p.calls.load(Ordering::SeqCst),0);
    let (store,_,_)=fixture().await;let p=provider(Err(SendFailure::Rejected));
    assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"rejected");
}
#[tokio::test]
async fn unapproved_cross_tenant_and_auto_minted_approvals_cannot_send() {
    for change in ["UPDATE agent_feed_items SET lifecycle_state='PENDING_APPROVAL'","UPDATE agent_feed_items SET tenant_id='tenant-b'","UPDATE agent_action_requests SET action_type='LOW'"] {
        let (store,pool,_)=fixture().await;sqlx::query(change).execute(&pool).await.unwrap();let p=provider(Ok("wamid.fixture".into()));
        assert!(dispatch_with(&store,"tenant-a","action-a",&p).await.is_err());assert_eq!(p.calls.load(Ordering::SeqCst),0);
    }
}
#[tokio::test]
async fn changed_recipient_account_and_source_are_blocked_before_http() {
    for change in ["UPDATE inbox_messages SET sender_id='other'","UPDATE inbox_messages SET source='instagram'","UPDATE integration_credentials SET from_phone='other'"] {
        let (store,pool,_)=fixture().await;sqlx::query(change).execute(&pool).await.unwrap();let p=provider(Ok("wamid.fixture".into()));
        let result=dispatch_with(&store,"tenant-a","action-a",&p).await;
        assert!(result.is_err() || result.unwrap().state=="blocked");assert_eq!(p.calls.load(Ordering::SeqCst),0);
    }
}
#[tokio::test]
async fn another_action_for_same_inbox_cannot_resend() {
    let (store,pool,payload)=fixture().await;let p=provider(Ok("wamid.fixture".into()));dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap();
    sqlx::query("INSERT INTO agent_feed_items VALUES('action-b','tenant-a','customer_success',$1,'APPROVED')").bind(payload.to_string()).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO agent_action_requests VALUES('action-b','tenant-a','HIGH')").execute(&pool).await.unwrap();
    assert!(dispatch_with(&store,"tenant-a","action-b",&p).await.is_err());assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn changed_approved_text_after_attempt_needs_reconciliation() {
    let (store,pool,mut payload)=fixture().await;let p=provider(Ok("wamid.fixture".into()));dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap();
    payload["generated_response"]=json!("Changed text");sqlx::query("UPDATE agent_feed_items SET proposed_action=$1").bind(payload.to_string()).execute(&pool).await.unwrap();
    assert!(dispatch_with(&store,"tenant-a","action-a",&p).await.is_err());assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn legacy_success_without_receipt_cannot_be_resent() {
    for status in ["sent","auto_replied","replied","delivered","delivery_unknown"] {
        let (store,pool,_)=fixture().await;sqlx::query("UPDATE inbox_messages SET status=$1").bind(status).execute(&pool).await.unwrap();let p=provider(Ok("wamid.fixture".into()));
        assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"unknown");assert_eq!(p.calls.load(Ordering::SeqCst),0);
    }
}
#[tokio::test]
async fn failure_to_commit_attempt_fence_prevents_http() {
    let (store,pool,_)=fixture().await;sqlx::raw_sql("CREATE TRIGGER fail_claim BEFORE INSERT ON department_message_dispatches BEGIN SELECT RAISE(ABORT,'fixture write failure'); END;").execute(&pool).await.unwrap();let p=provider(Ok("wamid.fixture".into()));
    assert!(dispatch_with(&store,"tenant-a","action-a",&p).await.is_err());assert_eq!(p.calls.load(Ordering::SeqCst),0);
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM inbox_messages").fetch_one(&pool).await.unwrap(),"pending_approval");
}
struct LostCommit { pool:sqlx::SqlitePool,calls:AtomicUsize }
#[async_trait::async_trait]
impl DeliveryProvider for LostCommit {
    async fn send(&self,_:&Binding,_:&str,_:&str,_:&Credential)->Result<String,SendFailure> {
        self.calls.fetch_add(1,Ordering::SeqCst);
        sqlx::raw_sql("CREATE TRIGGER fail_receipt BEFORE UPDATE OF provider_message_id ON department_message_dispatches BEGIN SELECT RAISE(ABORT,'fixture receipt failure'); END;").execute(&self.pool).await.unwrap();
        Ok("wamid.fixture".into())
    }
}
#[tokio::test]
async fn acceptance_followed_by_storage_failure_stays_unknown_after_restart() {
    let (store,pool,_)=fixture().await;let p=LostCommit{pool:pool.clone(),calls:AtomicUsize::new(0)};
    assert!(dispatch_with(&store,"tenant-a","action-a",&p).await.is_err());
    assert_eq!(dispatch_with(&Store::Sqlite(pool.clone()),"tenant-a","action-a",&p).await.unwrap().state,"unknown");assert_eq!(p.calls.load(Ordering::SeqCst),1);
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM inbox_messages").fetch_one(&pool).await.unwrap(),"delivery_unknown");
}
struct PausedProvider { entered:tokio::sync::Notify,release:tokio::sync::Notify,calls:AtomicUsize }
#[async_trait::async_trait]
impl DeliveryProvider for PausedProvider {
    async fn send(&self,_:&Binding,_:&str,_:&str,_:&Credential)->Result<String,SendFailure> {
        self.calls.fetch_add(1,Ordering::SeqCst);self.entered.notify_one();self.release.notified().await;Ok("wamid.fixture".into())
    }
}
#[tokio::test]
async fn concurrent_dispatch_and_cancelled_worker_cannot_resend() {
    let (store,pool,_)=fixture().await;let p=std::sync::Arc::new(PausedProvider{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
    let worker_store=store.clone();let worker_provider=p.clone();let worker=tokio::spawn(async move {dispatch_with(&worker_store,"tenant-a","action-a",worker_provider.as_ref()).await});
    p.entered.notified().await;
    assert_eq!(dispatch_with(&store,"tenant-a","action-a",p.as_ref()).await.unwrap().state,"unknown");
    worker.abort();let _=worker.await;
    assert_eq!(dispatch_with(&Store::Sqlite(pool),"tenant-a","action-a",p.as_ref()).await.unwrap().state,"unknown");assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn empty_provider_receipt_never_becomes_acceptance() {
    let (store,_,_)=fixture().await;let p=provider(Ok(String::new()));assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"unknown");
}
#[tokio::test]
async fn any_mirror_with_historical_outcome_prevents_a_new_send() {
    for status in ["sent","auto_replied","delivery_unknown","provider_accepted"] {
        let (store,pool,_)=fixture().await;
        sqlx::query("INSERT INTO omni_inbox_messages VALUES('inbox-a','tenant-a','whatsapp','14155550123',$1,NULL)").bind(status).execute(&pool).await.unwrap();
        let p=provider(Ok("wamid.fixture".into()));assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"unknown");assert_eq!(p.calls.load(Ordering::SeqCst),0);
    }
}
#[tokio::test]
async fn fresh_review_can_recover_proven_no_effect_but_original_action_never_retries() {
    for blocked in [true,false] {
        let (store,pool,mut payload)=fixture().await;
        if blocked {sqlx::query("UPDATE integration_credentials SET api_token=NULL").execute(&pool).await.unwrap();}
        let first=provider(Err(SendFailure::Rejected));let receipt=dispatch_with(&store,"tenant-a","action-a",&first).await.unwrap();assert_eq!(receipt.state,if blocked {"blocked"} else {"rejected"});
        sqlx::query("UPDATE integration_credentials SET api_token='restored-fixture-token'").execute(&pool).await.unwrap();
        payload=prepare(&store,"tenant-a",payload).await.unwrap();
        sqlx::query("INSERT INTO agent_feed_items VALUES('action-b','tenant-a','customer_success',$1,'APPROVED')").bind(payload.to_string()).execute(&pool).await.unwrap();sqlx::query("INSERT INTO agent_action_requests VALUES('action-b','tenant-a','HIGH')").execute(&pool).await.unwrap();
        let recovered=provider(Ok("wamid.fixture".into()));assert_eq!(dispatch_with(&store,"tenant-a","action-a",&recovered).await.unwrap(),receipt);
        assert_eq!(dispatch_with(&store,"tenant-a","action-b",&recovered).await.unwrap().state,"accepted");assert_eq!(recovered.calls.load(Ordering::SeqCst),1);
    }
}
#[tokio::test]
async fn resolved_or_dismissed_mirror_invalidates_stale_reply() {
    for status in ["resolved","dismissed","closed","paused","cancelled","canceled"] {
        let (store,pool,_)=fixture().await;
        sqlx::query("INSERT INTO omni_inbox_messages VALUES('inbox-a','tenant-a','whatsapp','14155550123',$1,NULL)").bind(status).execute(&pool).await.unwrap();
        let p=provider(Ok("wamid.fixture".into()));assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"blocked");assert_eq!(p.calls.load(Ordering::SeqCst),0);
        assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM omni_inbox_messages").fetch_one(&pool).await.unwrap(),status);
    }
}
struct BsuidProvider { calls:AtomicUsize }
#[async_trait::async_trait]
impl DeliveryProvider for BsuidProvider {
    async fn send(&self,binding:&Binding,recipient:&str,_:&str,_:&Credential)->Result<String,SendFailure> {
        assert_eq!(binding.integration_id,"whatsapp_cloud_api");assert_eq!(recipient,"BR.ENT.123456789");self.calls.fetch_add(1,Ordering::SeqCst);Ok("wamid.fixture".into())
    }
}
#[tokio::test]
async fn whatsapp_cloud_preserves_existing_business_scoped_recipients() {
    let (store,pool,payload)=fixture().await;sqlx::query("UPDATE inbox_messages SET sender_id='BR.ENT.123456789'").execute(&pool).await.unwrap();
    let payload=prepare(&store,"tenant-a",payload).await.unwrap();sqlx::query("UPDATE agent_feed_items SET proposed_action=$1").bind(payload.to_string()).execute(&pool).await.unwrap();
    let p=BsuidProvider{calls:AtomicUsize::new(0)};assert_eq!(dispatch_with(&store,"tenant-a","action-a",&p).await.unwrap().state,"accepted");assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
