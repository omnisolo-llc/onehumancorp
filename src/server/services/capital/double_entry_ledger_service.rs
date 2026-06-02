use tonic::{Request, Response, Status};
use std::sync::Arc;
use sqlx::PgPool;

use crate::services::capital::double_entry_repo::{DoubleEntryRepo, EntryInput};

// Include the generated proto types for ledger
pub mod ledgerpb {
    tonic::include_proto!("ohc.ledger");
}

use ledgerpb::ledger_service_server::LedgerService;
use ledgerpb::{
    RecordTransactionRequest, RecordTransactionResponse,
    GetBalanceRequest, GetBalanceResponse,
    GetStatementRequest, GetStatementResponse,
    LedgerEntry as ProtoLedgerEntry,
};

pub struct MyLedgerService {
    repo: Arc<DoubleEntryRepo>,
}

impl MyLedgerService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            repo: Arc::new(DoubleEntryRepo::new(pool)),
        }
    }
}

#[tonic::async_trait]
impl LedgerService for MyLedgerService {
    async fn record_transaction(
        &self,
        request: Request<RecordTransactionRequest>,
    ) -> Result<Response<RecordTransactionResponse>, Status> {
        let req = request.into_inner();

        let tenant_id = ::server_common::auth_utils::extract_tenant_from_metadata(request.metadata())
            .unwrap_or_else(|| "DEFAULT".to_string());

        let mut inputs = Vec::new();
        for e in req.entries {
            inputs.push(EntryInput {
                account_id: e.account_id,
                amount_cents: e.amount_cents,
                direction: e.direction,
            });
        }

        let desc_opt = if req.description.is_empty() { None } else { Some(req.description) };
        let ref_type_opt = if req.reference_type.is_empty() { None } else { Some(req.reference_type) };
        let ref_id_opt = if req.reference_id.is_empty() { None } else { Some(req.reference_id) };

        match self.repo.record_transaction(
            &tenant_id,
            &req.organization_id,
            &req.currency,
            desc_opt,
            ref_type_opt,
            ref_id_opt,
            inputs,
        ).await {
            Ok(tx_id) => Ok(Response::new(RecordTransactionResponse {
                transaction_id: tx_id,
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = ::server_common::auth_utils::extract_tenant_from_metadata(request.metadata())
            .unwrap_or_else(|| "DEFAULT".to_string());

        match self.repo.get_balance(&tenant_id, &req.organization_id, &req.account_id).await {
            Ok(bal) => Ok(Response::new(GetBalanceResponse {
                balance_cents: bal,
                currency: req.currency,
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn get_statement(
        &self,
        request: Request<GetStatementRequest>,
    ) -> Result<Response<GetStatementResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = ::server_common::auth_utils::extract_tenant_from_metadata(request.metadata())
            .unwrap_or_else(|| "DEFAULT".to_string());

        match self.repo.get_statement(&tenant_id, &req.organization_id, &req.account_id, req.limit, req.offset).await {
            Ok((entries, total_count)) => {
                let mut proto_entries = Vec::new();
                for e in entries {
                    proto_entries.push(ProtoLedgerEntry {
                        entry_id: e.id,
                        transaction_id: e.transaction_id,
                        account_id: e.account_id,
                        amount_cents: e.amount_cents,
                        direction: e.direction,
                        created_at: e.created_at.timestamp_millis(),
                    });
                }
                Ok(Response::new(GetStatementResponse {
                    entries: proto_entries,
                    total_count,
                }))
            }
            Err(e) => Err(Status::internal(e)),
        }
    }
}
