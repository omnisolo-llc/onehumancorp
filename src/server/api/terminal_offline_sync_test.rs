use super::*;
use sqlx::postgres::PgPoolOptions;
async fn fixture() -> PgPool {
    let url = std::env::var("OHC_SYNC_TEST_DATABASE_URL").expect("isolated database required");
    let schema = format!("terminal_test_{}", uuid::Uuid::new_v4().simple());
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(12)
        .after_connect(move |connection, _| {
            let schema = schema.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}"))
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("durable_sync_test_schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../migrations/236_pos_offline_request_identity.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO products(id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    pool
}
fn item() -> PosOfflineTransaction {
    PosOfflineTransaction {
        id: Some("sale".into()),
        client_id: Some("client".into()),
        amount_cents: 500,
        currency: "USD".into(),
        payload: json!([{"product_id":"p","quantity":2}]).to_string(),
        timestamp: None,
        mutation_type: Some("cash_sale".into()),
        device_signature: Some("sig_original".into()),
        terminal_id: Some("terminal".into()),
    }
}
async fn count(pool: &PgPool, table: &str) -> i64 {
    sqlx::query_scalar(&format!("SELECT count(*) FROM {table}"))
        .fetch_one(pool)
        .await
        .unwrap()
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn eight_concurrent_replays_apply_every_side_effect_once() {
    let pool = fixture().await;
    let mut tasks = vec![];
    for _ in 0..8 {
        let p = pool.clone();
        tasks.push(tokio::spawn(async move {
            apply(&p, "tenant-a", None, &item()).await.unwrap()
        }));
    }
    for task in tasks {
        assert_eq!(task.await.unwrap().status, "acknowledged");
    }
    assert_eq!(count(&pool, "pos_offline_transactions").await, 1);
    assert_eq!(count(&pool, "orders").await, 1);
    assert_eq!(count(&pool, "order_items").await, 1);
    assert_eq!(count(&pool, "ohc_job_queue").await, 1);
    assert_eq!(count(&pool, "agent_action_requests").await, 2);
    let stock: i32 = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stock, 8);
    let changes: i32 =
        sqlx::query_scalar("SELECT offline_changes_count FROM pos_terminal_sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(changes, 1);
}
#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn changed_signature_replays_but_original_is_immutable_and_shape_is_validated() {
    let pool = fixture().await;
    assert_eq!(
        apply(&pool, "tenant-a", None, &item())
            .await
            .unwrap()
            .status,
        "acknowledged"
    );
    let mut retry = item();
    retry.device_signature = Some("sig_regenerated".into());
    assert_eq!(
        apply(&pool, "tenant-a", None, &retry).await.unwrap().status,
        "acknowledged"
    );
    retry.device_signature = Some("invalid".into());
    assert_eq!(
        apply(&pool, "tenant-a", None, &retry).await.unwrap().status,
        "blocked"
    );
    let signature: String =
        sqlx::query_scalar("SELECT device_signature FROM pos_offline_transactions")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(signature, "sig_original");
    retry = item();
    retry.amount_cents += 1;
    assert_eq!(
        apply(&pool, "tenant-a", None, &retry).await.unwrap().status,
        "reconciliation"
    );
    retry = item();
    retry.terminal_id = Some("changed".into());
    assert_eq!(
        apply(&pool, "tenant-a", None, &retry).await.unwrap().status,
        "reconciliation"
    );
    assert_eq!(
        apply(&pool, "tenant-b", None, &item())
            .await
            .unwrap()
            .status,
        "reconciliation"
    );
    assert_eq!(count(&pool, "orders").await, 1);
}
#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn failed_order_rolls_back_stock_receipt_and_job() {
    let pool = fixture().await;
    sqlx::query("ALTER TABLE orders ADD CONSTRAINT reject_order CHECK(false)")
        .execute(&pool)
        .await
        .unwrap();
    assert!(apply(&pool, "tenant-a", None, &item()).await.is_err());
    assert_eq!(count(&pool, "pos_offline_transactions").await, 0);
    assert_eq!(count(&pool, "ohc_job_queue").await, 0);
    let stock: i32 = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stock, 10);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn terminal_commit_failure_never_reports_success() {
    let pool = fixture().await;
    sqlx::raw_sql("CREATE FUNCTION reject_terminal_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'injected deferred commit failure'; END $$; CREATE CONSTRAINT TRIGGER reject_terminal AFTER INSERT ON pos_offline_transactions DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_terminal_commit();").execute(&pool).await.unwrap();
    let response = sync_offline_response(
        &pool,
        "tenant-a",
        &SyncOfflineTransactionsRequest {
            session_id: None,
            transactions: vec![item()],
        },
        None,
    )
    .await;
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let response: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(response["synced_count"], 0);
    assert_eq!(response["acknowledged_transaction_ids"], json!([]));
    assert_eq!(response["outcomes"][0]["status"], "blocked");
    assert_eq!(count(&pool, "orders").await, 0);
    assert_eq!(count(&pool, "pos_offline_transactions").await, 0);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn terminal_partial_success_and_shortage_replays_do_not_repeat_effects() {
    let pool = fixture().await;
    let mut invalid = item();
    invalid.id = Some("invalid".into());
    invalid.payload = json!([{"product_id":"p","quantity":-1}]).to_string();
    let response = sync_offline_response(
        &pool,
        "tenant-a",
        &SyncOfflineTransactionsRequest {
            session_id: None,
            transactions: vec![invalid, item()],
        },
        None,
    )
    .await;
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let response: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(response["synced_count"], 1);
    assert_eq!(response["outcomes"][0]["status"], "blocked");
    assert_eq!(response["outcomes"][1]["status"], "acknowledged");
    let mut shortage = item();
    shortage.id = Some("shortage".into());
    shortage.payload = json!([{"product_id":"p","quantity":20}]).to_string();
    for _ in 0..2 {
        assert_eq!(
            apply(&pool, "tenant-a", None, &shortage)
                .await
                .unwrap()
                .status,
            "reconciliation"
        );
    }
    assert_eq!(count(&pool, "orders").await, 2);
    assert_eq!(count(&pool, "ohc_job_queue").await, 2);
    let changes: i32 =
        sqlx::query_scalar("SELECT offline_changes_count FROM pos_terminal_sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(changes, 2);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn non_usd_card_payment_is_never_queued_to_usd_only_worker() {
    let pool = fixture().await;
    let mut unsupported = item();
    unsupported.mutation_type = Some("tap_to_pay".into());
    unsupported.currency = "EUR".into();
    assert_eq!(
        apply(&pool, "tenant-a", None, &unsupported)
            .await
            .unwrap()
            .status,
        "reconciliation"
    );
    assert_eq!(count(&pool, "ohc_job_queue").await, 0);
    assert_eq!(count(&pool, "orders").await, 0);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn existing_pn_counter_inventory_is_updated_once_with_plain_stock() {
    let pool = fixture().await;
    sqlx::raw_sql("ALTER TABLE products ADD COLUMN pn_counter_p INTEGER DEFAULT 10;ALTER TABLE products ADD COLUMN pn_counter_n INTEGER DEFAULT 0;").execute(&pool).await.unwrap();
    for _ in 0..2 {
        assert_eq!(
            apply(&pool, "tenant-a", None, &item())
                .await
                .unwrap()
                .status,
            "acknowledged"
        );
    }
    let row: (i32, i32, i32) = sqlx::query_as(
        "SELECT pn_counter_n,inventory_count,available_quantity FROM products WHERE id='p'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row, (2, 8, 8));
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn canonical_cash_producer_keeps_one_sale_when_its_receipt_is_completed_twice() {
    let pool = fixture().await;
    sqlx::query("ALTER TABLE pos_offline_transactions ADD COLUMN updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP").execute(&pool).await.unwrap();
    assert_eq!(apply(&pool, "tenant-a", None, &item()).await.unwrap().status, "acknowledged");
    let job: Value = sqlx::query_scalar("SELECT payload FROM ohc_job_queue WHERE tenant_id='tenant-a' AND job_type='offline_pos_sync'").fetch_one(&pool).await.unwrap();
    for _ in 0..2 {
        offline_authority::complete_committed_operation(&pool, "tenant-a", &job).await.unwrap();
    }
    assert_eq!(count(&pool, "orders").await, 1);
    let stock: i32 = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id='p'").fetch_one(&pool).await.unwrap();
    assert_eq!(stock, 8);
    let status: (String,String) = sqlx::query_as("SELECT status,_sync_status FROM pos_offline_transactions WHERE id='sale'").fetch_one(&pool).await.unwrap();
    assert_eq!(status, ("RESOLVED".into(), "synced".into()));
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn canonical_cash_producer_rejects_contradictory_card_evidence_before_sale_effects() {
    let pool = fixture().await;
    let mut request = item();
    request.payload = json!([{"product_id":"p","quantity":2,"payment_method":"card","payment_intent_id":"pi_original"}]).to_string();
    let result = apply(&pool, "tenant-a", None, &request).await.unwrap();
    assert_eq!(result.status, "reconciliation");
    assert_eq!(result.reason, Some("contradictory_cash_payment_evidence"));
    for table in ["orders", "pos_offline_transactions", "ohc_job_queue"] { assert_eq!(count(&pool, table).await, 0); }
    let stock: i32 = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id='p'").fetch_one(&pool).await.unwrap();
    assert_eq!(stock, 10);
}
