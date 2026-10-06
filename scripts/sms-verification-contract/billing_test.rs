//! Execute the mounted billing service functions with real selected SQLite SMS
//! state. No Redis or provider availability can turn these tests into a skip.
use super::*;
use crate::billing::{PaymentFailureMessageGenerator,PaymentFailureNotifier};
use axum::http::Method;
use serde_json::json;
use std::sync::atomic::{AtomicUsize,Ordering};
struct Notifier<'a>(&'a SmsService);
#[async_trait::async_trait]
impl PaymentFailureNotifier for Notifier<'_> {
    async fn send_payment_failure_sms(&self,tenant:&str,_subscriber:&str,message:&str)->Result<(),String> {
        self.0.dispatch(tenant,"stripe:stable-dunning-event","failed_payment",message).await.map(|_|()).map_err(|e|format!("{e:?}"))
    }
}
struct ChangingGenerator(AtomicUsize);
#[async_trait::async_trait]
impl PaymentFailureMessageGenerator for ChangingGenerator {
    async fn generate_payment_failure_message(&self,_:&str,_:&str)->String { format!("Generated wording {}",self.0.fetch_add(1,Ordering::SeqCst)) }
}
async fn state(f:&super::tests::Fixture)->crate::billing::WebhookState {
    sqlx::raw_sql("CREATE TABLE subscribers(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,customer_id TEXT NOT NULL,status TEXT NOT NULL,stripe_subscription_id TEXT);INSERT INTO subscribers VALUES('subscriber','tenant-a','customer','ACTIVE','subscription');").execute(&f.pool).await.unwrap();
    let unused=sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://fixture@127.0.0.1:1/unused").unwrap();
    crate::billing::WebhookState{db:Arc::new(crate::db::DB{pool:unused,store:crate::db::DbStore::Sqlite(f.pool.clone())})}
}
#[tokio::test]
async fn billing_empty_optin_commits_past_due_and_terminal_noop_without_retry_error() {
    let f=super::tests::Fixture::new().await;let state=state(&f).await;let generator=ChangingGenerator(AtomicUsize::new(0));
    let input=json!({"customer":"customer","subscription":"subscription"});
    for _ in 0..2 { assert_eq!(crate::billing::process_invoice_payment_failed(&state,&input,&Notifier(&f.service),&generator).await.unwrap().as_deref(),Some("subscriber")); }
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),0);
    let status:String=sqlx::query_scalar("SELECT status FROM subscribers").fetch_one(&f.pool).await.unwrap();assert_eq!(status,"PAST_DUE");
    let event:(String,String)=sqlx::query_as("SELECT status,message FROM sms_notification_events").fetch_one(&f.pool).await.unwrap();assert_eq!(event,("no_recipients".into(),"Generated wording 0".into()));
}
#[tokio::test]
async fn billing_replay_after_changed_generator_returns_first_durable_receipt_without_sms() {
    let f=super::tests::Fixture::new().await;let state=state(&f).await;let proof=f.verify(0).await;
    assert_eq!(f.request(0,"sms-preferences",Method::POST,json!({"phone":"+14155550123","verification_id":proof,"urgent_booking":false,"failed_payment":true,"new_order":false})).await.0,StatusCode::OK);
    let input=json!({"customer":"customer","subscription":"subscription"});let generator=ChangingGenerator(AtomicUsize::new(0));
    for _ in 0..2 { crate::billing::process_invoice_payment_failed(&state,&input,&Notifier(&f.service),&generator).await.unwrap(); }
    assert_eq!(f.provider.calls.load(Ordering::SeqCst),2); // one proof SMS, one actual notice
    let row:(String,String)=sqlx::query_as("SELECT status,message FROM sms_notification_events").fetch_one(&f.pool).await.unwrap();assert_eq!(row,("provider_accepted".into(),"Generated wording 0".into()));
}
