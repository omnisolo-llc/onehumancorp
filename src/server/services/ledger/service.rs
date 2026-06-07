use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;

use ::server_ohc::ledger::ledger_service_server::LedgerService;
use ::server_ohc::ledger::{
    AccountBalance, GetBalanceRequest,
    GetBalanceResponse, GetStatementRequest, GetStatementResponse, LedgerEntry,
    RecordTransactionRequest, RecordTransactionResponse, Transaction,
};
use crate::db::DB;
use sqlx::Row;

pub struct LedgerServiceImpl {
    pub db: Arc<DB>,
}

impl LedgerServiceImpl {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl LedgerService for LedgerServiceImpl {
    async fn record_transaction(
        &self,
        request: Request<RecordTransactionRequest>,
    ) -> Result<Response<RecordTransactionResponse>, Status> {
        let req = request.into_inner();

        let tenant_id = req.tenant_id;
        let tx_id = if req.tx_id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            req.tx_id
        };

        let mut debit_sum = 0;
        let mut credit_sum = 0;

        for entry in &req.entries {
            if entry.direction == "DEBIT" {
                debit_sum += entry.amount_cents;
            } else if entry.direction == "CREDIT" {
                credit_sum += entry.amount_cents;
            } else {
                return Err(Status::invalid_argument("Invalid entry direction"));
            }
        }

        if debit_sum != credit_sum {
            return Err(Status::invalid_argument("Debits and credits must balance"));
        }

        let mut tx = self.db.pool.begin().await.map_err(|e| {
            Status::internal(format!("Failed to begin transaction: {}", e))
        })?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to set RLS context: {}", e)))?;

        let timestamp = Utc::now();

        sqlx::query(
            "INSERT INTO transactions (tenant_id, tx_id, amount, currency, timestamp) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&tenant_id)
        .bind(&tx_id)
        .bind(&req.amount_cents)
        .bind(&req.currency)
        .bind(timestamp)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert transaction: {}", e)))?;

        for entry in req.entries {
            let entry_id = if entry.entry_id.is_empty() {
                Uuid::new_v4().to_string()
            } else {
                entry.entry_id
            };

            sqlx::query(
                "INSERT INTO entries (tenant_id, entry_id, tx_id, account_id, direction, amount, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&tenant_id)
            .bind(&entry_id)
            .bind(&tx_id)
            .bind(&entry.account_id)
            .bind(&entry.direction)
            .bind(entry.amount_cents)
            .bind(timestamp)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to insert entry: {}", e)))?;

            // Update account balance
            let balance_change = if entry.direction == "DEBIT" {
                entry.amount_cents
            } else {
                -entry.amount_cents
            };

            sqlx::query(
                "INSERT INTO accounts (tenant_id, account_id, currency, balance, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $5) ON CONFLICT (tenant_id, account_id) DO UPDATE SET balance = accounts.balance + EXCLUDED.balance, updated_at = EXCLUDED.updated_at"
            )
            .bind(&tenant_id)
            .bind(&entry.account_id)
            .bind(&req.currency)
            .bind(balance_change)
            .bind(timestamp)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to update account balance: {}", e)))?;
        }

        tx.commit()
            .await
            .map_err(|e| Status::internal(format!("Failed to commit transaction: {}", e)))?;

        Ok(Response::new(RecordTransactionResponse {
            tx_id,
            success: true,
        }))
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.db.pool.begin().await.map_err(|e| {
            Status::internal(format!("Failed to begin transaction: {}", e))
        })?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to set RLS context: {}", e)))?;

        let record = sqlx::query("SELECT balance, currency FROM accounts WHERE tenant_id = $1 AND account_id = $2")
        .bind(&req.tenant_id)
        .bind(&req.account_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch balance: {}", e)))?;

        let balance = if let Some(r) = record {
            AccountBalance {
                tenant_id: req.tenant_id,
                account_id: req.account_id,
                currency: r.get("currency"),
                balance_cents: r.get::<i64, _>("balance"),
            }
        } else {
            AccountBalance {
                tenant_id: req.tenant_id,
                account_id: req.account_id,
                currency: req.currency.clone(),
                balance_cents: 0,
            }
        };

        Ok(Response::new(GetBalanceResponse {
            balance: Some(balance),
        }))
    }

    async fn get_statement(
        &self,
        request: Request<GetStatementRequest>,
    ) -> Result<Response<GetStatementResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.db.pool.begin().await.map_err(|e| {
            Status::internal(format!("Failed to begin transaction: {}", e))
        })?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to set RLS context: {}", e)))?;

        let entries_records = sqlx::query("SELECT entry_id, tx_id, direction, amount FROM entries WHERE tenant_id = $1 AND account_id = $2")
        .bind(&req.tenant_id)
        .bind(&req.account_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch entries: {}", e)))?;

        let mut tx_ids = std::collections::HashSet::new();
        for e in &entries_records {
            tx_ids.insert(e.get::<String, _>("tx_id"));
        }

        let mut transactions = vec![];

        for tx_id in tx_ids {
            let tx_record = sqlx::query("SELECT amount, currency, timestamp FROM transactions WHERE tenant_id = $1 AND tx_id = $2")
            .bind(&req.tenant_id)
            .bind(&tx_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch transaction: {}", e)))?;

            if let Some(r) = tx_record {
                let tx_entries: Vec<LedgerEntry> = entries_records
                    .iter()
                    .filter(|e| e.get::<String, _>("tx_id") == *tx_id)
                    .map(|e| LedgerEntry {
                        entry_id: e.get::<String, _>("entry_id"),
                        tx_id: tx_id.clone(),
                        account_id: req.account_id.clone(),
                        direction: e.get::<String, _>("direction"),
                        amount_cents: e.get::<i64, _>("amount"),
                    })
                    .collect();

                transactions.push(Transaction {
                    tx_id: tx_id,
                    amount_cents: r.get::<i64, _>("amount"),
                    currency: r.get("currency"),
                    timestamp: r.get::<chrono::DateTime<Utc>, _>("timestamp").timestamp(),
                    entries: tx_entries,
                });
            }
        }

        Ok(Response::new(GetStatementResponse { transactions }))
    }
}
