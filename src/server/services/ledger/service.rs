use async_trait::async_trait;
use ledger_proto::ohc::ledger::{
    ledger_service_server::LedgerService, Account, Entry, GetBalanceRequest,
    GetBalanceResponse, GetStatementRequest, GetStatementResponse, RecordTransactionRequest,
    RecordTransactionResponse, Transaction,
};
use sqlx::{PgPool, Postgres, Row, Transaction as SqlxTransaction};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct LedgerServiceImpl {
    pool: PgPool,
}

impl LedgerServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LedgerService for LedgerServiceImpl {
    async fn record_transaction(
        &self,
        request: Request<RecordTransactionRequest>,
    ) -> Result<Response<RecordTransactionResponse>, Status> {
        let req = request.into_inner();

        let tx_id = Uuid::new_v4().to_string();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin transaction: {}", e)))?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id).await.map_err(|e| Status::internal(format!("Failed to set RLS tenant: {}", e)))?;


        // 1. Record the transaction
        sqlx::query(r#"
            INSERT INTO ledger_transactions (tenant_id, tx_id, amount, currency)
            VALUES ($1, $2, $3, $4)
            "#).bind(req.tenant_id.clone()).bind(tx_id.clone()).bind(req.amount.clone()).bind(req.currency.clone())
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert transaction: {}", e)))?;

        // 2. Create the entries (Debit from, Credit to)
        let debit_entry_id = Uuid::new_v4().to_string();
        sqlx::query(r#"
            INSERT INTO ledger_entries (tenant_id, entry_id, tx_id, account_id, direction, amount)
            VALUES ($1, $2, $3, $4, 'DEBIT', $5)
            "#).bind(req.tenant_id.clone()).bind(debit_entry_id).bind(tx_id.clone()).bind(req.from_account_id.clone()).bind(req.amount.clone())
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert debit entry: {}", e)))?;

        let credit_entry_id = Uuid::new_v4().to_string();
        sqlx::query(r#"
            INSERT INTO ledger_entries (tenant_id, entry_id, tx_id, account_id, direction, amount)
            VALUES ($1, $2, $3, $4, 'CREDIT', $5)
            "#).bind(req.tenant_id.clone()).bind(credit_entry_id).bind(tx_id.clone()).bind(req.to_account_id.clone()).bind(req.amount.clone())
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert credit entry: {}", e)))?;

        // 3. Update account balances (Upsert)
        // Debit account
        sqlx::query(r#"
            INSERT INTO ledger_accounts (tenant_id, account_id, currency, balance)
            VALUES ($1, $2, $3, -$4)
            ON CONFLICT (tenant_id, account_id)
            DO UPDATE SET balance = ledger_accounts.balance + EXCLUDED.balance, updated_at = CURRENT_TIMESTAMP
            "#).bind(req.tenant_id.clone()).bind(req.from_account_id.clone()).bind(req.currency.clone()).bind(req.amount.clone())
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update debit account balance: {}", e)))?;

        // Credit account
        sqlx::query(r#"
            INSERT INTO ledger_accounts (tenant_id, account_id, currency, balance)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (tenant_id, account_id)
            DO UPDATE SET balance = ledger_accounts.balance + EXCLUDED.balance, updated_at = CURRENT_TIMESTAMP
            "#).bind(req.tenant_id.clone()).bind(req.to_account_id.clone()).bind(req.currency.clone()).bind(req.amount.clone())
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update credit account balance: {}", e)))?;

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

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id).await.map_err(|e| Status::internal(format!("Failed to set RLS tenant: {}", e)))?;

        let account = sqlx::query(
r#"
            SELECT balance, currency FROM ledger_accounts
            WHERE tenant_id = $1 AND account_id = $2
            "#).bind(req.tenant_id.clone()).bind(req.account_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch account balance: {}", e)))?;

        match account {
            Some(acc) => Ok(Response::new(GetBalanceResponse {
                balance: acc.get::<Option<f64>, _>("balance").unwrap_or(0.0),
                currency: acc.get("currency"),
            })),
            None => Err(Status::not_found("Account not found")),
        }
    }

    async fn get_statement(
        &self,
        request: Request<GetStatementRequest>,
    ) -> Result<Response<GetStatementResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id).await.map_err(|e| Status::internal(format!("Failed to set RLS tenant: {}", e)))?;

        let entries_records = sqlx::query(
r#"
            SELECT entry_id, tx_id, account_id, direction, amount
            FROM ledger_entries
            WHERE tenant_id = $1 AND account_id = $2
            ORDER BY created_at DESC
            "#).bind(req.tenant_id.clone()).bind(req.account_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch entries: {}", e)))?;

        let tx_ids: Vec<String> = entries_records.iter().map(|e| e.get::<String, _>("tx_id")).collect();

        let transactions_records = if !tx_ids.is_empty() {
            sqlx::query(r#"
                SELECT tx_id, amount, currency, timestamp
                FROM ledger_transactions
                WHERE tenant_id = $1 AND tx_id = ANY($2)
                ORDER BY timestamp DESC
                "#).bind(req.tenant_id.clone()).bind(&tx_ids)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch transactions: {}", e)))?
        } else {
            vec![]
        };

        let mut entries = Vec::new();
        for record in entries_records {
            entries.push(Entry {
                tenant_id: req.tenant_id.clone(),
                entry_id: record.get("entry_id"),
                tx_id: record.get("tx_id"),
                account_id: record.get("account_id"),
                direction: record.get("direction"),
                amount: record.get("amount"),
            });
        }

        let mut transactions = Vec::new();
        for record in transactions_records {
            transactions.push(Transaction {
                tenant_id: req.tenant_id.clone(),
                tx_id: record.get("tx_id"),
                amount: record.get("amount"),
                currency: record.get("currency"),
                timestamp: record.get::<Option<chrono::DateTime<chrono::Utc>>, _>("timestamp").unwrap_or_default().timestamp_millis(),
            });
        }

        Ok(Response::new(GetStatementResponse {
            transactions,
            entries,
        }))
    }
}
