//! Required disposable PostgreSQL tests: no environment-based successful skips.
use super::*;
use axum::{body::{Body,to_bytes},http::{Method,Request}};
use sea_orm::{ConnectionTrait,Schema};
use serde_json::{Value,json};
use sqlx::{PgPool,postgres::{PgPoolOptions,PgConnectOptions}};
use std::{sync::{Mutex,atomic::{AtomicUsize,Ordering}},time::Duration};
use tower::ServiceExt;

struct Provider { calls:AtomicUsize, codes:Mutex<Vec<String>> }
#[async_trait::async_trait]
impl TwilioClientWrapper for Provider {
    async fn send_sms(&self,_:&str,_:&str,message:&str)->Result<MessageReceipt,MessageSendError>{
        self.calls.fetch_add(1,Ordering::SeqCst);self.codes.lock().unwrap().push(message.rsplit(' ').next().unwrap().into());
        Ok(MessageReceipt{sid:"SM22222222222222222222222222222222".into()})
    }
    async fn send_whatsapp(&self,_:&str,_:&str,_:&str)->Result<MessageReceipt,MessageSendError>{panic!("No provider calls allowed")}
    async fn provision_number(&self,_:&str)->Result<String,String>{panic!("No provider calls allowed")}
}
struct Fixture { admin:PgPool,pool:PgPool,service:SmsService,provider:Arc<Provider>,tokens:Vec<String>,application:String,schema:String,role:String }
impl Fixture {
    async fn new()->Self {
        let raw=std::env::var("OHC_SMS_TEST_DATABASE_URL").expect("OHC_SMS_TEST_DATABASE_URL is mandatory; PostgreSQL SMS verification cannot skip");
        let url=url::Url::parse(&raw).expect("valid owned PostgreSQL URL");
        assert!(matches!(url.scheme(),"postgres"|"postgresql") && matches!(url.host_str(),Some("127.0.0.1"|"localhost"|"[::1]"|"::1")) && url.path()=="/ohc_sms_test" && url.query().is_none() && url.fragment().is_none(),"explicit loopback ohc_sms_test database required");
        let suffix=uuid::Uuid::new_v4().simple().to_string();let schema=format!("sms_{suffix}");let role=format!("sms_role_{suffix}");let application=format!("sms_app_{suffix}");
        let password=uuid::Uuid::new_v4().to_string();
        let options:PgConnectOptions=raw.parse().unwrap();
        let options=options.options([("search_path",schema.as_str())]);
        let admin=PgPoolOptions::new().max_connections(6).connect_with(options.clone()).await.unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}")).execute(&admin).await.unwrap();
        sqlx::raw_sql(include_str!("core_pg.sql")).execute(&admin).await.unwrap();
        let orm=sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(admin.clone());let backend=sea_orm::DatabaseBackend::Postgres;
        for ddl in [Schema::new(backend).create_table_from_entity(server_auth::seaorm_store::entities::identity_user_role::Entity),Schema::new(backend).create_table_from_entity(server_auth::seaorm_store::entities::revoked_token::Entity)] { orm.execute(backend.build(&ddl)).await.unwrap(); }
        // Apply the actual new migration, not a separately invented SMS schema.
        sqlx::raw_sql(include_str!("../../src/server/migrations/1039_sms_verification_receipts.sql")).execute(&admin).await.unwrap();
        sqlx::raw_sql(include_str!("../../src/server/persistence/token_revocation_fence_postgres.sql")).execute(&admin).await.unwrap();
        for table in ["users","identity_user_roles","auth_revoked_tokens"] { sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY;ALTER TABLE {table} FORCE ROW LEVEL SECURITY;CREATE POLICY tenant_scope ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap(); }
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}';GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let pool=PgPoolOptions::new().max_connections(6).connect_with(options.username(&role).password(&password).application_name(&application)).await.unwrap();
        let privileges:(bool,bool)=sqlx::query_as("SELECT rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();assert_eq!(privileges,(false,false));
        let store=Arc::new(server_auth::Store::with_portable_repo(Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone())))));
        let mut tokens=vec![];
        for (actor,tenant) in [("owner-a","tenant-a"),("owner-b","tenant-b")] {
            sqlx::query("INSERT INTO tenants(id,name) VALUES($1,$1)").bind(tenant).execute(&admin).await.unwrap();
            sqlx::query("INSERT INTO users(id,username,email,tenant_id) VALUES($1,$1,$2,$3)").bind(actor).bind(format!("{actor}@example.test")).bind(tenant).execute(&admin).await.unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES($1,'ADMIN',$2,0)").bind(actor).bind(tenant).execute(&admin).await.unwrap();
            let now=chrono::Utc::now();let user=server_auth::User{id:actor.into(),username:actor.into(),email:format!("{actor}@example.test"),password_hash:"unused synthetic fixture".into(),roles:vec!["ADMIN".into()],active:true,organization_id:Some(tenant.into()),created_at:now,updated_at:now,oidc_subject:None};tokens.push(store.issue_token(&user).unwrap());
        }
        let provider=Arc::new(Provider{calls:AtomicUsize::new(0),codes:Mutex::new(vec![])});
        let service=SmsService::with_provider(store,provider.clone(),"+14155550000".into(),true);
        Self{admin,pool,service,provider,tokens,application,schema,role}
    }
    async fn request(&self,user:usize,path:&str,method:Method,body:Value)->(StatusCode,Value) {
        let app=router::<()>(self.service.clone()).route_layer(axum::middleware::from_fn_with_state(self.service.store.clone(),server_auth::strict_bearer_auth_middleware));
        let response=app.oneshot(Request::builder().method(method).uri(format!("/api/v1/settings/{path}")).header("authorization",format!("Bearer {}",self.tokens[user])).header("content-type","application/json").body(Body::from(body.to_string())).unwrap()).await.unwrap();
        let status=response.status();let bytes=to_bytes(response.into_body(),65536).await.unwrap();(status,serde_json::from_slice(&bytes).unwrap_or(Value::Null))
    }
    async fn send(&self)->String {
        let id=uuid::Uuid::new_v4().to_string();let (status,body)=self.request(0,"sms-verify",Method::POST,json!({"request_id":id,"phone":"+14155550123"})).await;assert_eq!(status,StatusCode::OK,"{body}");id
    }
    async fn subscribe(&self)->String {
        let id=self.send().await;let code=self.provider.codes.lock().unwrap().last().unwrap().clone();
        assert_eq!(self.request(0,"sms-confirm",Method::POST,json!({"challenge_id":id,"phone":"+14155550123","otp":code})).await.0,StatusCode::OK);
        assert_eq!(self.request(0,"sms-preferences",Method::POST,json!({"verification_id":id,"phone":"+14155550123","urgent_booking":true,"failed_payment":false,"new_order":true})).await.0,StatusCode::OK);id
    }
    async fn wait_for_app_lock(&self) {
        tokio::time::timeout(Duration::from_secs(2),async {
            loop {
                let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE application_name=$1 AND wait_event_type='Lock')").bind(&self.application).fetch_one(&self.admin).await.unwrap();
                if waiting { break; }tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }).await.expect("production SMS claim must reach the real PostgreSQL row-lock barrier");
    }
    async fn finish(self) {
        self.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE",self.schema)).execute(&self.admin).await.unwrap();
        sqlx::query(&format!("DROP ROLE {}",self.role)).execute(&self.admin).await.unwrap();self.admin.close().await;
    }
}
const CHANGES:[&str;3]=["UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='owner-a'", "UPDATE users SET active=FALSE WHERE id='owner-a'", "UPDATE sms_notification_preferences SET new_order=FALSE WHERE actor_id='owner-a'"];

