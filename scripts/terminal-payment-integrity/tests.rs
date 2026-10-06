use crate::protocol::{self, CaptureInput, IntentInput, Provider};
use server_integrations_stripe::terminal::TerminalIntentReceipt;
use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};

pub(crate) struct Fixture { pub(crate) pool:sqlx::PgPool, admin:sqlx::PgPool, schema:String }
impl Fixture {
    pub(crate) async fn new()->Self {
        let url=std::env::var("OHC_TERMINAL_TEST_DATABASE_URL").expect("owned disposable PostgreSQL is required; no skipped tests");
        let admin=sqlx::PgPool::connect(&url).await.expect("connect owned test database");
        let schema=format!("terminal_integrity_{}",uuid::Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}")).execute(&admin).await.unwrap();
        let search=schema.clone();
        let pool=sqlx::postgres::PgPoolOptions::new().max_connections(6).after_connect(move|c,_|{
            let query=format!("SET search_path TO {search}");
            Box::pin(async move{sqlx::query(&query).execute(c).await?;Ok(())})
        }).connect(&url).await.unwrap();
        // Exact production DDL, including the ledger's identity and RLS schema.
        sqlx::raw_sql(include_str!("../../src/server/migrations/142_omni_payment_ledger.sql")).execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!("../../src/server/migrations/1040_terminal_payment_identity.sql")).execute(&pool).await.unwrap();
        Self {pool,admin,schema}
    }
    pub(crate) async fn done(self){self.pool.close().await;sqlx::query(&format!("DROP SCHEMA {} CASCADE",self.schema)).execute(&self.admin).await.unwrap();self.admin.close().await;}
}
#[derive(Default)]
struct Spy {
    creates:AtomicUsize, reads:AtomicUsize, captures:AtomicUsize,
    receipt:Mutex<Option<TerminalIntentReceipt>>, fail_capture:bool,
}
impl Spy {
    fn authorize(&self){let mut guard=self.receipt.lock().unwrap();let receipt=guard.as_mut().unwrap();receipt.status="requires_capture".into();receipt.amount_capturable=receipt.amount;}
    fn reset_capture_counts(&self){self.reads.store(0,Ordering::SeqCst);self.captures.store(0,Ordering::SeqCst);}
    fn no_capture_effects(&self){assert_eq!(self.reads.load(Ordering::SeqCst),0);assert_eq!(self.captures.load(Ordering::SeqCst),0);}
}
#[async_trait::async_trait]
impl Provider for Spy {
    async fn create(&self,tenant:&str,operation:&str,amount:i64,currency:&str)->Result<TerminalIntentReceipt,String>{
        let n=self.creates.fetch_add(1,Ordering::SeqCst);
        let id=format!("pi_fixture{n}");
        let receipt=TerminalIntentReceipt {id:id.clone(),amount,amount_received:0,amount_capturable:0,currency:currency.into(),status:"requires_payment_method".into(),metadata:std::collections::HashMap::from([("tenant_id".into(),tenant.into()),("idempotency_key".into(),operation.into())]),client_secret:Some(format!("{id}_secret_fixture"))};
        *self.receipt.lock().unwrap()=Some(receipt.clone());Ok(receipt)
    }
    async fn retrieve(&self,_:&str)->Result<TerminalIntentReceipt,String>{self.reads.fetch_add(1,Ordering::SeqCst);Ok(self.receipt.lock().unwrap().clone().unwrap())}
    async fn capture(&self,_:&str,amount_cents:i64)->Result<TerminalIntentReceipt,String>{
        self.captures.fetch_add(1,Ordering::SeqCst);
        if self.fail_capture{return Err("Response lost after external effect".into());}
        let mut receipt=self.receipt.lock().unwrap().clone().unwrap();receipt.status="succeeded".into();receipt.amount_received=amount_cents;receipt.amount_capturable=0;Ok(receipt)
    }
}
fn input()->IntentInput{IntentInput{operation_id:"operation_one".into(),amount_cents:2500,currency:"usd".into()}}
fn capture(id:&str)->CaptureInput{CaptureInput{payment_intent_id:id.into(),amount_cents:Some(2500)}}
async fn prepared(f:&Fixture,p:&Spy)->String{let r=protocol::create(&f.pool,"tenant_a",input(),"provider_a",p).await.unwrap();p.authorize();r.payment_intent_id}

