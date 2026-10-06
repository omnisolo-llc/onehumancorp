//! Mandatory owned PostgreSQL verification. Never silently skips missing PG.
use super::*;
use serde_json::json;
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
use std::time::Duration;
struct Provider { calls:AtomicUsize, entered:tokio::sync::Notify, release:tokio::sync::Notify, paused:bool }
#[async_trait::async_trait]
impl DeliveryProvider for Provider {
    async fn send(&self,_:&Binding,_:&str,body:&str,_:&Credential)->Result<String,SendFailure> {
        assert_eq!(body,"Exact approved reply");self.calls.fetch_add(1,Ordering::SeqCst);self.entered.notify_one();if self.paused {self.release.notified().await;}Ok("wamid.fixture".into())
    }
}
struct Fixture { admin:sqlx::PgPool,pool:sqlx::PgPool,store:Store,schema:String,role:String,application:String }
impl Fixture {
    async fn new()->Self {
        let raw=std::env::var("OHC_DEPARTMENT_TEST_DATABASE_URL").expect("Owned PostgreSQL fixture is required; no successful skip");
        let url=url::Url::parse(&raw).unwrap();assert!(matches!(url.scheme(),"postgres"|"postgresql")&&matches!(url.host_str(),Some("127.0.0.1"|"localhost"|"::1"|"[::1]"))&&url.path()=="/ohc_department_test"&&url.query().is_none()&&url.fragment().is_none());
        let suffix=uuid::Uuid::new_v4().simple().to_string();let schema=format!("department_{suffix}");let role=format!("department_role_{suffix}");let application=format!("department_app_{suffix}");let password=uuid::Uuid::new_v4().to_string();
        let options:sqlx::postgres::PgConnectOptions=raw.parse().unwrap();let options=options.options([("search_path",schema.as_str())]);
        let admin=sqlx::postgres::PgPoolOptions::new().max_connections(5).connect_with(options.clone()).await.unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}")).execute(&admin).await.unwrap();
        sqlx::raw_sql("CREATE TABLE agent_feed_decisions(tenant_id TEXT,action_id TEXT,decision_state TEXT,dispatch_status TEXT,job_id TEXT,dispatch_payload JSONB);CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,proposed_action JSONB,lifecycle_state TEXT);CREATE TABLE agent_action_requests(id TEXT,tenant_id TEXT,action_type TEXT,status TEXT,department_type TEXT,description TEXT);CREATE TABLE inbox_messages(id TEXT,tenant_id TEXT,source TEXT,sender_id TEXT,status TEXT,draft_reply TEXT);CREATE TABLE omni_inbox_messages(id TEXT,tenant_id TEXT,source TEXT,sender_id TEXT,status TEXT,draft_reply TEXT);CREATE TABLE integration_credentials(id TEXT,tenant_id TEXT,integration_id TEXT,bot_token TEXT,api_token TEXT,from_phone TEXT);").execute(&admin).await.unwrap();
        sqlx::raw_sql(include_str!("../../src/server/migrations/1044_department_message_delivery_receipts.sql")).execute(&admin).await.unwrap();
        for table in ["agent_feed_items","agent_action_requests","inbox_messages","omni_inbox_messages","integration_credentials"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY;ALTER TABLE {table} FORCE ROW LEVEL SECURITY;CREATE POLICY tenant_scope ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap();
        }
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}';GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let pool=sqlx::postgres::PgPoolOptions::new().max_connections(5).connect_with(options.username(&role).password(&password).application_name(&application)).await.unwrap();
        let flags:(bool,bool)=sqlx::query_as("SELECT rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();assert_eq!(flags,(false,false));
        sqlx::query("INSERT INTO inbox_messages VALUES('inbox-a','tenant-a','whatsapp','14155550123','pending_approval',NULL)").execute(&admin).await.unwrap();
        sqlx::query("INSERT INTO integration_credentials VALUES('credential-a','tenant-a','whatsapp_cloud_api','','fixture-token','14155550000')").execute(&admin).await.unwrap();
        let store=Store::Postgres(pool.clone());let payload=prepare(&store,"tenant-a",json!({"feature_type":"ambassador_reply","inbox_message_id":"inbox-a","generated_response":"Exact approved reply"})).await.unwrap();
        sqlx::query("INSERT INTO agent_feed_items VALUES('action-a','tenant-a','customer_success',$1,'APPROVED')").bind(sqlx::types::Json(payload)).execute(&admin).await.unwrap();
        record_pending_review(&store,"tenant-a","action-a").await.unwrap();
        Self{admin,pool,store,schema,role,application}
    }
    fn provider(paused:bool)->Arc<Provider> {Arc::new(Provider{calls:AtomicUsize::new(0),entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new(),paused})}
    async fn wait_for_claim_lock(&self) {
        tokio::time::timeout(Duration::from_secs(3),async {
            loop {let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE application_name=$1 AND wait_event_type='Lock')").bind(&self.application).fetch_one(&self.admin).await.unwrap();if waiting {break;}tokio::time::sleep(Duration::from_millis(10)).await;}
        }).await.expect("dispatch must reach PostgreSQL row lock");
    }
    async fn finish(self) {self.pool.close().await;sqlx::query(&format!("DROP SCHEMA {} CASCADE",self.schema)).execute(&self.admin).await.unwrap();sqlx::query(&format!("DROP ROLE {}",self.role)).execute(&self.admin).await.unwrap();self.admin.close().await;}
}
#[tokio::test]
async fn pg_receipt_and_tenant_rls() {
    let f=Fixture::new().await;let p=Fixture::provider(false);assert_eq!(dispatch_with(&f.store,"tenant-a","action-a",p.as_ref()).await.unwrap().state,"accepted");
    assert_eq!(dispatch_with(&f.store,"tenant-a","action-a",p.as_ref()).await.unwrap().provider_message_id.as_deref(),Some("wamid.fixture"));assert_eq!(p.calls.load(Ordering::SeqCst),1);
    assert_eq!(sqlx::query_scalar::<_,String>("SELECT status FROM inbox_messages").fetch_one(&f.admin).await.unwrap(),"provider_accepted");
    let flags:(bool,bool)=sqlx::query_as("SELECT relrowsecurity,relforcerowsecurity FROM pg_class WHERE oid='department_message_dispatches'::regclass").fetch_one(&f.admin).await.unwrap();assert_eq!(flags,(true,true));
    let mut tx=f.pool.begin().await.unwrap();sqlx::query("SELECT set_config('app.current_tenant','tenant-b',true)").execute(&mut *tx).await.unwrap();
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT COUNT(*) FROM department_message_dispatches WHERE tenant_id='tenant-a'").fetch_one(&mut *tx).await.unwrap(),0);
    assert!(sqlx::query("INSERT INTO department_message_dispatches SELECT * FROM department_message_dispatches WHERE tenant_id='tenant-a'").execute(&mut *tx).await.unwrap().rows_affected()==0);tx.rollback().await.unwrap();
    assert!(dispatch_with(&f.store,"tenant-b","action-a",p.as_ref()).await.is_err());f.finish().await;
}
#[tokio::test]
async fn pg_approval_revocation_wins_before_claim() {
    let f=Fixture::new().await;let p=Fixture::provider(false);let mut revoker=f.admin.begin().await.unwrap();sqlx::query("UPDATE agent_feed_items SET lifecycle_state='PAUSED' WHERE id='action-a'").execute(&mut *revoker).await.unwrap();
    let (result,())=tokio::join!(dispatch_with(&f.store,"tenant-a","action-a",p.as_ref()),async{f.wait_for_claim_lock().await;revoker.commit().await.unwrap();});
    assert!(result.is_err());assert_eq!(p.calls.load(Ordering::SeqCst),0);f.finish().await;
}
#[tokio::test]
async fn pg_credential_revocation_wins_before_claim() {
    let f=Fixture::new().await;let p=Fixture::provider(false);let mut revoker=f.admin.begin().await.unwrap();sqlx::query("UPDATE integration_credentials SET api_token=NULL WHERE id='credential-a'").execute(&mut *revoker).await.unwrap();
    let (result,())=tokio::join!(dispatch_with(&f.store,"tenant-a","action-a",p.as_ref()),async{f.wait_for_claim_lock().await;revoker.commit().await.unwrap();});
    assert_eq!(result.unwrap().state,"blocked");assert_eq!(p.calls.load(Ordering::SeqCst),0);f.finish().await;
}
#[tokio::test]
async fn pg_concurrent_and_restarted_attempts_are_fenced() {
    let f=Fixture::new().await;let p=Fixture::provider(true);let store=f.store.clone();let worker_provider=p.clone();let worker=tokio::spawn(async move {dispatch_with(&store,"tenant-a","action-a",worker_provider.as_ref()).await});
    p.entered.notified().await;assert_eq!(dispatch_with(&f.store,"tenant-a","action-a",p.as_ref()).await.unwrap().state,"unknown");worker.abort();let _=worker.await;
    assert_eq!(dispatch_with(&Store::Postgres(f.pool.clone()),"tenant-a","action-a",p.as_ref()).await.unwrap().state,"unknown");assert_eq!(p.calls.load(Ordering::SeqCst),1);f.finish().await;
}
#[tokio::test]
async fn pg_modern_cancellation_and_snapshot_drift_cannot_be_bypassed() {
    for status in ["CANCELLED","RECONCILIATION_REQUIRED","PENDING","ATTEMPTING"] {
        let f=Fixture::new().await;let p=Fixture::provider(false);
        let canonical:sqlx::types::Json<Value>=sqlx::query_scalar("SELECT proposed_action FROM agent_feed_items").fetch_one(&f.admin).await.unwrap();
        let snapshot=json!({"action_id":"action-a","tenant_id":"tenant-a","payload":canonical.0});
        sqlx::query("INSERT INTO agent_feed_decisions VALUES('tenant-a','action-a','APPROVED',$1,'job-a',$2)").bind(status).bind(sqlx::types::Json(&snapshot)).execute(&f.admin).await.unwrap();
        // A department trigger cannot bypass the modern durable dispatch fence.
        assert!(dispatch_with(&f.store,"tenant-a","action-a",p.as_ref()).await.is_err());
        let mut forged=snapshot.clone();forged["payload"]["generated_response"]=json!("Changed after admission");
        assert!(dispatch_inner(&f.store,"tenant-a","action-a",p.as_ref(),Some(("job-a",&forged))).await.is_err());
        if status!="ATTEMPTING" {assert!(dispatch_inner(&f.store,"tenant-a","action-a",p.as_ref(),Some(("job-a",&snapshot))).await.is_err());}
        assert_eq!(p.calls.load(Ordering::SeqCst),0);
        if status=="ATTEMPTING" {assert_eq!(dispatch_inner(&f.store,"tenant-a","action-a",p.as_ref(),Some(("job-a",&snapshot))).await.unwrap().state,"accepted");}
        f.finish().await;
    }
}
