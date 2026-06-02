use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LedgerAccount {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct LedgerTransaction {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub currency: String,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub id: String,
    pub tenant_id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub amount_cents: i64,
    pub direction: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct EntryInput {
    pub account_id: String,
    pub amount_cents: i64,
    pub direction: String, // "CREDIT" or "DEBIT"
}

pub struct DoubleEntryRepo {
    pool: Arc<PgPool>,
}

impl DoubleEntryRepo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_or_create_account(&self, tenant_id: &str, org_id: &str, account_id: &str, currency: &str) -> Result<LedgerAccount, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let account_opt: Option<LedgerAccount> = sqlx::query_as!(
            LedgerAccount,
            "SELECT id, tenant_id, organization_id, currency FROM ledger_accounts WHERE tenant_id = $1 AND id = $2",
            tenant_id, account_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(account) = account_opt {
            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(account);
        }

        sqlx::query(
            "INSERT INTO ledger_accounts (id, tenant_id, organization_id, currency) VALUES ($1, $2, $3, $4)"
        )
        .bind(account_id)
        .bind(tenant_id)
        .bind(org_id)
        .bind(currency)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(LedgerAccount {
            id: account_id.to_string(),
            tenant_id: tenant_id.to_string(),
            organization_id: org_id.to_string(),
            currency: currency.to_string(),
        })
    }

    pub async fn record_transaction(
        &self,
        tenant_id: &str,
        org_id: &str,
        currency: &str,
        description: Option<String>,
        reference_type: Option<String>,
        reference_id: Option<String>,
        entries: Vec<EntryInput>,
    ) -> Result<String, String> {
        let mut total_debit: i64 = 0;
        let mut total_credit: i64 = 0;

        for entry in &entries {
            if entry.amount_cents <= 0 {
                return Err("Entry amount must be positive".to_string());
            }
            match entry.direction.as_str() {
                "DEBIT" => total_debit += entry.amount_cents,
                "CREDIT" => total_credit += entry.amount_cents,
                _ => return Err("Invalid direction. Must be 'CREDIT' or 'DEBIT'".to_string()),
            }
        }

        if total_debit != total_credit {
            return Err("Transaction unbalanced: Debits must equal Credits".to_string());
        }

        let transaction_id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        // 1. Insert Transaction
        sqlx::query(
            r#"
            INSERT INTO ledger_transactions
            (id, tenant_id, organization_id, currency, description, reference_type, reference_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&transaction_id)
        .bind(tenant_id)
        .bind(org_id)
        .bind(currency)
        .bind(&description)
        .bind(&reference_type)
        .bind(&reference_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 2. Insert Entries
        for entry in entries {
            let entry_id = Uuid::new_v4().to_string();

            // Ensure account exists
            let _ = sqlx::query(
                "INSERT INTO ledger_accounts (id, tenant_id, organization_id, currency) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
            )
            .bind(&entry.account_id)
            .bind(tenant_id)
            .bind(org_id)
            .bind(currency)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;


            sqlx::query(
                r#"
                INSERT INTO ledger_entries
                (id, tenant_id, transaction_id, account_id, amount_cents, direction)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#
            )
            .bind(&entry_id)
            .bind(tenant_id)
            .bind(&transaction_id)
            .bind(&entry.account_id)
            .bind(entry.amount_cents)
            .bind(&entry.direction)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(transaction_id)
    }

    pub async fn get_balance(&self, tenant_id: &str, org_id: &str, account_id: &str) -> Result<i64, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let rows = sqlx::query(
            r#"
            SELECT amount_cents, direction
            FROM ledger_entries
            WHERE tenant_id = $1 AND account_id = $2
            "#
        )
        .bind(tenant_id)
        .bind(account_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut balance: i64 = 0;
        for row in rows {
            let amount: i64 = row.get("amount_cents");
            let direction: String = row.get("direction");

            if direction == "CREDIT" {
                balance += amount;
            } else if direction == "DEBIT" {
                balance -= amount;
            }
        }

        Ok(balance)
    }

    pub async fn get_statement(&self, tenant_id: &str, org_id: &str, account_id: &str, limit: i64, offset: i64) -> Result<(Vec<LedgerEntry>, i64), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let count_row = sqlx::query(
            "SELECT COUNT(*) as total FROM ledger_entries WHERE tenant_id = $1 AND account_id = $2"
        )
        .bind(tenant_id)
        .bind(account_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let total_count: i64 = count_row.get("total");

        let entries_res = sqlx::query_as!(
            LedgerEntry,
            r#"
            SELECT id, tenant_id, transaction_id, account_id, amount_cents, direction, created_at
            FROM ledger_entries
            WHERE tenant_id = $1 AND account_id = $2
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            tenant_id, account_id, limit, offset
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok((entries_res, total_count))
    }
}
