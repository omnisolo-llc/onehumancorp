use crate::{offline_worker::PosSyncWorker, producer_envelopes::*, queue::{Job, TaskJobHandler}, tests::Fixture};
use serde_json::{Value, json};
use std::sync::Arc;

async fn fixture() -> Fixture {
    let f = Fixture::new().await;
    for ddl in [
        include_str!("../../src/server/migrations/060_job_queue_and_ledger.sql"),
        include_str!("../../src/server/migrations/076_pos_offline_transactions.sql"),
        include_str!("../../src/server/migrations/130_mutation_queue_and_sync_events.sql"),
        include_str!("../../src/server/migrations/236_pos_offline_request_identity.sql"),
    ] { sqlx::raw_sql(ddl).execute(&f.pool).await.unwrap(); }
    // Only prerequisites for the exact production receipt migration.
    sqlx::raw_sql("CREATE TABLE tenants(id TEXT PRIMARY KEY); CREATE TABLE products(id TEXT PRIMARY KEY,updated_at TIMESTAMPTZ); CREATE TABLE orders(id TEXT PRIMARY KEY,updated_at TIMESTAMPTZ); CREATE TABLE appointments(id TEXT PRIMARY KEY,updated_at TIMESTAMPTZ);").execute(&f.pool).await.unwrap();
    sqlx::raw_sql(include_str!("../../src/server/migrations/234_sync_durable_receipts.sql")).execute(&f.pool).await.unwrap();
    f
}
async fn row(f: &Fixture, id: &str, original: &Value, identity: Option<&Value>) {
    sqlx::query("INSERT INTO pos_offline_transactions (id,tenant_id,client_id,amount_cents,currency,payload,request_identity,request_status) VALUES ($1,'tenant_a','device_a',2500,'usd',$2,$3,'acknowledged')")
        .bind(id).bind(original).bind(identity).execute(&f.pool).await.unwrap();
}
async fn receipt(f: &Fixture, id: &str, kind: &str, identity: &Value) {
    sqlx::query("INSERT INTO sync_events(id,tenant_id,action_type,payload,request_identity,receipt_status,receipt_route) VALUES ($1,'tenant_a',$2,'{}',$3,'acknowledged','/api/v1/sync/offline')")
        .bind(id).bind(kind).bind(identity).execute(&f.pool).await.unwrap();
}
async fn run(f: &Fixture, tenant: &str, payload: &Value) -> Result<(), String> {
    let id=uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO ohc_job_queue(id,tenant_id,job_type,payload) VALUES ($1,$2,'offline_pos_sync',$3)")
        .bind(&id).bind(tenant).bind(payload).execute(&f.pool).await.unwrap();
    let worker=PosSyncWorker::new(Arc::new(crate::db::DB{pool:f.pool.clone()}));
    let now=chrono::Utc::now();
    let job=Job{id:id.clone(),tenant_id:tenant.into(),parent_task_id:String::new(),job_type:"offline_pos_sync".into(),payload:payload.to_string(),status:"PENDING".into(),retry_count:0,max_retries:5,next_retry_at:now,locked_until:None,created_at:now,updated_at:now};
    let result=worker.handle(job).await;
    // Exercise the real handler; queue's caller completes only Ok outcomes.
    if result.is_err() {
        let saved:(String,Value)=sqlx::query_as("SELECT status,payload FROM ohc_job_queue WHERE id=$1").bind(&id).fetch_one(&f.pool).await.unwrap();
        assert_eq!(saved,("PENDING".into(),payload.clone()));
    }
    let ledger:i64=sqlx::query_scalar("SELECT count(*) FROM ohc_universal_ledger").fetch_one(&f.pool).await.unwrap();
    assert_eq!(ledger,0,"worker must never invent ledger credit from uploaded work");
    result
}
async fn held(f: &Fixture,id: &str,original: &Value) {
    let saved:(String,String,Value)=sqlx::query_as("SELECT status,_sync_status,payload FROM pos_offline_transactions WHERE id=$1").bind(id).fetch_one(&f.pool).await.unwrap();
    assert_eq!(saved,("RECONCILIATION_REQUIRED".into(),"pending".into(),original.clone()));
}
fn cash_identity(id: &str, payload: &Value) -> Value {
    json!({"tenant_id":"tenant_a","id":id,"client_id":"device_a","amount_cents":2500,"currency":"usd","mutation_type":"cash_sale","payload":payload})
}

#[tokio::test]
async fn real_grpc_and_hybrid_envelopes_hold_wrapped_card_evidence_on_restart() {
    let f=fixture().await;
    let original=json!({"mutation_type":"tap_to_pay","payment_method":"card","payment_intent_id":"pi_original"});
    for (index,builder) in [grpc_offline_job,hybrid_offline_job].into_iter().enumerate() {
        let id=format!("wrapped_{index}");row(&f,&id,&original,None).await;
        let envelope=builder(&id,"device_a",2500,"usd",&original.to_string());
        assert_eq!(envelope["mutation_type"],"tap_to_pay");
        for _ in 0..2 {assert!(run(&f,"tenant_a",&envelope).await.is_err());held(&f,&id,&original).await;}
    }
    f.done().await;
}

