use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
struct Fake { calls: AtomicUsize, result: Result<String, SendFailure> }
#[async_trait::async_trait]
impl DeliveryProvider for Fake {
    async fn send(&self, binding: &Binding, recipient: &str, body: &str, _: &Credential) -> Result<String, SendFailure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(binding.credential_id, "credential-a");
        assert_eq!(recipient, "14155550123");
        assert_eq!(body, "Exact manual reply");
        self.result.clone()
    }
}
fn provider(result: Result<String, SendFailure>) -> Fake { Fake { calls: AtomicUsize::new(0), result } }
fn action(id: Option<&str>, prepare_only: bool) -> ManualAction {
    ManualAction { message_id: "inbox-a".into(), approved: true, edited_reply: Some("Exact manual reply".into()), request_id: id.map(str::to_owned), prepare_only }
}
async fn fixture() -> (Store, sqlx::SqlitePool) {
    let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
    sqlx::raw_sql("CREATE TABLE inbox_messages(id TEXT,tenant_id TEXT,source TEXT,sender_id TEXT,status TEXT,draft_reply TEXT); CREATE TABLE omni_inbox_messages(id TEXT,tenant_id TEXT,source TEXT,sender_id TEXT,status TEXT,draft_reply TEXT); CREATE TABLE integration_credentials(id TEXT,tenant_id TEXT,integration_id TEXT,bot_token TEXT,api_token TEXT,from_phone TEXT);").execute(&pool).await.unwrap();
    for ddl in [include_str!("../../src/server/persistence/department_message_delivery_sqlite.sql"), include_str!("../../src/server/persistence/manual_inbox_requests_sqlite.sql")] { sqlx::raw_sql(ddl).execute(&pool).await.unwrap(); }
    sqlx::query("INSERT INTO omni_inbox_messages VALUES('inbox-a','tenant-a','whatsapp','14155550123','unread',NULL)").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO integration_credentials VALUES('credential-a','tenant-a','whatsapp_cloud_api','','fixture-token','14155550000')").execute(&pool).await.unwrap();
    (Store::Sqlite(pool.clone()), pool)
}
#[tokio::test]
async fn manual_prepare_does_not_send_and_acceptance_is_not_delivery() {
    let (store, pool) = fixture().await; let p = provider(Ok("wamid.fixture".into()));
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap().state,"pending");
    assert_eq!(p.calls.load(Ordering::SeqCst),0);
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM omni_inbox_messages").fetch_one(&pool).await.unwrap(),"unread");
    let accepted=apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap();
    assert_eq!(accepted.state,"accepted"); assert_eq!(accepted.provider_message_id.as_deref(),Some("wamid.fixture"));
    assert_eq!(read(&store,"tenant-a","owner-a","inbox-a",None).await.unwrap().unwrap(),accepted);
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap(),accepted);
    assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn manual_legacy_client_has_stable_identity_without_agent_approval() {
    let (store, _) = fixture().await; let p = provider(Ok("wamid.fixture".into()));
    let first=apply_with(&store,"tenant-a","owner-a",&action(None,false),&p).await.unwrap();
    assert!(first.request_id.starts_with("legacy-"));
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(None,false),&p).await.unwrap(),first);
    assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn manual_request_identity_rejects_actor_body_and_inbox_changes() {
    let (store, _) = fixture().await; let p=provider(Ok("wamid.fixture".into()));
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
    assert!(apply_with(&store,"tenant-a","owner-b",&action(Some("request-a"),false),&p).await.is_err());
    let mut altered=action(Some("request-a"),false); altered.edited_reply=Some("Changed body".into());
    assert!(apply_with(&store,"tenant-a","owner-a",&altered,&p).await.is_err());
    altered.message_id="inbox-b".into();
    assert!(apply_with(&store,"tenant-a","owner-a",&altered,&p).await.is_err());
    assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
#[tokio::test]
async fn manual_retired_pending_identity_never_regains_ownership() {
    let (store, _) = fixture().await; let p=provider(Ok("wamid.fixture".into()));
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-b"),true),&p).await.unwrap();
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap().state,"retired");
    assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
#[tokio::test]
async fn manual_dismiss_retires_pending_and_blocks_stale_send() {
    let (store, pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
    let mut dismiss=action(None,false); dismiss.approved=false;
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&dismiss,&p).await.unwrap().state,"dismissed");
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap().state,"retired");
    assert!(apply_with(&store,"tenant-a","owner-a",&action(Some("new-request"),false),&p).await.is_err());
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM omni_inbox_messages").fetch_one(&pool).await.unwrap(),"dismissed");
    assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
