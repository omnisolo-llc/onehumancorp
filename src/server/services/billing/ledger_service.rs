use bigdecimal::BigDecimal;
use sqlx::{PgPool, Row};
use std::str::FromStr;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::ohc::ledger::v1::ledger_service_server::LedgerService;
use crate::ohc::ledger::v1::{
    GetBalanceRequest, GetBalanceResponse, GetStatementRequest, GetStatementResponse,
    RecordTransactionRequest, RecordTransactionResponse,
    record_transaction_request::entry::Direction,
};

pub struct LedgerServiceImpl {
    pub pool: PgPool,
}

impl LedgerServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl LedgerService for LedgerServiceImpl {
    async fn record_transaction(
        &self,
        request: Request<RecordTransactionRequest>,
    ) -> Result<Response<RecordTransactionResponse>, Status> {
        let req = request.into_inner();

        let tenant_id = req.tenant_id.clone();
        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        let tx_id = req.transaction_id.clone();
        if tx_id.is_empty() {
            return Err(Status::invalid_argument("transaction_id is required"));
        }

        let currency = req.currency.clone();
        if currency.is_empty() {
            return Err(Status::invalid_argument("currency is required"));
        }

        if req.entries.is_empty() {
            return Err(Status::invalid_argument("entries are required"));
        }

        // Validate double entry bookkeeping: Debits must equal Credits
        let mut total_debit = BigDecimal::from(0);
        let mut total_credit = BigDecimal::from(0);

        for entry in &req.entries {
            let amount = BigDecimal::from_f64(entry.amount)
                .ok_or_else(|| Status::invalid_argument("Invalid amount precision"))?;

            if amount < BigDecimal::from(0) {
                return Err(Status::invalid_argument("Amount must be positive"));
            }

            match entry.direction() {
                Direction::Debit => {
                    total_debit += amount;
                }
                Direction::Credit => {
                    total_credit += amount;
                }
                Direction::DirectionUnspecified => {
                    return Err(Status::invalid_argument("Direction must be specified"));
                }
            }
        }

        if total_debit != total_credit {
            return Err(Status::invalid_argument(format!(
                "Debits must equal credits. Debits: {}, Credits: {}",
                total_debit, total_credit
            )));
        }

        let mut db_tx = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("Failed to begin database transaction: {}", e))
        })?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *db_tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        // Insert Transaction Record
        let insert_tx_result = sqlx::query(
            "INSERT INTO ledger_transactions (tx_id, tenant_id, amount, currency) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
        )
        .bind(&tx_id)
        .bind(&tenant_id)
        .bind(&total_debit)
        .bind(&currency)
        .execute(&mut *db_tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert transaction: {}", e)))?;

        // If no rows were affected, it means the idempotency key (tx_id) already exists.
        // We'll return success to adhere to idempotent semantics.
        if insert_tx_result.rows_affected() == 0 {
             db_tx.rollback().await.ok();
             return Ok(Response::new(RecordTransactionResponse {
                transaction_id: tx_id,
                success: true,
             }));
        }

        // Insert Entries and Update Balances
        for entry in &req.entries {
            let entry_id = Uuid::new_v4().to_string();
            let amount = BigDecimal::from_f64(entry.amount).unwrap();
            let direction_str = match entry.direction() {
                Direction::Debit => "DEBIT",
                Direction::Credit => "CREDIT",
                _ => unreachable!(),
            };

            // Upsert Account
            // Note: In real double-entry, Asset/Expense accounts increase with Debit.
            // Liability/Equity/Revenue accounts increase with Credit.
            // For simplicity here, we assume a unified balance approach where Credit adds to the balance (e.g. merchant balance).
            // This logic should be adapted to specific account types in production.
            let balance_change = match entry.direction() {
                Direction::Credit => amount.clone(),
                Direction::Debit => -amount.clone(),
                _ => unreachable!(),
            };

            sqlx::query(
                "INSERT INTO ledger_accounts (account_id, tenant_id, currency, balance)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (account_id) DO UPDATE SET
                 balance = ledger_accounts.balance + $4, updated_at = CURRENT_TIMESTAMP"
            )
            .bind(&entry.account_id)
            .bind(&tenant_id)
            .bind(&currency)
            .bind(&balance_change)
            .execute(&mut *db_tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to upsert account {}: {}", entry.account_id, e)))?;

            // Insert Entry
            sqlx::query(
                "INSERT INTO double_entry_ledger (entry_id, tenant_id, tx_id, account_id, direction, amount)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&entry_id)
            .bind(&tenant_id)
            .bind(&tx_id)
            .bind(&entry.account_id)
            .bind(direction_str)
            .bind(&amount)
            .execute(&mut *db_tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to insert ledger entry: {}", e)))?;
        }

        db_tx.commit().await.map_err(|e| {
            Status::internal(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(Response::new(RecordTransactionResponse {
            transaction_id: tx_id,
            success: true,
        }))
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let req = request.into_inner();

        let mut db_tx = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("Failed to begin db transaction: {}", e))
        })?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *db_tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let row_result = sqlx::query(
            "SELECT currency, balance FROM ledger_accounts WHERE account_id = $1 AND tenant_id = $2"
        )
        .bind(&req.account_id)
        .bind(&req.tenant_id)
        .fetch_optional(&mut *db_tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch balance: {}", e)))?;

        if let Some(row) = row_result {
            let currency: String = row.get("currency");
            let balance: BigDecimal = row.get("balance");

            // convert bigdecimal to f64 for proto
            use num_traits::ToPrimitive;
            let balance_f64 = balance.to_f64().unwrap_or(0.0);

            Ok(Response::new(GetBalanceResponse {
                account_id: req.account_id,
                currency,
                balance: balance_f64,
            }))
        } else {
            Err(Status::not_found("Account not found"))
        }
    }

    async fn get_statement(
        &self,
        request: Request<GetStatementRequest>,
    ) -> Result<Response<GetStatementResponse>, Status> {
        let req = request.into_inner();

        let mut db_tx = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("Failed to begin db transaction: {}", e))
        })?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *db_tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let limit = if req.limit > 0 { req.limit as i64 } else { 20 };

        let query_str = if req.cursor.is_empty() {
             "SELECT tx_id, direction, amount, created_at FROM double_entry_ledger WHERE account_id = $1 AND tenant_id = $2 ORDER BY created_at DESC LIMIT $3"
        } else {
             // For simplicity, using created_at as cursor. In prod, use something deterministic.
             // Assume cursor is timestamp string.
             "SELECT tx_id, direction, amount, created_at FROM double_entry_ledger WHERE account_id = $1 AND tenant_id = $2 AND created_at < $4::timestamptz ORDER BY created_at DESC LIMIT $3"
        };

        let mut query = sqlx::query(query_str)
            .bind(&req.account_id)
            .bind(&req.tenant_id)
            .bind(limit);

        if !req.cursor.is_empty() {
             query = query.bind(&req.cursor);
        }

        let rows = query.fetch_all(&mut *db_tx).await.map_err(|e| {
            Status::internal(format!("Failed to fetch statement entries: {}", e))
        })?;

        let mut entries = Vec::new();
        let mut next_cursor = String::new();

        for row in rows {
            let tx_id: String = row.get("tx_id");
            let direction: String = row.get("direction");
            let amount: BigDecimal = row.get("amount");
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

            use num_traits::ToPrimitive;
            entries.push(crate::ohc::ledger::v1::get_statement_response::TransactionEntry {
                transaction_id: tx_id,
                amount: amount.to_f64().unwrap_or(0.0),
                direction,
                timestamp: created_at.to_rfc3339(),
            });

            next_cursor = created_at.to_rfc3339();
        }

        Ok(Response::new(GetStatementResponse {
            account_id: req.account_id,
            entries,
            next_cursor,
        }))
    }
}