#[tokio::test]
async fn pg_migration_installs_forced_tenant_rls() {
    let f=Fixture::new().await;f.subscribe().await;
    for table in ["sms_notification_preferences","sms_verification_challenges","sms_notification_dispatches","sms_notification_events"] {
        let flags:(bool,bool)=sqlx::query_as("SELECT relrowsecurity,relforcerowsecurity FROM pg_class WHERE oid=$1::regclass").bind(table).fetch_one(&f.admin).await.unwrap();assert_eq!(flags,(true,true));
    }
    let mut tx=f.pool.begin().await.unwrap();sqlx::query("SELECT set_config('app.current_tenant','tenant-b',true)").execute(&mut *tx).await.unwrap();
    let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_preferences WHERE tenant_id='tenant-a'").fetch_one(&mut *tx).await.unwrap();assert_eq!(count,0);
    assert!(sqlx::query("INSERT INTO sms_notification_preferences(tenant_id,actor_id) VALUES('tenant-a','forged')").execute(&mut *tx).await.is_err());tx.rollback().await.unwrap();f.finish().await;
}
#[tokio::test]
async fn pg_claim_locks_fence_optout_demotion_and_deactivation() {
    for change in CHANGES {
        let f=Fixture::new().await;let proof=f.subscribe().await;
        let claim=Dispatch{actor_id:"owner-a".into(),phone:"+14155550123".into(),verification_id:proof,message_hash:"synthetic event hash".into(),state:"prepared".into(),provider_sid:None};
        let mut claim_tx=f.service.background_transaction("tenant-a").await.unwrap();assert!(lock_notification_authority(&mut claim_tx,"tenant-a","new_order",&claim).await.unwrap());
        let mut writer=f.admin.begin().await.unwrap();sqlx::query("SET LOCAL lock_timeout='100ms'").execute(&mut *writer).await.unwrap();
        let error=sqlx::query(change).execute(&mut *writer).await.expect_err("revocation/opt-out cannot commit before the SMS claim releases its authority locks");
        assert_eq!(error.as_database_error().and_then(|e|e.code()).as_deref(),Some("55P03"));writer.rollback().await.unwrap();
        claim_tx.commit().await.unwrap();sqlx::query(change).execute(&f.admin).await.unwrap();f.finish().await;
    }
}
#[tokio::test]
async fn pg_revocation_winning_before_claim_produces_no_send() {
    for change in CHANGES {
        let f=Fixture::new().await;f.subscribe().await;
        let mut writer=f.admin.begin().await.unwrap();sqlx::query(change).execute(&mut *writer).await.unwrap();
        let (result,())=tokio::join!(f.service.dispatch("tenant-a","race-event","new_order","Frozen order notice"),async{f.wait_for_app_lock().await;writer.commit().await.unwrap();});
        let receipt=result.unwrap();assert_eq!(receipt.status,"no_eligible_recipients");assert!(receipt.provider_message_ids.is_empty());assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);f.finish().await;
    }
}
#[tokio::test]
async fn pg_rate_and_cooldown_are_durable() {
    let f=Fixture::new().await;f.send().await;
    let send=||json!({"request_id":uuid::Uuid::new_v4(),"phone":"+14155550123"});
    assert_eq!(f.request(0,"sms-verify",Method::POST,send()).await.0,StatusCode::TOO_MANY_REQUESTS);
    sqlx::query("UPDATE sms_notification_preferences SET last_requested_at=last_requested_at-61,send_count=5").execute(&f.admin).await.unwrap();
    assert_eq!(f.request(0,"sms-verify",Method::POST,send()).await.0,StatusCode::TOO_MANY_REQUESTS);assert_eq!(f.provider.calls.load(Ordering::SeqCst),1);
    sqlx::query("UPDATE sms_notification_preferences SET send_window=send_window-3601").execute(&f.admin).await.unwrap();assert_eq!(f.request(0,"sms-verify",Method::POST,send()).await.0,StatusCode::OK);assert_eq!(f.provider.calls.load(Ordering::SeqCst),2);f.finish().await;
}
#[tokio::test]
async fn pg_empty_event_and_changed_generator_replay_are_terminal() {
    let f=Fixture::new().await;
    assert_eq!(f.service.dispatch("tenant-a","empty","new_order","first text").await.unwrap().status,"no_recipients");
    f.subscribe().await;let calls=f.provider.calls.load(Ordering::SeqCst);
    assert_eq!(f.service.dispatch("tenant-a","empty","new_order","new LLM text").await.unwrap().status,"no_recipients");assert_eq!(f.provider.calls.load(Ordering::SeqCst),calls);
    let first=f.service.dispatch("tenant-a","accepted","new_order","original generated message").await.unwrap();
    let replay=f.service.dispatch("tenant-a","accepted","new_order","different generated message after lost acknowledgement").await.unwrap();
    assert_eq!(first.provider_message_ids,replay.provider_message_ids);assert_eq!(f.provider.calls.load(Ordering::SeqCst),calls+1);
    let message:String=sqlx::query_scalar("SELECT message FROM sms_notification_events WHERE event_id='accepted'").fetch_one(&f.admin).await.unwrap();assert_eq!(message,"original generated message");f.finish().await;
}
#[tokio::test]
async fn pg_wrong_expired_replayed_and_cross_tenant_proofs_are_rejected() {
    let f=Fixture::new().await;let id=f.send().await;let code=f.provider.codes.lock().unwrap()[0].clone();let wrong=if code=="000000"{"999999"}else{"000000"};
    let payload=|value:&str|json!({"phone":"+14155550123","challenge_id":id,"otp":value});
    assert_eq!(f.request(0,"sms-confirm",Method::POST,payload(wrong)).await.0,StatusCode::BAD_REQUEST);
    assert_eq!(f.request(1,"sms-confirm",Method::POST,payload(&code)).await.0,StatusCode::BAD_REQUEST);
    sqlx::query("UPDATE sms_verification_challenges SET created_at=0,expires_at=1").execute(&f.admin).await.unwrap();assert_eq!(f.request(0,"sms-confirm",Method::POST,payload(&code)).await.0,StatusCode::BAD_REQUEST);
    sqlx::query("UPDATE sms_verification_challenges SET expires_at=$1").bind(chrono::Utc::now().timestamp()+300).execute(&f.admin).await.unwrap();assert_eq!(f.request(0,"sms-confirm",Method::POST,payload(&code)).await.0,StatusCode::OK);
    assert_eq!(f.request(0,"sms-confirm",Method::POST,payload(&code)).await.0,StatusCode::BAD_REQUEST);f.finish().await;
}