#[tokio::test] async fn foreign_missing_and_malformed_ids_have_zero_provider_calls(){
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    for (tenant,id) in [("tenant_b",id.as_str()),("tenant_a","pi_missing"),("tenant_a","../../capture")] {
        assert_eq!(protocol::require_owned(&f.pool,tenant,id).await.unwrap_err().0,axum::http::StatusCode::NOT_FOUND);
        assert_eq!(protocol::capture(&f.pool,tenant,capture(id),"provider_a",&p).await.unwrap_err().0,axum::http::StatusCode::NOT_FOUND);p.no_capture_effects();
    }f.done().await;
}
#[tokio::test] async fn capture_uses_owned_saved_binding_and_replays_only_persisted_success(){
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    let receipt=protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p).await.unwrap();assert!(receipt.success);assert_eq!(receipt.amount_cents,2500);
    assert_eq!(p.captures.load(Ordering::SeqCst),1);p.reset_capture_counts();
    let replay=protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p).await.unwrap();assert_eq!(replay.operation_id,"operation_one");p.no_capture_effects();
    let stored:serde_json::Value=sqlx::query_scalar("SELECT provider_receipt FROM terminal_payment_operations").fetch_one(&f.pool).await.unwrap();assert!(stored.get("client_secret").is_none());
    let status:String=sqlx::query_scalar("SELECT status FROM payment_intents").fetch_one(&f.pool).await.unwrap();assert_eq!(status,"pending","ledger status stays webhook-owned");f.done().await;
}
#[tokio::test] async fn changed_amount_or_connection_cannot_rebind_a_payment(){
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    let mut changed=capture(&id);changed.amount_cents=Some(1);
    assert!(protocol::capture(&f.pool,"tenant_a",changed,"provider_a",&p).await.is_err());
    assert!(protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_b",&p).await.is_err());p.no_capture_effects();f.done().await;
}
#[tokio::test] async fn provider_amount_mismatch_is_found_before_capture(){
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    p.receipt.lock().unwrap().as_mut().unwrap().amount=1;
    assert!(protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p).await.is_err());assert_eq!(p.captures.load(Ordering::SeqCst),0);f.done().await;
}
#[tokio::test] async fn unknown_capture_is_durable_and_never_automatically_retried(){
    let f=Fixture::new().await;let p=Spy{fail_capture:true,..Default::default()};let id=prepared(&f,&p).await;
    assert!(protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p).await.is_err());assert_eq!(p.captures.load(Ordering::SeqCst),1);
    let state:String=sqlx::query_scalar("SELECT state FROM terminal_payment_operations").fetch_one(&f.pool).await.unwrap();assert_eq!(state,"reconciliation_required");
    p.reset_capture_counts();assert!(protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p).await.is_err());p.no_capture_effects();f.done().await;
}
#[tokio::test] async fn duplicate_create_does_not_issue_another_provider_operation(){
    let f=Fixture::new().await;let p=Spy::default();prepared(&f,&p).await;
    assert!(protocol::create(&f.pool,"tenant_a",input(),"provider_a",&p).await.is_err());let mut changed=input();changed.amount_cents=999;
    assert!(protocol::create(&f.pool,"tenant_a",changed,"provider_a",&p).await.is_err());assert_eq!(p.creates.load(Ordering::SeqCst),1);f.done().await;
}
#[tokio::test] async fn concurrent_capture_has_only_one_provider_write(){
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    let (a,b)=tokio::join!(protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p),protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p));
    assert!(a.is_ok()||b.is_ok());assert_eq!(p.captures.load(Ordering::SeqCst),1);f.done().await;
}
#[tokio::test] async fn old_unbound_payment_record_cannot_be_captured(){
    let f=Fixture::new().await;let p=Spy::default();
    sqlx::query("INSERT INTO payment_intents (tenant_id,payment_id,idempotency_key,amount,currency,source,stripe_payment_intent_id) VALUES ('tenant_a','old','old_key',25,'usd','in_person','pi_legacy')").execute(&f.pool).await.unwrap();
    assert_eq!(protocol::capture(&f.pool,"tenant_a",capture("pi_legacy"),"provider_a",&p).await.unwrap_err().0,axum::http::StatusCode::NOT_FOUND);p.no_capture_effects();f.done().await;
}
#[tokio::test] async fn persistence_failure_after_provider_create_never_returns_a_usable_intent(){
    let f=Fixture::new().await;let p=Spy::default();
    sqlx::raw_sql("CREATE FUNCTION reject_payment_insert() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'fixture persistence failure'; END $$; CREATE TRIGGER reject_payment BEFORE INSERT ON payment_intents FOR EACH ROW EXECUTE FUNCTION reject_payment_insert();").execute(&f.pool).await.unwrap();
    assert!(protocol::create(&f.pool,"tenant_a",input(),"provider_a",&p).await.is_err());assert_eq!(p.creates.load(Ordering::SeqCst),1);
    let state:String=sqlx::query_scalar("SELECT state FROM terminal_payment_operations").fetch_one(&f.pool).await.unwrap();assert_eq!(state,"reconciliation_required");
    assert!(protocol::create(&f.pool,"tenant_a",input(),"provider_a",&p).await.is_err());assert_eq!(p.creates.load(Ordering::SeqCst),1);f.done().await;
}
#[tokio::test] async fn closed_database_prevents_all_provider_calls(){
    let f=Fixture::new().await;let p=Spy::default();f.pool.close().await;
    assert!(protocol::create(&f.pool,"tenant_a",input(),"provider_a",&p).await.is_err());assert!(protocol::capture(&f.pool,"tenant_a",capture("pi_missing"),"provider_a",&p).await.is_err());assert_eq!(p.creates.load(Ordering::SeqCst),0);p.no_capture_effects();f.done().await;
}
#[test] fn idempotency_namespace_is_tenant_bound(){assert_ne!(protocol::provider_operation("tenant_a","same"),protocol::provider_operation("tenant_b","same"));}

