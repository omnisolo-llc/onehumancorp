//! Production-module tests. Provider I/O is replaced only at the existing Twilio
//! trait boundary; selected auth/storage, routes and receipt code remain real.
use super::*;
use axum::{body::Body,http::{Method,Request}};
use serde_json::{Value,json};
use std::sync::{Mutex,atomic::{AtomicBool,AtomicUsize,Ordering}};
use tower::ServiceExt;

pub(super) struct TestProvider { outcome:Mutex<Result<MessageReceipt,MessageSendError>>, codes:Mutex<Vec<String>>, pub(super) calls:AtomicUsize, pause:AtomicBool, started:tokio::sync::Notify, release:tokio::sync::Notify }
impl Default for TestProvider {
    fn default()->Self { Self { outcome:Mutex::new(Ok(MessageReceipt{sid:"SM11111111111111111111111111111111".into()})),codes:Mutex::new(vec![]),calls:AtomicUsize::new(0),pause:AtomicBool::new(false),started:tokio::sync::Notify::new(),release:tokio::sync::Notify::new() } }
}
#[async_trait::async_trait]
impl TwilioClientWrapper for TestProvider {
    async fn send_sms(&self,_to:&str,_from:&str,body:&str)->Result<MessageReceipt,MessageSendError>{
        self.calls.fetch_add(1,Ordering::SeqCst);
        self.codes.lock().unwrap().push(body.rsplit(' ').next().unwrap().to_string());
        if self.pause.load(Ordering::SeqCst) { self.started.notify_one();self.release.notified().await; }
        self.outcome.lock().unwrap().clone()
    }
    async fn send_whatsapp(&self,_:&str,_:&str,_:&str)->Result<MessageReceipt,MessageSendError>{panic!("No WhatsApp calls authorized")}
    async fn provision_number(&self,_:&str)->Result<String,String>{panic!("No number provisioning authorized")}
}
pub(super) struct Fixture { pub(super) pool:sqlx::SqlitePool, store:Arc<server_auth::Store>, pub(super) provider:Arc<TestProvider>, tokens:Vec<String>, pub(super) service:SmsService, _directory:tempfile::TempDir }
impl Fixture {
    pub(super) async fn new()->Self {
        let directory=tempfile::tempdir().unwrap();
        let pool=sqlx::sqlite::SqlitePoolOptions::new().max_connections(4).connect_with(sqlx::sqlite::SqliteConnectOptions::new().filename(directory.path().join("sms.sqlite")).create_if_missing(true).foreign_keys(true)).await.unwrap();
        let database=crate::persistence::AppDatabase::from_connection(sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone()));
        crate::persistence::migration::migrate(&database).await.unwrap();
        let store=Arc::new(server_auth::Store::with_portable_repo(Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone()))));
        let mut tokens=vec![];
        for (name,tenant,role) in [("alice","tenant-a","ADMIN"),("bob","tenant-a","ADMIN"),("carol","tenant-b","ADMIN"),("viewer","tenant-a","MEMBER")] {
            let user=store.create_user(name.into(),format!("{name}@example.test"),"local synthetic fixture password".into(),vec![role.into()],tenant.into()).await.unwrap();
            tokens.push(store.issue_token(&user).unwrap());
        }
        let provider=Arc::new(TestProvider::default());
        let service=SmsService::with_provider(store.clone(),provider.clone(),"+14155550000".into(),true);
        Self {pool,store,provider,tokens,service,_directory:directory}
    }
    fn app(&self)->Router {
        router(self.service.clone()).route_layer(axum::middleware::from_fn_with_state(self.store.clone(),server_auth::strict_bearer_auth_middleware))
    }
    pub(super) async fn request(&self,user:usize,path:&str,method:Method,payload:Value)->(StatusCode,Value) {
        let response=self.app().oneshot(Request::builder().method(method).uri(format!("/api/v1/settings/{path}")).header("authorization",format!("Bearer {}",self.tokens[user])).header("content-type","application/json").body(Body::from(payload.to_string())).unwrap()).await.unwrap();
        let status=response.status();let bytes=axum::body::to_bytes(response.into_body(),64*1024).await.unwrap();
        (status,serde_json::from_slice(&bytes).unwrap_or(Value::Null))
    }
    async fn send(&self,user:usize)->String {
        let id=uuid::Uuid::new_v4().to_string();
        let (status,body)=self.request(user,"sms-verify",Method::POST,json!({"phone":"+14155550123","request_id":id})).await;
        assert_eq!(status,StatusCode::OK,"{body}");assert_eq!(body["status"],"provider_accepted");assert_eq!(body["challenge_id"],id);id
    }
    pub(super) async fn verify(&self,user:usize)->String {
        let id=self.send(user).await;let code=self.provider.codes.lock().unwrap().last().unwrap().clone();
        let (status,body)=self.request(user,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await;
        assert_eq!(status,StatusCode::OK,"{body}");assert_eq!(body["status"],"verified");id
    }
}

