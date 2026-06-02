use super::double_entry_repo::{DoubleEntryRepo, EntryInput};
use crate::db::{DB, DbStore};
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::PgPool;

async fn setup_test_db() -> Arc<PgPool> {
    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/test")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ledger_accounts (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            organization_id TEXT NOT NULL,
            currency TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (tenant_id, id)
        );

        CREATE TABLE IF NOT EXISTS ledger_transactions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            organization_id TEXT NOT NULL,
            currency TEXT NOT NULL,
            description TEXT,
            reference_type TEXT,
            reference_id TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (tenant_id, id)
        );

        CREATE TABLE IF NOT EXISTS ledger_entries (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            transaction_id TEXT NOT NULL,
            account_id TEXT NOT NULL,
            amount_cents BIGINT NOT NULL,
            direction TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (tenant_id, id)
        );
        "#
    ).execute(&pg_pool).await.unwrap();

    Arc::new(pg_pool)
}

#[tokio::test]
async fn test_record_balanced_transaction() {
    let pool = setup_test_db().await;
    let repo = DoubleEntryRepo::new(pool);

    let _ = repo.get_or_create_account("DEFAULT", "org_1", "acc_1", "USD").await.unwrap();
    let _ = repo.get_or_create_account("DEFAULT", "org_1", "acc_2", "USD").await.unwrap();

    let entries = vec![
        EntryInput {
            account_id: "acc_1".to_string(),
            amount_cents: 1000,
            direction: "DEBIT".to_string(),
        },
        EntryInput {
            account_id: "acc_2".to_string(),
            amount_cents: 1000,
            direction: "CREDIT".to_string(),
        }
    ];

    let tx_id = repo.record_transaction(
        "DEFAULT",
        "org_1",
        "USD",
        Some("Test Transfer".to_string()),
        None,
        None,
        entries
    ).await;

    assert!(tx_id.is_ok());

    let bal1 = repo.get_balance("DEFAULT", "org_1", "acc_1").await.unwrap();
    let bal2 = repo.get_balance("DEFAULT", "org_1", "acc_2").await.unwrap();

    assert_eq!(bal1, -1000);
}

#[tokio::test]
async fn test_record_unbalanced_transaction_fails() {
    let pool = setup_test_db().await;
    let repo = DoubleEntryRepo::new(pool);

    let _ = repo.get_or_create_account("DEFAULT", "org_1", "acc_3", "USD").await.unwrap();
    let _ = repo.get_or_create_account("DEFAULT", "org_1", "acc_4", "USD").await.unwrap();

    let entries = vec![
        EntryInput {
            account_id: "acc_3".to_string(),
            amount_cents: 1000,
            direction: "DEBIT".to_string(),
        },
        EntryInput {
            account_id: "acc_4".to_string(),
            amount_cents: 900, // Unbalanced!
            direction: "CREDIT".to_string(),
        }
    ];

    let tx_id = repo.record_transaction(
        "DEFAULT",
        "org_1",
        "USD",
        None,
        None,
        None,
        entries
    ).await;

    assert!(tx_id.is_err());
    assert_eq!(tx_id.unwrap_err(), "Transaction unbalanced: Debits must equal Credits");
}