#[tokio::test] async fn insufficient_capturable_amount_prevents_capture(){
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    p.receipt.lock().unwrap().as_mut().unwrap().amount_capturable=2400;
    assert!(protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&p).await.is_err());
    assert_eq!(p.captures.load(Ordering::SeqCst),0);f.done().await;
}

struct ProviderBaseGuard(Option<std::ffi::OsString>);
impl ProviderBaseGuard {
    fn set(base:&str)->Self {
        let previous=std::env::var_os("STRIPE_API_BASE");
        // This dedicated gate runs with --test-threads=1 and has no live keys.
        unsafe {std::env::set_var("STRIPE_API_BASE",base);}
        Self(previous)
    }
}
impl Drop for ProviderBaseGuard {
    fn drop(&mut self){unsafe {match self.0.take(){Some(value)=>std::env::set_var("STRIPE_API_BASE",value),None=>std::env::remove_var("STRIPE_API_BASE")}}}
}
#[tokio::test] async fn recording_http_provider_cannot_expand_capture_between_read_and_write(){
    use axum::{extract::{State,Form},routing::{get,post},Json,Router};
    use std::sync::Arc;
    struct HttpState { receipt:Mutex<TerminalIntentReceipt>, posted:Mutex<Vec<std::collections::HashMap<String,String>>> }
    async fn read(State(state):State<Arc<HttpState>>)->Json<TerminalIntentReceipt>{
        let mut receipt=state.receipt.lock().unwrap();let observed=receipt.clone();
        // Concurrent provider-side adjustment after the GET, before capture POST.
        receipt.amount=9000;receipt.amount_capturable=9000;
        Json(observed)
    }
    async fn write(State(state):State<Arc<HttpState>>,Form(form):Form<std::collections::HashMap<String,String>>)->Json<TerminalIntentReceipt>{
        state.posted.lock().unwrap().push(form.clone());
        let mut receipt=state.receipt.lock().unwrap().clone();
        receipt.amount_received=form.get("amount_to_capture").and_then(|value|value.parse().ok()).unwrap_or(receipt.amount);
        receipt.amount_capturable=0;receipt.status="succeeded".into();Json(receipt)
    }
    let f=Fixture::new().await;let p=Spy::default();let id=prepared(&f,&p).await;
    let state=Arc::new(HttpState{receipt:Mutex::new(p.receipt.lock().unwrap().clone().unwrap()),posted:Mutex::new(Vec::new())});
    let app=Router::new().route("/v1/payment_intents/pi_fixture0",get(read)).route("/v1/payment_intents/pi_fixture0/capture",post(write)).with_state(state.clone());
    let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();let address=listener.local_addr().unwrap();
    let server=tokio::spawn(async move{axum::serve(listener,app).await.unwrap()});
    let _base=ProviderBaseGuard::set(&format!("http://{address}"));
    let client=server_integrations_stripe::client::StripeClient::new("local-terminal-recording-fixture".into());
    let result=protocol::capture(&f.pool,"tenant_a",capture(&id),"provider_a",&client).await;
    assert!(result.is_err(),"provider receipt drift must never report success");
    {
        let requests=state.posted.lock().unwrap();assert_eq!(requests.len(),1);
        assert_eq!(requests[0].get("amount_to_capture").map(String::as_str),Some("2500"),"capture must be bounded by persisted minor units, not the changed 9000 provider amount");
    }
    let state_name:String=sqlx::query_scalar("SELECT state FROM terminal_payment_operations").fetch_one(&f.pool).await.unwrap();assert_eq!(state_name,"reconciliation_required");
    assert!(client.capture_terminal_payment_intent(&id).await.is_err(),"legacy ID-only helper must fail closed");
    assert_eq!(state.posted.lock().unwrap().len(),1,"legacy helper must not make a bypass provider call");
    server.abort();f.done().await;
}