#[tokio::test]
async fn confirmed_empty_is_distinct_from_selected_storage_outage() {
    let f=Fixture::new().await;
    let (status,body)=f.request(0,"sms-preferences",Method::GET,Value::Null).await;
    assert_eq!(status,StatusCode::OK);assert_eq!(body["status"],"unverified");assert!(body["phone"].is_null());
    sqlx::query("DROP TABLE sms_verification_challenges").execute(&f.pool).await.unwrap();
    let (status,body)=f.request(0,"sms-preferences",Method::GET,Value::Null).await;
    assert_eq!(status,StatusCode::SERVICE_UNAVAILABLE);assert_eq!(body["success"],false);
}
#[tokio::test]
async fn wrong_codes_attempt_lock_expiry_and_replay_never_produce_verification() {
    let f=Fixture::new().await;let id=f.send(0).await;let code=f.provider.codes.lock().unwrap()[0].clone();
    let wrong=if code=="000000"{"999999"}else{"000000"};
    for _ in 0..5 {
        let (status,body)=f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":wrong})).await;
        assert_eq!(status,StatusCode::BAD_REQUEST);assert_eq!(body["success"],false);
    }
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await.0,StatusCode::BAD_REQUEST);
    sqlx::query("UPDATE sms_verification_challenges SET attempts=0,created_at=0,expires_at=1").execute(&f.pool).await.unwrap();
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await.0,StatusCode::BAD_REQUEST);
    let (_,state)=f.request(0,"sms-preferences",Method::GET,Value::Null).await;assert_eq!(state["status"],"unverified");
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn phone_proof_is_actor_tenant_and_number_bound_and_consumed_once() {
    let f=Fixture::new().await;let id=f.send(0).await;let code=f.provider.codes.lock().unwrap()[0].clone();
    for user in [1,2] { assert_eq!(f.request(user,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await.0,StatusCode::BAD_REQUEST); }
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550999","challenge_id":id,"otp":code})).await.0,StatusCode::BAD_REQUEST);
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await.0,StatusCode::OK);
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await.0,StatusCode::BAD_REQUEST);
    for user in [1,2] { assert_eq!(f.request(user,"sms-preferences",Method::POST,json!({"phone":"+14155550123","verification_id":id,"urgent_booking":true,"failed_payment":false,"new_order":false})).await.0,StatusCode::CONFLICT); }
}
#[tokio::test]
async fn provider_rejection_and_unknown_outcome_are_persisted_without_false_acceptance_or_resend() {
    for outcome in [MessageSendError::Rejected{status:400},MessageSendError::UnknownOutcome{reason:"response lost"}] {
        let f=Fixture::new().await;*f.provider.outcome.lock().unwrap()=Err(outcome.clone());
        let id=uuid::Uuid::new_v4().to_string();
        let body=json!({"phone":"+14155550123","request_id":id});
        let (status,value)=f.request(0,"sms-verify",Method::POST,body.clone()).await;
        assert!(!status.is_success());assert_eq!(value["success"],false);
        let code=f.provider.codes.lock().unwrap()[0].clone();
        assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":id,"otp":code})).await.0,StatusCode::BAD_REQUEST);
        assert!(!f.request(0,"sms-verify",Method::POST,body).await.0.is_success());
        if matches!(outcome,MessageSendError::UnknownOutcome{..}) {
            let later=json!({"phone":"+14155550123","request_id":uuid::Uuid::new_v4()});
            assert_eq!(f.request(0,"sms-verify",Method::POST,later).await.0,StatusCode::CONFLICT);
        }
        assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
    }
}
#[tokio::test]
async fn acknowledgement_replay_and_duplicate_requests_do_not_send_again() {
    let f=Fixture::new().await;let id=uuid::Uuid::new_v4().to_string();let payload=json!({"phone":"+14155550123","request_id":id});
    let (a,b)=tokio::join!(f.request(0,"sms-verify",Method::POST,payload.clone()),f.request(0,"sms-verify",Method::POST,payload.clone()));
    assert!(a.0==StatusCode::OK || b.0==StatusCode::OK);
    assert_eq!(f.request(0,"sms-verify",Method::POST,payload).await.0,StatusCode::OK);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn saved_preferences_reload_and_only_confirmed_tenant_recipients_receive_events() {
    let mut f=Fixture::new().await;let id=f.verify(0).await;
    let wanted=json!({"phone":"+14155550123","verification_id":id,"urgent_booking":true,"failed_payment":false,"new_order":true});
    assert_eq!(f.request(0,"sms-preferences",Method::POST,wanted).await.0,StatusCode::OK);
    // Close and reopen the actual database file and canonical identity repository.
    f.pool.close().await;
    f.pool=sqlx::sqlite::SqlitePoolOptions::new().max_connections(4).connect_with(sqlx::sqlite::SqliteConnectOptions::new().filename(f._directory.path().join("sms.sqlite")).foreign_keys(true)).await.unwrap();
    let orm=sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(f.pool.clone());
    f.store=Arc::new(server_auth::Store::with_portable_repo(Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(orm))));
    f.service=SmsService::with_provider(f.store.clone(),f.provider.clone(),"+14155550000".into(),true);
    let (_,body)=f.request(0,"sms-preferences",Method::GET,Value::Null).await;
    assert_eq!(body["preferences"]["urgent_booking"],true);assert_eq!(body["status"],"verified");
    assert_eq!(f.service.dispatch("tenant-b","event-1","new_order","New order").await.unwrap().status,"no_recipients");
    assert_eq!(f.service.dispatch("tenant-a","event-1","failed_payment","Payment failure").await.unwrap().status,"no_recipients");
    let first=f.service.dispatch("tenant-a","event-1","new_order","New order").await.unwrap();
    assert_eq!(first.status,"provider_accepted");
    let replay=f.service.dispatch("tenant-a","event-1","new_order","New order").await.unwrap();
    assert_eq!(first.provider_message_ids,replay.provider_message_ids);
    assert_eq!(f.service.dispatch("tenant-a","event-1","new_order","Changed message").await.unwrap().provider_message_ids,first.provider_message_ids);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),2);
}
#[tokio::test]
async fn current_authority_and_saved_proof_are_required_before_preferences_or_provider_work() {
    let f=Fixture::new().await;
    assert_eq!(f.request(3,"sms-verify",Method::POST,json!({"phone":"+14155550123","request_id":uuid::Uuid::new_v4()})).await.0,StatusCode::FORBIDDEN);
    assert_eq!(f.request(0,"sms-preferences",Method::POST,json!({"phone":"+14155550123","verification_id":uuid::Uuid::new_v4(),"urgent_booking":true,"failed_payment":true,"new_order":true})).await.0,StatusCode::CONFLICT);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),0);
    let id=f.verify(0).await;
    f.store.logout_token(&f.tokens[0]).await.unwrap();
    assert!(!f.request(0,"sms-preferences",Method::POST,json!({"phone":"+14155550123","verification_id":id,"urgent_booking":true,"failed_payment":false,"new_order":false})).await.0.is_success());
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn storage_failure_before_provider_claim_sends_nothing() {
    let f=Fixture::new().await;
    sqlx::query("CREATE TRIGGER reject_sms BEFORE INSERT ON sms_verification_challenges BEGIN SELECT RAISE(ABORT,'synthetic write failure'); END").execute(&f.pool).await.unwrap();
    let (status,body)=f.request(0,"sms-verify",Method::POST,json!({"phone":"+14155550123","request_id":uuid::Uuid::new_v4()})).await;
    assert_eq!(status,StatusCode::SERVICE_UNAVAILABLE);assert_eq!(body["success"],false);assert_eq!(f.provider.calls.load(Ordering::SeqCst),0);
}
#[test]
fn sms_phone_validation_and_mac_do_not_confuse_identity_boundaries() {
    for value in ["","14155550123","+0123456789","+123","+14155550123\n"] { assert!(!valid_phone(value)); }
    assert!(valid_phone("+14155550123"));
    let key=[7;32];let value=code_mac(&key,"a","b","c","+14155550123","123456");
    assert_ne!(value,code_mac(&key,"ab","","c","+14155550123","123456"));
    assert_ne!(value,code_mac(&key,"other","b","c","+14155550123","123456"));
}

