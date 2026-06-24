use sqlx::PgPool;
use uuid::Uuid;
use super::tax::{LedgerEntry, record_transaction, update_tax_reserve, get_tax_reserve};

#[sqlx::test]
async fn test_ledger_and_tax_reserve(pool: PgPool) {
    let tenant_id = Uuid::new_v4();
    let entry = LedgerEntry {
        id: Uuid::new_v4(),
        tenant_id,
        channel: "online".to_string(),
        amount: 100.0,
        tax_amount: 10.0,
        tax_region: "US-CA".to_string(),
    };

    record_transaction(&pool, &entry).await.expect("Failed to record transaction");
    update_tax_reserve(&pool, tenant_id, entry.tax_amount).await.expect("Failed to update reserve");

    let reserve = get_tax_reserve(&pool, tenant_id).await.expect("Failed to get reserve");
    assert_eq!(reserve, 10.0);
}
