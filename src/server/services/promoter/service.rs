use std::sync::Arc;
use crate::db::DB;
use crate::ohc::app::promoter_service_server::PromoterService;
use crate::ohc::app::{ApproveSeoRequest, ApproveSeoResponse};
use tonic::{Request, Response, Status};

pub struct MyPromoterService {
    db: Arc<DB>,
}

impl MyPromoterService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl PromoterService for MyPromoterService {
    async fn approve_seo(
        &self,
        request: Request<ApproveSeoRequest>,
    ) -> Result<Response<ApproveSeoResponse>, Status> {
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
        let pool = crate::db::get_pool();

        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => return Err(Status::internal(format!("Failed to begin tx: {}", e))),
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
            return Err(Status::internal(format!("Failed to set org context: {}", e)));
        }

        let _ = sqlx::query(
            "UPDATE ohc_seo_metadata SET status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.seo_metadata_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update status: {}", e)))?;

        let _ = tx.commit().await;

        Ok(Response::new(ApproveSeoResponse { success: true }))
    }
}