#[tokio::test]
async fn manual_frozen_binding_rejects_recipient_source_account_and_revoked_credentials() {
    for mutation in ["UPDATE omni_inbox_messages SET sender_id='14155550999'","UPDATE omni_inbox_messages SET source='sms'","UPDATE integration_credentials SET from_phone='14155559999'","UPDATE integration_credentials SET api_token=NULL"] {
        let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
        apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
        sqlx::query(mutation).execute(&pool).await.unwrap();
        let result=apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await;
        assert!(result.is_err() || result.unwrap().state=="blocked"); assert_eq!(p.calls.load(Ordering::SeqCst),0);
    }
}
#[tokio::test]
async fn manual_unknown_survives_restart_and_cross_tab_new_id_cannot_replay() {
    let (store,pool)=fixture().await;let p=provider(Err(SendFailure::Unknown));
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap().state,"unknown");
    assert_eq!(apply_with(&Store::Sqlite(pool.clone()),"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap().state,"unknown");
    assert!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-b"),true),&p).await.is_err());
    assert_eq!(read(&store,"tenant-a","owner-a","inbox-a",None).await.unwrap().unwrap().draft_reply,"Exact manual reply");
    assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn manual_rejected_or_blocked_new_explicit_request_can_recover() {
    for blocked in [true,false] {
        let (store,pool)=fixture().await; let p=provider(Err(SendFailure::Rejected));
        if blocked { sqlx::query("UPDATE integration_credentials SET api_token=NULL").execute(&pool).await.unwrap(); }
        let first=apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap();
        assert_eq!(first.state,if blocked {"blocked"} else {"rejected"});
        sqlx::query("UPDATE integration_credentials SET api_token='fixture-restored'").execute(&pool).await.unwrap();
        let p=provider(Ok("wamid.fixture".into()));
        assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap(),first);
        assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-b"),false),&p).await.unwrap().state,"accepted");
        assert_eq!(p.calls.load(Ordering::SeqCst),1);
    }
}
#[tokio::test]
async fn manual_empty_acceptance_id_is_unknown() {
    let (store,_)=fixture().await; let p=provider(Ok(String::new()));
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(None,false),&p).await.unwrap().state,"unknown");
}
#[tokio::test]
async fn manual_failed_claim_commit_prevents_provider_http() {
    let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
    sqlx::raw_sql("CREATE TRIGGER reject_claim BEFORE INSERT ON department_message_dispatches BEGIN SELECT RAISE(ABORT,'fixture'); END;").execute(&pool).await.unwrap();
    assert!(apply_with(&store,"tenant-a","owner-a",&action(None,false),&p).await.is_err());assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
#[tokio::test]
async fn manual_cross_tenant_and_actor_readback_is_not_found() {
    let (store,_)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
    assert!(read(&store,"tenant-b","owner-a","inbox-a",None).await.unwrap().is_none());
    assert!(read(&store,"tenant-a","owner-b","inbox-a",None).await.unwrap().is_none());
    assert!(apply_with(&store,"tenant-b","owner-a",&action(None,false),&p).await.is_err());
    assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
#[tokio::test]
async fn manual_expired_prepare_cannot_send() {
    let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
    sqlx::query("UPDATE manual_inbox_requests SET expires_at=0").execute(&pool).await.unwrap();
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap().state,"retired");assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
struct Pause { entered:tokio::sync::Notify, release:tokio::sync::Notify, calls:AtomicUsize }
#[async_trait::async_trait]
impl DeliveryProvider for Pause {
    async fn send(&self,_:&Binding,_:&str,_:&str,_:&Credential)->Result<String,SendFailure> {
        self.calls.fetch_add(1,Ordering::SeqCst);self.entered.notify_one();self.release.notified().await;Ok("wamid.fixture".into())
    }
}
#[tokio::test]
async fn manual_cancelled_http_owner_and_different_tab_cannot_replay() {
    let (store,pool)=fixture().await;let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
    let child=store.clone();let cp=p.clone();let task=tokio::spawn(async move {apply_with(&child,"tenant-a","owner-a",&action(Some("request-a"),false),cp.as_ref()).await});
    p.entered.notified().await;
    assert!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-b"),false),p.as_ref()).await.is_err());
    task.abort();let _=task.await;
    assert_eq!(read(&Store::Sqlite(pool),"tenant-a","owner-a","inbox-a",None).await.unwrap().unwrap().state,"unknown");
    assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn manual_dismiss_during_http_keeps_dismissal_and_provider_truth() {
    let (store,pool)=fixture().await;let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
    let child=store.clone();let cp=p.clone();let task=tokio::spawn(async move {apply_with(&child,"tenant-a","owner-a",&action(Some("request-a"),false),cp.as_ref()).await});
    p.entered.notified().await;let mut dismiss=action(None,false);dismiss.approved=false;
    apply_with(&store,"tenant-a","owner-a",&dismiss,p.as_ref()).await.unwrap();p.release.notify_one();
    assert_eq!(task.await.unwrap().unwrap().state,"accepted");
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM omni_inbox_messages").fetch_one(&pool).await.unwrap(),"dismissed");
    let latest=read(&store,"tenant-a","owner-a","inbox-a",None).await.unwrap().unwrap();
    assert_eq!(latest.state,"dismissed");let earlier=latest.prior_send.unwrap();assert_eq!(earlier.request_id,"request-a");assert_eq!(earlier.state,"accepted");assert_eq!(earlier.draft_reply,"Exact manual reply");
}
struct LostReceipt { pool:sqlx::SqlitePool,calls:AtomicUsize }
#[async_trait::async_trait]
impl DeliveryProvider for LostReceipt {
    async fn send(&self,_:&Binding,_:&str,_:&str,_:&Credential)->Result<String,SendFailure> {
        self.calls.fetch_add(1,Ordering::SeqCst);
        sqlx::raw_sql("CREATE TRIGGER fail_receipt BEFORE UPDATE OF provider_message_id ON department_message_dispatches BEGIN SELECT RAISE(ABORT,'fixture'); END;").execute(&self.pool).await.unwrap();
        Ok("wamid.fixture".into())
    }
}
#[tokio::test]
async fn manual_lost_finalization_keeps_draft_and_unknown_claim() {
    let (store,pool)=fixture().await;let p=LostReceipt{pool:pool.clone(),calls:AtomicUsize::new(0)};
    assert!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.is_err());
    let saved=read(&store,"tenant-a","owner-a","inbox-a",None).await.unwrap().unwrap();assert_eq!(saved.state,"unknown");assert_eq!(saved.draft_reply,"Exact manual reply");
    assert_eq!(apply_with(&Store::Sqlite(pool),"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap(),saved);assert_eq!(p.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn manual_request_identity_is_database_immutable() {
    let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),true),&p).await.unwrap();
    for column in ["actor_id","inbox_message_id","recipient","source","body","provider_binding","payload_hash"] {
        assert!(sqlx::query(&format!("UPDATE manual_inbox_requests SET {column}='tampered'")).execute(&pool).await.is_err());
    }
}
#[tokio::test]
async fn manual_legacy_close_only_retires_without_sending() {
    let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));let mut close=action(None,false);close.edited_reply=None;
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&close,&p).await.unwrap().state,"resolved");assert_eq!(p.calls.load(Ordering::SeqCst),0);
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM omni_inbox_messages").fetch_one(&pool).await.unwrap(),"resolved");
}
struct OwnedMeta { client:crate::integrations::meta::client::RealMetaClient }
#[async_trait::async_trait]
impl DeliveryProvider for OwnedMeta {
    async fn send(&self,binding:&Binding,recipient:&str,body:&str,_:&Credential)->Result<String,SendFailure> {
        use crate::integrations::meta::client::{MetaClientWrapper,MetaSendError};
        self.client.send_message_receipt(&binding.source,Some(&binding.from),recipient,body).await.map(|receipt|receipt.message_id)
            .map_err(|error|match error {MetaSendError::InvalidConfiguration=>SendFailure::Blocked,MetaSendError::Rejected{..}=>SendFailure::Rejected,MetaSendError::UnknownOutcome{..}=>SendFailure::Unknown})
    }
}
#[tokio::test]
async fn manual_owned_http_validated_acceptance_and_malformed_rejection_unknown() {
    for (status,body,expected) in [
        (200,r#"{"messaging_product":"whatsapp","contacts":[{"input":"14155550123","wa_id":"14155550123"}],"messages":[{"id":"wamid.fixture"}]}"#,"accepted"),
        (200,"{}","unknown"),(400,"{}","rejected"),(500,"{}","unknown")
    ] {
        let (store,_)=fixture().await;
        let (client,worker)=crate::integrations::meta::client::receipt_tests::fixture(status,body);
        let p=OwnedMeta{client};let first=apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap();assert_eq!(first.state,expected);
        let request=worker.join().unwrap();assert!(request.starts_with("POST /v19.0/14155550000/messages "));
        let payload:Value=serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();assert_eq!(payload["to"],"14155550123");assert_eq!(payload["text"]["body"],"Exact manual reply");
        // Listener has closed: replay would fail or hang instead of reproducing receipt.
        assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap(),first);
    }
}
#[tokio::test]
async fn pg_manual_concurrent_legacy_retry_cross_tab_and_rls() {
    let f=super::super::postgres_tests::Fixture::new().await;
    let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
    let child=f.store.clone();let cp=p.clone();let task=tokio::spawn(async move {apply_with(&child,"tenant-a","owner-a",&action(None,false),cp.as_ref()).await});
    p.entered.notified().await;
    assert_eq!(apply_with(&f.store,"tenant-a","owner-a",&action(None,false),p.as_ref()).await.unwrap().state,"unknown");
    assert!(apply_with(&f.store,"tenant-a","owner-b",&action(Some("other-tab"),true),p.as_ref()).await.is_err());
    assert!(read(&f.store,"tenant-b","owner-a","inbox-a",None).await.unwrap().is_none());
    assert!(read(&f.store,"tenant-a","owner-b","inbox-a",None).await.unwrap().is_none());
    let mut tx=f.pool.begin().await.unwrap();sqlx::query("SELECT set_config('app.current_tenant','tenant-b',true)").execute(&mut *tx).await.unwrap();
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT COUNT(*) FROM manual_inbox_requests").fetch_one(&mut *tx).await.unwrap(),0);tx.rollback().await.unwrap();
    task.abort();let _=task.await;assert_eq!(p.calls.load(Ordering::SeqCst),1);
    assert_eq!(apply_with(&Store::Postgres(f.pool.clone()),"tenant-a","owner-a",&action(None,false),p.as_ref()).await.unwrap().state,"unknown");f.finish().await;
}
#[tokio::test]
async fn pg_manual_stale_prepare_and_dismissal_cannot_send() {
    let f=super::super::postgres_tests::Fixture::new().await;let p=provider(Ok("wamid.fixture".into()));
    apply_with(&f.store,"tenant-a","owner-a",&action(Some("old"),true),&p).await.unwrap();
    apply_with(&f.store,"tenant-a","owner-a",&action(Some("new"),true),&p).await.unwrap();
    assert_eq!(apply_with(&f.store,"tenant-a","owner-a",&action(Some("old"),false),&p).await.unwrap().state,"retired");
    let mut dismiss=action(None,false);dismiss.approved=false;apply_with(&f.store,"tenant-a","owner-a",&dismiss,&p).await.unwrap();
    assert_eq!(apply_with(&f.store,"tenant-a","owner-a",&action(Some("new"),false),&p).await.unwrap().state,"retired");
    assert_eq!(p.calls.load(Ordering::SeqCst),0);f.finish().await;
}
#[tokio::test]
async fn manual_intent_revision_advances_only_for_new_admitted_identity() {
    let (store,pool)=fixture().await;let p=provider(Err(SendFailure::Rejected));
    let draft=action(Some("request-a"),true);apply_with(&store,"tenant-a","owner-a",&draft,&p).await.unwrap();
    let revision=sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap();assert_eq!(revision,1);
    apply_with(&store,"tenant-a","owner-a",&draft,&p).await.unwrap();
    let mut tampered=draft.clone();tampered.edited_reply=Some("tamper".into());assert!(apply_with(&store,"tenant-a","owner-a",&tampered,&p).await.is_err());
    assert!(apply_with(&store,"tenant-a","other-owner",&draft,&p).await.is_err());
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),revision);
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap();
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),revision);
    apply_with(&store,"tenant-a","owner-a",&action(Some("request-b"),true),&p).await.unwrap();
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),revision+1);
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&action(Some("request-a"),false),&p).await.unwrap().state,"rejected");
    assert_eq!(read(&store,"tenant-a","owner-a","inbox-a",Some("request-b")).await.unwrap().unwrap().state,"pending");
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),revision+1);
}
#[tokio::test]
async fn manual_dismiss_receipt_is_durable_and_replay_does_not_retire_new_work() {
    let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));let mut dismiss=action(Some("dismiss-a"),false);dismiss.approved=false;
    let first=apply_with(&store,"tenant-a","owner-a",&dismiss,&p).await.unwrap();assert_eq!(first.state,"dismissed");
    assert_eq!(read(&store,"tenant-a","owner-a","inbox-a",Some("dismiss-a")).await.unwrap().unwrap(),first);
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&dismiss,&p).await.unwrap(),first);
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),1);
    // A subsequent explicit reopen/new pending intent must survive old close replay.
    sqlx::query("UPDATE omni_inbox_messages SET status='unread'").execute(&pool).await.unwrap();
    apply_with(&store,"tenant-a","owner-a",&action(Some("new-pending"),true),&p).await.unwrap();
    assert_eq!(apply_with(&store,"tenant-a","owner-a",&dismiss,&p).await.unwrap(),first);
    assert_eq!(read(&store,"tenant-a","owner-a","inbox-a",Some("new-pending")).await.unwrap().unwrap().state,"pending");
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM omni_inbox_messages").fetch_one(&pool).await.unwrap(),"unread");
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT revision FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),2);
}
#[tokio::test]
async fn pg_manual_rejection_retires_old_department_approval_but_new_review_recovers() {
    let f=super::super::postgres_tests::Fixture::new().await;let p=provider(Err(SendFailure::Rejected));
    assert_eq!(apply_with(&f.store,"tenant-a","owner-a",&action(Some("manual-a"),false),&p).await.unwrap().state,"rejected");
    let department=super::super::postgres_tests::Fixture::provider(false);
    assert_eq!(super::super::dispatch_with(&f.store,"tenant-a","action-a",department.as_ref()).await.unwrap().state,"blocked");
    let projection:(String,String)=sqlx::query_as("SELECT status,draft_reply FROM inbox_messages").fetch_one(&f.admin).await.unwrap();
    assert_eq!(projection,("delivery_failed".into(),"Exact manual reply".into()),"retired department intent must not replace newer manual draft or status");
    let prepared=super::super::prepare(&f.store,"tenant-a",serde_json::json!({"feature_type":"ambassador_reply","inbox_message_id":"inbox-a","generated_response":"Exact approved reply"})).await.unwrap();
    sqlx::query("INSERT INTO agent_feed_items VALUES('action-new','tenant-a','customer_success',$1,'APPROVED')").bind(sqlx::types::Json(prepared)).execute(&f.admin).await.unwrap();
    super::super::record_pending_review(&f.store,"tenant-a","action-new").await.unwrap();
    assert_eq!(super::super::dispatch_with(&f.store,"tenant-a","action-new",department.as_ref()).await.unwrap().state,"accepted");f.finish().await;
}
#[tokio::test]
async fn pg_manual_department_concurrent_attempts_share_one_effect_fence() {
    let f=super::super::postgres_tests::Fixture::new().await;let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
    let child=f.store.clone();let cp=p.clone();let task=tokio::spawn(async move {apply_with(&child,"tenant-a","owner-a",&action(Some("manual-a"),false),cp.as_ref()).await});
    p.entered.notified().await;let department=super::super::postgres_tests::Fixture::provider(false);
    let result=super::super::dispatch_with(&f.store,"tenant-a","action-a",department.as_ref()).await;
    assert!(result.is_err() || result.unwrap().state=="blocked");
    p.release.notify_one();assert_eq!(task.await.unwrap().unwrap().state,"accepted");
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT COUNT(*) FROM department_message_dispatches WHERE state='accepted'").fetch_one(&f.admin).await.unwrap(),1);f.finish().await;
}
#[tokio::test]
async fn manual_signed_identity_is_required_before_admission() {
    let (store,pool)=fixture().await;let p=provider(Ok("wamid.fixture".into()));
    for (tenant,actor) in [("tenant-a",""),("system","owner"),("","owner")] {
        assert!(apply_with(&store,tenant,actor,&action(None,false),&p).await.is_err());
    }
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT COUNT(*) FROM manual_inbox_intents").fetch_one(&pool).await.unwrap(),0);
    assert_eq!(p.calls.load(Ordering::SeqCst),0);
}
#[tokio::test]
async fn pg_department_inflight_manual_dismiss_preserves_visibility_and_receipt() {
    let f=super::super::postgres_tests::Fixture::new().await;let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
    let child=f.store.clone();let cp=p.clone();let task=tokio::spawn(async move {super::super::dispatch_with(&child,"tenant-a","action-a",cp.as_ref()).await});
    p.entered.notified().await;let mut dismiss=action(Some("dismiss-a"),false);dismiss.approved=false;
    apply_with(&f.store,"tenant-a","owner-a",&dismiss,p.as_ref()).await.unwrap();p.release.notify_one();
    assert_eq!(task.await.unwrap().unwrap().state,"accepted");
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM inbox_messages").fetch_one(&f.admin).await.unwrap(),"dismissed");
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT state FROM department_message_dispatches WHERE action_id='action-a'").fetch_one(&f.admin).await.unwrap(),"accepted");f.finish().await;
}
#[tokio::test]
async fn manual_one_mirror_feed_dismissal_survives_late_provider_acceptance() {
    for state in ["dismissed","paused"] {
        let (store,pool)=fixture().await;
        sqlx::query("INSERT INTO inbox_messages SELECT * FROM omni_inbox_messages").execute(&pool).await.unwrap();
        sqlx::query("ALTER TABLE omni_inbox_messages ADD COLUMN updated_at TEXT").execute(&pool).await.unwrap();
        let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
        let child=store.clone();let cp=p.clone();let task=tokio::spawn(async move {apply_with(&child,"tenant-a","owner-a",&action(Some("request-a"),false),cp.as_ref()).await});
        p.entered.notified().await;
        // Exact mounted agent-feed sync_legacy statement changes only omni.
        sqlx::query("UPDATE omni_inbox_messages SET status=$1,draft_reply=COALESCE($2,draft_reply),updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4")
            .bind(state).bind("Newer owner draft").bind("tenant-a").bind("inbox-a").execute(&pool).await.unwrap();
        p.release.notify_one();assert_eq!(task.await.unwrap().unwrap().state,"accepted");
        let omni:(String,String)=sqlx::query_as("SELECT status,draft_reply FROM omni_inbox_messages").fetch_one(&pool).await.unwrap();assert_eq!(omni,(state.into(),"Newer owner draft".into()));
        let inbox:(String,String)=sqlx::query_as("SELECT status,draft_reply FROM inbox_messages").fetch_one(&pool).await.unwrap();assert_eq!(inbox,("delivery_unknown".into(),"Exact manual reply".into()));
        assert_eq!(read(&store,"tenant-a","owner-a","inbox-a",None).await.unwrap().unwrap().state,"accepted");
    }
}
#[tokio::test]
async fn pg_department_one_mirror_feed_dismissal_preserves_projection_and_acceptance() {
    for state in ["dismissed","paused"] {
        let f=super::super::postgres_tests::Fixture::new().await;
        sqlx::query("INSERT INTO omni_inbox_messages SELECT * FROM inbox_messages").execute(&f.admin).await.unwrap();
        sqlx::query("ALTER TABLE omni_inbox_messages ADD COLUMN updated_at TIMESTAMPTZ").execute(&f.admin).await.unwrap();
        let p=std::sync::Arc::new(Pause{entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),calls:AtomicUsize::new(0)});
        let child=f.store.clone();let cp=p.clone();let task=tokio::spawn(async move {super::super::dispatch_with(&child,"tenant-a","action-a",cp.as_ref()).await});
        p.entered.notified().await;
        sqlx::query("UPDATE omni_inbox_messages SET status=$1,draft_reply=COALESCE($2,draft_reply),updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4")
            .bind(state).bind("Newer owner draft").bind("tenant-a").bind("inbox-a").execute(&f.admin).await.unwrap();
        p.release.notify_one();assert_eq!(task.await.unwrap().unwrap().state,"accepted");
        let omni:(String,String)=sqlx::query_as("SELECT status,draft_reply FROM omni_inbox_messages").fetch_one(&f.admin).await.unwrap();assert_eq!(omni,(state.into(),"Newer owner draft".into()));
        let inbox:(String,String)=sqlx::query_as("SELECT status,draft_reply FROM inbox_messages").fetch_one(&f.admin).await.unwrap();assert_eq!(inbox,("delivery_unknown".into(),"Exact approved reply".into()));
        assert_eq!(sqlx::query_scalar::<_,String>("SELECT state FROM department_message_dispatches").fetch_one(&f.admin).await.unwrap(),"accepted");f.finish().await;
    }
}