#[tokio::test]
async fn acceptance_after_token_revocation_is_not_a_verified_or_successful_setting() {
    let f=Fixture::new().await;f.provider.pause.store(true,Ordering::SeqCst);
    let id=uuid::Uuid::new_v4();
    let (result,())=tokio::join!(
        f.request(0,"sms-verify",Method::POST,json!({"request_id":id,"phone":"+14155550123"})),
        async { f.provider.started.notified().await;f.store.logout_token(&f.tokens[0]).await.unwrap();f.provider.release.notify_one(); }
    );
    assert!(!result.0.is_success());assert_eq!(result.1["success"],false);
    let state:String=sqlx::query_scalar("SELECT state FROM sms_verification_challenges").fetch_one(&f.pool).await.unwrap();
    assert_eq!(state,"sending");assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
}
#[tokio::test]
async fn notification_replay_keeps_its_original_audience_after_new_opt_in() {
    let f=Fixture::new().await;
    let alice=f.verify(0).await;
    let choice=|proof:String|json!({"phone":"+14155550123","verification_id":proof,"urgent_booking":false,"failed_payment":false,"new_order":true});
    assert_eq!(f.request(0,"sms-preferences",Method::POST,choice(alice)).await.0,StatusCode::OK);
    f.service.dispatch("tenant-a","event-old","new_order","New order").await.unwrap();
    let bob=f.verify(1).await;
    assert_eq!(f.request(1,"sms-preferences",Method::POST,choice(bob)).await.0,StatusCode::OK);
    let calls=f.provider.calls.load(Ordering::SeqCst);
    f.service.dispatch("tenant-a","event-old","new_order","New order").await.unwrap();
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),calls);
    let recipients:i64=sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_dispatches WHERE event_id='event-old'").fetch_one(&f.pool).await.unwrap();
    assert_eq!(recipients,1);
}
#[tokio::test]
async fn malformed_provider_success_is_held_as_unknown_instead_of_an_acceptance_receipt() {
    let f=Fixture::new().await;*f.provider.outcome.lock().unwrap()=Ok(MessageReceipt{sid:"not-a-provider-receipt".into()});
    let (status,body)=f.request(0,"sms-verify",Method::POST,json!({"request_id":uuid::Uuid::new_v4(),"phone":"+14155550123"})).await;
    assert_eq!(status,StatusCode::CONFLICT);assert_eq!(body["success"],false);
    let (_,state)=f.request(0,"sms-preferences",Method::GET,Value::Null).await;assert_eq!(state["challenge"]["state"],"unknown");
}

