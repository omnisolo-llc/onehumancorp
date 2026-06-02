use std::sync::Arc;
use sqlx::PgPool;
use server_ohc::ledger::ledger_service_server::LedgerService;
use server_ohc::ledger::{
    RecordTransactionRequest, RecordTransactionResponse, GetBalanceRequest,
    GetBalanceResponse, GetStatementRequest, GetStatementResponse, TransactionRecord
};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use prost_types::Timestamp;
use Timestamp;
use prost::alloc::string::String;
use sqlx::Row;

pub struct LedgerServiceImpl {
    pool: Arc<PgPool>,
}

impl LedgerServiceImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
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
        let tenant_id = req.tenant_id;
        let currency = req.currency;

        // Validate total amounts match (double entry)
        let mut total_credit = 0;
        let mut total_debit = 0;
        for entry in &req.entries {
            match server_ohc::ledger::transaction_entry::Direction::try_from(entry.direction).unwrap_or(server_ohc::ledger::transaction_entry::Direction::Credit) {
                server_ohc::ledger::transaction_entry::Direction::Credit => {
                    total_credit += entry.amount;
                }
                server_ohc::ledger::transaction_entry::Direction::Debit => {
                    total_debit += entry.amount;
                }
            }
        }

        if total_credit != total_debit {
            return Err(Status::invalid_argument("Total credits must equal total debits"));
        }

        let tx_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let tx_amount = total_credit; // or total_debit, since they are equal

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Create transaction record
        sqlx::query(
            "INSERT INTO transactions (tenant_id, tx_id, amount, currency, timestamp) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&tenant_id)
        .bind(&tx_id)
        .bind(tx_amount)
        .bind(&currency)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Process entries and update balances
        for entry in &req.entries {
            let entry_id = Uuid::new_v4().to_string();
            let dir_str = match server_ohc::ledger::transaction_entry::Direction::try_from(entry.direction).unwrap_or(server_ohc::ledger::transaction_entry::Direction::Credit) {
                server_ohc::ledger::transaction_entry::Direction::Credit => "CREDIT",
                server_ohc::ledger::transaction_entry::Direction::Debit => "DEBIT",
            };

            sqlx::query(
                "INSERT INTO entries (tenant_id, entry_id, tx_id, account_id, direction, amount) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&tenant_id)
            .bind(&entry_id)
            .bind(&tx_id)
            .bind(&entry.account_id)
            .bind(dir_str)
            .bind(entry.amount)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            // Upsert account to ensure it exists, default balance 0
            sqlx::query(
                "INSERT INTO accounts (tenant_id, account_id, currency, balance) VALUES ($1, $2, $3, 0) ON CONFLICT (tenant_id, account_id) DO NOTHING"
            )
            .bind(&tenant_id)
            .bind(&entry.account_id)
            .bind(&currency)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            // Update balance
            let balance_change = match server_ohc::ledger::transaction_entry::Direction::try_from(entry.direction).unwrap_or(server_ohc::ledger::transaction_entry::Direction::Credit) {
                server_ohc::ledger::transaction_entry::Direction::Credit => entry.amount,
                server_ohc::ledger::transaction_entry::Direction::Debit => -entry.amount,
            };

            sqlx::query(
                "UPDATE accounts SET balance = balance + $1 WHERE tenant_id = $2 AND account_id = $3"
            )
            .bind(balance_change)
            .bind(&tenant_id)
            .bind(&entry.account_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RecordTransactionResponse {
            tx_id,
            timestamp: Some(Timestamp {
                seconds: now.timestamp(),
                nanos: now.timestamp_subsec_nanos() as i32,
            }),
        }))
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let account_id = req.account_id;

        let account = sqlx::query(
            "SELECT balance, currency FROM accounts WHERE tenant_id = $1 AND account_id = $2"
        )
        .bind(&tenant_id)
        .bind(&account_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(acc) = account {
            let balance: i64 = acc.try_get("balance").unwrap_or(0);
            let currency: String = acc.try_get("currency").unwrap_or_else(|_| "USD".to_string());
            Ok(Response::new(GetBalanceResponse {
                balance,
                currency,
            }))
        } else {
            Ok(Response::new(GetBalanceResponse {
                balance: 0,
                currency: "USD".to_string(), // Default if not found
            }))
        }
    }

    async fn get_statement(
        &self,
        request: Request<GetStatementRequest>,
    ) -> Result<Response<GetStatementResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let account_id = req.account_id;

        let records = sqlx::query(
            r#"
            SELECT t.tx_id, e.amount, t.currency, t.timestamp, e.direction
            FROM transactions t
            JOIN entries e ON t.tx_id = e.tx_id
            WHERE t.tenant_id = $1 AND e.account_id = $2
            ORDER BY t.timestamp DESC
            "#
        )
        .bind(&tenant_id)
        .bind(&account_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let transactions = records.into_iter().map(|r| {
            let timestamp: chrono::DateTime<chrono::Utc> = r.try_get("timestamp").unwrap_or(chrono::Utc::now());
            TransactionRecord {
                tx_id: r.try_get("tx_id").unwrap_or_default(),
                amount: r.try_get("amount").unwrap_or(0),
                currency: r.try_get("currency").unwrap_or_default(),
                timestamp: Some(Timestamp {
                    seconds: timestamp.timestamp(),
                    nanos: timestamp.timestamp_subsec_nanos() as i32,
                }),
                direction: r.try_get("direction").unwrap_or_default(),
            }
        }).collect();

        Ok(Response::new(GetStatementResponse {
            transactions
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use server_db::test_utils::setup_test_db;
    // use server_ohc::ledger::TransactionEntry;

    // #[tokio::test]
    // async fn test_ledger_record_transaction_and_get_balance() {
    //     // test dummy
    // }
}
