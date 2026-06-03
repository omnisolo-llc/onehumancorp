use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCard {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub code: String,
    pub card_type: String, // 'GIFT_CARD' | 'STORE_CREDIT'
    pub status: String,
    pub balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardLedgerEntry {
    pub id: String,
    pub tenant_id: String,
    pub gift_card_id: String,
    pub amount: i64,
    pub transaction_ref: Option<String>,
    pub is_offline_sync: bool,
}

pub struct GiftCardLedger {
    pool: Arc<PgPool>,
}

impl GiftCardLedger {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Issues a new gift card or store credit
    pub async fn issue_card(
        &self,
        tenant_id: &str,
        customer_id: Option<String>,
        code: &str,
        card_type: &str,
        initial_amount: i64,
        transaction_ref: Option<String>,
        is_offline_sync: bool,
    ) -> Result<GiftCard, String> {
        let card_id = Uuid::new_v4().to_string();
        let entry_id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        // 1. Create the Gift Card record
        sqlx::query(
            "INSERT INTO ohc_gift_cards
             (id, tenant_id, customer_id, code, card_type, status)
             VALUES ($1, $2, $3, $4, $5, 'ACTIVE')"
        )
        .bind(&card_id)
        .bind(tenant_id)
        .bind(&customer_id)
        .bind(code)
        .bind(card_type)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 2. Append the initial balance to the ledger
        sqlx::query(
            "INSERT INTO ohc_gift_card_ledger_entries
             (id, tenant_id, gift_card_id, amount, transaction_ref, is_offline_sync)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&entry_id)
        .bind(tenant_id)
        .bind(&card_id)
        .bind(initial_amount)
        .bind(&transaction_ref)
        .bind(is_offline_sync)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(GiftCard {
            id: card_id,
            tenant_id: tenant_id.to_string(),
            customer_id,
            code: code.to_string(),
            card_type: card_type.to_string(),
            status: "ACTIVE".to_string(),
            balance: initial_amount,
        })
    }

    /// Appends a redemption or reload entry to the ledger. Negative amount = redemption.
    pub async fn apply_transaction(
        &self,
        tenant_id: &str,
        code: &str,
        amount: i64,
        transaction_ref: Option<String>,
        is_offline_sync: bool,
    ) -> Result<GiftCardLedgerEntry, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        // 1. Get the card ID and check current balance to prevent negative balance (double-spend)
        let card_row = sqlx::query(
            "SELECT id FROM ohc_gift_cards WHERE tenant_id = $1 AND code = $2 FOR UPDATE"
        )
        .bind(tenant_id)
        .bind(code)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let card_id: String = match card_row {
            Some(row) => {
                use sqlx::Row;
                row.get("id")
            }
            None => return Err(format!("Gift card with code {} not found", code)),
        };

        // 2. Calculate balance
        let balance_row = sqlx::query(
            "SELECT COALESCE(SUM(amount), 0) as current_balance
             FROM ohc_gift_card_ledger_entries
             WHERE tenant_id = $1 AND gift_card_id = $2"
        )
        .bind(tenant_id)
        .bind(&card_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let current_balance: i64 = {
            use sqlx::Row;
            // Depending on numeric type mapped, casting to i64
            // Since we defined amount as BIGINT, it parses as i64 in sqlx
            row_to_i64(&balance_row, "current_balance")?
        };

        // 3. Prevent double-spend if amount is negative and exceeds balance
        // If it's an offline sync, we still append it (to represent the offline state reality),
        // which might result in a negative balance that the system will flag for review.
        if !is_offline_sync && amount < 0 && current_balance + amount < 0 {
            return Err("Insufficient balance".to_string());
        }

        // 4. Append ledger entry
        let entry_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO ohc_gift_card_ledger_entries
             (id, tenant_id, gift_card_id, amount, transaction_ref, is_offline_sync)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&entry_id)
        .bind(tenant_id)
        .bind(&card_id)
        .bind(amount)
        .bind(&transaction_ref)
        .bind(is_offline_sync)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(GiftCardLedgerEntry {
            id: entry_id,
            tenant_id: tenant_id.to_string(),
            gift_card_id: card_id,
            amount,
            transaction_ref,
            is_offline_sync,
        })
    }

    /// Gets the current state of a gift card by its code, including calculated balance
    pub async fn get_card_by_code(&self, tenant_id: &str, code: &str) -> Result<Option<GiftCard>, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let card_row = sqlx::query(
            "SELECT id, customer_id, card_type, status
             FROM ohc_gift_cards
             WHERE tenant_id = $1 AND code = $2"
        )
        .bind(tenant_id)
        .bind(code)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let row = match card_row {
            Some(r) => r,
            None => return Ok(None),
        };

        use sqlx::Row;
        let card_id: String = row.get("id");
        let customer_id: Option<String> = row.try_get("customer_id").ok().flatten();
        let card_type: String = row.get("card_type");
        let status: String = row.get("status");

        let balance_row = sqlx::query(
            "SELECT COALESCE(SUM(amount), 0) as current_balance
             FROM ohc_gift_card_ledger_entries
             WHERE tenant_id = $1 AND gift_card_id = $2"
        )
        .bind(tenant_id)
        .bind(&card_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let balance: i64 = row_to_i64(&balance_row, "current_balance")?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(Some(GiftCard {
            id: card_id,
            tenant_id: tenant_id.to_string(),
            customer_id,
            code: code.to_string(),
            card_type,
            status,
            balance,
        }))
    }
}

fn row_to_i64(row: &sqlx::postgres::PgRow, column: &str) -> Result<i64, String> {
    use sqlx::Row;
    // In PostgreSQL, SUM(BIGINT) returns a NUMERIC if there are rows, or null (handled by COALESCE).
    // Try to get as i64 directly if BIGINT was preserved:
    if let Ok(val) = row.try_get::<i64, _>(column) {
        return Ok(val);
    }
    // Fallback if sqlx decodes numeric SUM as BigDecimal/String/f64 depending on features
    if let Ok(val) = row.try_get::<f64, _>(column) {
        return Ok(val as i64);
    }
    // As a string:
    if let Ok(val) = row.try_get::<String, _>(column) {
        if let Ok(parsed) = val.parse::<i64>() {
            return Ok(parsed);
        }
    }
    Err("Failed to parse numeric ledger balance".to_string())
}
