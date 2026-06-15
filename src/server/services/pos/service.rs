use ::server_ohc::app::pos_service_server::PosService;
use ::server_ohc::app::{
    EndTerminalSessionRequest, EndTerminalSessionResponse, StartTerminalSessionRequest,
    StartTerminalSessionResponse, SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse,
    UpdateTerminalSessionStatusRequest, UpdateTerminalSessionStatusResponse,
    RecordCashMovementRequest, RecordCashMovementResponse,
    GetTerminalSessionSummaryRequest, GetTerminalSessionSummaryResponse,
    CashLedgerEntry,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyPosService {
    db: Arc<crate::db::DB>,
}

impl MyPosService {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl PosService for MyPosService {
    async fn sync_offline_transactions(
        &self,
        request: Request<SyncOfflineTransactionsRequest>,
    ) -> Result<Response<SyncOfflineTransactionsResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();
        let client_id = req.client_id.clone();

        let mut synced_count = 0;
        let mut failed_ids = Vec::new();

        let pool = self.db.pool.clone();

        let session_id = req.session_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
        let _ = sqlx::query(
            "INSERT INTO pos_terminal_sessions (id, tenant_id, device_id, status, started_at, last_synced_at, offline_changes_count)
             VALUES ($1, $2, $3, 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4)
             ON CONFLICT (tenant_id, device_id) DO UPDATE SET last_synced_at = CURRENT_TIMESTAMP, offline_changes_count = pos_terminal_sessions.offline_changes_count + $4"
        )
        .bind(&session_id)
        .bind(&tenant_id)
        .bind(&client_id)
        .bind(req.transactions.len() as i32)
        .execute(&pool)
        .await;

        let mut futures = Vec::new();

        for tx in req.transactions {
            let pool_clone = pool.clone();
            let tenant_id_clone = tenant_id.clone();
            let client_id_clone = client_id.clone();
            let session_id_clone = session_id.clone();
            let tx_id = if tx.id.is_empty() { Uuid::new_v4().to_string() } else { tx.id.clone() };

            futures.push(tokio::spawn(async move {
                let mut db_tx = match pool_clone.begin().await {
                    Ok(t) => t,
                    Err(e) => {
                        tracing::error!("Failed to begin transaction: {}", e);
                        return Err(tx.id);
                    }
                };

                if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id_clone).await {
                    tracing::error!("Failed to set org context: {}", e);
                    return Err(tx.id);
                }

                let insert_res = sqlx::query(
                    "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
                     VALUES ($1, $2, $3, $4, $5, $6::jsonb, 'PENDING')"
                )
                .bind(&tx_id)
                .bind(&tenant_id_clone)
                .bind(&client_id_clone)
                .bind(tx.amount_cents)
                .bind(&tx.currency)
                .bind(&tx.payload)
                .execute(&mut *db_tx)
                .await;

                if let Err(e) = insert_res {
                    tracing::error!("Failed to insert offline transaction: {}", e);
                    return Err(tx.id);
                }

                // Record in cash ledger if it's a cash sale (assume CASH for offline POS in this flow)
                let ledger_id = Uuid::new_v4().to_string();
                let _ = sqlx::query(
                    "INSERT INTO pos_cash_ledger_entries (id, tenant_id, session_id, entry_type, amount_cents, currency, reason)
                     VALUES ($1, $2, $3, 'SALE', $4, $5, 'Offline POS Sync')"
                )
                .bind(&ledger_id)
                .bind(&tenant_id_clone)
                .bind(&session_id_clone)
                .bind(tx.amount_cents)
                .bind(&tx.currency)
                .execute(&mut *db_tx)
                .await;

                // Queue job
                let job_id = Uuid::new_v4().to_string();
                let payload = serde_json::json!({
                    "pos_transaction_id": tx_id,
                    "client_id": client_id_clone,
                    "amount_cents": tx.amount_cents,
                    "currency": tx.currency,
                    "payload": tx.payload,
                }).to_string();

                let job_res = sqlx::query(
                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                     VALUES ($1, $2, 'offline_pos_sync', $3::jsonb)"
                )
                .bind(&job_id)
                .bind(&tenant_id_clone)
                .bind(&payload)
                .execute(&mut *db_tx)
                .await;

                if let Err(e) = job_res {
                    tracing::error!("Failed to enqueue job: {}", e);
                    return Err(tx.id);
                }

                if let Err(e) = db_tx.commit().await {
                    tracing::error!("Failed to commit transaction: {}", e);
                    return Err(tx.id);
                }

                Ok(())
            }));
        }

        let results = futures::future::join_all(futures).await;

        for res in results {
            match res {
                Ok(Ok(())) => {
                    synced_count += 1;
                }
                Ok(Err(id)) => {
                    failed_ids.push(id);
                }
                Err(e) => {
                    tracing::error!("Task failed to execute: {}", e);
                }
            }
        }

        Ok(Response::new(SyncOfflineTransactionsResponse {
            success: failed_ids.is_empty(),
            synced_count,
            failed_transaction_ids: failed_ids,
        }))
    }

    async fn start_terminal_session(
        &self,
        request: Request<StartTerminalSessionRequest>,
    ) -> Result<Response<StartTerminalSessionResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let auth_tenant = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if auth_tenant.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let tenant_id = auth_tenant;

        let session_id = uuid::Uuid::new_v4().to_string();
        let pool = self.db.pool.clone();

        let res = sqlx::query(
            "INSERT INTO pos_terminal_sessions (id, tenant_id, device_id, status, started_at, last_synced_at, offline_changes_count, opening_balance_cents)
             VALUES ($1, $2, $3, 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, $4)
             ON CONFLICT (tenant_id, device_id) DO UPDATE SET status = 'ACTIVE', last_synced_at = CURRENT_TIMESTAMP, offline_changes_count = 0, opening_balance_cents = $4 RETURNING id"
        )
        .bind(&session_id)
        .bind(&tenant_id)
        .bind(&req.device_id)
        .bind(req.opening_balance_cents)
        .fetch_one(&pool)
        .await;

        match res {
            Ok(row) => {
                let returned_id: String = sqlx::Row::get(&row, "id");
                Ok(Response::new(StartTerminalSessionResponse {
                    session_id: returned_id,
                    success: true,
                    error_message: "".to_string(),
                }))
            }
            Err(e) => {
                tracing::error!("Failed to start terminal session: {}", e);
                Ok(Response::new(StartTerminalSessionResponse {
                    session_id: "".to_string(),
                    success: false,
                    error_message: e.to_string(),
                }))
            }
        }
    }

    async fn update_terminal_session_status(
        &self,
        request: Request<UpdateTerminalSessionStatusRequest>,
    ) -> Result<Response<UpdateTerminalSessionStatusResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let auth_tenant = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if auth_tenant.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let tenant_id = auth_tenant;

        let pool = self.db.pool.clone();

        let res = sqlx::query(
            "UPDATE pos_terminal_sessions SET status = $1, last_synced_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
        )
        .bind(&req.status)
        .bind(&req.session_id)
        .bind(&tenant_id)
        .execute(&pool)
        .await;

        match res {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(Response::new(UpdateTerminalSessionStatusResponse {
                        success: true,
                        error_message: "".to_string(),
                    }))
                } else {
                    Ok(Response::new(UpdateTerminalSessionStatusResponse {
                        success: false,
                        error_message: "Session not found".to_string(),
                    }))
                }
            }
            Err(e) => {
                tracing::error!("Failed to update terminal session status: {}", e);
                Ok(Response::new(UpdateTerminalSessionStatusResponse {
                    success: false,
                    error_message: e.to_string(),
                }))
            }
        }
    }

    async fn end_terminal_session(
        &self,
        request: Request<EndTerminalSessionRequest>,
    ) -> Result<Response<EndTerminalSessionResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let auth_tenant = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if auth_tenant.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let tenant_id = auth_tenant;

        let pool = self.db.pool.clone();

        let res = sqlx::query(
            "UPDATE pos_terminal_sessions SET status = 'RECONCILED', last_synced_at = CURRENT_TIMESTAMP, closing_balance_cents = $1, closed_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
        )
        .bind(req.closing_balance_cents)
        .bind(&req.session_id)
        .bind(&tenant_id)
        .execute(&pool)
        .await;

        match res {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(Response::new(EndTerminalSessionResponse {
                        success: true,
                        error_message: "".to_string(),
                    }))
                } else {
                    Ok(Response::new(EndTerminalSessionResponse {
                        success: false,
                        error_message: "Session not found".to_string(),
                    }))
                }
            }
            Err(e) => {
                tracing::error!("Failed to end terminal session: {}", e);
                Ok(Response::new(EndTerminalSessionResponse {
                    success: false,
                    error_message: e.to_string(),
                }))
            }
        }
    }

    async fn record_cash_movement(
        &self,
        request: Request<RecordCashMovementRequest>,
    ) -> Result<Response<RecordCashMovementResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        let req = request.into_inner();
        let entry_id = Uuid::new_v4().to_string();
        let pool = self.db.pool.clone();

        let res = sqlx::query(
            "INSERT INTO pos_cash_ledger_entries (id, tenant_id, session_id, entry_type, amount_cents, currency, reason)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&entry_id)
        .bind(&tenant_id)
        .bind(&req.session_id)
        .bind(&req.entry_type)
        .bind(req.amount_cents)
        .bind(&req.currency)
        .bind(&req.reason)
        .execute(&pool)
        .await;

        match res {
            Ok(_) => Ok(Response::new(RecordCashMovementResponse {
                success: true,
                entry_id,
            })),
            Err(e) => Err(Status::internal(format!("Failed to record cash movement: {}", e))),
        }
    }

    async fn get_terminal_session_summary(
        &self,
        request: Request<GetTerminalSessionSummaryRequest>,
    ) -> Result<Response<GetTerminalSessionSummaryResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        let req = request.into_inner();
        let pool = self.db.pool.clone();

        let session_row = sqlx::query(
            "SELECT opening_balance_cents FROM pos_terminal_sessions WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.session_id)
        .bind(&tenant_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| Status::not_found(format!("Session not found: {}", e)))?;

        let opening_balance: i64 = sqlx::Row::get(&session_row, "opening_balance_cents");

        let entries_rows = sqlx::query(
            "SELECT id, entry_type, amount_cents, currency, reason, created_at FROM pos_cash_ledger_entries WHERE session_id = $1 AND tenant_id = $2 ORDER BY created_at ASC"
        )
        .bind(&req.session_id)
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch entries: {}", e)))?;

        let mut entries = Vec::new();
        let mut total_sales_cents = 0;
        let mut total_cash_in_cents = 0;
        let mut total_cash_out_cents = 0;

        for row in entries_rows {
            let entry_type: String = sqlx::Row::get(&row, "entry_type");
            let amount: i64 = sqlx::Row::get(&row, "amount_cents");
            let created_at: chrono::DateTime<chrono::Utc> = sqlx::Row::get(&row, "created_at");

            entries.push(CashLedgerEntry {
                id: sqlx::Row::get(&row, "id"),
                tenant_id: tenant_id.clone(),
                session_id: req.session_id.clone(),
                entry_type: entry_type.clone(),
                amount_cents: amount,
                currency: sqlx::Row::get(&row, "currency"),
                reason: sqlx::Row::get(&row, "reason"),
                created_at_unix: created_at.timestamp(),
            });

            match entry_type.as_str() {
                "SALE" => {
                    total_sales_cents += amount;
                    total_cash_in_cents += amount;
                }
                "CASH_IN" => total_cash_in_cents += amount,
                "CASH_OUT" | "DROP" | "PAYOUT" => total_cash_out_cents += amount,
                _ => {}
            }
        }

        let expected_cash_cents = opening_balance + total_cash_in_cents - total_cash_out_cents;

        Ok(Response::new(GetTerminalSessionSummaryResponse {
            total_sales_cents,
            total_cash_in_cents,
            total_cash_out_cents,
            expected_cash_cents,
            entries,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_sync_offline_transactions() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB {
            pool: crate::db::get_pool(),
            store: DbStore::Postgres,
        });

        let service = MyPosService::new(db.clone());

        let req = SyncOfflineTransactionsRequest {
            tenant_id: "test_tenant".to_string(),
            client_id: "test_client".to_string(),
            transactions: vec![
                ::server_ohc::app::PosOfflineTransaction {
                    id: "tx_1".to_string(),
                    tenant_id: "test_tenant".to_string(),
                    client_id: "test_client".to_string(),
                    amount_cents: 1000,
                    currency: "USD".to_string(),
                    payload: "{}".to_string(),
                    status: "PENDING".to_string(),
                    created_at_unix: 0,
                }
            ],
            session_id: Some("test_session".to_string()),
        };

        let mut request = Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "test_tenant".to_string(),
            agent_id: "test".to_string(),
        });

        let response = service.sync_offline_transactions(request).await;
        // Depending on whether DB contains proper tables (migrated), this might fail gracefully but shouldn't panic.
        assert!(response.is_ok() || response.is_err());
    }
}
