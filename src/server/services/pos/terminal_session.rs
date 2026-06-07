use ::server_ohc::app::terminal_session_service_server::TerminalSessionService;
use ::server_ohc::app::{
    StartTerminalSessionRequest, StartTerminalSessionResponse,
    UpdateTerminalSessionStatusRequest, UpdateTerminalSessionStatusResponse,
    EndTerminalSessionRequest, EndTerminalSessionResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyTerminalSessionService {
    db: Arc<crate::db::DB>,
}

impl MyTerminalSessionService {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl TerminalSessionService for MyTerminalSessionService {
    async fn start_terminal_session(
        &self,
        request: Request<StartTerminalSessionRequest>,
    ) -> Result<Response<StartTerminalSessionResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("missing tenant identity")),
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity"));
        }

        let req = request.into_inner();
        let session_id = Uuid::new_v4().to_string();

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // UPSERT session
        let res = sqlx::query(
            "INSERT INTO pos_terminal_sessions (session_id, tenant_id, hardware_id, status, last_synced_at)
             VALUES ($1, $2, $3, 'active', CURRENT_TIMESTAMP)
             ON CONFLICT (tenant_id, hardware_id) DO UPDATE SET session_id = $1, status = 'active', last_synced_at = CURRENT_TIMESTAMP"
        )
        .bind(&session_id)
        .bind(&tenant_id)
        .bind(&req.hardware_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            return Err(Status::internal(format!("DB error: {}", e)));
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(StartTerminalSessionResponse {
            session_id,
            status: "active".to_string(),
        }))
    }

    async fn update_terminal_session_status(
        &self,
        request: Request<UpdateTerminalSessionStatusRequest>,
    ) -> Result<Response<UpdateTerminalSessionStatusResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("missing tenant identity")),
        };

        let req = request.into_inner();

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let res = sqlx::query(
            "UPDATE pos_terminal_sessions SET status = $1, last_synced_at = CURRENT_TIMESTAMP WHERE session_id = $2 AND tenant_id = $3"
        )
        .bind(&req.status)
        .bind(&req.session_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            return Err(Status::internal(format!("DB error: {}", e)));
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateTerminalSessionStatusResponse {
            success: true,
        }))
    }

    async fn end_terminal_session(
        &self,
        request: Request<EndTerminalSessionRequest>,
    ) -> Result<Response<EndTerminalSessionResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("missing tenant identity")),
        };

        let req = request.into_inner();

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let res = sqlx::query(
            "UPDATE pos_terminal_sessions SET status = 'offline', last_synced_at = CURRENT_TIMESTAMP WHERE session_id = $1 AND tenant_id = $2"
        )
        .bind(&req.session_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            return Err(Status::internal(format!("DB error: {}", e)));
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EndTerminalSessionResponse {
            success: true,
        }))
    }
}