#[tokio::test]
async fn durable_producer_preserves_tap_type_with_null_provider_fields_and_no_pos_row() {
    let f=fixture().await;
    let original=json!({"transaction_id":"durable_card","mutation_type":"tap_to_pay","product_id":"owned","quantity_deducted":1,"amount":2500,"payment_method":null,"payment_intent_id":null,"currency":"usd"});
    receipt(&f,"receipt_card","tap_to_pay",&original).await;
    let envelope=offline_mutation_job("receipt_card",&original);
    assert_eq!(envelope["mutation_type"],"tap_to_pay");assert_eq!(envelope["mutation"],original);
    for _ in 0..2 {assert!(run(&f,"tenant_a",&envelope).await.is_err());}
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM pos_offline_transactions").fetch_one(&f.pool).await.unwrap();assert_eq!(count,0);
    f.done().await;
}

#[tokio::test]
async fn terminal_payment_unknown_and_serialized_array_card_remain_pending() {
    let f=fixture().await;
    let original=json!([{"product_id":"p","quantity":1,"payment_method":"card","payment_intent_id":"pi_original"}]);
    for (index,kind) in [None,Some("payment"),Some("")].into_iter().enumerate() {
        let id=format!("unknown_{index}");
        let mut identity=cash_identity(&id,&original);identity["mutation_type"]=json!(kind);
        row(&f,&id,&original,Some(&identity)).await;
        let envelope=terminal_offline_job(&id,"device_a",2500,"usd",kind,&original);
        assert!(run(&f,"tenant_a",&envelope).await.is_err());held(&f,&id,&original).await;
    }
    f.done().await;
}

#[tokio::test]
async fn cash_label_cannot_override_nested_card_or_malformed_evidence() {
    let f=fixture().await;
    for (index,original) in [json!({"payload":"{\"payment_method\":\"card\"}"}),json!({"payload":"not-json"}),json!({"mutation":{"mutation_type":"tap_to_pay"}}),json!({"sale":{"payment_method":"card"}}),json!([{"payment_intent_id":"pi_original"}])].into_iter().enumerate() {
        let id=format!("conflict_{index}");let identity=cash_identity(&id,&original);
        assert_eq!(crate::offline_card::explicit_kind(&identity),None);
        row(&f,&id,&original,Some(&identity)).await;
        let envelope=terminal_offline_job(&id,"device_a",2500,"usd",Some("cash_sale"),&original);
        assert!(run(&f,"tenant_a",&envelope).await.is_err());held(&f,&id,&original).await;
    }
    f.done().await;
}

#[tokio::test]
async fn proved_canonical_cash_completes_idempotently_without_repeating_sale_effects() {
    let f=fixture().await;let original=json!([{"product_id":"p","quantity":1}]);let identity=cash_identity("cash",&original);
    row(&f,"cash",&original,Some(&identity)).await;
    let envelope=terminal_offline_job("cash","device_a",2500,"usd",Some("cash_sale"),&original);
    for _ in 0..2 {
        assert!(run(&f,"tenant_a",&envelope).await.is_ok());
        let status:(String,String)=sqlx::query_as("SELECT status,_sync_status FROM pos_offline_transactions WHERE id='cash'").fetch_one(&f.pool).await.unwrap();assert_eq!(status,("RESOLVED".into(),"synced".into()));
    }
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM orders").fetch_one(&f.pool).await.unwrap();assert_eq!(count,0,"the worker must not create a second sale");f.done().await;
}

#[tokio::test]
async fn explicit_durable_cash_and_nonpayment_inventory_preserve_their_receipt_semantics() {
    let f=fixture().await;
    for kind in ["cash_sale","inventory_sale"] {
        let original=json!({"transaction_id":kind,"mutation_type":kind,"product_id":"p","quantity_deducted":1,"amount":2500,"payment_method":null,"payment_intent_id":null,"payload":null});
        receipt(&f,kind,kind,&original).await;let envelope=offline_mutation_job(kind,&original);
        for _ in 0..2 {assert!(run(&f,"tenant_a",&envelope).await.is_ok());}
        let mut changed=envelope.clone();changed["mutation"]["amount"]=json!(1);assert!(run(&f,"tenant_a",&changed).await.is_err());
    }
    f.done().await;
}

#[tokio::test]
async fn missing_foreign_and_markerless_operations_cannot_complete_or_credit() {
    let f=fixture().await;let original=json!({});let identity=cash_identity("owned",&original);
    row(&f,"owned",&original,Some(&identity)).await;
    let envelope=terminal_offline_job("owned","device_a",2500,"usd",Some("cash_sale"),&original);
    assert!(run(&f,"tenant_b",&envelope).await.is_err());
    let status:String=sqlx::query_scalar("SELECT status FROM pos_offline_transactions WHERE id='owned'").fetch_one(&f.pool).await.unwrap();assert_eq!(status,"PENDING");
    for envelope in [json!({}),json!({"pos_transaction_id":"missing"}),json!([]),json!({"pos_transaction_id":"owned"}),json!({"pos_transaction_id":"owned","transaction_id":"different"})] {
        assert!(run(&f,"tenant_a",&envelope).await.is_err());
    }
    f.done().await;
}

#[tokio::test]
async fn absent_or_unknown_durable_classification_is_not_inventory_or_cash() {
    let f=fixture().await;
    for (index,kind) in [Value::Null,json!(""),json!("payment"),json!("unknown")].into_iter().enumerate() {
        let id=format!("unknown_receipt_{index}");let original=json!({"transaction_id":id,"mutation_type":kind,"amount":2500});
        receipt(&f,&id,"inventory_sale",&original).await;
        let envelope=offline_mutation_job(&id,&original);
        assert!(run(&f,"tenant_a",&envelope).await.is_err());
    }
    f.done().await;
}
