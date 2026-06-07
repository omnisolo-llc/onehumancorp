use ::server_ohc::app::pos_service_server::PosService;
use ::server_ohc::app::{
    SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse,
    TerminalSession, StartTerminalSessionRequest,
    UpdateTerminalSessionStatusRequest, EndTerminalSessionRequest
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use sqlx::Row;

pub struct MyPosService {}

impl MyPosService {
    pub fn new(_db: Arc<crate::db::DB>) -> Self {
        Self { }
    }
}

#[tonic::async_trait]
impl PosService for MyPosService {
    async fn start_terminal_session(
        &self,
        request: Request<StartTerminalSessionRequest>,
    ) -> Result<Response<TerminalSession>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("unauthenticated")),
        };

        let req = request.into_inner();
        let session_id = Uuid::new_v4().to_string();
        let hardware_id = req.hardware_id;
        let pool = crate::db::get_pool();

        let row = sqlx::query(
            "INSERT INTO pos_terminal_sessions (id, tenant_id, hardware_id, status)
             VALUES ($1, $2, $3, 'ACTIVE')
             ON CONFLICT (tenant_id, hardware_id)
             DO UPDATE SET status = 'ACTIVE', started_at = CURRENT_TIMESTAMP, last_synced_at = CURRENT_TIMESTAMP, offline_changes_count = 0
             RETURNING id, tenant_id, hardware_id, status, started_at, last_synced_at, offline_changes_count"
        )
        .bind(&session_id)
        .bind(&tenant_id)
        .bind(&hardware_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(TerminalSession {
            session_id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            hardware_id: row.get("hardware_id"),
            status: row.get("status"),
            started_at_unix: row.get::<Option<DateTime<Utc>>, _>("started_at").map(|t| t.timestamp()).unwrap_or(0),
            last_synced_at_unix: row.get::<Option<DateTime<Utc>>, _>("last_synced_at").map(|t| t.timestamp()).unwrap_or(0),
            offline_changes_count: row.get::<Option<i32>, _>("offline_changes_count").unwrap_or(0),
        }))
    }

    async fn update_terminal_session_status(
        &self,
        request: Request<UpdateTerminalSessionStatusRequest>,
    ) -> Result<Response<TerminalSession>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("unauthenticated")),
        };

        let req = request.into_inner();
        let status = req.status.to_uppercase();
        if !["ACTIVE", "PAUSED", "OFFLINE", "RECONCILED"].contains(&status.as_str()) {
            return Err(Status::invalid_argument("invalid status: must be ACTIVE, PAUSED, OFFLINE, or RECONCILED"));
        }

        let pool = crate::db::get_pool();

        let row = sqlx::query(
            "UPDATE pos_terminal_sessions SET status = $1, last_synced_at = CURRENT_TIMESTAMP
             WHERE id = $2 AND tenant_id = $3
             RETURNING id, tenant_id, hardware_id, status, started_at, last_synced_at, offline_changes_count"
        )
        .bind(&status)
        .bind(&req.session_id)
        .bind(&tenant_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| Status::not_found("session not found for this tenant"))?;

        Ok(Response::new(TerminalSession {
            session_id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            hardware_id: row.get("hardware_id"),
            status: row.get("status"),
            started_at_unix: row.get::<Option<DateTime<Utc>>, _>("started_at").map(|t| t.timestamp()).unwrap_or(0),
            last_synced_at_unix: row.get::<Option<DateTime<Utc>>, _>("last_synced_at").map(|t| t.timestamp()).unwrap_or(0),
            offline_changes_count: row.get::<Option<i32>, _>("offline_changes_count").unwrap_or(0),
        }))
    }

    async fn end_terminal_session(
        &self,
        request: Request<EndTerminalSessionRequest>,
    ) -> Result<Response<TerminalSession>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("unauthenticated")),
        };

        let req = request.into_inner();
        let pool = crate::db::get_pool();

        let row = sqlx::query(
            "UPDATE pos_terminal_sessions SET status = 'RECONCILED', last_synced_at = CURRENT_TIMESTAMP
             WHERE id = $1 AND tenant_id = $2
             RETURNING id, tenant_id, hardware_id, status, started_at, last_synced_at, offline_changes_count"
        )
        .bind(&req.session_id)
        .bind(&tenant_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| Status::not_found("session not found for this tenant"))?;

        Ok(Response::new(TerminalSession {
            session_id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            hardware_id: row.get("hardware_id"),
            status: row.get("status"),
            started_at_unix: row.get::<Option<DateTime<Utc>>, _>("started_at").map(|t| t.timestamp()).unwrap_or(0),
            last_synced_at_unix: row.get::<Option<DateTime<Utc>>, _>("last_synced_at").map(|t| t.timestamp()).unwrap_or(0),
            offline_changes_count: row.get::<Option<i32>, _>("offline_changes_count").unwrap_or(0),
        }))
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

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();
        let client_id = req.client_id;
        let session_id = req.session_id;

        let pool = crate::db::get_pool();

        // Update session sync info if session_id is provided
        if !session_id.is_empty() {
            let _ = sqlx::query(
                "UPDATE pos_terminal_sessions SET last_synced_at = CURRENT_TIMESTAMP, offline_changes_count = offline_changes_count + $1 WHERE id = $2 AND tenant_id = $3"
            )
            .bind(req.transactions.len() as i32)
            .bind(&session_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await;
        }

        let mut synced_count = 0;
        let mut failed_ids = Vec::new();

        for tx in req.transactions {
            let tenant_id_clone = tenant_id.clone();
            let client_id_clone = client_id.clone();
            let session_id_clone = session_id.clone();
            let tx_id = if tx.id.is_empty() { Uuid::new_v4().to_string() } else { tx.id.clone() };

            let mut db_tx = match pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {}", e);
                    failed_ids.push(tx.id);
                    continue;
                }
            };

            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id_clone).await {
                tracing::error!("Failed to set org context: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            let insert_res = sqlx::query(
                "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status, session_id)
                 VALUES ($1, $2, $3, $4, $5, $6::jsonb, 'PENDING', $7)
                 ON CONFLICT (id) DO NOTHING"
            )
            .bind(&tx_id)
            .bind(&tenant_id_clone)
            .bind(&client_id_clone)
            .bind(tx.amount_cents)
            .bind(&tx.currency)
            .bind(&tx.payload)
            .bind(&session_id_clone)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = insert_res {
                tracing::error!("Failed to insert offline transaction: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            // Queue job
            let job_id = Uuid::new_v4().to_string();
            let job_payload = serde_json::json!({
                "pos_transaction_id": tx_id,
                "client_id": client_id_clone,
                "amount_cents": tx.amount_cents,
                "currency": tx.currency,
                "payload": tx.payload,
                "session_id": session_id_clone,
            }).to_string();

            let job_res = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                 VALUES ($1, $2, 'offline_pos_sync', $3::jsonb)"
            )
            .bind(&job_id)
            .bind(&tenant_id_clone)
            .bind(&job_payload)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = job_res {
                tracing::error!("Failed to enqueue job: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            if let Err(e) = db_tx.commit().await {
                tracing::error!("Failed to commit transaction: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            synced_count += 1;
        }

        Ok(Response::new(SyncOfflineTransactionsResponse {
            success: failed_ids.is_empty(),
            synced_count,
            failed_transaction_ids: failed_ids,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::db::DbStore;

    fn setup_test_context<T>(req: &mut Request<T>, tenant_id: &str) {
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: format!("spiffe://ohc/org/{}/agent/test", tenant_id),
            org_id: tenant_id.to_string(),
            agent_id: "test".to_string(),
        });
    }

    #[tokio::test]
    async fn test_terminal_session_lifecycle() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB {
            pool: crate::db::get_pool(),
            store: DbStore::Postgres,
        });

        let service = MyPosService::new(db.clone());
        let tenant_id = "test_tenant_lifecycle";

        // 1. Start Session
        let mut start_req = Request::new(StartTerminalSessionRequest {
            tenant_id: tenant_id.to_string(),
            hardware_id: "hw_123".to_string(),
        });
        setup_test_context(&mut start_req, tenant_id);

        let start_res = service.start_terminal_session(start_req).await.unwrap().into_inner();
        assert_eq!(start_res.status, "ACTIVE");
        assert_eq!(start_res.hardware_id, "hw_123");
        let session_id = start_res.session_id;

        // 2. Update Status
        let mut update_req = Request::new(UpdateTerminalSessionStatusRequest {
            session_id: session_id.clone(),
            status: "PAUSED".to_string(),
        });
        setup_test_context(&mut update_req, tenant_id);
        let update_res = service.update_terminal_session_status(update_req).await.unwrap().into_inner();
        assert_eq!(update_res.status, "PAUSED");

        // 3. End Session
        let mut end_req = Request::new(EndTerminalSessionRequest {
            session_id: session_id.clone(),
        });
        setup_test_context(&mut end_req, tenant_id);
        let end_res = service.end_terminal_session(end_req).await.unwrap().into_inner();
        assert_eq!(end_res.status, "RECONCILED");
    }

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
        let tenant_id = "test_tenant_sync";

        let mut req = Request::new(SyncOfflineTransactionsRequest {
            tenant_id: tenant_id.to_string(),
            client_id: "test_client".to_string(),
            transactions: vec![
                PosOfflineTransaction {
                    id: format!("tx_{}", Uuid::new_v4()),
                    tenant_id: tenant_id.to_string(),
                    client_id: "test_client".to_string(),
                    amount_cents: 1000,
                    currency: "USD".to_string(),
                    payload: "{}".to_string(),
                    status: "PENDING".to_string(),
                    created_at_unix: 0,
                    session_id: "session_1".to_string(),
                }
            ],
            session_id: "session_1".to_string(),
        });
        setup_test_context(&mut req, tenant_id);

        let response = service.sync_offline_transactions(req).await.unwrap().into_inner();
        assert!(response.success);
        assert_eq!(response.synced_count, 1);
    }
}
