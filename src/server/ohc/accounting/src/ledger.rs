use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub account_id: Uuid,
    pub amount_cents: i64,
    pub currency: String,
    pub entry_type: String, // "CREDIT" or "DEBIT"
    pub reference_id: Option<Uuid>, // E.g., Invoice ID, PO ID
    pub description: String,
    pub created_at: DateTime<Utc>,
}

pub struct LedgerService {
    pool: PgPool,
}

impl LedgerService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Records a double-entry transaction.
    /// In a double-entry system, the sum of credits must equal the sum of debits.
    pub async fn record_transaction(
        &self,
        tenant_id: Uuid,
        entries: Vec<LedgerEntry>,
    ) -> Result<(), sqlx::Error> {
        if entries.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        // Enforce RLS at the application level as a backup, though DB should enforce it
        // and enforce the double-entry balance rule.
        let mut balance: i64 = 0;
        let mut has_credit = false;
        let mut has_debit = false;

        for entry in &entries {
            if entry.tenant_id != tenant_id {
                return Err(sqlx::Error::Protocol(format!("Tenant mismatch: expected {}, got {}", tenant_id, entry.tenant_id).into()));
            }

            match entry.entry_type.as_str() {
                "CREDIT" => {
                    balance += entry.amount_cents;
                    has_credit = true;
                },
                "DEBIT" => {
                    balance -= entry.amount_cents;
                    has_debit = true;
                },
                _ => return Err(sqlx::Error::Protocol(format!("Invalid entry type: {}", entry.entry_type).into())),
            }
        }

        if balance != 0 {
            return Err(sqlx::Error::Protocol(format!("Transaction unbalanced: net change is {}", balance).into()));
        }

        if !has_credit || !has_debit {
            return Err(sqlx::Error::Protocol("Transaction must contain at least one CREDIT and one DEBIT".to_string().into()));
        }

        for entry in entries {
            sqlx::query(
                r#"
                INSERT INTO ledger_entries (
                    id, tenant_id, account_id, amount_cents, currency, entry_type, reference_id, description, created_at
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9
                )
                "#
            )
            .bind(entry.id)
            .bind(entry.tenant_id)
            .bind(entry.account_id)
            .bind(entry.amount_cents)
            .bind(entry.currency)
            .bind(entry.entry_type)
            .bind(entry.reference_id)
            .bind(entry.description)
            .bind(entry.created_at)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    /// Calculates the current balance for a tenant's account.
    pub async fn get_account_balance(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let row_credit = sqlx::query(
            r#"
            SELECT COALESCE(SUM(amount_cents), 0) as total
            FROM ledger_entries
            WHERE tenant_id = $1 AND account_id = $2 AND entry_type = 'CREDIT'
            "#
        )
        .bind(tenant_id)
        .bind(account_id)
        .fetch_one(&self.pool)
        .await?;
        let credits: i64 = row_credit.try_get("total").unwrap_or(0);

        let row_debit = sqlx::query(
            r#"
            SELECT COALESCE(SUM(amount_cents), 0) as total
            FROM ledger_entries
            WHERE tenant_id = $1 AND account_id = $2 AND entry_type = 'DEBIT'
            "#
        )
        .bind(tenant_id)
        .bind(account_id)
        .fetch_one(&self.pool)
        .await?;
        let debits: i64 = row_debit.try_get("total").unwrap_or(0);

        // Standard convention: Credits increase liability/equity/revenue, Debits increase assets/expenses.
        // For a simple cash account, Debit is positive, Credit is negative.
        // Assuming this is a general calculation, return Debits - Credits.
        Ok(debits - credits)
    }
}
