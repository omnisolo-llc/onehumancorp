use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use super::gift_card_ledger::GiftCardLedger;
use uuid::Uuid;

async fn setup_db() -> Arc<sqlx::PgPool> {
    let pool = PgPoolOptions::new()
        .connect("postgres://ohc:ohc@localhost:5432/ohc")
        .await
        .unwrap();

    Arc::new(pool)
}

#[tokio::test]
async fn test_issue_and_redeem_gift_card() {
    if std::env::var("DATABASE_URL").is_err() {
        // Skip if no DB is available (e.g., standard bazel run outside docker)
        // Note: tests usually run against a configured DB or via an e2e env setup
        return;
    }

    let pool = setup_db().await;
    let ledger = GiftCardLedger::new(pool.clone());
    let tenant_id = format!("tenant_{}", Uuid::new_v4());
    let code = format!("GC-{}", Uuid::new_v4());

    // 1. Issue a new gift card with $50.00 (5000 cents)
    let card = ledger.issue_card(
        &tenant_id,
        None,
        &code,
        "GIFT_CARD",
        5000,
        Some("pos_issue_ref".to_string()),
        false,
    ).await.unwrap();

    assert_eq!(card.balance, 5000);
    assert_eq!(card.code, code);

    // 2. Fetch balance to verify
    let fetched = ledger.get_card_by_code(&tenant_id, &code).await.unwrap().unwrap();
    assert_eq!(fetched.balance, 5000);

    // 3. Redeem $20.00 (2000 cents)
    let entry = ledger.apply_transaction(
        &tenant_id,
        &code,
        -2000,
        Some("checkout_ref".to_string()),
        false,
    ).await.unwrap();

    assert_eq!(entry.amount, -2000);

    // 4. Verify new balance is $30.00
    let fetched2 = ledger.get_card_by_code(&tenant_id, &code).await.unwrap().unwrap();
    assert_eq!(fetched2.balance, 3000);

    // 5. Attempt to overdraw $40.00 online (should fail)
    let res = ledger.apply_transaction(
        &tenant_id,
        &code,
        -4000,
        Some("overdraw_ref".to_string()),
        false,
    ).await;
    assert!(res.is_err(), "Should prevent overdraw online");

    // 6. Attempt to overdraw $40.00 offline sync (should succeed and result in negative balance)
    let entry_offline = ledger.apply_transaction(
        &tenant_id,
        &code,
        -4000,
        Some("offline_overdraw_ref".to_string()),
        true, // is_offline_sync
    ).await.unwrap();
    assert_eq!(entry_offline.amount, -4000);

    let fetched3 = ledger.get_card_by_code(&tenant_id, &code).await.unwrap().unwrap();
    assert_eq!(fetched3.balance, -1000, "Offline sync should allow negative balance for later reconciliation");
}