#[tokio::test]
async fn empty_event_stays_empty_after_opt_in_and_different_generated_wording() {
    let f=Fixture::new().await;
    let empty=f.service.dispatch("tenant-a","no-audience","new_order","first generated message").await.unwrap();
    assert_eq!(empty.status,"no_recipients");assert_eq!(f.provider.calls.load(Ordering::SeqCst),0);
    let proof=f.verify(0).await;
    assert_eq!(f.request(0,"sms-preferences",Method::POST,json!({"phone":"+14155550123","verification_id":proof,"urgent_booking":false,"failed_payment":false,"new_order":true})).await.0,StatusCode::OK);
    let calls=f.provider.calls.load(Ordering::SeqCst);
    let replay=f.service.dispatch("tenant-a","no-audience","new_order","different generated message").await.unwrap();
    assert_eq!(replay.status,"no_recipients");assert_eq!(f.provider.calls.load(Ordering::SeqCst),calls);
    let message:String=sqlx::query_scalar("SELECT message FROM sms_notification_events WHERE event_id='no-audience'").fetch_one(&f.pool).await.unwrap();
    assert_eq!(message,"first generated message");
}
#[tokio::test]
async fn newer_challenge_and_saved_preferences_survive_late_older_code() {
    let f=Fixture::new().await;let a=f.send(0).await;let a_code=f.provider.codes.lock().unwrap()[0].clone();
    sqlx::query("UPDATE sms_notification_preferences SET last_requested_at=last_requested_at-61").execute(&f.pool).await.unwrap();
    let b=uuid::Uuid::new_v4().to_string();
    assert_eq!(f.request(0,"sms-verify",Method::POST,json!({"phone":"+14155550999","request_id":b})).await.0,StatusCode::OK);
    let b_code=f.provider.codes.lock().unwrap()[1].clone();
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550999","challenge_id":b,"otp":b_code})).await.0,StatusCode::OK);
    assert_eq!(f.request(0,"sms-preferences",Method::POST,json!({"phone":"+14155550999","verification_id":b,"urgent_booking":true,"failed_payment":false,"new_order":true})).await.0,StatusCode::OK);
    assert_eq!(f.request(0,"sms-confirm",Method::POST,json!({"phone":"+14155550123","challenge_id":a,"otp":a_code})).await.0,StatusCode::BAD_REQUEST);
    let (_,saved)=f.request(0,"sms-preferences",Method::GET,Value::Null).await;
    assert_eq!(saved["phone"],"+14155550999");assert_eq!(saved["preferences"]["urgent_booking"],true);assert_eq!(saved["preferences"]["new_order"],true);
}
#[tokio::test]
async fn cooldown_and_hourly_rate_limits_survive_service_recreation() {
    let f=Fixture::new().await;f.send(0).await;
    assert_eq!(f.request(0,"sms-verify",Method::POST,json!({"phone":"+14155550123","request_id":uuid::Uuid::new_v4()})).await.0,StatusCode::TOO_MANY_REQUESTS);
    sqlx::query("UPDATE sms_notification_preferences SET last_requested_at=last_requested_at-61,send_count=5").execute(&f.pool).await.unwrap();
    let mut f=f;f.service=SmsService::with_provider(f.store.clone(),f.provider.clone(),"+14155550000".into(),true);
    assert_eq!(f.request(0,"sms-verify",Method::POST,json!({"phone":"+14155550123","request_id":uuid::Uuid::new_v4()})).await.0,StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
}
