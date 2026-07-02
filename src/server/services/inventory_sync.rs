use ::server_ohc::inventory::inventory_sync_service_server::InventorySyncService;
use ::server_ohc::inventory::{ReserveInventoryRequest, ReserveInventoryResponse, CommitInventoryRequest, CommitInventoryResponse, SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse, SyncTransactionResult};

use tonic::{Request, Response, Status};

pub struct MyInventorySyncService {

    redis_client: Option<redis::Client>,
}

impl MyInventorySyncService {
    pub fn new( redis_client: Option<redis::Client>) -> Self {
        Self { redis_client }
    }
}

#[tonic::async_trait]
impl InventorySyncService for MyInventorySyncService {
    async fn reserve_inventory(
        &self,
        request: Request<ReserveInventoryRequest>,
    ) -> Result<Response<ReserveInventoryResponse>, Status> {
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

        let req = request.into_inner();
        let service = crate::services::inventory::InventoryService::new(self.redis_client.clone());

        match service.reserve_inventory(&tenant_id, &req.product_id, req.quantity, req.ttl_seconds).await {
            Ok(result) => {
                Ok(Response::new(ReserveInventoryResponse {
                    success: result.success,
                    lock_id: result.lock_id,
                    error_message: result.error_message,
                }))
            },
            Err(e) => Err(Status::internal(e))
        }
    }

    async fn commit_inventory(
        &self,
        request: Request<CommitInventoryRequest>,
    ) -> Result<Response<CommitInventoryResponse>, Status> {
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

        let req = request.into_inner();
        let service = crate::services::inventory::InventoryService::new(self.redis_client.clone());

        match service.commit_inventory(&tenant_id, &req.product_id, req.quantity, &req.lock_id).await {
            Ok(result) => {
                Ok(Response::new(CommitInventoryResponse {
                    success: result.success,
                    error_message: result.error_message,
                }))
            },
            Err(e) => Err(Status::internal(e))
        }
    }

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

        let req = request.into_inner();
        let service = crate::services::inventory::InventoryService::new(self.redis_client.clone());

        let mut results = Vec::new();
        for tx in req.transactions {
            match service.sync_offline_transaction(&tenant_id, &tx.product_id, tx.quantity, &tx.client_timestamp, &tx.client_transaction_id).await {
                Ok(result) => {
                    results.push(SyncTransactionResult {
                        client_transaction_id: tx.client_transaction_id.clone(),
                        success: result.success,
                        error_message: result.error_message,
                    });
                },
                Err(e) => {
                    results.push(SyncTransactionResult {
                        client_transaction_id: tx.client_transaction_id.clone(),
                        success: false,
                        error_message: e,
                    });
                }
            }
        }

        Ok(Response::new(SyncOfflineTransactionsResponse {
            results,
        }))
    }
}
